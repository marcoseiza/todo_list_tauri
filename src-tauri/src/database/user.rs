use firebase_rs::RequestError;
use serde::{Deserialize, Serialize};
use tokio::sync::{MappedMutexGuard, Mutex, MutexGuard};

use crate::database::board::Board;
use crate::database::reset::Reset;
use crate::firebase_interface::refresh_token;
use crate::firebase_interface::sign_in_with_oauth;

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
    pub board: Board,
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
        db.at("users").at(&self.firebase_uid).put(self).await?;
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
        let user_response = db.at("users").at(&self.firebase_uid).get::<User>().await;
        let user = user_response.as_ref();
        self.board = user.map_or_else(|_| Board::default(), |u| u.board.clone());
        self.user_info = user.map_or_else(|_| GithubUserInfo::default(), |u| u.user_info.clone());
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
