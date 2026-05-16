#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod audio_capture;

use tauri::Emitter;
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(|app, _shortcut, event| {
                    if event.state() == ShortcutState::Pressed {
                        let _ = app.emit("recording-started", ());
                    } else {
                        let _ = app.emit("recording-stopped", ());
                    }
                })
                .build(),
        )
        .invoke_handler(tauri::generate_handler![greet])
        .setup(|app| {
            let shortcut = Shortcut::new(Some(Modifiers::CONTROL | Modifiers::SHIFT), Code::KeyJ);
            if let Err(e) = app.global_shortcut().register(shortcut) {
                let msg = format!("Win+J registration failed: {}\n", e);
                let _ = std::fs::write("C:\\edrVoz\\shortcut_error.txt", &msg);
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
