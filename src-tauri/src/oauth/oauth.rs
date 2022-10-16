use firebase_rs;
use oauth2::basic::BasicErrorResponseType;
use oauth2::reqwest::async_http_client;
use oauth2::{CsrfToken, Scope, TokenResponse};
use oauth2::{RequestTokenError, StandardErrorResponse};
use reqwest;
use serde::Serialize;
use tokio::sync::MutexGuard;

use crate::database::board::Board;
use crate::database::user::{parse_user_state, User, UserState};
use crate::oauth::github_client::parse_github_client_state;
use crate::oauth::oauth_helper::{listen_for_code, ListenForCodeError, ReceivedCode};
use crate::oauth::sign_in_with_oauth;

use super::github_client::GithubClient;

#[derive(thiserror::Error, Debug)]
pub enum GetGithubTokenError {
    #[error(transparent)]
    ListenForCodeError(#[from] ListenForCodeError),
    #[error("Github returned the following state:\n{expected:?} (expected `{returned:?}`)")]
    CsrfTokenMismatch { expected: String, returned: String },
    #[error(transparent)]
    RequestTokenError(
        #[from]
        RequestTokenError<
            oauth2::reqwest::Error<reqwest::Error>,
            StandardErrorResponse<BasicErrorResponseType>,
        >,
    ),
    #[error(transparent)]
    TauriError(#[from] tauri::Error),
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
    #[error(transparent)]
    FirebaseUrlParseError(#[from] firebase_rs::errors::UrlParseError),
    #[error(transparent)]
    FirebaseRequestError(#[from] firebase_rs::errors::RequestError),
}

impl Serialize for GetGithubTokenError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

#[tauri::command]
pub async fn login_with_github(
    app: tauri::AppHandle,
    ghclient_state: tauri::State<'_, GithubClient>,
    user_state: tauri::State<'_, UserState>,
) -> anyhow::Result<User, GetGithubTokenError> {
    let ghclient = parse_github_client_state(&ghclient_state).await;

    // Generate the authorization URL to which we'll redirect the user.
    let (authorize_url, csrf_state) = ghclient
        .authorize_url(CsrfToken::new_random)
        // This example is requesting access to the user's public repos and email.
        .add_scope(Scope::new("public_repo".to_string()))
        .add_scope(Scope::new("user:email".to_string()))
        .url();

    let code_future = listen_for_code(8080);

    let window = tauri::WindowBuilder::new(
        &app,
        format!("github_auth_{}", csrf_state.secret()),
        tauri::WindowUrl::External(authorize_url),
    )
    .build()
    .unwrap();

    let ReceivedCode { code, state } = code_future.await?;

    if state.secret() != csrf_state.secret() {
        return Err(GetGithubTokenError::CsrfTokenMismatch {
            expected: csrf_state.secret().to_string(),
            returned: state.secret().to_string(),
        });
    }

    let token_res = ghclient
        .exchange_code(code)
        .request_async(async_http_client)
        .await?;

    let access_token = token_res.access_token().secret().to_string();

    window.close()?;

    let mut user: MutexGuard<User> = {
        // Log in user
        let reqclient = reqwest::Client::new();
        let response = reqclient
            .post(sign_in_with_oauth::get_post_url())
            .json(&sign_in_with_oauth::RequestBody::make_body_for_github(
                access_token,
            ))
            .send()
            .await?
            .json::<sign_in_with_oauth::ResponseBody>()
            .await?;

        let mut user = parse_user_state(&user_state).await;
        *user = User::from_oauth_response(response);
        user
    };

    {
        // Get save state.
        let db_uri = "https://todo-list-474ef-default-rtdb.firebaseio.com/";
        let db = firebase_rs::Firebase::auth(db_uri, &user.firebase_auth_token).unwrap();
        let board = db
            .at("users")
            .at(&user.firebase_uid)
            .at("board")
            .get::<Board>()
            .await?;
        (*user).board = board;
    }

    Ok((*user).clone())
}
