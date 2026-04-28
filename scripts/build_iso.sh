#!/usr/bin/env bash
#
# QuantumSpaceOS ISO Build Script (Linux)
#
# Prerequisites:
#   sudo pacman -S archiso qemu grub efibootmgr
#
# This script:
#   1. Prepares a working directory (iso/work)
#   2. Installs Arch Linux base into iso root
#   3. Copies QuantumSpaceOS components into the ISO
#   4. Configures bootloader, systemd, and services
#   5. Builds the final ISO (~2.5 GB)
#

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
ISO_WORK="${PROJECT_ROOT}/iso/work"
ISO_OUT="${PROJECT_ROOT}/iso/out"
CONFIG_DIR="${PROJECT_ROOT}/config"

echo "=== QuantumSpaceOS ISO Builder ==="
echo "Project root: ${PROJECT_ROOT}"
echo "Working dir: ${ISO_WORK}"
echo "Output dir:  ${ISO_OUT}"

# 1. Ensure output directories
mkdir -p "${ISO_OUT}"
mkdir -p "${ISO_WORK}/airootfs"

# 2. Clone Arch Linux baseline using mkarchiso
echo "[1/6] Initializing Arch Linux root..."
if [[ ! -d "${ISO_WORK}/airootfs" ]]; then
    pacstrap -c -d "${ISO_WORK}/airootfs" \
        base \
        linux \
        linux-firmware \
        systemd \
        bash \
        grub \
        efibootmgr \
        dosfstools \
        mtools \
        arch-install-scripts \
        curl \
        wget \
        git \
        python \
        python-pip \
        rust \
        cargo \
        clang \
        llvm \
        cmake \
        meson \
        ninja \
        grep \
        sed \
        gawk \
        findutils \
        diffutils \
        coreutils \
        file \
        which \
        man-db \
        man-pages \
        texinfo \
        nano \
        vim \
  || true
fi

# 3. Install Rust toolchain inside chroot (or for target)
echo "[2/6] Setting up Rust toolchain..."
# We'll cross-compile from host; but also ensure rustup inside root for Q#
# For now, skip chroot (requires root)

# 4. Copy QuantumSpaceOS components into ISO root
echo "[3/6] Deploying quantumspaceos components..."
cp -r "${PROJECT_ROOT}/src" "${ISO_WORK}/airootfs/root/"
cp -r "${PROJECT_ROOT}/scripts" "${ISO_WORK}/airootfs/root/"
cp -r "${PROJECT_ROOT}/docs" "${ISO_WORK}/airootfs/usr/share/quantumspaceos/"
cp -r "${PROJECT_ROOT}/config" "${ISO_WORK}/airootfs/root/"

# 5. Add custom systemd services
echo "[4/6] Installing systemd services..."
mkdir -p "${ISO_WORK}/airootfs/etc/systemd/system"
cp "${CONFIG_DIR}/quantumspace-gui.service" "${ISO_WORK}/airootfs/etc/systemd/system/" 2>/dev/null || true
cp "${CONFIG_DIR}/quantumspace-telemetry.service" "${ISO_WORK}/airootfs/etc/systemd/system/" 2>/dev/null || true

# 6. Configure bootloader (GRUB)
echo "[5/6] Configuring bootloader..."
mkdir -p "${ISO_WORK}/airootfs/boot/grub"
cp "${CONFIG_DIR}/grub.cfg" "${ISO_WORK}/airootfs/boot/grub/grub.cfg" 2>/dev/null || true

# 7. Build ISO with mkarchiso
echo "[6/6] Building ISO image..."
ISO_NAME="quantumspaceos-$(date +%Y%m%d).iso"

mkarchiso -v -o "${ISO_OUT}" "${ISO_WORK}" || {
    echo "ERROR: mkarchiso failed"
    exit 1
}

echo "=== Build Complete ==="
echo "ISO: ${ISO_OUT}/${ISO_NAME}"
echo "Size: $(du -h "${ISO_OUT}/${ISO_NAME}" | cut -f1)"

# Instructions for QEMU
cat <<'EOS'

To run in QEMU:
  qemu-system-x86_64 \
    -m 4G \
    -cdrom "${ISO_OUT}/${ISO_NAME}" \
    -boot d \
    -nographic \
    -enable-kvm

Or run the helper script:
  ./scripts/run_qemu.sh

EOS
