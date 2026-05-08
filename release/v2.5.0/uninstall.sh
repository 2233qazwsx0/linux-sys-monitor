#!/bin/bash
set -e

echo "======================================"
echo "  Linux System Monitor Uninstaller"
echo "======================================"

if [ "$EUID" -ne 0 ]; then
    echo "Please run as root: sudo $0"
    exit 1
fi

echo "[1/3] Stopping service..."
systemctl stop linux-system-monitor.service 2>/dev/null || true
systemctl disable linux-system-monitor.service 2>/dev/null || true

echo "[2/3] Removing files..."
rm -f /etc/systemd/system/linux-system-monitor.service
rm -rf /opt/linux-system-monitor

echo "[3/3] Reloading systemd..."
systemctl daemon-reload

echo ""
echo "Uninstallation complete!"
echo ""
