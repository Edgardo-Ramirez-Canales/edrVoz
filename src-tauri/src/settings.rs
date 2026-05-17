use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum TranscriptionMode {
    #[default]
    Api,
    Local,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Settings {
    #[serde(default)]
    pub mode: TranscriptionMode,
}

// Directorio donde vive el ejecutable (src-tauri/target/release/ en desarrollo)
fn exe_dir() -> PathBuf {
    std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
        .unwrap_or_default()
}

fn settings_path(_app: &tauri::AppHandle) -> PathBuf {
    exe_dir().join("settings.json")
}

pub fn config_file_path(_app: &tauri::AppHandle) -> PathBuf {
    exe_dir().join("config.env")
}

pub fn load(app: &tauri::AppHandle) -> Settings {
    let path = settings_path(app);
    std::fs::read_to_string(&path)
        .ok()
        .and_then(|content| serde_json::from_str(&content).ok())
        .unwrap_or_default()
}

pub fn save(app: &tauri::AppHandle, settings: &Settings) -> Result<(), String> {
    let path = settings_path(app);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let content = serde_json::to_string_pretty(settings).map_err(|e| e.to_string())?;
    std::fs::write(path, content).map_err(|e| e.to_string())
}

// Lee la API key desde variable de entorno o desde el archivo config.env
pub fn load_api_key(app: &tauri::AppHandle) -> Option<String> {
    // Prioridad 1: variable de entorno del sistema
    if let Ok(key) = std::env::var("OPENAI_API_KEY") {
        let key = key.trim().to_string();
        if !key.is_empty() {
            return Some(key);
        }
    }

    // Prioridad 2: archivo config.env en AppData\Local\edrvoz\
    let content = std::fs::read_to_string(config_file_path(app)).ok()?;
    for line in content.lines() {
        let line = line.trim();
        if line.starts_with('#') || line.is_empty() {
            continue;
        }
        if let Some(value) = line.strip_prefix("OPENAI_API_KEY=") {
            let key = value.trim().trim_matches('"');
            if !key.is_empty() {
                return Some(key.to_string());
            }
        }
    }

    None
}

// Crea el archivo config.env con plantilla si no existe
pub fn ensure_config_file(app: &tauri::AppHandle) {
    let path = config_file_path(app);
    if path.exists() {
        return;
    }
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let template = "# Configuracion de EDR Voz\n# Agrega tu API Key de OpenAI y guarda el archivo.\n\nOPENAI_API_KEY=\n";
    let _ = std::fs::write(&path, template);
}
