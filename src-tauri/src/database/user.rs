use cocoon::Cocoon;
use firebase_rs::RequestError;
use serde::{Deserialize, Serialize};
use tokio::sync::{MappedMutexGuard, Mutex, MutexGuard};

use crate::database::crypto::Secure;
use crate::database::{Board, EncryptedBoard, Reset};
use crate::firebase_interface::{refresh_token, sign_in_with_oauth};

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct GithubUserInfo {
    pub avatar_url: String,
    pub html_url: String,
    pub full_name: String,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct User {
    #[serde(skip)]
    pub firebase_auth_token: String,
    #[serde(skip)]
    pub firebase_uid: String,
    #[serde(default)]
    pub board: Board,
    pub user_info: GithubUserInfo,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct EncryptedUser {
    #[serde(skip)]
    pub firebase_auth_token: String,
    #[serde(skip)]
    pub firebase_uid: String,
    #[serde(default)]
    pub board: Option<EncryptedBoard>,
    pub user_info: GithubUserInfo,
}

impl User {
    pub fn from_oauth_response(body: &sign_in_with_oauth::ResponseBody) -> Self {
        Self {
            firebase_auth_token: body.idToken.clone(),
            firebase_uid: body.localId.clone(),
            user_info: (GithubUserInfo {
                avatar_url: body.rawUserInfo.avatar_url.clone(),
                html_url: body.rawUserInfo.html_url.clone(),
                full_name: body.fullName.clone().unwrap_or_else(|| "Unkown".into()),
            }),
            ..User::default()
        }
    }

    pub async fn save(&self) -> Result<(), RequestError> {
        let db_uri = "https://todo-list-474ef-default-rtdb.firebaseio.com/";
        let db = firebase_rs::Firebase::auth(db_uri, &self.firebase_auth_token).unwrap();
        let encrypted_user = self.encrypt(&Cocoon::new(self.firebase_uid.as_bytes()));
        db.at("users")
            .at(&self.firebase_uid)
            .put(&encrypted_user)
            .await?;
        Ok(())
    }

    pub async fn save_only_user_info(&self) -> Result<(), RequestError> {
        let db_uri = "https://todo-list-474ef-default-rtdb.firebaseio.com/";
        let db = firebase_rs::Firebase::auth(db_uri, &self.firebase_auth_token).unwrap();
        db.at("users")
            .at(&self.firebase_uid)
            .at("user_info")
            .put(&self.user_info)
            .await?;
        Ok(())
    }

    pub async fn load(&mut self) -> Result<Board, RequestError> {
        let db_uri = "https://todo-list-474ef-default-rtdb.firebaseio.com/";
        let db = firebase_rs::Firebase::auth(db_uri, &self.firebase_auth_token).unwrap();
        let user_response = db
            .at("users")
            .at(&self.firebase_uid)
            .get::<EncryptedUser>()
            .await;

        let cocoon = Cocoon::new(self.firebase_uid.as_bytes());

        let user = user_response.as_ref().map(|u| User::decrypt(&cocoon, u));
        self.board = user.clone().map_or_else(|_| Board::default(), |u| u.board);
        self.user_info = user.map_or_else(|_| GithubUserInfo::default(), |u| u.user_info);
        Ok(self.board.clone())
    }

    pub async fn load_only_board(&mut self) -> Result<Board, RequestError> {
        let db_uri = "https://todo-list-474ef-default-rtdb.firebaseio.com/";
        let db = firebase_rs::Firebase::auth(db_uri, &self.firebase_auth_token).unwrap();
        let board = db
            .at("users")
            .at(&self.firebase_uid)
            .at("board")
            .get::<Board>()
            .await?;
        self.board = board;
        Ok(self.board.clone())
    }

    pub async fn refresh_access_token(&mut self, refresh_token: String) -> anyhow::Result<String> {
        let response = reqwest::Client::new()
            .post(refresh_token::get_post_url())
            .form(&refresh_token::RequestBody::new(refresh_token))
            .send()
            .await?
            .json::<refresh_token::ResponseBody>()
            .await?;

        self.firebase_auth_token = response.id_token;
        self.firebase_uid = response.user_id;
        Ok(response.refresh_token)
    }
}

impl Reset for User {
    fn reset(&mut self) {
        self.board.reset();
    }
}

impl Secure for User {
    type Encrypted = EncryptedUser;

    fn encrypt(&self, cocoon: &cocoon::Cocoon<cocoon::Creation>) -> Self::Encrypted {
        Self::Encrypted {
            board: Some(self.board.encrypt(cocoon)),
            firebase_auth_token: self.firebase_auth_token.clone(),
            firebase_uid: self.firebase_uid.clone(),
            user_info: self.user_info.clone(),
        }
    }

    fn decrypt(cocoon: &cocoon::Cocoon<cocoon::Creation>, container: &Self::Encrypted) -> Self {
        let board = if let Some(container_board) = &container.board {
            Board::decrypt(cocoon, container_board)
        } else {
            Board::default()
        };
        Self {
            board,
            firebase_auth_token: container.firebase_auth_token.clone(),
            firebase_uid: container.firebase_uid.clone(),
            user_info: container.user_info.clone(),
        }
    }
}

pub struct UserState(pub Mutex<Option<User>>);

pub async fn parse_user_state<'a>(
    state: &'a tauri::State<'a, UserState>,
) -> MutexGuard<Option<User>> {
    state.0.lock().await
}

pub async fn parse_user_state_expected<'a>(
    state: &'a tauri::State<'a, UserState>,
) -> MappedMutexGuard<User> {
    MutexGuard::map(state.0.lock().await, |f| {
        f.as_mut().expect("Expect user to be logged in")
    })
}
