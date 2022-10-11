#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::Manager;
use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial};

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
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            #[cfg(target_os = "macos")]
            apply_vibrancy(&window, NSVisualEffectMaterial::HudWindow, None, None)
                .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
