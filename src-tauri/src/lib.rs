mod types;
mod instances;
mod paths;
mod window;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            window::minimize_window,
            window::maximize_window,
            window::close_window,
            instances::save_instance,
            instances::get_instances
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
