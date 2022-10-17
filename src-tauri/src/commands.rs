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
    remove_task_from_group, trim_whitespace,
};

#[tauri::command]
pub async fn save(state: tauri::State<'_, UserState>) -> Result<(), RequestError> {
    let user = parse_user_state_expected(&state).await;
    user.save().await
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

    let group_pos = find_group_pos(group_id.into(), &mut user.board).expect("Group exists");
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

    if find_group_with_name(&name, &mut user.board).is_some() {
        return Err(());
    }
    let group_pos = find_group_pos(group_id.into(), &user.board).expect("Group exists");
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
            .and_then(|id| find_task(id, &new_group))
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
