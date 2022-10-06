#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use crate::commands::{add_group, add_task, get_board, greet, reset};
use crate::database::board::BoardState;

pub mod commands;
pub mod database;
pub mod helpers;

fn main() {
    tauri::Builder::default()
        .manage(BoardState(Default::default()))
        .invoke_handler(tauri::generate_handler![
            greet, get_board, add_group, add_task, reset
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
