use firebase_rs::RequestError;

use crate::database::{
    Board, {parse_user_state, parse_user_state_expected, User, UserState},
};
use crate::helpers::stringify_error;
use crate::secure_storage::refresh_token;

#[tauri::command]
pub async fn save(app: tauri::AppHandle, state: tauri::State<'_, UserState>) -> Result<(), String> {
    let mut user = parse_user_state_expected(&state).await;
    match user.save().await {
        Ok(_) => Ok(()),
        Err(_) => {
            // Fallback if access_token didn't work.
            let bundle_identifier = app.config().tauri.bundle.identifier.clone();
            let refresh_token =
                refresh_token::read(bundle_identifier.clone()).map_err(stringify_error)?;
            let new_refresh_token = user
                .refresh_access_token(refresh_token)
                .await
                .map_err(stringify_error)?;
            refresh_token::write(new_refresh_token, bundle_identifier).map_err(stringify_error)?;
            user.save().await.map_err(stringify_error)
        }
    }
}

#[tauri::command]
pub async fn load(state: tauri::State<'_, UserState>) -> Result<Board, RequestError> {
    let mut user = parse_user_state_expected(&state).await;
    user.load().await
}

#[tauri::command]
pub async fn get_user(state: tauri::State<'_, UserState>) -> Result<Option<User>, ()> {
    let user = parse_user_state(&state).await;
    Ok((*user).clone())
}

#[tauri::command]
pub async fn refresh_user_token(
    app: tauri::AppHandle,
    state: tauri::State<'_, UserState>,
) -> Result<(), String> {
    let bundle_identifier = app.config().tauri.bundle.identifier.clone();
    let mut user = parse_user_state_expected(&state).await;

    let refresh_token = refresh_token::read(bundle_identifier.clone())
        .map_err(|_| String::from("ReadRefreshTokenError"))?;

    let refresh_token = user
        .refresh_access_token(refresh_token)
        .await
        .map_err(|_| String::from("RefreshTokenError"))?;

    refresh_token::write(refresh_token, bundle_identifier)
        .map_err(|_| String::from("WriteRefreshTokenError"))?;
    Ok(())
}
