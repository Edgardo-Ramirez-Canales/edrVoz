import { useState, useEffect } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import "./App.css";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");
  const [isRecording, setIsRecording] = useState(false);
  const [recordingDuration, setRecordingDuration] = useState(0);

  useEffect(() => {
    let unlistenStart: (() => void) | null = null;
    let unlistenStop: (() => void) | null = null;
    let durationInterval: ReturnType<typeof setInterval> | null = null;

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
      if (durationInterval) clearInterval(durationInterval);
    };
  }, []);

  useEffect(() => {
    if (!isRecording) return;

    const interval = setInterval(() => {
      setRecordingDuration((prev) => prev + 0.1);
    }, 100);

    return () => clearInterval(interval);
  }, [isRecording]);

  async function greet() {
    setGreetMsg(await invoke("greet", { name }));
  }

  return (
    <main className="container">
      <h1>Welcome to EDR Voz</h1>

      {isRecording ? (
        <div className="recording-indicator">
          <h2>🎤 Recording...</h2>
          <p className="duration">{recordingDuration.toFixed(1)}s</p>
        </div>
      ) : (
        <p className="hint">Press Ctrl+Shift+J to start recording</p>
      )}

      <div className="row">
        <a href="https://vite.dev" target="_blank">
          <img src="/vite.svg" className="logo vite" alt="Vite logo" />
        </a>
        <a href="https://tauri.app" target="_blank">
          <img src="/tauri.svg" className="logo tauri" alt="Tauri logo" />
        </a>
        <a href="https://react.dev" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>

      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          greet();
        }}
      >
        <input
          id="greet-input"
          onChange={(e) => setName(e.currentTarget.value)}
          placeholder="Enter a name..."
        />
        <button type="submit">Greet</button>
      </form>
      <p>{greetMsg}</p>
    </main>
  );
}

export default App;
