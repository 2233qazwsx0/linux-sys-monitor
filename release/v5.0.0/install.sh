#!/bin/bash

set -e

PROJECT_NAME="linux-system-monitor"
INSTALL_DIR="${HOME}/${PROJECT_NAME}"
CONFIG_DIR="${HOME}/.config/${PROJECT_NAME}"

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
            apt-get install -y build-essential pkg-config libssl-dev curl git
            ;;
        fedora|rhel|centos)
            yum groupinstall -y "Development Tools"
            yum install -y openssl-devel curl git
            ;;
        arch)
            pacman -S --noconfirm base-devel openssl curl git
            ;;
        *)
            echo "Installing basic tools..."
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
        echo "Rust already installed: $(rustc --version | cut -d' ' -f2)"
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
                echo "Node.js not installed, will use pre-built binary"
                ;;
        esac
    else
        echo "Node.js already installed: $(node --version | cut -d'v' -f2)"
    fi
}

build_project() {
    echo ""
    echo "Building project..."
    cargo build --release

    if [ -d "frontend" ] && [ -f "frontend/package.json" ]; then
        echo "Building frontend..."
        cd frontend
        npm install --silent
        npm run build
        cd ..
    fi

    echo ""
    echo "Build completed successfully!"
}

main() {
    detect_os
    install_dependencies
    install_rust
    install_node

    if [[ -f "$HOME/.cargo/env" ]]; then
        source "$HOME/.cargo/env"
    fi

    if [ -d ".git" ]; then
        echo "Pulling latest changes..."
        git pull
    fi

    build_project

    mkdir -p "$CONFIG_DIR"

    if [ ! -f "$CONFIG_DIR/config.toml" ]; then
        cp config.example.toml "$CONFIG_DIR/config.toml" 2>/dev/null || true
    fi

    echo ""
    echo "========================================="
    echo "  Installation Complete!"
    echo "========================================="
    echo ""
    echo "Binary location: ./target/release/linux-system-monitor"
    echo ""
    echo "To run:"
    echo "  ./target/release/linux-system-monitor"
    echo ""
    echo "Then open http://localhost:8080"
    echo ""
}

main "$@"
