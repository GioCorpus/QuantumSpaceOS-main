#!/usr/bin/env bash
#
# QuantumSpaceOS QEMU Runner
#
# Launches QEMU with the latest built ISO.
# Requires: qemu-system-x86_64, KVM enabled (optional)
#

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
ISO_DIR="${PROJECT_ROOT}/iso/out"

echo "=== QuantumSpaceOS QEMU Launcher ==="

# Find latest ISO
ISO_FILE=$(ls -t "${ISO_DIR}"/*.iso 2>/dev/null | head -n1)

if [[ -z "$ISO_FILE" ]]; then
    echo "ERROR: No ISO found in ${ISO_DIR}"
    echo "Please build the ISO first: ./scripts/build_iso.sh"
    exit 1
fi

echo "Using ISO: ${ISO_FILE}"

# Check for KVM
KVM_FLAG=""
if [[ -e /dev/kvm ]]; then
    echo "KVM detected — enabling hardware acceleration"
    KVM_FLAG="-enable-kvm"
else
    echo "KVM not available — using software emulation (slow)"
fi

# Memory: 4GB
# CPUs: 2
# Display: nc (no graphics) for headless; use -display gtk for GUI
# Network: user-mode NAT

qemu-system-x86_64 \
    -m 4096 \
    -smp 2 \
    -cdrom "${ISO_FILE}" \
    -boot d \
    -nographic \
    ${KVM_FLAG} \
    -netdev user,id=net0 \
    -device e1000,netdev=net0 \
    -monitor unix:"./qemu-monitor.sock,server,nowait" \
    "$@"
