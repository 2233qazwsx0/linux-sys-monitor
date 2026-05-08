#!/bin/bash
set -e
VERSION="4.0.0"
INSTALL_DIR="/opt/linux-system-monitor"
echo "======================================"
echo "  Linux System Monitor v${VERSION} Installer"
echo "======================================"
if [ "$EUID" -ne 0 ]; then
    echo "Please run as root: sudo $0"
    exit 1
fi
mkdir -p "$INSTALL_DIR"
cp linux-system-monitor "$INSTALL_DIR/"
chmod +x "$INSTALL_DIR/linux-system-monitor"
cat > /etc/systemd/system/linux-system-monitor.service << 'SRV'
[Unit]
Description=Linux System Monitor v4.0.0
After=network.target
[Service]
Type=simple
ExecStart=/opt/linux-system-monitor/linux-system-monitor
Restart=always
RestartSec=5
[Install]
WantedBy=multi-user.target
SRV
systemctl daemon-reload
systemctl enable linux-system-monitor.service
systemctl start linux-system-monitor.service
echo "======================================"
echo "  Installation Complete!"
echo "  Web UI: http://localhost:8080"
echo "======================================"
