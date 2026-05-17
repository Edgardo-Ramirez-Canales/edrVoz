use crate::settings::TranscriptionMode;

pub async fn transcribe(
    samples: Vec<f32>,
    mode: &TranscriptionMode,
    api_key: Option<&str>,
) -> Result<String, String> {
    if samples.is_empty() {
        return Err("No hay audio grabado".to_string());
    }

    match mode {
        TranscriptionMode::Api => transcribe_api(samples, api_key).await,
        TranscriptionMode::Local => {
            Err("Modo local no disponible. Descarga el modelo primero.".to_string())
        }
    }
}

async fn transcribe_api(samples: Vec<f32>, api_key: Option<&str>) -> Result<String, String> {
    let api_key = api_key
        .filter(|k| !k.is_empty())
        .ok_or("API Key no configurada. Edita el archivo config.env.")?;

    let wav_bytes = encode_wav(&samples)?;

    let client = reqwest::Client::new();
    let part = reqwest::multipart::Part::bytes(wav_bytes)
        .file_name("audio.wav")
        .mime_str("audio/wav")
        .map_err(|e| e.to_string())?;

    let form = reqwest::multipart::Form::new()
        .text("model", "whisper-1")
        .text("language", "es")
        .part("file", part);

    let response = client
        .post("https://api.openai.com/v1/audio/transcriptions")
        .header("Authorization", format!("Bearer {}", api_key))
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
