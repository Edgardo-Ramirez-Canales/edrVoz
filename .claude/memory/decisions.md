# Decisiones Técnicas — EDR Voz

## Hotkey: plugin vs hook manual

**Decisión:** Usar `tauri-plugin-global-shortcut` en lugar del hook manual `WH_KEYBOARD_LL`.

**Por qué:** El hook manual (`hotkey.rs`) requería unsafe Rust, manejo manual de threads y un message loop propio. El plugin encapsula todo eso, es mantenido por el equipo de Tauri y es más estable en Windows 10/11.

**Consecuencia:** `hotkey.rs` quedó en el proyecto pero ya no se usa. Puede eliminarse.

---

## Hotkey: Ctrl+Shift+J

**Decisión:** `Ctrl+Shift+J` como combinación de grabación.

**Por qué:** `Win+J` estaba ocupado por una aplicación del sistema en el equipo de desarrollo. Se descartó `Ctrl+Win+D` por conflictos similares.

---

## Audio: CPAL @ 16kHz mono f32

**Decisión:** Capturar audio PCM f32, sample rate 16kHz, 1 canal (mono).

**Por qué:** 16kHz es el formato nativo que esperan los modelos de transcripción (Whisper y similares). Capturar a mayor resolución solo aumentaría el tamaño del buffer sin beneficio real para la transcripción de voz.

---

## Stack: Tauri v2 + React 19 + Rust

**Decisión:** Tauri v2 como framework de la app de escritorio.

**Por qué:** Permite escribir el backend en Rust (rendimiento, acceso a WinAPI) y el frontend en React (velocidad de desarrollo UI). El binario resultante es nativo de Windows y significativamente más liviano que Electron.

---

## Windows-only

**Decisión:** El proyecto no soportará Linux ni Mac en ninguna fase.

**Por qué:** El caso de uso es dictar texto en aplicaciones Windows. Las dependencias de WinAPI y el comportamiento del hotkey son inherentemente Windows. No tiene sentido el esfuerzo de portabilidad.

---

## Transcripción: pendiente de definir

**Decisión:** Aún no se ha elegido el servicio de transcripción.

**Opciones evaluadas:**
- Whisper local (sin costo, privacidad total, requiere GPU o es lento en CPU)
- Azure Speech (costo por minuto, baja latencia, requiere API key)
- OpenAI Whisper API (costo por minuto, simple de integrar)

**Criterio de decisión:** Se definirá cuando se implemente la Fase 2. Prioridad: latencia baja y sin necesidad de GPU dedicada.
