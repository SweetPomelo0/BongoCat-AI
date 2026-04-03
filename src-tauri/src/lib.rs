mod ai;
mod core;
mod memory;
mod utils;

use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

use ai::{send_chat_message, send_chat_message_stream};
use core::{
    device::start_device_listening,
    gamepad::{start_gamepad_listing, stop_gamepad_listing},
    prevent_default, setup,
};
use memory::{get_memory_status, open_memory_path};
use tauri::{AppHandle, Manager, WindowEvent, generate_handler};
use tauri_plugin_autostart::MacosLauncher;
use tauri_plugin_custom_window::{
    CHAT_WINDOW_LABEL, MAIN_WINDOW_LABEL, PREFERENCE_WINDOW_LABEL, show_preference_window,
};
use utils::fs_extra::copy_dir;

#[derive(Clone)]
struct AppLifecycleState {
    allow_close: Arc<AtomicBool>,
}

impl AppLifecycleState {
    fn new() -> Self {
        Self {
            allow_close: Arc::new(AtomicBool::new(false)),
        }
    }

    fn allow_close(&self) {
        self.allow_close.store(true, Ordering::SeqCst);
    }

    fn should_allow_close(&self) -> bool {
        self.allow_close.load(Ordering::SeqCst)
    }
}

#[tauri::command]
fn quit_app(app_handle: AppHandle, lifecycle: tauri::State<AppLifecycleState>) {
    lifecycle.allow_close();
    app_handle.exit(0);
}

#[tauri::command]
fn restart_app(app_handle: AppHandle, lifecycle: tauri::State<AppLifecycleState>) -> Result<(), String> {
    lifecycle.allow_close();
    app_handle.restart();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let lifecycle_state = AppLifecycleState::new();
    let app = tauri::Builder::default()
        .manage(lifecycle_state.clone())
        .setup(|app| {
            let app_handle = app.handle();

            let main_window = app.get_webview_window(MAIN_WINDOW_LABEL).unwrap();

            let preference_window = app.get_webview_window(PREFERENCE_WINDOW_LABEL).unwrap();
            let _chat_window = app.get_webview_window(CHAT_WINDOW_LABEL).unwrap();

            setup::default(&app_handle, main_window.clone(), preference_window.clone());

            Ok(())
        })
        .invoke_handler(generate_handler![
            copy_dir,
            start_device_listening,
            start_gamepad_listing,
            stop_gamepad_listing,
            send_chat_message,
            send_chat_message_stream,
            get_memory_status,
            open_memory_path,
            quit_app,
            restart_app
        ])
        .plugin(tauri_plugin_custom_window::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_pinia::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(prevent_default::init())
        .plugin(tauri_plugin_single_instance::init(
            |app_handle, _argv, _cwd| {
                show_preference_window(app_handle);
            },
        ))
        .plugin(
            tauri_plugin_log::Builder::new()
                .timezone_strategy(tauri_plugin_log::TimezoneStrategy::UseLocal)
                .filter(|metadata| !metadata.target().contains("gilrs"))
                .build(),
        )
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            None,
        ))
        .plugin(tauri_plugin_macos_permissions::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_locale::init())
        .on_window_event(|window, event| match event {
            WindowEvent::CloseRequested { api, .. } => {
                let lifecycle = window.state::<AppLifecycleState>();

                if lifecycle.should_allow_close() {
                    return;
                }

                let _ = window.hide();

                api.prevent_close();
            }
            _ => {}
        })
        .build(tauri::generate_context!())
        .expect("error while running tauri application");

    app.run(|app_handle, event| match event {
        #[cfg(target_os = "macos")]
        tauri::RunEvent::Reopen { .. } => {
            show_preference_window(app_handle);
        }
        _ => {
            let _ = app_handle;
        }
    });
}
