use crate::database::board::{Board, BoardState};
use crate::database::group::Group;
use crate::database::reset::Reset;
use crate::database::task::Task;
use crate::helpers::{find_group, find_task, find_task_or_create, remove_task_from_group};

#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
pub fn get_board(state: tauri::State<BoardState>) -> Board {
    let board = state.0.lock().unwrap();
    (*board).clone()
}

#[tauri::command]
pub fn add_group(name: String, state: tauri::State<BoardState>) {
    let board = &mut *(state.0.lock().unwrap());
    board.groups.push(Group::default_from(name));
}

#[tauri::command]
pub fn add_task(group_id: String, body: String, state: tauri::State<BoardState>) {
    let board = &mut *(state.0.lock().unwrap());
    let group = find_group(group_id, board).expect("Group exists");
    group.tasks.push(Task::default_from(body));
}

#[tauri::command]
pub fn remove_task(group_id: String, task_id: String, state: tauri::State<BoardState>) {
    let board = &mut *(state.0.lock().unwrap());
    let group = find_group(group_id, board).expect("Group to exist");
    let (_, task_pos) = find_task(task_id, group).expect("Task to exist");
    group.tasks.remove(task_pos);
}

#[tauri::command]
pub fn change_task_group(
    task_id: String,
    old_group_id: String,
    new_group_id: String,
    state: tauri::State<BoardState>,
) {
    let board = &mut *(state.0.lock().unwrap());
    let task_clone: Task;
    {
        let old_group = find_group(old_group_id, board).expect("Group to exist");
        task_clone = remove_task_from_group(task_id, old_group).expect("Task to exist");
    }
    {
        let new_group = find_group(new_group_id, board).expect("Group to exist");
        new_group.tasks.push(task_clone);
    }
}

#[tauri::command]
pub fn change_task_body(
    group_id: String,
    task_id: String,
    body: String,
    state: tauri::State<BoardState>,
) {
    let board = &mut *(state.0.lock().unwrap());
    let group = find_group(group_id, board).expect("Group to exist");
    let task_pos = find_task_or_create(task_id, group);
    group.tasks[task_pos].body = body;
}

#[tauri::command]
pub fn reset(state: tauri::State<BoardState>) {
    let board = &mut *(state.0.lock().unwrap());
    board.reset();
}
