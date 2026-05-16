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

## Uso

1. Abre la aplicación
2. Presiona y mantén `Ctrl+Shift+J`
3. Habla
4. Suelta las teclas — el texto transcrito se pega en el cursor activo

## Stack

- [Tauri v2](https://tauri.app) — framework de escritorio
- [React 19](https://react.dev) + TypeScript — interfaz de usuario
- [CPAL](https://github.com/RustAudio/cpal) — captura de audio
- Rust — backend y lógica del sistema

## Estado del proyecto

En desarrollo activo. Ver estado detallado de implementación en `.claude/memory/status.md`.
