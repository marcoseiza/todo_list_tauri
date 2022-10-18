#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
#[macro_use]
extern crate dotenv_codegen;

pub mod commands;
pub mod database;
pub mod firebase_interface;
pub mod helpers;
pub mod oauth;
pub mod secure_storage;

#[cfg(target_os = "macos")]
pub mod macos;

use database::user::UserState;
#[cfg(target_os = "macos")]
use macos::apply_title_bar_options;

use tauri::Manager;
use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial};

use crate::commands::{
    add_group, add_task, change_task_body_or_create, change_task_group, get_board, get_user, load,
    refresh_user_token, remove_group, remove_task, reset, save, update_group_color,
    update_group_name, update_group_pos,
};

use crate::oauth::login_signin_commands::{login, sign_up_with_github};

fn main() {
    tauri::Builder::default()
        .manage(UserState(Default::default()))
        .invoke_handler(tauri::generate_handler![
            add_group,
            add_task,
            change_task_body_or_create,
            change_task_group,
            get_board,
            get_user,
            load,
            login,
            refresh_user_token,
            remove_group,
            remove_task,
            reset,
            save,
            sign_up_with_github,
            update_group_color,
            update_group_name,
            update_group_pos,
        ])
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            #[cfg(target_os = "macos")]
            apply_vibrancy(&window, NSVisualEffectMaterial::HudWindow, None, None)
                .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");
            #[cfg(target_os = "macos")]
            apply_title_bar_options(&window, true, false).expect(
                "Unsupported platform! 'apply_transparent_title_bar' is only supported on macOS",
            );
            Ok(())
        })
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
