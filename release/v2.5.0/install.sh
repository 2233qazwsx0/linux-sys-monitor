#!/bin/bash
set -e

VERSION="2.5.0"
INSTALL_DIR="/opt/linux-system-monitor"
BINARY_NAME="linux-system-monitor"

echo "======================================"
echo "  Linux System Monitor v${VERSION} Installer"
echo "======================================"

if [ "$EUID" -ne 0 ]; then
    echo "Please run as root: sudo $0"
    exit 1
fi

echo "[1/5] Creating installation directory..."
mkdir -p "$INSTALL_DIR"

echo "[2/5] Installing binary..."
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
cp "$SCRIPT_DIR/$BINARY_NAME" "$INSTALL_DIR/$BINARY_NAME"
chmod +x "$INSTALL_DIR/$BINARY_NAME"

echo "[3/5] Creating systemd service..."
cat > /etc/systemd/system/linux-system-monitor.service << EOF
[Unit]
Description=Linux System Monitor
After=network.target

[Service]
Type=simple
ExecStart=$INSTALL_DIR/$BINARY_NAME
Restart=always
RestartSec=5

[Install]
WantedBy=multi-user.target
EOF

echo "[4/5] Enabling service..."
systemctl daemon-reload
systemctl enable linux-system-monitor.service

echo "[5/5] Starting service..."
systemctl start linux-system-monitor.service

echo ""
echo "======================================"
echo "  Installation Complete!"
echo "======================================"
echo ""
echo "Service:  systemctl start linux-system-monitor"
echo "Stop:     systemctl stop linux-system-monitor"
echo "Status:   systemctl status linux-system-monitor"
echo "Web UI:   http://localhost:8080"
echo ""
echo "======================================"
