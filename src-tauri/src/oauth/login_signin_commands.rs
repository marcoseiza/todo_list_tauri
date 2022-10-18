use oauth2::reqwest::async_http_client;
use oauth2::{CsrfToken, TokenResponse};

use super::github_client::make_github_client;
use crate::database::{parse_user_state, User, UserState};
use crate::firebase_interface::sign_in_with_oauth;
use crate::helpers::stringify_error;
use crate::oauth::callback_server::{listen_for_code, ReceivedCode};
use crate::secure_storage::refresh_token;

#[tauri::command]
pub async fn sign_up_with_github(
    app: tauri::AppHandle,
    user_state: tauri::State<'_, UserState>,
) -> Result<User, String> {
    let ghclient = make_github_client().map_err(stringify_error)?;

    // Generate the authorization URL to which we'll redirect the user.
    let (authorize_url, csrf_state) = ghclient.authorize_url(CsrfToken::new_random).url();

    let code_future = listen_for_code(8080);

    let window = tauri::WindowBuilder::new(
        &app,
        format!("github_auth_{}", csrf_state.secret()),
        tauri::WindowUrl::External(authorize_url),
    )
    .title("Github Authentication")
    .build()
    .unwrap();

    let ReceivedCode { code, state } = code_future.await.map_err(stringify_error)?;

    if state.secret() != csrf_state.secret() {
        return Err(format!(
            "expected={}, \nactual={}",
            csrf_state.secret(),
            state.secret()
        ));
    };

    let token_res = ghclient
        .exchange_code(code)
        .request_async(async_http_client)
        .await
        .map_err(stringify_error)?;

    let access_token = token_res.access_token().secret().to_string();

    window.close().map_err(stringify_error)?;

    let response = sign_in_with_oauth::get_response(access_token)
        .await
        .map_err(stringify_error)?;

    let mut user_guard = parse_user_state(&user_state).await;
    *user_guard = Some(User::from_oauth_response(&response));
    let user = (*user_guard).as_mut().unwrap();
    user.save_only_user_info().await.map_err(stringify_error)?;
    let _ = user.load_only_board().await;

    let bundle_identifier = app.config().tauri.bundle.identifier.clone();
    refresh_token::write(response.refreshToken.clone(), bundle_identifier)?;

    Ok(user.clone())
}

#[tauri::command]
pub async fn login(
    app: tauri::AppHandle,
    user_state: tauri::State<'_, UserState>,
) -> Result<User, String> {
    let bundle_identifier = app.config().tauri.bundle.identifier.clone();
    let refresh_token = refresh_token::read(bundle_identifier.clone())?;
    let mut user_guard = parse_user_state(&user_state).await;
    *user_guard = Some(User::default());
    let user = (*user_guard).as_mut().unwrap();
    let new_refresh_token = user
        .refresh_access_token(refresh_token.to_string())
        .await
        .map_err(stringify_error)?;
    user.load().await.map_err(stringify_error)?;

    refresh_token::write(new_refresh_token, bundle_identifier)?;
    Ok(user.clone())
}
