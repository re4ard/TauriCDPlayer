use tauri::Manager;
use std::env;

#[tauri::command]
fn get_os() -> String {
    env::consts::OS.to_string()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_os])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
