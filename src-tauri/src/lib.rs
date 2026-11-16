mod features;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            features::app_launcher::init(&app);
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            features::app_launcher::commands::list_cached
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
