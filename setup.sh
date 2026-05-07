#!/bin/bash

set -e

PROJECT_NAME="linux-system-monitor"
INSTALL_DIR="$(cd "$(dirname "$0")" && pwd)"
FRONTEND_DIR="${INSTALL_DIR}/frontend"

echo "========================================="
echo "  Linux System Monitor Installer"
echo "========================================="
echo ""

detect_os() {
    if [[ -f /etc/os-release ]]; then
        . /etc/os-release
        OS=$ID
    else
        echo "Cannot detect OS"
        exit 1
    fi
    echo "Detected OS: $OS"
}

install_dependencies() {
    echo ""
    echo "Installing system dependencies..."

    case $OS in
        ubuntu|debian)
            apt-get update
            apt-get install -y build-essential pkg-config libssl-dev curl
            ;;
        fedora|rhel|centos)
            yum groupinstall -y "Development Tools"
            yum install -y openssl-devel curl
            ;;
        arch)
            pacman -S --noconfirm base-devel openssl curl
            ;;
        *)
            echo "Unknown OS. Please install build essentials manually:"
            echo "  - build-essential (gcc, make)"
            echo "  - pkg-config"
            echo "  - OpenSSL development libraries"
            ;;
    esac
}

install_rust() {
    if ! command -v rustc &> /dev/null; then
        echo ""
        echo "Installing Rust..."
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        if [[ -f "$HOME/.cargo/env" ]]; then
            source "$HOME/.cargo/env"
        fi
    else
        echo "Rust already installed: $(rustc --version)"
    fi
}

install_node() {
    if ! command -v node &> /dev/null; then
        echo ""
        echo "Installing Node.js..."
        case $OS in
            ubuntu|debian)
                curl -fsSL https://deb.nodesource.com/setup_20.x | bash -
                apt-get install -y nodejs
                ;;
            fedora|rhel|centos)
                curl -fsSL https://rpm.nodesource.com/setup_20.x | bash -
                yum install -y nodejs
                ;;
            arch)
                pacman -S --noconfirm nodejs npm
                ;;
            *)
                echo "Please install Node.js manually"
                ;;
        esac
    else
        echo "Node.js already installed: $(node --version)"
    fi
}

build_frontend() {
    echo ""
    echo "Building frontend..."
    cd "$FRONTEND_DIR"
    npm install
    npm run build
    echo "Frontend built successfully!"
}

build_backend() {
    echo ""
    echo "Building backend..."
    cd "$INSTALL_DIR"
    cargo build --release
    echo "Backend built successfully!"
}

main() {
    detect_os
    install_dependencies
    install_rust
    install_node

    if [[ -f "$HOME/.cargo/env" ]]; then
        source "$HOME/.cargo/env"
    fi

    build_frontend
    build_backend

    echo ""
    echo "========================================="
    echo "  Installation Complete!"
    echo "========================================="
    echo ""
    echo "To run the monitor:"
    echo "  cd $INSTALL_DIR"
    echo "  ./target/release/linux-system-monitor"
    echo ""
    echo "Then open http://localhost:8080 in your browser"
    echo ""
}

main "$@"
