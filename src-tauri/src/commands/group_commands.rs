use std::str::FromStr;

use crate::database::{
    Color, Group, {parse_user_state_expected, UserState},
};
use crate::helpers::{find_group_pos, find_group_with_name};

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
