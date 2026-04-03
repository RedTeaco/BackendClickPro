mod logic;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![logic::utils::window_capture::get_windows])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
