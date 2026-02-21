mod commands;
mod error;
mod storage;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_http::init())
        .invoke_handler(tauri::generate_handler![
            commands::philosopher::list_philosophers,
            commands::philosopher::get_philosopher,
            commands::philosopher::save_philosopher,
            commands::philosopher::delete_philosopher,
            commands::dialogue::list_dialogues,
            commands::dialogue::get_dialogue,
            commands::dialogue::save_dialogue,
            commands::me::get_my_philosophy,
            commands::me::save_my_philosophy,
            commands::me::add_insight,
            commands::ai::get_api_key,
            commands::ai::save_api_key,
            commands::ai::chat,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
