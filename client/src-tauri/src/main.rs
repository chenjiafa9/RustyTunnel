// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod error;
mod models;
mod state;

use state::AppState;
use tauri::Manager;

fn main() {
    env_logger::init();

    tauri::Builder::default()
        .manage(AppState::new())
        .invoke_handler(tauri::generate_handler![
            commands::login,
            commands::logout,
            commands::get_current_user,
            commands::get_vpn_nodes,
            commands::get_connection_stats,
            commands::update_connection_stats,
            commands::start_connection,
            commands::stop_connection,
            commands::get_settings,
            commands::update_settings,
            commands::test_node_connection,
        ])
        .setup(|app| {
            #[cfg(debug_assertions)]
            {
                let window = app.get_webview_window("main").unwrap();
                window.open_devtools();
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
