#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::error::Error;

use anyhow::Result;
use tauri::{App, Manager};

#[tauri::command]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

fn setup(app: &mut App) -> Result<(), Box<dyn Error>> {
    let window = app.get_window("main").unwrap();

    #[cfg(debug_assertions)]
    {
        window.open_devtools();
    }

    Ok(())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .setup(setup)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
