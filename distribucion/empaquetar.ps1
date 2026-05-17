# empaquetar.ps1
# Actualiza la carpeta de distribución con el ejecutable más reciente.
# Correr desde la raíz del proyecto: .\distribucion\empaquetar.ps1

$version = "0.1.0"
$carpetaDist = "distribucion\EDR-Voz-v$version"
$exeOrigen   = "src-tauri\target\release\edrvoz.exe"

Write-Host ""
Write-Host "EDR Voz — Empaquetador v$version" -ForegroundColor Cyan
Write-Host "=================================" -ForegroundColor Cyan

# Verificar que el exe existe
if (-not (Test-Path $exeOrigen)) {
    Write-Host ""
    Write-Host "ERROR: No se encontró el ejecutable en:" -ForegroundColor Red
    Write-Host "  $exeOrigen" -ForegroundColor Red
    Write-Host ""
    Write-Host "Compila primero el proyecto con:" -ForegroundColor Yellow
    Write-Host "  pnpm tauri build" -ForegroundColor Yellow
    Write-Host ""
    exit 1
}

# Copiar exe
Write-Host ""
Write-Host "Copiando edrvoz.exe..." -ForegroundColor White
Copy-Item $exeOrigen "$carpetaDist\edrvoz.exe" -Force

$tamano = (Get-Item "$carpetaDist\edrvoz.exe").Length / 1MB
Write-Host "  OK — $([math]::Round($tamano, 1)) MB" -ForegroundColor Green

Write-Host ""
Write-Host "Distribución lista en: $carpetaDist" -ForegroundColor Green
Write-Host ""
Write-Host "Próximos pasos:" -ForegroundColor Yellow
Write-Host "  1. Abre $carpetaDist\config.env y agrega la API Key de OpenAI"
Write-Host "  2. Verifica que LEEME.txt esté actualizado"
Write-Host "  3. Haz commit: git add distribucion/ && git commit"
Write-Host "  4. Comparte la carpeta $carpetaDist (zipéala o súbela al repo)"
Write-Host ""
