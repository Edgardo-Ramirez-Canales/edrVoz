#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod audio_capture;
mod paste;
mod settings;
mod transcription;

use audio_capture::AudioCapture;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{LazyLock, Mutex};
use tauri::{Emitter, Manager};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};
use tauri_plugin_opener::OpenerExt;

/// Recorta la ventana Win32 a la forma exacta de la píldora usando SetWindowRgn.
/// Esto elimina las esquinas del rectángulo 260×80, haciendo que solo la píldora sea visible.
/// No depende de WebView2 — opera a nivel de OS antes de que WebView2 renderice nada.
#[cfg(target_os = "windows")]
fn apply_pill_region(window: &tauri::WebviewWindow, scale: f64) {
    use raw_window_handle::{HasWindowHandle, RawWindowHandle};

    let Ok(handle) = window.window_handle() else { return };
    let RawWindowHandle::Win32(raw) = handle.as_ref() else { return };
    let hwnd = raw.hwnd.get() as isize;

    // Píldora: 240×52 lógicos, centrada en 260×80 → convertir a pixels físicos
    let pill_w = (240.0 * scale).round() as i32;
    let pill_h = (52.0 * scale).round() as i32;
    let win_w  = (260.0 * scale).round() as i32;
    let win_h  = (80.0  * scale).round() as i32;
    let x = (win_w - pill_w) / 2;
    let y = (win_h - pill_h) / 2;
    let r = pill_h;

    unsafe {
        #[link(name = "gdi32")]
        extern "system" { fn CreateRoundRectRgn(x1: i32, y1: i32, x2: i32, y2: i32, cx: i32, cy: i32) -> isize; }
        #[link(name = "user32")]
        extern "system" { fn SetWindowRgn(hwnd: isize, hrgn: isize, redraw: i32) -> i32; }

        let rgn = CreateRoundRectRgn(x, y, x + pill_w, y + pill_h, r, r);
        SetWindowRgn(hwnd, rgn, 1);
    }
}

static AUDIO: LazyLock<Mutex<Option<AudioCapture>>> = LazyLock::new(|| Mutex::new(None));
static LAST_BUFFER: LazyLock<Mutex<Vec<f32>>> = LazyLock::new(|| Mutex::new(Vec::new()));
static SESSION: AtomicU32 = AtomicU32::new(0);

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
fn set_transparent_background(app: tauri::AppHandle) {
    if let Some(w) = app.get_webview_window("main") {
        let _ = w.set_background_color(Some(tauri::webview::Color(0, 0, 0, 0)));
    }
}

#[tauri::command]
fn hide_window(app: tauri::AppHandle) {
    if let Some(w) = app.get_webview_window("main") {
        let _ = w.hide();
    }
}

#[tauri::command]
fn cancel_recording() {
    drop(AUDIO.lock().unwrap().take());
    LAST_BUFFER.lock().unwrap().clear();
}

async fn do_transcribe(app: tauri::AppHandle, buffer: Vec<f32>, session: u32) {
    let s = settings::load(&app);
    let mode = format!("{:?}", s.mode).to_lowercase();
    log::info!("[sesión {session}] Transcripción iniciada — {} samples, modo: {mode}", buffer.len());

    let start = std::time::Instant::now();
    let api_key = settings::load_api_key(&app);
    match transcription::run(buffer, &s.mode, api_key).await {
        Ok(text) => {
            let elapsed = start.elapsed().as_millis();
            log::info!("[sesión {session}] Transcripción completada en {elapsed}ms ({} chars)", text.len());
            paste::paste_text(&text);
            let _ = app.emit("transcription-ready", text);
        }
        Err(e) => {
            log::error!("[sesión {session}] Error en transcripción: {e}");
            let _ = app.emit("transcription-error", e);
        }
    }
}

#[tauri::command]
async fn force_stop_recording(app: tauri::AppHandle) {
    let capture = AUDIO.lock().unwrap().take();
    if let Some(capture) = capture {
        let session = SESSION.load(Ordering::Relaxed);
        log::warn!("[sesión {session}] Grabación detenida automáticamente por límite de tiempo");
        let buffer = capture.stop_and_get_buffer();
        *LAST_BUFFER.lock().unwrap() = buffer.clone();
        let _ = app.emit("recording-stopped", ());
        let _ = app.emit("transcribing", ());
        do_transcribe(app, buffer, session).await;
    }
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
                        // WebView2 ya está inicializado aquí — set_background_color funciona
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.set_background_color(Some(tauri::webview::Color(0, 0, 0, 0)));
                            let _ = window.show();
                        }
                        let mut audio = AUDIO.lock().unwrap();
                        if audio.is_none() {
                            match AudioCapture::new() {
                                Ok(capture) => {
                                    let session = SESSION.fetch_add(1, Ordering::Relaxed);
                                    log::info!("[sesión {session}] Grabación iniciada");
                                    *audio = Some(capture);
                                    let _ = app.emit("recording-started", ());
                                }
                                Err(e) => {
                                    log::error!("Error al iniciar captura de audio: {e}");
                                    let _ = app.emit("recording-error", e);
                                }
                            }
                        }
                    } else if event.state() == ShortcutState::Released {
                        let capture = AUDIO.lock().unwrap().take();
                        if let Some(capture) = capture {
                            let session = SESSION.load(Ordering::Relaxed);
                            let buffer = capture.stop_and_get_buffer();
                            log::info!("[sesión {session}] Grabación detenida — {} samples", buffer.len());
                            *LAST_BUFFER.lock().unwrap() = buffer.clone();
                            let _ = app.emit("recording-stopped", ());
                            let _ = app.emit("transcribing", ());

                            let app_clone = app.clone();
                            tauri::async_runtime::spawn(async move {
                                do_transcribe(app_clone, buffer, session).await;
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
            force_stop_recording,
            hide_window,
            cancel_recording,
            set_transparent_background,
        ])
        .setup(|app| {
            env_logger::Builder::new()
                .filter_level(log::LevelFilter::Info)
                .format_timestamp_secs()
                .init();
            log::info!("EDR Voz iniciando");
            settings::ensure_config_file(&app.handle());

            let window = app.get_webview_window("main").expect("ventana main no encontrada");


            // Posicionar centrado horizontalmente, 80px sobre la taskbar
            if let Ok(Some(monitor)) = window.primary_monitor() {
                let size = monitor.size();
                let scale = monitor.scale_factor();
                let win_w = 260.0_f64;
                let win_h = 80.0_f64;
                let x = (size.width as f64 / scale - win_w) / 2.0;
                let y = size.height as f64 / scale - win_h - 80.0;
                let _ = window.set_position(tauri::LogicalPosition::new(x, y));

                // Recortar la ventana a la forma de la píldora a nivel Win32
                #[cfg(target_os = "windows")]
                apply_pill_region(&window, scale);
            }

            let shortcut = Shortcut::new(Some(Modifiers::CONTROL | Modifiers::SHIFT), Code::KeyJ);
            if let Err(e) = app.global_shortcut().register(shortcut) {
                eprintln!("Error al registrar hotkey: {}", e);
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
