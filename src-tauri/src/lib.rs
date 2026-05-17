#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod audio_capture;
mod settings;
mod transcription;

use audio_capture::AudioCapture;
use std::sync::{LazyLock, Mutex};
use tauri::Emitter;
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};
use tauri_plugin_opener::OpenerExt;

static AUDIO: LazyLock<Mutex<Option<AudioCapture>>> = LazyLock::new(|| Mutex::new(None));
static LAST_BUFFER: LazyLock<Mutex<Vec<f32>>> = LazyLock::new(|| Mutex::new(Vec::new()));

#[tauri::command]
fn get_settings(app: tauri::AppHandle) -> settings::Settings {
    settings::load(&app)
}

#[tauri::command]
fn save_settings(app: tauri::AppHandle, mode: String) -> Result<(), String> {
    let mode = match mode.as_str() {
        "api" => settings::TranscriptionMode::Api,
        "local" => settings::TranscriptionMode::Local,
        _ => return Err("Modo no válido".to_string()),
    };
    settings::save(&app, &settings::Settings { mode })
}

#[tauri::command]
fn get_api_key_status(app: tauri::AppHandle) -> bool {
    settings::load_api_key(&app).is_some()
}

#[tauri::command]
fn get_config_path(app: tauri::AppHandle) -> String {
    settings::config_file_path(&app)
        .to_string_lossy()
        .to_string()
}

#[tauri::command]
fn open_config_file(app: tauri::AppHandle) -> Result<(), String> {
    settings::ensure_config_file(&app);
    let path = settings::config_file_path(&app);
    app.opener()
        .open_path(path.to_string_lossy().as_ref(), None::<&str>)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn get_model_status() -> &'static str {
    "not_installed"
}

#[tauri::command]
async fn download_model() -> Result<(), String> {
    Err("Próximamente disponible".to_string())
}

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
                        let capture = AUDIO.lock().unwrap().take();
                        if let Some(capture) = capture {
                            let buffer = capture.stop_and_get_buffer();
                            *LAST_BUFFER.lock().unwrap() = buffer.clone();
                            let _ = app.emit("recording-stopped", ());
                            let _ = app.emit("transcribing", ());

                            let app_clone = app.clone();
                            tauri::async_runtime::spawn(async move {
                                let s = settings::load(&app_clone);
                                let api_key = settings::load_api_key(&app_clone);
                                match transcription::transcribe(buffer, &s.mode, api_key.as_deref()).await {
                                    Ok(text) => {
                                        let _ = app_clone.emit("transcription-ready", text);
                                    }
                                    Err(e) => {
                                        let _ = app_clone.emit("transcription-error", e);
                                    }
                                }
                            });
                        }
                    }
                })
                .build(),
        )
        .invoke_handler(tauri::generate_handler![
            get_settings,
            save_settings,
            get_api_key_status,
            get_config_path,
            open_config_file,
            get_model_status,
            download_model,
            get_recording_buffer,
            clear_recording,
        ])
        .setup(|app| {
            settings::ensure_config_file(&app.handle());
            let shortcut = Shortcut::new(Some(Modifiers::CONTROL | Modifiers::SHIFT), Code::KeyJ);
            if let Err(e) = app.global_shortcut().register(shortcut) {
                eprintln!("Error al registrar hotkey: {}", e);
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
