# QuantumSpaceOS ISO Build Script (Windows PowerShell)
#
# Prerequisites:
#   - WSL2 with Arch Linux installed, OR
#   - Native Linux environment
#
# This is a wrapper that invokes the bash script under WSL if available.

$ErrorActionPreference = "Stop"

$ProjectRoot = Split-Path -Parent $MyInvocation.MyCommand.Path
$ScriptPath = Join-Path $ProjectRoot "build_iso.sh"

Write-Host "=== QuantumSpaceOS ISO Builder (Windows) ===" -ForegroundColor Cyan

# Check for WSL
$wsl = Get-Command wsl -ErrorAction SilentlyContinue
if ($wsl) {
    Write-Host "WSL detected. Building inside WSL2..." -ForegroundColor Yellow
    wsl bash -c "cd '$ProjectRoot' && chmod +x '$ScriptPath' && sudo '$ScriptPath'"
} else {
    Write-Warning "WSL not found. This script requires a Linux environment."
    Write-Host "Please either:" -ForegroundColor Yellow
    Write-Host "  1. Install WSL2 with an Arch Linux distribution" -ForegroundColor Gray
    Write-Host "  2. Run scripts/build_iso.sh directly on Linux" -ForegroundColor Gray
    exit 1
}
