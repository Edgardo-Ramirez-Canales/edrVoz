use crate::settings::TranscriptionMode;
use std::sync::LazyLock;

static HTTP_CLIENT: LazyLock<reqwest::Client> = LazyLock::new(reqwest::Client::new);

// Contrato que debe cumplir cualquier implementación de transcripción.
// Para agregar modo local real: implementar este trait en LocalTranscriber.
trait Transcriber {
    async fn transcribe(&self, samples: &[f32]) -> Result<String, String>;
}

pub struct ApiTranscriber {
    api_key: String,
}

pub struct LocalTranscriber;

impl Transcriber for ApiTranscriber {
    async fn transcribe(&self, samples: &[f32]) -> Result<String, String> {
        let wav_bytes = encode_wav(&samples)?;

        let part = reqwest::multipart::Part::bytes(wav_bytes)
            .file_name("audio.wav")
            .mime_str("audio/wav")
            .map_err(|e| e.to_string())?;

        let form = reqwest::multipart::Form::new()
            .text("model", "whisper-1")
            .text("language", "es")
            .part("file", part);

        let response = HTTP_CLIENT
            .post("https://api.openai.com/v1/audio/transcriptions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .multipart(form)
            .send()
            .await
            .map_err(|e| format!("Error de conexión: {}", e))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(format!("Error de API ({}): {}", status, body));
        }

        let json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| format!("Error al leer respuesta: {}", e))?;

        json["text"]
            .as_str()
            .map(|s| s.trim().to_string())
            .ok_or("Respuesta inesperada de la API".to_string())
    }
}

impl Transcriber for LocalTranscriber {
    async fn transcribe(&self, _samples: &[f32]) -> Result<String, String> {
        // Para implementar: cargar modelo Whisper local (ej. whisper-rs) y procesar aquí.
        Err("Modo local no disponible. Descarga el modelo primero.".to_string())
    }
}

/// Punto de entrada público. Selecciona la implementación según el modo
/// y delega — lib.rs no necesita saber qué transcriber se usa.
pub async fn run(
    samples: &[f32],
    mode: &TranscriptionMode,
    api_key: Option<String>,
) -> Result<String, String> {
    if samples.is_empty() {
        return Err("No hay audio grabado".to_string());
    }
    match mode {
        TranscriptionMode::Api => {
            let key = api_key
                .filter(|k| !k.is_empty())
                .ok_or("API Key no configurada. Edita el archivo config.env.")?;
            ApiTranscriber { api_key: key }.transcribe(samples).await
        }
        TranscriptionMode::Local => LocalTranscriber.transcribe(samples).await,
    }
}

fn encode_wav(samples: &[f32]) -> Result<Vec<u8>, String> {
    let mut cursor = std::io::Cursor::new(Vec::new());
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 16000,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = hound::WavWriter::new(&mut cursor, spec).map_err(|e| e.to_string())?;
    for &sample in samples {
        let s = (sample * i16::MAX as f32).clamp(i16::MIN as f32, i16::MAX as f32) as i16;
        writer.write_sample(s).map_err(|e| e.to_string())?;
    }
    writer.finalize().map_err(|e| e.to_string())?;
    Ok(cursor.into_inner())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_wav_has_valid_riff_header() {
        let samples = vec![0.0f32; 16000];
        let wav = encode_wav(&samples).unwrap();
        assert_eq!(&wav[0..4], b"RIFF");
        assert_eq!(&wav[8..12], b"WAVE");
        assert_eq!(&wav[12..16], b"fmt ");
    }

    #[test]
    fn encode_wav_correct_sample_rate_in_header() {
        let samples = vec![0.0f32; 100];
        let wav = encode_wav(&samples).unwrap();
        let sample_rate = u32::from_le_bytes([wav[24], wav[25], wav[26], wav[27]]);
        assert_eq!(sample_rate, 16000);
    }

    #[test]
    fn encode_wav_clamps_out_of_range_samples() {
        let samples = vec![2.0f32, -2.0f32];
        let wav = encode_wav(&samples).unwrap();
        assert_eq!(&wav[0..4], b"RIFF");
        assert!(wav.len() > 44);
    }

    #[test]
    fn encode_wav_empty_input_is_valid() {
        let wav = encode_wav(&[]).unwrap();
        assert_eq!(&wav[0..4], b"RIFF");
    }
}
