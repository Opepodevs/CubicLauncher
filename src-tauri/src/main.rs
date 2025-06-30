// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use tracing_subscriber;

fn main() {
    better_panic::install();
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
    tracing::info!("Logger iniciado");
    cubic_launcher_lib::run()
}
