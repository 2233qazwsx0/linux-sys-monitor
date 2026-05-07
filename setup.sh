#!/bin/bash

set -e

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
    echo "Installing system dependencies..."

    case $OS in
        ubuntu|debian)
            apt-get update
            apt-get install -y build-essential pkg-config libssl-dev curl git npm
            ;;
        fedora|rhel|centos)
            yum groupinstall -y "Development Tools"
            yum install -y openssl-devel curl git
            ;;
        arch)
            pacman -S --noconfirm base-devel openssl curl git npm
            ;;
        *)
            echo "Installing basic tools..."
            ;;
    esac
}

install_rust() {
    if ! command -v rustc &> /dev/null; then
        echo "Installing Rust..."
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source "$HOME/.cargo/env" 2>/dev/null || true
    fi
}

main() {
    detect_os
    install_dependencies
    install_rust

    source "$HOME/.cargo/env" 2>/dev/null || true

    echo "Building frontend..."
    cd frontend && npm install && npm run build && cd ..

    echo "Building backend..."
    cargo build --release

    chmod +x target/release/linux-system-monitor

    echo ""
    echo "Installation complete!"
    echo "Run: ./target/release/linux-system-monitor"
    echo "Open: http://localhost:8080"
}

main "$@"
