import { useState, useEffect } from "react";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

interface Settings {
  mode: "api" | "local";
}

function App() {
  const [isRecording, setIsRecording] = useState(false);
  const [recordingDuration, setRecordingDuration] = useState(0);
  const [isTranscribing, setIsTranscribing] = useState(false);
  const [transcription, setTranscription] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [settings, setSettings] = useState<Settings>({ mode: "api" });
  const [apiKeyConfigured, setApiKeyConfigured] = useState(false);
  const [configPath, setConfigPath] = useState("");
  const [modelStatus, setModelStatus] = useState<"installed" | "not_installed">("not_installed");

  const loadStatus = () => {
    invoke<Settings>("get_settings").then(setSettings);
    invoke<boolean>("get_api_key_status").then(setApiKeyConfigured);
    invoke<string>("get_config_path").then(setConfigPath);
    invoke<string>("get_model_status").then((s) =>
      setModelStatus(s as "installed" | "not_installed")
    );
  };

  useEffect(() => {
    loadStatus();
  }, []);

  useEffect(() => {
    const unlisteners: (() => void)[] = [];

    const setup = async () => {
      unlisteners.push(
        await listen("recording-started", () => {
          setIsRecording(true);
          setRecordingDuration(0);
          setTranscription(null);
          setError(null);
        }),
        await listen("recording-stopped", () => {
          setIsRecording(false);
        }),
        await listen("transcribing", () => {
          setIsTranscribing(true);
        }),
        await listen<string>("transcription-ready", (e) => {
          setIsTranscribing(false);
          setTranscription(e.payload);
        }),
        await listen<string>("transcription-error", (e) => {
          setIsTranscribing(false);
          setError(e.payload);
        }),
        await listen<string>("recording-error", (e) => {
          setIsRecording(false);
          setError(e.payload);
        }),
      );
    };

    setup();
    return () => unlisteners.forEach((u) => u());
  }, []);

  useEffect(() => {
    if (!isRecording) return;
    const interval = setInterval(() => {
      setRecordingDuration((prev) => prev + 0.1);
    }, 100);
    return () => clearInterval(interval);
  }, [isRecording]);

  const handleModeChange = async (mode: "api" | "local") => {
    setSettings({ mode });
    await invoke("save_settings", { mode });
  };

  const handleOpenConfig = async () => {
    try {
      await invoke("open_config_file");
      // Recargar estado después de un momento para capturar cambios
      setTimeout(loadStatus, 2000);
    } catch (e) {
      setError(e as string);
    }
  };

  const handleDownloadModel = async () => {
    try {
      await invoke("download_model");
    } catch (e) {
      setError(e as string);
    }
  };

  return (
    <main className="container">
      <h1>EDR Voz</h1>

      <div className="mode-switch">
        <button
          className={settings.mode === "api" ? "active" : ""}
          onClick={() => handleModeChange("api")}
        >
          🌐 Online (API)
        </button>
        <button
          className={settings.mode === "local" ? "active" : ""}
          onClick={() => handleModeChange("local")}
        >
          💻 Local
        </button>
      </div>

      {settings.mode === "api" ? (
        <div className="settings-section">
          <label>API Key de OpenAI</label>
          {apiKeyConfigured ? (
            <div className="api-key-status configured">
              <span>✅ Configurada correctamente</span>
              <button className="link-btn" onClick={handleOpenConfig}>
                Editar
              </button>
            </div>
          ) : (
            <div className="api-key-missing">
              <p className="status-missing">❌ No configurada</p>
              <p className="config-hint">
                Archivo de configuración:<br />
                <code>{configPath}</code>
              </p>
              <button className="action-btn" onClick={handleOpenConfig}>
                📝 Abrir archivo de configuración
              </button>
            </div>
          )}
        </div>
      ) : (
        <div className="settings-section">
          <label>Modelo local (Whisper small)</label>
          {modelStatus === "installed" ? (
            <p className="status-ok">✅ Modelo instalado</p>
          ) : (
            <div className="model-not-installed">
              <p className="status-missing">❌ Modelo no instalado</p>
              <button className="action-btn" onClick={handleDownloadModel}>
                ⬇ Descargar modelo (~466 MB)
              </button>
            </div>
          )}
        </div>
      )}

      <div className="divider" />

      {error && (
        <div className="error-indicator">
          <p>⚠️ {error}</p>
        </div>
      )}

      {isRecording ? (
        <div className="recording-indicator">
          <h2>🎤 Grabando...</h2>
          <p className="duration">{recordingDuration.toFixed(1)}s</p>
        </div>
      ) : isTranscribing ? (
        <div className="transcribing-indicator">
          <p>⏳ Transcribiendo...</p>
        </div>
      ) : transcription ? (
        <div className="transcription-result">
          <p>{transcription}</p>
        </div>
      ) : (
        <p className="hint">Presiona Ctrl+Shift+J para iniciar la grabación</p>
      )}
    </main>
  );
}

export default App;
