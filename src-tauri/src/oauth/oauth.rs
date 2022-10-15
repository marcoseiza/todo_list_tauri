use std::env;

use oauth2::basic::BasicErrorResponseType;
use oauth2::reqwest::async_http_client;
use oauth2::{CsrfToken, Scope, TokenResponse};
use oauth2::{RequestTokenError, StandardErrorResponse};
use reqwest;
use serde::Serialize;
use serde_json::{json, Value};

use crate::oauth::github_client::parse_github_client_state;
use crate::oauth::oauth_helper::{listen_for_code, ListenForCodeError, ReceivedCode};

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
    VarError(#[from] env::VarError),
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
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
) -> anyhow::Result<(), GetGithubTokenError> {
    let ghclient = parse_github_client_state(&ghclient_state).await;

    // Generate the authorization URL to which we'll redirect the user.
    let (authorize_url, csrf_state) = ghclient
        .authorize_url(CsrfToken::new_random)
        // This example is requesting access to the user's public repos and email.
        .add_scope(Scope::new("public_repo".to_string()))
        .add_scope(Scope::new("user:email".to_string()))
        .url();

    let window = tauri::WindowBuilder::new(
        &app,
        format!("github_auth_{}", csrf_state.secret()),
        tauri::WindowUrl::External(authorize_url),
    )
    .build()
    .unwrap();

    let ReceivedCode { code, state } = listen_for_code(8080).await?;

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

    let access_token = token_res.access_token().secret();

    window.close()?;

    let reqclient = reqwest::Client::new();
    let _response = reqclient
        .post(format!(
            "https://identitytoolkit.googleapis.com/v1/accounts:signInWithIdp?key={}",
            env::var("FIREBASE_WEB_API_KEY")?
        ))
        .json(&json!({
            "postBody": format!("access_token={}&providerId=github.com", access_token),
            "requestUri": "http://localhost",
            "returnIdpCredential":true,
            "returnSecureToken":true
        }))
        .send()
        .await?
        .json::<Value>()
        .await?;

    Ok(())
}
