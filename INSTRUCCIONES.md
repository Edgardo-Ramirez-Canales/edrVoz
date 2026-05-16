# EDR Voz — Instrucciones de Configuración y Ejecución

## Requisitos instalados (Windows nativo)

| Herramienta | Versión | Cómo se instaló |
|---|---|---|
| Rust + Cargo | 1.95.0 | `winget install Rustlang.Rustup` |
| pnpm | 11.x | `npm install -g pnpm` |
| Visual Studio Build Tools 2022 | 17.14 | `winget install Microsoft.VisualStudio.2022.BuildTools` |
| Windows SDK | 10.0.26100 | `winget install Microsoft.WindowsSDK.10.0.26100` |
| WebView2 Runtime | 148.x | `winget install Microsoft.EdgeWebView2Runtime` |
| Node.js | 24.x | Ya instalado |

---

## Directorio del proyecto

```
C:\edrVoz
```

---

## Ejecutar la app (modo rápido)

```powershell
& "C:\edrVoz\src-tauri\target\release\edrvoz.exe"
```

---

## Compilar y ejecutar (cuando hay cambios en el código)

Abre PowerShell en `C:\edrVoz` y ejecuta el siguiente bloque completo:

```powershell
# 1. Situarse en el proyecto
cd C:\edrVoz

# 2. Configurar el entorno MSVC (necesario en cada sesión nueva de PowerShell)
$env:PATH = [System.Environment]::GetEnvironmentVariable("PATH", "Machine") + ";" + [System.Environment]::GetEnvironmentVariable("PATH", "User")
$msvcVer = "14.44.35207"
$sdkVer  = "10.0.26100.0"
$env:PATH = "C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Tools\MSVC\$msvcVer\bin\Hostx64\x64;$env:PATH"
$env:LIB  = "C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Tools\MSVC\$msvcVer\lib\x64;C:\Program Files (x86)\Windows Kits\10\Lib\$sdkVer\um\x64;C:\Program Files (x86)\Windows Kits\10\Lib\$sdkVer\ucrt\x64"

# 3. Compilar
pnpm tauri build

# 4. Ejecutar
& "C:\edrVoz\src-tauri\target\release\edrvoz.exe"
```

> **Nota:** La primera compilación tarda ~10 min. Las siguientes son incrementales (~2 min).

---

## Modo desarrollo (hot reload del frontend)

```powershell
cd C:\edrVoz

$env:PATH = [System.Environment]::GetEnvironmentVariable("PATH", "Machine") + ";" + [System.Environment]::GetEnvironmentVariable("PATH", "User")
$msvcVer = "14.44.35207"
$sdkVer  = "10.0.26100.0"
$env:PATH = "C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Tools\MSVC\$msvcVer\bin\Hostx64\x64;$env:PATH"
$env:LIB  = "C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Tools\MSVC\$msvcVer\lib\x64;C:\Program Files (x86)\Windows Kits\10\Lib\$sdkVer\um\x64;C:\Program Files (x86)\Windows Kits\10\Lib\$sdkVer\ucrt\x64"

pnpm tauri dev
```

---

## Instaladores generados

Después de cada `pnpm tauri build`, los instaladores quedan en:

```
C:\edrVoz\src-tauri\target\release\bundle\
  ├── msi\edrvoz_0.1.0_x64_en-US.msi      ← instalador MSI
  └── nsis\edrvoz_0.1.0_x64-setup.exe     ← instalador NSIS
```

---

## Hotkey de la app

| Acción | Teclas |
|---|---|
| Iniciar / detener grabación | `Ctrl + Shift + J` |

---

## Estructura del proyecto

```
C:\edrVoz
├── src\                        # Frontend React + TypeScript
│   ├── App.tsx                 # Componente principal + lógica de UI
│   ├── App.css
│   └── main.tsx
├── src-tauri\                  # Backend Rust
│   ├── src\
│   │   ├── lib.rs              # Entrada principal + registro de hotkey
│   │   ├── main.rs             # Entry point del ejecutable
│   │   ├── audio_capture.rs   # Módulo de captura de audio (cpal)
│   │   └── hotkey.rs          # Hook manual (reemplazado por plugin)
│   ├── Cargo.toml              # Dependencias Rust
│   └── tauri.conf.json         # Configuración de la app
├── package.json                # Dependencias JS
├── vite.config.ts
└── INSTRUCCIONES.md            # Este archivo
```

---

## Dependencias clave

### Rust (`src-tauri/Cargo.toml`)
- `tauri 2` — framework de la app
- `tauri-plugin-global-shortcut 2` — hotkeys globales
- `tauri-plugin-opener 2` — abrir URLs/archivos
- `cpal 0.17` — captura de audio
- `windows 0.58` — WinAPI

### JavaScript (`package.json`)
- `react 19` + `react-dom`
- `@tauri-apps/api 2` — bridge JS↔Rust
- `vite 7` + `typescript`

---

## Git

```powershell
# Ver historial
git log --oneline

# Guardar cambios
git add .
git commit -m "descripción del cambio"
```
