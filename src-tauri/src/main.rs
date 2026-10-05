// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process;
use tauri::Manager;
use tauri_plugin_autostart::MacosLauncher;

#[tauri::command]
fn exit_app() {
    process::exit(0);
}

#[tauri::command]
fn toggle_always_on_top(app_handle: tauri::AppHandle, state: bool) -> Result<(), String> {
    if let Some(window) = app_handle.get_webview_window("main") {
        window.set_always_on_top(state).map_err(|e| e.to_string())?;
    }
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            Some(vec![]),
        ))
        .invoke_handler(tauri::generate_handler![exit_app, toggle_always_on_top])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
