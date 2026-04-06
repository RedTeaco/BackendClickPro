use std::sync::Mutex;
use crate::logic::commands::commands::{execute_events, get_execution_status, load_events, save_events, save_shortcuts, load_shortcuts, log_message,set_mode, stop_execution};
use crate::logic::utils::thread_manager::{ThreadManager};
use crate::logic::utils::window_capture::get_windows;

mod logic;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let thread_manager = ThreadManager::new(); // 全局唯一
    tauri::Builder::default()
        .manage(Mutex::new(thread_manager))
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_windows,
            set_mode, execute_events, stop_execution, get_execution_status,
            save_events, load_events, save_shortcuts, load_shortcuts, log_message])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
