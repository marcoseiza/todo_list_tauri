use serde::Serialize;
use tokio::sync::{Mutex, MutexGuard};

use crate::database::board::Board;
use crate::database::reset::Reset;
use crate::oauth::sign_in_with_oauth;

#[derive(Serialize, Clone, Default, Debug)]
pub struct User {
    pub firebase_auth_token: String,
    pub firebase_uid: String,
    pub full_name: String,
    pub board: Board,
}

impl User {
    pub fn from_oauth_response(body: sign_in_with_oauth::ResponseBody) -> Self {
        Self {
            firebase_auth_token: body.idToken,
            firebase_uid: body.localId,
            full_name: body.fullName.unwrap_or("Unkown".into()),
            ..User::default()
        }
    }
}

impl Reset for User {
    fn reset(&mut self) {
        self.board.reset();
    }
}

pub struct UserState(pub Mutex<User>);

pub async fn parse_user_state<'a>(state: &'a tauri::State<'a, UserState>) -> MutexGuard<'a, User> {
    state.0.lock().await
}
