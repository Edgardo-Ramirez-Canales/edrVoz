# EDR Voz

Aplicación de escritorio para Windows que permite dictar texto por voz usando un hotkey global. Presiona `Ctrl+Shift+J`, habla, suelta la tecla y el texto transcrito aparece donde esté el cursor.

## Requisitos

| Herramienta | Versión mínima |
|---|---|
| Windows | 10 / 11 |
| Rust + Cargo | 1.95+ |
| Node.js | 20+ |
| pnpm | 9+ |
| Visual Studio Build Tools | 2022 (Desktop development with C++) |
| Windows SDK | 10.0.26100+ |

## Instalación

```powershell
git clone https://github.com/Edgardo-Ramirez-Canales/edrVoz.git
cd edrVoz
pnpm install
```

## Ejecutar en modo desarrollo

```powershell
pnpm tauri dev
```

## Compilar

```powershell
pnpm tauri build
```

El instalador queda en `src-tauri/target/release/bundle/`.

## Configuración

Copia `config.env.example` como `config.env` en el mismo directorio que el ejecutable y agrega tu API Key de OpenAI:

```powershell
# En src-tauri\target\release\ (producción) o src-tauri\target\debug\ (desarrollo)
copy config.env.example config.env
# Luego edita config.env y agrega tu API Key
```

Alternativamente, define la variable de entorno del sistema:

```powershell
[System.Environment]::SetEnvironmentVariable("OPENAI_API_KEY", "sk-...", "User")
```

Ver `config.env.example` para la lista completa de variables.

## Uso

1. Abre la aplicación
2. Presiona y mantén `Ctrl+Shift+J`
3. Habla (máximo 60 segundos por grabación)
4. Suelta las teclas — el texto transcrito se pega automáticamente en el cursor activo

## Stack

- [Tauri v2](https://tauri.app) — framework de escritorio
- [React 19](https://react.dev) + TypeScript + Tailwind CSS v4 — interfaz
- [CPAL](https://github.com/RustAudio/cpal) — captura de audio (16kHz mono f32)
- [OpenAI Whisper API](https://platform.openai.com/docs/guides/speech-to-text) — transcripción
- Rust — backend, captura de audio y pegado automático

## Logs

Durante el desarrollo los logs se muestran en la terminal. Cada grabación tiene un ID de sesión para correlacionar los eventos:

```
[INFO] EDR Voz iniciando
[INFO] [sesión 1] Grabación iniciada
[INFO] [sesión 1] Grabación detenida — 51200 samples
[INFO] [sesión 1] Transcripción iniciada — 51200 samples, modo: api
[INFO] [sesión 1] Transcripción completada en 1234ms (42 chars)
```

## Estado del proyecto

v0.1.0 — funcional. Transcripción online vía OpenAI Whisper API operativa. Modo local (Whisper offline) pendiente para versión futura.
