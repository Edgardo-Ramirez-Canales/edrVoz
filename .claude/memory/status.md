# Estado de Implementación — EDR Voz

**Última actualización:** 2026-05-16
**Rama activa:** `edr-voz-desarrollo`
**Fase actual:** 1.5 — Infraestructura lista, audio pendiente de conectar

---

## Lo que funciona hoy

- App Tauri v2 compila y abre en Windows 10 nativo
- `Ctrl+Shift+J` registrado con `tauri-plugin-global-shortcut`
- Al presionar el hotkey → UI muestra "🎤 Recording..." con timer
- Al soltar el hotkey → UI regresa al estado inicial
- `audio_capture.rs` existe y captura PCM en buffer, pero **nadie lo llama aún**

---

## Fase 1 — Setup + Infraestructura (60% completado)

| Tarea                                      | Estado |
|--------------------------------------------|--------|
| Configurar dependencias en Cargo.toml      | ✅     |
| Crear `audio_capture.rs` con CPAL          | ✅     |
| Crear `hotkey.rs` (manual, obsoleto)       | ✅     |
| Registrar hotkey con plugin global         | ✅     |
| Frontend con listeners y timer             | ✅     |
| Conectar `AudioCapture` al hotkey          | ⏳     |
| Commands Tauri: `get_recording_buffer()`   | ⏳     |
| Commands Tauri: `clear_recording()`        | ⏳     |

---

## Fase 2 — Integración Audio + Hotkey (0% completado)

| Tarea                                               | Estado |
|-----------------------------------------------------|--------|
| Iniciar `AudioCapture` al presionar Ctrl+Shift+J    | ⏳     |
| Detener captura y obtener buffer al soltar          | ⏳     |
| Emitir evento `audio-buffer-ready` al frontend      | ⏳     |
| Manejo de errores: sin micrófono, hook fallido      | ⏳     |
| Threading correcto entre CPAL y Tauri               | ⏳     |

---

## Fase 3 — Transcripción (0% completado)

| Tarea                                               | Estado |
|-----------------------------------------------------|--------|
| Elegir servicio de transcripción                    | ⏳     |
| Integrar API de transcripción                       | ⏳     |
| Enviar buffer de audio al servicio                  | ⏳     |
| Recibir texto transcrito                            | ⏳     |
| Pegar texto en el cursor activo del sistema         | ⏳     |

---

## Fase 4 — Frontend y UX (30% completado)

| Tarea                                               | Estado |
|-----------------------------------------------------|--------|
| Listeners de eventos implementados                  | ✅     |
| Timer de grabación funcionando                      | ✅     |
| Limpiar código de template (greet, logos de Tauri)  | ⏳     |
| CSS: animación pulse en indicador de grabación      | ⏳     |
| Feedback visual mejorado                            | ⏳     |
| Manejo de errores en UI                             | ⏳     |

---

## Fase 5 — Testing y QA (0% completado)

| Tarea                                               | Estado |
|-----------------------------------------------------|--------|
| Testing del hotkey en Windows 10                    | ⏳     |
| Testing de captura de audio                         | ⏳     |
| Edge cases: sin micrófono, grabación larga          | ⏳     |
| Performance: latencia < 100ms, CPU < 20%            | ⏳     |

---

## Deuda técnica conocida

- `hotkey.rs` — archivo obsoleto, no se usa, pendiente de eliminar
- `greet` command en `lib.rs` — código de template de Tauri, pendiente de eliminar
- Logos de Vite/Tauri/React en `App.tsx` — código de template, pendiente de limpiar
- Ruta hardcodeada `C:\edrVoz\shortcut_error.txt` en `lib.rs` — debe reemplazarse por `app.path()`
