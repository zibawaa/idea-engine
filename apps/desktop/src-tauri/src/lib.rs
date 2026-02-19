mod ipc;

use idea_engine_core::Storage;
use std::path::PathBuf;
use std::sync::Arc;
use tauri::Manager;
use tauri_plugin_shell::ShellExt;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            let app_data = app.path().app_data_dir().expect("app data dir");
            std::fs::create_dir_all(&app_data)?;
            let db_path = app_data.join("idea_engine.db");
            let storage = Storage::new(&db_path)?;
            app.manage(Arc::new(storage));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            ipc::create_chat,
            ipc::list_chats,
            ipc::get_chat_messages,
            ipc::send_message,
            ipc::set_feedback,
            ipc::list_recipes,
            ipc::save_recipe,
            ipc::get_api_keys,
            ipc::set_api_key,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
