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

pub fn parse_api_key_from_str(content: &str) -> Option<String> {
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

// Lee la API key desde variable de entorno o desde el archivo config.env
pub fn load_api_key(app: &tauri::AppHandle) -> Option<String> {
    // Prioridad 1: variable de entorno del sistema
    if let Ok(key) = std::env::var("OPENAI_API_KEY") {
        let key = key.trim().to_string();
        if !key.is_empty() {
            return Some(key);
        }
    }

    // Prioridad 2: archivo config.env junto al ejecutable
    let content = std::fs::read_to_string(config_file_path(app)).ok()?;
    parse_api_key_from_str(&content)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_api_key_simple() {
        let content = "OPENAI_API_KEY=sk-test123\n";
        assert_eq!(parse_api_key_from_str(content), Some("sk-test123".to_string()));
    }

    #[test]
    fn parses_api_key_with_quotes() {
        let content = "OPENAI_API_KEY=\"sk-test123\"";
        assert_eq!(parse_api_key_from_str(content), Some("sk-test123".to_string()));
    }

    #[test]
    fn ignores_comments_and_blank_lines() {
        let content = "# Configuracion\n\nOPENAI_API_KEY=sk-abc\n";
        assert_eq!(parse_api_key_from_str(content), Some("sk-abc".to_string()));
    }

    #[test]
    fn returns_none_for_empty_value() {
        let content = "OPENAI_API_KEY=\n";
        assert_eq!(parse_api_key_from_str(content), None);
    }

    #[test]
    fn returns_none_when_key_absent() {
        let content = "# sin API key\nOTRA_VAR=valor\n";
        assert_eq!(parse_api_key_from_str(content), None);
    }

    #[test]
    fn settings_default_mode_is_api() {
        let s = Settings::default();
        assert!(matches!(s.mode, TranscriptionMode::Api));
    }

    #[test]
    fn settings_roundtrip_json() {
        let original = Settings { mode: TranscriptionMode::Local };
        let json = serde_json::to_string(&original).unwrap();
        let parsed: Settings = serde_json::from_str(&json).unwrap();
        assert!(matches!(parsed.mode, TranscriptionMode::Local));
    }
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
