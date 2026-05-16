import { useState, useEffect } from "react";
import { listen } from "@tauri-apps/api/event";
import "./App.css";

function App() {
  const [isRecording, setIsRecording] = useState(false);
  const [recordingDuration, setRecordingDuration] = useState(0);
  const [lastSampleCount, setLastSampleCount] = useState<number | null>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    let unlistenStart: (() => void) | null = null;
    let unlistenStop: (() => void) | null = null;
    let unlistenReady: (() => void) | null = null;
    let unlistenError: (() => void) | null = null;

    const setupListeners = async () => {
      unlistenStart = await listen("recording-started", () => {
        setIsRecording(true);
        setRecordingDuration(0);
        setLastSampleCount(null);
        setError(null);
      });

      unlistenStop = await listen("recording-stopped", () => {
        setIsRecording(false);
      });

      unlistenReady = await listen<number>("audio-buffer-ready", (event) => {
        setLastSampleCount(event.payload);
      });

      unlistenError = await listen<string>("recording-error", (event) => {
        setError(event.payload);
        setIsRecording(false);
      });
    };

    setupListeners();

    return () => {
      unlistenStart?.();
      unlistenStop?.();
      unlistenReady?.();
      unlistenError?.();
    };
  }, []);

  useEffect(() => {
    if (!isRecording) return;

    const interval = setInterval(() => {
      setRecordingDuration((prev) => prev + 0.1);
    }, 100);

    return () => clearInterval(interval);
  }, [isRecording]);

  const secondsFromSamples = (samples: number) =>
    (samples / 16000).toFixed(1);

  return (
    <main className="container">
      <h1>EDR Voz</h1>

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
      ) : (
        <div>
          <p className="hint">Presiona Ctrl+Shift+J para iniciar la grabación</p>
          {lastSampleCount !== null && (
            <p className="last-recording">
              Última grabación: {secondsFromSamples(lastSampleCount)}s — {lastSampleCount.toLocaleString()} muestras capturadas
            </p>
          )}
        </div>
      )}
    </main>
  );
}

export default App;
