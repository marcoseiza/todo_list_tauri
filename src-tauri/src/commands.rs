use std::str::FromStr;

use crate::database::board::{Board, BoardState};
use crate::database::color::Color;
use crate::database::group::Group;
use crate::database::reset::Reset;
use crate::database::task::Task;
use crate::helpers::{
    find_group, find_group_pos, find_group_with_name, find_task, find_task_or_create,
    remove_task_from_group, trim_whitespace,
};

#[tauri::command]
pub fn get_board(state: tauri::State<BoardState>) -> Board {
    let board = state.0.lock().unwrap();
    (*board).clone()
}

#[tauri::command]
pub fn add_group(name: String, state: tauri::State<BoardState>) {
    let board = &mut *(state.0.lock().unwrap());
    if find_group_with_name(&name, board).is_some() {
        return;
    }
    board.groups.push(Group::default_from(name));
}

#[tauri::command]
pub fn remove_group(group_id: String, state: tauri::State<BoardState>) {
    let board = &mut *(state.0.lock().unwrap());
    let group_pos = find_group_pos(group_id.into(), board).expect("Group exists");
    board.groups.remove(group_pos);
}

#[tauri::command]
pub fn update_group_pos(
    group_id: String,
    neighbor_group_id: Option<String>,
    state: tauri::State<BoardState>,
) {
    let board = &mut *(state.0.lock().unwrap());
    let group_pos = find_group_pos(group_id.into(), board).expect("Group exists");
    let group = board.groups.remove(group_pos);
    let neighbor_pos = neighbor_group_id
        .map(|str| str.into())
        .and_then(|id| find_group_pos(id, board))
        .unwrap_or(board.groups.len());
    board.groups.insert(neighbor_pos, group);
}

#[tauri::command]
pub fn update_group_name(group_id: String, name: String, state: tauri::State<BoardState>) {
    let board = &mut *(state.0.lock().unwrap());
    if find_group_with_name(&name, board).is_some() {
        return;
    }
    let group_pos = find_group_pos(group_id.into(), board).expect("Group exists");
    board.groups[group_pos].name = name;
}

#[tauri::command]
pub fn update_group_color(group_id: String, color: String, state: tauri::State<BoardState>) {
    let board = &mut *(state.0.lock().unwrap());
    let color: Color = Color::from_str(&*color).unwrap_or_default();
    let group_pos = find_group_pos(group_id.into(), board).expect("Group exists");
    board.groups[group_pos].color = color;
}

#[tauri::command]
pub fn add_task(group_id: String, body: String, state: tauri::State<BoardState>) {
    let board = &mut *(state.0.lock().unwrap());
    let group = find_group(group_id.into(), board).expect("Group exists");
    group.tasks.push(Task::default_from(body));
}

#[tauri::command]
pub fn remove_task(group_id: String, task_id: String, state: tauri::State<BoardState>) {
    let board = &mut *(state.0.lock().unwrap());
    let group = find_group(group_id.into(), board).expect("Group to exist");
    let task_pos = find_task(task_id.into(), group).expect("Task to exist");
    group.tasks.remove(task_pos);
}

#[tauri::command]
pub fn change_task_group(
    task_id: String,
    neighbor_task_id: Option<String>,
    old_group_id: String,
    new_group_id: String,
    state: tauri::State<BoardState>,
) {
    let board = &mut *(state.0.lock().unwrap());
    let task_clone: Task;
    {
        let old_group = find_group(old_group_id.into(), board).expect("Group to exist");
        task_clone = remove_task_from_group(task_id.into(), old_group).expect("Task to exist");
    }
    {
        let new_group = find_group(new_group_id.into(), board).expect("Group to exist");
        let neighbor_index = neighbor_task_id
            .map(|str| str.into())
            .and_then(|id| find_task(id, &new_group))
            .unwrap_or(new_group.tasks.len());
        new_group.tasks.insert(neighbor_index, task_clone);
    }
}

#[tauri::command]
pub fn change_task_body_or_create(
    group_id: String,
    task_id: Option<String>,
    body: String,
    state: tauri::State<BoardState>,
) {
    let board = &mut *(state.0.lock().unwrap());
    let group = find_group(group_id.into(), board).expect("Group to exist");
    let task_pos = find_task_or_create(task_id.map(|str| str.into()), group);
    group.tasks[task_pos].body = trim_whitespace(body.trim());
}

#[tauri::command]
pub fn reset(state: tauri::State<BoardState>) {
    let board = &mut *(state.0.lock().unwrap());
    board.reset();
}
