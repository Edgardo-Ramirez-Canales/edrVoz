# CLAUDE.md — EDR Voz

## Instrucciones de inicio

Al comenzar cualquier sesión en este proyecto, leer obligatoriamente en este orden:

1. `.claude/memory/.memory.md` — reglas del proyecto e idioma
2. `.claude/memory/status.md` — estado actual de implementación
3. `.claude/memory/decisions.md` — decisiones técnicas ya tomadas

Antes de escribir código, consultar el skill correspondiente en `.claude/skills/`.

---

## Qué es este proyecto

**EDR Voz** es una aplicación de escritorio para Windows que permite dictar texto por voz usando un hotkey global. Al presionar `Ctrl+Shift+J`, la app captura el audio del micrófono; al soltar, transcribe el audio y pega el texto en donde esté el cursor del usuario.

Es un clon funcional de Typeless, construido 100% con tecnología nativa de Windows.

---

## Stack técnico

| Capa       | Tecnología                              |
|------------|-----------------------------------------|
| Framework  | Tauri v2                                |
| Backend    | Rust                                    |
| Frontend   | React 19 + TypeScript                   |
| Hotkey     | tauri-plugin-global-shortcut            |
| Audio      | CPAL 0.17 (PCM f32, 16kHz, mono)        |
| Build      | pnpm + Vite 7                           |
| Target     | Windows 10/11 únicamente                |

---

## Estructura del proyecto

```
C:\edrVoz
├── src/                        # Frontend React + TypeScript
│   ├── App.tsx                 # Componente principal y lógica de UI
│   ├── App.css                 # Estilos (pendiente de completar)
│   └── main.tsx                # Entry point del frontend
├── src-tauri/                  # Backend Rust
│   ├── src/
│   │   ├── lib.rs              # Configuración de Tauri y registro del hotkey
│   │   ├── main.rs             # Entry point del ejecutable
│   │   ├── audio_capture.rs    # Captura de audio con CPAL
│   │   └── hotkey.rs           # OBSOLETO — no modificar, pendiente de eliminar
│   ├── Cargo.toml              # Dependencias Rust
│   └── tauri.conf.json         # Configuración de la app
├── .claude/
│   ├── memory/                 # Memoria persistente del proyecto
│   └── skills/                 # Reglas de trabajo por área
├── CLAUDE.md                   # Este archivo
├── INSTRUCCIONES.md            # Local: guía de compilación (no está en git)
└── README.md                   # Documentación pública
```

---

## Eventos del sistema (contrato Rust → Frontend)

| Evento              | Cuándo se emite                        | Payload |
|---------------------|----------------------------------------|---------|
| `recording-started` | Al presionar Ctrl+Shift+J              | ninguno |
| `recording-stopped` | Al soltar Ctrl+Shift+J                 | ninguno |
| `audio-buffer-ready`| Al terminar captura (pendiente)        | Vec<f32>|

---

## Skills por situación

| Situación                          | Skill                                        |
|------------------------------------|----------------------------------------------|
| Crear o modificar código Rust      | `.claude/skills/architecture-clean-code/`    |
| Hacer un commit                    | `.claude/skills/git-workflow/`               |
| Agregar logging o manejo de errores| `.claude/skills/monitoring-observability/`   |
| Optimizar audio o performance      | `.claude/skills/performance-optimization/`   |
| Integrar API keys o secrets        | `.claude/skills/security/`                   |
| Algo falla                         | `.claude/skills/debugging-troubleshooting/`  |
| CI/CD o compilación                | `.claude/skills/devops/`                     |
| Documentar algo                    | `.claude/skills/documentation/`              |

---

## Lo que NO hacer

- No modificar `hotkey.rs` — está obsoleto
- No sugerir soluciones WSL o cross-platform
- No hacer push directo a `master`
- No dejar rutas absolutas hardcodeadas en el código Rust
- No guardar API keys en el código — van en `.env`
