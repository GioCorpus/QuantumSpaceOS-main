#!/usr/bin/env bash
#
# Integration Test Suite for QuantumSpaceOS
#
# Prerequisites:
#   - Built ISO available at iso/out/
#   - QEMU installed
#
# This script runs a headless QEMU session and verifies:
#   - Boot success
#   - Service startup (flight-sim, telemetry)
#   - Basic telemetry output
#

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
ISO_DIR="${PROJECT_ROOT}/iso/out"

# Find latest ISO
ISO=$(ls -t "${ISO_DIR}"/*.iso 2>/dev/null | head -n1)

if [[ -z "$ISO" ]]; then
    echo "FAIL: No ISO found in ${ISO_DIR}"
    echo "Build first: ./scripts/build_iso.sh"
    exit 1
fi

echo "=== QuantumSpaceOS Integration Tests ==="
echo "Using ISO: ${ISO}"

# 1. Smoke test: boot to shell
echo "[1/4] Boot smoke test..."
timeout 120 qemu-system-x86_64 \
    -m 2048 \
    -cdrom "${ISO}" \
    -boot d \
    -nographic \
    -no-reboot \
    -display none \
    -serial stdio 2>&1 | tee /tmp/qemu_boot.log | grep -q "login:" && echo "PASS: System booted to login prompt" || echo "FAIL: Boot did not reach login"

# 2. Flight-sim runs without panic
echo "[2/4] Flight-sim executable test..."
# We'd need to automate serial interaction; placeholder
echo "SKIP: requires serial automation"

# 3. Telemetry API reachable
echo "[3/4] Telemetry API test..."
# placeholder; requires networking options
echo "SKIP: requires user networking"

# 4. Quantum core RPC (future)
echo "[4/4] Quantum core integration..."
echo "SKIP: awaiting hardware stub"

echo ""
echo "=== Test Summary ==="
echo "Not all tests implemented yet. Run manual validation:"
echo "  ./scripts/run_qemu.sh"
