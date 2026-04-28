# QuantumSpaceOS QEMU Launcher (Windows PowerShell)
#
# Prerequisites:
#   - QEMU installed and in PATH (choco install qemu)
#   - WSL2 with built ISO (or Linux-native build)
#
# This script launches QEMU with the latest ISO from iso/out/

$ErrorActionPreference = "Stop"

$ProjectRoot = Split-Path -Parent $MyInvocation.MyCommand.Path
$IsoDir = Join-Path $ProjectRoot "iso/out"

Write-Host "=== QuantumSpaceOS QEMU Launcher (Windows) ===" -ForegroundColor Cyan

# Find latest ISO
try {
    $IsoFile = Get-ChildItem $IsoDir -Filter "*.iso" | Sort-Object LastWriteTime -Descending | Select-Object -First 1
} catch {
    Write-Error "No ISO found in $IsoDir. Build first with .\scripts\build_iso.ps1"
    exit 1
}

if (-not $IsoFile) {
    Write-Error "ISO not found. Run build script first."
    exit 1
}

Write-Host "Using ISO: $($IsoFile.FullName)"

# Check if QEMU exists
$qemu = Get-Command qemu-system-x86_64 -ErrorAction SilentlyContinue
if (-not $qemu) {
    Write-Error "qemu-system-x86_64 not found in PATH."
    Write-Host "Install via: choco install qemu" -ForegroundColor Yellow
    exit 1
}

# Construct arguments
$Memory = 4096   # MB
$Smp = 2
$Cdrom = $IsoFile.FullName

Write-Host "Starting QEMU (4GB RAM, 2 vCPUs)..." -ForegroundColor Green

& qemu-system-x86_64 `
    -m $Memory `
    -smp $Smp `
    -cdrom $Cdrom `
    -boot d `
    -nographic `
    -netdev user,id=net0 `
    -device e1000,netdev=net0
