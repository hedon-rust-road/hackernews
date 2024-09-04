// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use hackernews_lib::app;

fn main() -> anyhow::Result<()> {
    app()?
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
