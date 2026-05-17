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
  const [isPasted, setIsPasted] = useState(false);
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
          setIsPasted(true);
          setTimeout(() => setIsPasted(false), 2500);
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
    <main className="max-w-sm mx-auto px-6 py-8 flex flex-col gap-4">
      <h1 className="text-center text-2xl font-bold mb-2">EDR Voz</h1>

      {/* Selector de modo */}
      <div className="flex border border-gray-700 rounded-lg overflow-hidden">
        <button
          className={`flex-1 py-2.5 text-sm cursor-pointer transition-colors ${
            settings.mode === "api"
              ? "bg-white text-gray-900"
              : "text-gray-500 hover:bg-gray-900"
          }`}
          onClick={() => handleModeChange("api")}
        >
          🌐 Online (API)
        </button>
        <button
          className={`flex-1 py-2.5 text-sm cursor-pointer transition-colors ${
            settings.mode === "local"
              ? "bg-white text-gray-900"
              : "text-gray-500 hover:bg-gray-900"
          }`}
          onClick={() => handleModeChange("local")}
        >
          💻 Local
        </button>
      </div>

      {/* Configuración según modo */}
      {settings.mode === "api" ? (
        <div className="flex flex-col gap-2">
          <label className="text-xs font-medium text-gray-500">API Key de OpenAI</label>
          {apiKeyConfigured ? (
            <div className="flex items-center justify-between text-sm text-green-400">
              <span>✅ Configurada correctamente</span>
              <button
                className="text-gray-500 text-xs underline cursor-pointer"
                onClick={handleOpenConfig}
              >
                Editar
              </button>
            </div>
          ) : (
            <div className="flex flex-col gap-2">
              <p className="text-sm text-red-400 m-0">❌ No configurada</p>
              <p className="text-xs text-gray-500 leading-relaxed m-0">
                Archivo de configuración:
                <code className="block bg-gray-900 px-2.5 py-1.5 rounded-md mt-1 break-all text-gray-300">
                  {configPath}
                </code>
              </p>
              <button
                className="px-4 py-2.5 bg-white text-gray-900 text-sm rounded-lg hover:opacity-80 transition-opacity text-left cursor-pointer"
                onClick={handleOpenConfig}
              >
                📝 Abrir archivo de configuración
              </button>
            </div>
          )}
        </div>
      ) : (
        <div className="flex flex-col gap-2">
          <label className="text-xs font-medium text-gray-500">Modelo local (Whisper small)</label>
          {modelStatus === "installed" ? (
            <p className="text-sm text-green-400 m-0">✅ Modelo instalado</p>
          ) : (
            <div className="flex flex-col gap-2">
              <p className="text-sm text-red-400 m-0">❌ Modelo no instalado</p>
              <button
                className="px-4 py-2.5 bg-white text-gray-900 text-sm rounded-lg hover:opacity-80 transition-opacity text-left cursor-pointer"
                onClick={handleDownloadModel}
              >
                ⬇ Descargar modelo (~466 MB)
              </button>
            </div>
          )}
        </div>
      )}

      <hr className="border-t border-gray-800 my-1" />

      {/* Error */}
      {error && (
        <div className="bg-amber-950 border border-amber-700 px-4 py-3 rounded-lg text-amber-400 text-sm">
          <p className="m-0">⚠️ {error}</p>
        </div>
      )}

      {/* Estado principal */}
      {isRecording ? (
        <div className="recording-glow bg-red-950 border-2 border-red-500 p-6 rounded-xl text-center">
          <h2 className="text-red-400 text-xl font-semibold m-0 mb-2 animate-pulse">
            🎤 Grabando...
          </h2>
          <p className="text-4xl font-bold font-mono m-0 tracking-wide text-white">
            {recordingDuration.toFixed(1)}s
          </p>
        </div>
      ) : isTranscribing ? (
        <div className="flex flex-col items-center gap-3 py-6 text-gray-500">
          <div className="w-7 h-7 rounded-full border-[3px] border-gray-700 border-t-gray-400 animate-spin" />
          <p className="m-0 text-sm">Transcribiendo...</p>
        </div>
      ) : transcription ? (
        <div className="relative bg-gray-900 border border-gray-700 rounded-xl px-6 py-5 leading-relaxed slide-up text-gray-100">
          {isPasted && (
            <span className="absolute -top-2.5 right-3.5 bg-green-700 text-white text-xs font-semibold px-3 py-0.5 rounded-full badge-fade">
              ✓ Pegado
            </span>
          )}
          <p className="m-0">{transcription}</p>
        </div>
      ) : (
        <p className="text-center text-gray-600 text-sm my-4">
          Presiona Ctrl+Shift+J para iniciar la grabación
        </p>
      )}
    </main>
  );
}

export default App;
