mod commands;
mod utils;

use commands::{get_app_dir, greet};
use tauri::{
    webview::PageLoadPayload, App, Builder, Webview, WebviewUrl, WebviewWindowBuilder, Window,
    WindowEvent, Wry,
};
use tauri_plugin_log::{Target, TargetKind};
use tracing::{debug, info};
use utils::log_dir;

pub fn app() -> anyhow::Result<Builder<Wry>> {
    let builder = tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(logger().build())
        .invoke_handler(tauri::generate_handler![greet, get_app_dir])
        .setup(setup)
        .on_page_load(on_page_load)
        .on_window_event(window_event_handler);
    Ok(builder)
}

fn setup(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    info!("Setting up app");

    let handle = app.handle();

    #[cfg(desktop)]
    {
        handle.plugin(tauri_plugin_window_state::Builder::default().build())?;
    }

    let mut builder = WebviewWindowBuilder::new(app, "main", WebviewUrl::default());

    #[cfg(desktop)]
    {
        builder = builder
            .user_agent(&format!("Hn app - {}", std::env::consts::OS))
            .title("Hacker News")
            .inner_size(1200., 800.)
            .resizable(true)
            .content_protected(true);
    }

    let webview = builder.build()?;

    #[cfg(debug_assertions)]
    webview.open_devtools();

    // initialize updater...
    Ok(())
}

fn on_page_load(webview: &Webview, _payload: &PageLoadPayload<'_>) {
    info!("Page loaded on {:?}", webview.label());
}

fn window_event_handler(window: &Window, event: &WindowEvent) {
    debug!("Window event: {:?} on {:?}", event, window.label());

    if let WindowEvent::CloseRequested { api, .. } = event {
        info!("Window close requested on {:?}", window.label());
        if window.label() == "main" {
            api.prevent_close();
            info!("prevent close in main page");
            window.hide().unwrap();
        }
    }
}

fn logger() -> tauri_plugin_log::Builder {
    tauri_plugin_log::Builder::default()
        .targets([
            Target::new(TargetKind::Webview),
            Target::new(TargetKind::Folder {
                path: log_dir(),
                file_name: None,
            }),
            Target::new(TargetKind::Stdout),
        ])
        .level(tracing::log::LevelFilter::Info)
}
