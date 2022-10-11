#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use crate::commands::{
    add_group, add_task, change_task_body_or_create, change_task_group, get_board, remove_group,
    remove_task, reset, update_group_color, update_group_name, update_group_pos,
};
use crate::database::board::BoardState;

pub mod commands;
pub mod database;
pub mod helpers;

fn main() {
    tauri::Builder::default()
        .manage(BoardState(Default::default()))
        .invoke_handler(tauri::generate_handler![
            get_board,
            add_group,
            add_task,
            reset,
            remove_task,
            change_task_group,
            change_task_body_or_create,
            update_group_pos,
            remove_group,
            update_group_name,
            update_group_color
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
