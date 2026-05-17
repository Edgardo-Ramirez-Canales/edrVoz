import { useState, useEffect } from "react";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function parseError(msg: string): { icon: string; hint: string | null } {
  if (msg.includes("input device") || msg.includes("No input"))
    return { icon: "🎙️", hint: "Verifica que el micrófono esté conectado y con permisos." };
  if (msg.includes("API Key") || msg.includes("config.env"))
    return { icon: "🔑", hint: "Abre el archivo de configuración y agrega tu API Key de OpenAI." };
  if (msg.includes("conexión"))
    return { icon: "🌐", hint: "Verifica tu conexión a internet." };
  if (msg.includes("No hay audio"))
    return { icon: "🎤", hint: "Mantén presionado Ctrl+Shift+J mientras hablas." };
  if (msg.includes("Modo local"))
    return { icon: "💻", hint: "Cambia a modo Online (API) o descarga el modelo local." };
  if (msg.includes("Error de API"))
    return { icon: "🔴", hint: "Revisa que tu API Key sea válida y tenga crédito disponible." };
  return { icon: "⚠️", hint: null };
}

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

  useEffect(() => {
    if (!error) return;
    const timer = setTimeout(() => setError(null), 6000);
    return () => clearTimeout(timer);
  }, [error]);

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
      {error && (() => {
        const { icon, hint } = parseError(error);
        return (
          <div className="flex items-start gap-3 bg-amber-950 border border-amber-700 px-4 py-3 rounded-lg text-sm">
            <span className="text-base mt-0.5 shrink-0">{icon}</span>
            <div className="flex-1 min-w-0">
              <p className="m-0 text-amber-400 font-medium">{error}</p>
              {hint && <p className="m-0 mt-1 text-amber-600 text-xs">{hint}</p>}
            </div>
            <button
              className="text-amber-700 hover:text-amber-400 cursor-pointer transition-colors shrink-0 text-base leading-none"
              onClick={() => setError(null)}
              aria-label="Cerrar"
            >
              ✕
            </button>
          </div>
        );
      })()}

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
