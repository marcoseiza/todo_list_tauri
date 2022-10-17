use std::str::FromStr;

use firebase_rs::RequestError;

use crate::database::board::Board;
use crate::database::color::Color;
use crate::database::group::Group;
use crate::database::reset::Reset;
use crate::database::task::Task;
use crate::database::user::{parse_user_state, parse_user_state_expected, User, UserState};
use crate::helpers::{
    find_group, find_group_pos, find_group_with_name, find_task, find_task_or_create,
    remove_task_from_group, stringify_error, trim_whitespace,
};
use crate::secure_storage;

#[tauri::command]
pub async fn save(app: tauri::AppHandle, state: tauri::State<'_, UserState>) -> Result<(), String> {
    let mut user = parse_user_state_expected(&state).await;
    match user.save().await {
        Ok(_) => Ok(()),
        Err(_) => {
            // Fallback if access_token didn't work.
            let bundle_identifier = app.config().tauri.bundle.identifier.clone();
            let refresh_token = secure_storage::refresh_token::read(bundle_identifier.clone())
                .map_err(stringify_error)?;
            let new_refresh_token = user
                .refresh_access_token(refresh_token)
                .await
                .map_err(stringify_error)?;
            secure_storage::refresh_token::write(new_refresh_token, bundle_identifier)
                .map_err(stringify_error)?;
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

    let refresh_token = secure_storage::refresh_token::read(bundle_identifier.clone())
        .map_err(|_| String::from("ReadRefreshTokenError"))?;

    let refresh_token = user
        .refresh_access_token(refresh_token)
        .await
        .map_err(|_| String::from("RefreshTokenError"))?;

    secure_storage::refresh_token::write(refresh_token, bundle_identifier)
        .map_err(|_| String::from("WriteRefreshTokenError"))?;
    Ok(())
}

#[tauri::command]
pub async fn get_board(state: tauri::State<'_, UserState>) -> Result<Board, ()> {
    match &*parse_user_state(&state).await {
        Some(user) => Ok(user.board.clone()),
        None => Ok(Board::default()),
    }
}

#[tauri::command]
pub async fn add_group(name: String, state: tauri::State<'_, UserState>) -> Result<(), ()> {
    let mut user = parse_user_state_expected(&state).await;

    if find_group_with_name(&name, &mut user.board).is_some() {
        return Err(());
    }
    (*user).board.groups.push(Group::default_from(name));
    Ok(())
}

#[tauri::command]
pub async fn remove_group(group_id: String, state: tauri::State<'_, UserState>) -> Result<(), ()> {
    let mut user = parse_user_state_expected(&state).await;

    let group_pos = find_group_pos(group_id.into(), &user.board).expect("Group exists");
    (*user).board.groups.remove(group_pos);
    Ok(())
}

#[tauri::command]
pub async fn update_group_pos(
    group_id: String,
    neighbor_group_id: Option<String>,
    state: tauri::State<'_, UserState>,
) -> Result<(), ()> {
    let mut user = parse_user_state_expected(&state).await;

    let group_pos = find_group_pos(group_id.into(), &user.board).expect("Group exists");
    let group = (*user).board.groups.remove(group_pos);
    let neighbor_pos = neighbor_group_id
        .map(|str| str.into())
        .and_then(|id| find_group_pos(id, &user.board))
        .unwrap_or((*user).board.groups.len());
    (*user).board.groups.insert(neighbor_pos, group);
    Ok(())
}

#[tauri::command]
pub async fn update_group_name(
    group_id: String,
    name: String,
    state: tauri::State<'_, UserState>,
) -> Result<(), ()> {
    let mut user = parse_user_state_expected(&state).await;
    let group_pos = find_group_pos(group_id.into(), &user.board).expect("Group exists");

    let similar_group_pos = find_group_with_name(&name, &mut user.board);
    if similar_group_pos.is_some() && similar_group_pos.unwrap() != group_pos {
        return Err(());
    }

    (*user).board.groups[group_pos].name = name;
    Ok(())
}

#[tauri::command]
pub async fn update_group_color(
    group_id: String,
    color: String,
    state: tauri::State<'_, UserState>,
) -> Result<(), ()> {
    let mut user = parse_user_state_expected(&state).await;

    let color: Color = Color::from_str(&*color).unwrap_or_default();
    let group_pos = find_group_pos(group_id.into(), &user.board).expect("Group exists");
    (*user).board.groups[group_pos].color = color;
    Ok(())
}

#[tauri::command]
pub async fn add_task(
    group_id: String,
    body: String,
    state: tauri::State<'_, UserState>,
) -> Result<(), ()> {
    let mut user = parse_user_state_expected(&state).await;

    let group = find_group(group_id.into(), &mut user.board).expect("Group exists");
    group.tasks.push(Task::default_from(body));
    Ok(())
}

#[tauri::command]
pub async fn remove_task(
    group_id: String,
    task_id: String,
    state: tauri::State<'_, UserState>,
) -> Result<(), ()> {
    let mut user = parse_user_state_expected(&state).await;

    let group = find_group(group_id.into(), &mut user.board).expect("Group to exist");
    let task_pos = find_task(task_id.into(), group).expect("Task to exist");
    group.tasks.remove(task_pos);
    Ok(())
}

#[tauri::command]
pub async fn change_task_group(
    task_id: String,
    neighbor_task_id: Option<String>,
    old_group_id: String,
    new_group_id: String,
    state: tauri::State<'_, UserState>,
) -> Result<(), ()> {
    let mut user = parse_user_state_expected(&state).await;

    let task_clone: Task;
    {
        let old_group = find_group(old_group_id.into(), &mut user.board).expect("Group to exist");
        task_clone = remove_task_from_group(task_id.into(), old_group).expect("Task to exist");
    }
    {
        let new_group = find_group(new_group_id.into(), &mut user.board).expect("Group to exist");
        let neighbor_index = neighbor_task_id
            .map(|str| str.into())
            .and_then(|id| find_task(id, new_group))
            .unwrap_or(new_group.tasks.len());
        new_group.tasks.insert(neighbor_index, task_clone);
    }
    Ok(())
}

#[tauri::command]
pub async fn change_task_body_or_create(
    group_id: String,
    task_id: Option<String>,
    body: String,
    state: tauri::State<'_, UserState>,
) -> Result<(), ()> {
    let mut user = parse_user_state_expected(&state).await;
    let group = find_group(group_id.into(), &mut user.board).expect("Group to exist");
    let task_pos = find_task_or_create(task_id.map(|str| str.into()), group);
    group.tasks[task_pos].body = trim_whitespace(body.trim());
    Ok(())
}

#[tauri::command]
pub async fn reset(state: tauri::State<'_, UserState>) -> Result<(), ()> {
    let mut user = parse_user_state_expected(&state).await;
    user.reset();
    Ok(())
}
