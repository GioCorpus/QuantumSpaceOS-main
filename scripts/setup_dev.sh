#!/usr/bin/env bash
#
# QuantumSpaceOS Development Environment Setup
#
# Installs:
#   - Rust toolchain (stable + components)
#   - .NET SDK (for Q#)
#   - Python dependencies
#   - Arch Linux build dependencies
#   - pre-commit hooks
#

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"

echo "=== QuantumSpaceOS Dev Environment Setup ==="

# --- OS detection ---
OS="unknown"
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    OS="linux"
elif [[ "$OSTYPE" == "darwin"* ]]; then
    OS="macos"
elif [[ "$OSTYPE" == "cygwin" ]] || [[ "$OSTYPE" == "msys" ]]; then
    OS="windows"
fi
echo "Detected OS: ${OS}"

# --- Rust ---
echo "[1/5] Installing Rust toolchain..."
if ! command -v cargo &>/dev/null; then
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable
    source "$HOME/.cargo/env"
else
    echo "Rust already installed: $(cargo --version)"
fi

# Add WASM target (for future WebAssembly GUI)
rustup target add wasm32-unknown-unknown

# Optional: nightly for certain features
rustup install nightly
rustup component add rustfmt --toolchain nightly

# --- .NET SDK (for Q#) ---
echo "[2/5] Checking .NET SDK..."
if ! command -v dotnet &>/dev/null; then
    echo ".NET SDK not found. Please install from: https://dotnet.microsoft.com/download"
    echo "Q# support requires .NET 8+ SDK."
    read -p "Install now? (y/n) " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        if [[ "$OS" == "linux" ]]; then
            wget https://dot.net/v1/dotnet-install.sh -O /tmp/dotnet-install.sh
            bash /tmp/dotnet-install.sh --channel 8.0
        elif [[ "$OS" == "macos" ]]; then
            brew install --cask dotnet-sdk
        fi
    fi
else
    echo ".NET SDK: $(dotnet --version)"
fi

# Q# extension
if command -v dotnet &>/dev/null; then
    dotnet new -i "Microsoft.Quantum.ProjectTemplates::0.1.*" 2>/dev/null || true
fi

# --- Python ---
echo "[3/5] Setting up Python virtualenv..."
if [[ ! -d "${PROJECT_ROOT}/.venv" ]]; then
    python3 -m venv "${PROJECT_ROOT}/.venv"
fi
source "${PROJECT_ROOT}/.venv/bin/activate"

echo "Installing Python dependencies..."
pip install --upgrade pip
pip install -r "${PROJECT_ROOT}/src/api/requirements.txt"

# --- System packages (Linux only) ---
if [[ "$OS" == "linux" ]]; then
    echo "[4/5] Installing system dependencies..."

    # Detect distro
    if command -v pacman &>/dev/null; then
        # Arch-based
        sudo pacman -S --needed \
            base-devel \
            git \
            curl \
            wget \
            python \
            python-pip \
            rust \
            qemu \
            archiso \
            grub \
            efibootmgr \
            dosfstools \
            mtools \
            wayland \
            wayland-protocols \
            meson \
            ninja \
            cmake \
            glslang \
            vulkan-headers || true
    elif command -v apt-get &>/dev/null; then
        # Debian/Ubuntu
        sudo apt-get update
        sudo apt-get install -y \
            build-essential \
            git \
            curl \
            wget \
            python3 \
            python3-pip \
            rustc \
            cargo \
            qemu-system-x86 \
            grub-pc-bin \
            grub-efi-amd64-bin \
            mtools \
            dosfstools \
            wayland-protocols \
            meson \
            ninja-build \
            cmake \
            glslang-tools \
            vulkan-sdk || true
    fi
fi

# --- Pre-commit hooks ---
echo "[5/5] Installing pre-commit hooks..."
if command -v pre-commit &>/dev/null; then
    pre-commit install
else
    pip install pre-commit
    pre-commit install
fi

echo ""
echo "=== Setup Complete ==="
echo ""
echo "Next steps:"
echo "  1. Build ISO:    ./scripts/build_iso.sh (Linux) or .\\scripts\\build_iso.ps1 (Windows/WSL)"
echo "  2. Run in QEMU:  ./scripts/run_qemu.sh"
echo "  3. Run API:      source .venv/bin/activate && python src/api/telemetry_api.py"
echo "  4. Run GUI:      cargo run -p wayland-gui --release"
echo ""
echo "Happy hacking! 🚀"
