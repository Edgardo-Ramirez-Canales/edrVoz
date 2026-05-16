#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod audio_capture;

use audio_capture::AudioCapture;
use std::sync::{LazyLock, Mutex};
use tauri::Emitter;
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};

// Captura activa — None cuando no se está grabando
static AUDIO: LazyLock<Mutex<Option<AudioCapture>>> = LazyLock::new(|| Mutex::new(None));

// Buffer del último audio grabado — disponible hasta que se limpie
static LAST_BUFFER: LazyLock<Mutex<Vec<f32>>> = LazyLock::new(|| Mutex::new(Vec::new()));

#[tauri::command]
fn get_recording_buffer() -> Vec<f32> {
    LAST_BUFFER.lock().unwrap().clone()
}

#[tauri::command]
fn clear_recording() {
    LAST_BUFFER.lock().unwrap().clear();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(|app, _shortcut, event| {
                    if event.state() == ShortcutState::Pressed {
                        // Tomar el lock, iniciar captura solo si no hay una activa
                        let mut audio = AUDIO.lock().unwrap();
                        if audio.is_none() {
                            match AudioCapture::new() {
                                Ok(capture) => {
                                    *audio = Some(capture);
                                    let _ = app.emit("recording-started", ());
                                }
                                Err(e) => {
                                    let _ = app.emit("recording-error", e);
                                }
                            }
                        }
                    } else if event.state() == ShortcutState::Released {
                        // Sacar la captura del Mutex antes de detenerla para liberar el lock cuanto antes
                        let capture = AUDIO.lock().unwrap().take();
                        if let Some(capture) = capture {
                            let buffer = capture.stop_and_get_buffer();
                            let sample_count = buffer.len();
                            *LAST_BUFFER.lock().unwrap() = buffer;
                            let _ = app.emit("recording-stopped", ());
                            let _ = app.emit("audio-buffer-ready", sample_count);
                        }
                    }
                })
                .build(),
        )
        .invoke_handler(tauri::generate_handler![get_recording_buffer, clear_recording])
        .setup(|app| {
            let shortcut = Shortcut::new(Some(Modifiers::CONTROL | Modifiers::SHIFT), Code::KeyJ);
            if let Err(e) = app.global_shortcut().register(shortcut) {
                eprintln!("Error al registrar hotkey: {}", e);
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
