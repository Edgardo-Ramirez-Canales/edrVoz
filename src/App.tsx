import { useState, useEffect } from "react";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function parseError(msg: string): { icon: string; hint: string } {
  if (msg.includes("input device") || msg.includes("No input"))
    return { icon: "🎙️", hint: "Sin micrófono" };
  if (msg.includes("API Key") || msg.includes("config.env"))
    return { icon: "🔑", hint: "API Key no configurada" };
  if (msg.includes("conexión"))
    return { icon: "🌐", hint: "Sin conexión" };
  if (msg.includes("No hay audio"))
    return { icon: "🎤", hint: "Mantén presionado mientras hablas" };
  if (msg.includes("Modo local"))
    return { icon: "💻", hint: "Modelo no instalado" };
  return { icon: "⚠️", hint: msg };
}

function App() {
  const [isRecording, setIsRecording] = useState(false);
  const [recordingDuration, setRecordingDuration] = useState(0);
  const [isTranscribing, setIsTranscribing] = useState(false);
  const [transcription, setTranscription] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);

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
        await listen("recording-stopped", () => setIsRecording(false)),
        await listen("transcribing", () => setIsTranscribing(true)),
        await listen<string>("transcription-ready", (e) => {
          setIsTranscribing(false);
          setTranscription(e.payload);
          setTimeout(() => invoke("hide_window"), 2500);
        }),
        await listen<string>("transcription-error", (e) => {
          setIsTranscribing(false);
          setError(e.payload);
          setTimeout(() => invoke("hide_window"), 4000);
        }),
        await listen<string>("recording-error", (e) => {
          setIsRecording(false);
          setError(e.payload);
          setTimeout(() => invoke("hide_window"), 4000);
        }),
      );
    };
    setup();
    return () => unlisteners.forEach((u) => u());
  }, []);

  useEffect(() => {
    if (!isRecording) return;
    const interval = setInterval(() => setRecordingDuration((p) => p + 0.1), 100);
    return () => clearInterval(interval);
  }, [isRecording]);

  useEffect(() => {
    if (isRecording && recordingDuration >= 60) {
      invoke("force_stop_recording");
    }
  }, [isRecording, recordingDuration]);

  const handleCancel = () => {
    invoke("cancel_recording");
    invoke("hide_window");
  };

  const handleStop = () => invoke("force_stop_recording");

  const isActive = isRecording || isTranscribing || !!transcription || !!error;
  const parsedError = error ? parseError(error) : null;

  return (
    <div className="flex items-center justify-center w-screen h-screen">
      <div className={`hud flex items-center justify-between px-4 gap-3 ${isActive ? "fade-in" : ""}`}>

        {/* Botón cancelar */}
        <button
          className="w-7 h-7 rounded-full flex items-center justify-center text-gray-500 hover:text-white hover:bg-white/10 transition-colors shrink-0 cursor-pointer text-xs"
          onClick={handleCancel}
          title="Cancelar"
        >
          ✕
        </button>

        {/* Zona central */}
        <div className="flex-1 flex items-center justify-center min-w-0 overflow-hidden">
          {isRecording ? (
            <div className="flex items-center gap-1.5">
              {[0, 1, 2, 3, 4].map((i) => (
                <div key={i} className="wave-bar" style={{ animationDelay: `${i * 0.1}s` }} />
              ))}
              <span className="ml-2 text-xs text-gray-400 font-mono tabular-nums">
                {recordingDuration.toFixed(1)}s
              </span>
            </div>
          ) : isTranscribing ? (
            <div className="flex items-center gap-2 text-gray-400">
              <div className="w-3.5 h-3.5 rounded-full border-2 border-gray-600 border-t-gray-300 animate-spin shrink-0" />
              <span className="text-xs">Transcribiendo...</span>
            </div>
          ) : transcription ? (
            <div className="flex items-center gap-2 min-w-0">
              <span className="text-green-400 text-xs shrink-0">✓</span>
              <p className="text-xs text-gray-200 truncate m-0">{transcription}</p>
            </div>
          ) : error ? (
            <div className="flex items-center gap-1.5 min-w-0">
              <span className="text-sm shrink-0">{parsedError?.icon}</span>
              <p className="text-xs text-amber-300 truncate m-0">{parsedError?.hint}</p>
            </div>
          ) : (
            // Estado idle: ondas atenuadas
            <div className="flex items-center gap-1.5">
              {[0, 1, 2, 3, 4].map((i) => (
                <div key={i} className="wave-bar opacity-20" style={{ animationDelay: `${i * 0.1}s` }} />
              ))}
            </div>
          )}
        </div>

        {/* Botón detener/confirmar */}
        <button
          className={`w-7 h-7 rounded-full flex items-center justify-center transition-colors shrink-0 text-xs ${
            isRecording
              ? "text-white bg-white/15 hover:bg-white/25 cursor-pointer"
              : "text-gray-700 cursor-default"
          }`}
          onClick={isRecording ? handleStop : undefined}
          disabled={!isRecording}
          title="Detener y transcribir"
        >
          ✓
        </button>

      </div>
    </div>
  );
}

export default App;
