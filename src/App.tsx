import { useState, useEffect } from "react";
import { listen } from "@tauri-apps/api/event";
import "./App.css";

function App() {
  const [isRecording, setIsRecording] = useState(false);
  const [recordingDuration, setRecordingDuration] = useState(0);

  useEffect(() => {
    let unlistenStart: (() => void) | null = null;
    let unlistenStop: (() => void) | null = null;

    const setupListeners = async () => {
      unlistenStart = await listen("recording-started", () => {
        setIsRecording(true);
        setRecordingDuration(0);
      });

      unlistenStop = await listen("recording-stopped", () => {
        setIsRecording(false);
      });
    };

    setupListeners();

    return () => {
      unlistenStart?.();
      unlistenStop?.();
    };
  }, []);

  useEffect(() => {
    if (!isRecording) return;

    const interval = setInterval(() => {
      setRecordingDuration((prev) => prev + 0.1);
    }, 100);

    return () => clearInterval(interval);
  }, [isRecording]);

  return (
    <main className="container">
      <h1>EDR Voz</h1>

      {isRecording ? (
        <div className="recording-indicator">
          <h2>🎤 Grabando...</h2>
          <p className="duration">{recordingDuration.toFixed(1)}s</p>
        </div>
      ) : (
        <p className="hint">Presiona Ctrl+Shift+J para iniciar la grabación</p>
      )}
    </main>
  );
}

export default App;
