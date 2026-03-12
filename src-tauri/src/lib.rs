use serde_json::Value;
use tauri::Manager;
use tokio::sync::{mpsc, oneshot};

mod prover;

#[derive(Debug)]
pub struct State {
    pub channel: mpsc::UnboundedSender<(String, Value, oneshot::Sender<Value>)>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    pretty_env_logger::init();

    let tx = prover::start_prover();

    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![prover::send_msg])
        .setup(|app| {
            app.manage(State { channel: tx });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
