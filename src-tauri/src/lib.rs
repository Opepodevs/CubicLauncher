mod customization;
mod discord;
mod error;
mod window;
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() {
    better_panic::install();
    tracing_subscriber::fmt::init();
    tokio::spawn(async {
        discord::connect().await.unwrap();
    });
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            window::close_window,
            window::minimize_window,
            window::maximize_window,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
