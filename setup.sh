#!/bin/bash
set -e

VERSION="v1.0.0"
INSTALL_DIR="${MONITOR_HOME:-$HOME/linux-system-monitor}"
PORT="${MONITOR_PORT:-8080}"

echo "🔧 Linux System Monitor Installer"
echo "================================"

detect_os() {
    if [[ -f /etc/os-release ]]; then
        . /etc/os-release
        echo "📦 Detected: $NAME"
    fi
}

install_deps() {
    echo "📥 Installing dependencies..."

    if command -v apt-get &> /dev/null; then
        apt-get update -qq && apt-get install -y -qq curl tar > /dev/null 2>&1
    elif command -v yum &> /dev/null; then
        yum install -y -q curl tar > /dev/null 2>&1
    elif command -v pacman &> /dev/null; then
        pacman -Sy --noconfirm curl tar > /dev/null 2>&1
    elif command -v dnf &> /dev/null; then
        dnf install -y -q curl tar > /dev/null 2>&1
    fi

    echo "✅ Dependencies installed"
}

download_and_install() {
    echo "📥 Downloading v$VERSION..."
    mkdir -p "$INSTALL_DIR"
    cd "$INSTALL_DIR"

    curl -sL "https://github.com/2233qazwsx0/linux-sys-monitor/releases/download/$VERSION/linux-system-monitor-${VERSION}-linux-x86_64.tar.gz" -o monitor.tar.gz

    echo "📦 Extracting..."
    tar -xzf monitor.tar.gz
    rm monitor.tar.gz
    chmod +x linux-system-monitor

    echo "✅ Installed to $INSTALL_DIR"
}

create_service() {
    echo "🔧 Creating systemd service..."

    SERVICE_FILE="/etc/systemd/system/linux-system-monitor.service"

    sudo tee "$SERVICE_FILE" > /dev/null << EOF
[Unit]
Description=Linux System Monitor
After=network.target

[Service]
Type=simple
User=$USER
WorkingDirectory=$INSTALL_DIR
Environment="MONITOR_PORT=$PORT"
ExecStart=$INSTALL_DIR/linux-system-monitor
Restart=always
RestartSec=5

[Install]
WantedBy=multi-user.target
EOF

    sudo systemctl daemon-reload
    echo "✅ Service created"
}

main() {
    detect_os
    install_deps
    download_and_install

    if command -v systemctl &> /dev/null; then
        create_service
        echo ""
        echo "🚀 To start:"
        echo "   sudo systemctl start linux-system-monitor"
        echo "   sudo systemctl enable linux-system-monitor  # auto-start"
    else
        echo ""
        echo "🚀 To run:"
        echo "   cd $INSTALL_DIR && ./linux-system-monitor"
    fi

    echo ""
    echo "🌐 Open: http://localhost:$PORT"
    echo ""
    echo "📝 Environment variables:"
    echo "   MONITOR_HOME  - Installation directory (default: ~/linux-system-monitor)"
    echo "   MONITOR_PORT  - Server port (default: 8080)"
    echo ""
}

main
