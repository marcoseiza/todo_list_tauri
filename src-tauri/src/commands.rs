use crate::database::board::{Board, BoardState};
use crate::database::group::Group;
use crate::database::reset::Reset;
use crate::database::task::Task;
use crate::helpers::find_group;

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
pub fn reset(state: tauri::State<BoardState>) {
    let board = &mut *(state.0.lock().unwrap());
    board.reset();
}
