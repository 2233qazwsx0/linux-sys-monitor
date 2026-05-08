#!/bin/bash
set -e
VERSION="3.0.0"
INSTALL_DIR="/opt/linux-system-monitor"
echo "======================================"
echo "  Linux System Monitor v${VERSION} Installer"
echo "======================================"
if [ "$EUID" -ne 0 ]; then
    echo "Please run as root: sudo $0"
    exit 1
fi
echo "[1/4] Creating installation directory..."
mkdir -p "$INSTALL_DIR"
echo "[2/4] Installing binary..."
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
cp "$SCRIPT_DIR/linux-system-monitor" "$INSTALL_DIR/"
chmod +x "$INSTALL_DIR/linux-system-monitor"
echo "[3/4] Creating systemd service..."
cat > /etc/systemd/system/linux-system-monitor.service << 'EOF'
[Unit]
Description=Linux System Monitor
After=network.target
[Service]
Type=simple
ExecStart=/opt/linux-system-monitor/linux-system-monitor
Restart=always
RestartSec=5
[Install]
WantedBy=multi-user.target
EOF
echo "[4/4] Enabling and starting service..."
systemctl daemon-reload
systemctl enable linux-system-monitor.service
systemctl start linux-system-monitor.service
echo ""
echo "======================================"
echo "  Installation Complete!"
echo "  Web UI: http://localhost:8080"
echo "======================================"
