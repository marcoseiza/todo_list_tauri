use firebase_rs::RequestError;
use serde::Serialize;
use tokio::sync::{MappedMutexGuard, Mutex, MutexGuard};

use crate::database::board::Board;
use crate::database::reset::Reset;
use crate::oauth::sign_in_with_oauth;

#[derive(Serialize, Clone, Default, Debug)]
pub struct User {
    pub avatar_url: String,
    pub html_url: String,
    pub firebase_auth_token: String,
    pub firebase_uid: String,
    pub full_name: String,
    pub board: Board,
}

impl User {
    pub fn from_oauth_response(body: &sign_in_with_oauth::ResponseBody) -> Self {
        Self {
            avatar_url: body.rawUserInfo.avatar_url.clone(),
            html_url: body.rawUserInfo.html_url.clone(),
            firebase_auth_token: body.idToken.clone(),
            firebase_uid: body.localId.clone(),
            full_name: body.fullName.clone().unwrap_or("Unkown".into()),
            ..User::default()
        }
    }

    pub async fn save(self: &Self) -> Result<(), RequestError> {
        let db_uri = "https://todo-list-474ef-default-rtdb.firebaseio.com/";
        let db = firebase_rs::Firebase::auth(db_uri, &self.firebase_auth_token).unwrap();
        db.at("users")
            .at(&self.firebase_uid)
            .at("board")
            .put(&self.board)
            .await?;
        Ok(())
    }

    pub async fn load(self: &mut Self) -> Result<Board, RequestError> {
        let db_uri = "https://todo-list-474ef-default-rtdb.firebaseio.com/";
        let db = firebase_rs::Firebase::auth(db_uri, &self.firebase_auth_token).unwrap();
        self.board = db
            .at("users")
            .at(&self.firebase_uid)
            .at("board")
            .get::<Board>()
            .await
            .unwrap_or_default();
        Ok(self.board.clone())
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
