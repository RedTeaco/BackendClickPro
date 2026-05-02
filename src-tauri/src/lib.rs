use crate::logic::commands::commands::{
    execute_plan, get_execution_status, load_root_groups, load_shortcuts, log_message,
    save_root_groups, save_shortcuts, stop_execution,
};
use crate::logic::utils::thread_manager::ThreadManager;
use crate::logic::utils::window_capture::get_windows;
use std::sync::Mutex;
use tauri::Manager;
use tauri_plugin_decorum::WebviewWindowExt;

mod logic;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let thread_manager = ThreadManager::new(); // 全局唯一
    tauri::Builder::default()
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .manage(Mutex::new(thread_manager))
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_decorum::init())
        .setup(|app| {
            let main_window = app.get_webview_window("main").unwrap();
            main_window.create_overlay_titlebar().unwrap();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_windows,
            stop_execution,
            get_execution_status,
            save_root_groups,
            load_root_groups,
            execute_plan,
            save_shortcuts,
            load_shortcuts,
            log_message
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
