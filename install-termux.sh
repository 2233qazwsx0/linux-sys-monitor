#!/data/data/com.termux/files/usr/bin/bash
# Termux System Monitor v5.0.0 - 50 Mobile Features
# 预编译二进制文件一键安装脚本

set -e

VERSION="5.0.0"
BINARY_NAME="termux-monitor"
GITHUB_REPO="2233qazwsx0/linux-sys-monitor"
RELEASE_TAG="v${VERSION}"

echo "========================================"
echo "  📱 Termux System Monitor v${VERSION}"
echo "  🔧 50 Mobile-Optimized Features"
echo "========================================"
echo ""

check_dependencies() {
    echo "[1/4] 检查依赖..."

    if ! command -v curl &> /dev/null; then
        echo "  安装 curl..."
        pkg update && pkg install -y curl
    fi

    if ! command -v termux-battery-status &> /dev/null; then
        echo "  安装 Termux API (电池/传感器等功能需要)..."
        pkg update && pkg install -y termux-api
    fi

    echo "  依赖检查完成 ✓"
}

download_binary() {
    echo "[2/4] 下载预编译二进制文件..."

    BINARY_URL="https://github.com/${GITHUB_REPO}/releases/download/${RELEASE_TAG}/${BINARY_NAME}"
    TEMP_DIR=$(mktemp -d)

    echo "  从 ${BINARY_URL} 下载..."

    if curl -L -o "$TEMP_DIR/$BINARY_NAME" "$BINARY_URL"; then
        chmod +x "$TEMP_DIR/$BINARY_NAME"
        echo "  下载完成 ✓"
    else
        echo "  从release下载失败，尝试直接从仓库下载..."
        RAW_URL="https://raw.githubusercontent.com/${GITHUB_REPO}/termux/release/${BINARY_NAME}"
        curl -L -o "$TEMP_DIR/$BINARY_NAME" "$RAW_URL"
        chmod +x "$TEMP_DIR/$BINARY_NAME"
    fi

    mkdir -p "$PREFIX/bin"
    cp "$TEMP_DIR/$BINARY_NAME" "$PREFIX/bin/"
    rm -rf "$TEMP_DIR"

    echo "  安装完成 ✓"
}

show_features() {
    echo "[3/4] v5.0.0 功能列表:"
    echo ""
    echo "  🔋 Battery: 电量、温度、健康状态、电压、电流"
    echo "  🧠 Memory: RAM使用率、缓存、活跃/非活跃、Swap"
    echo "  💾 Storage: 内部存储、外部存储、Inodes"
    echo "  ⚡ CPU: 使用率、核心数、频率范围、温度"
    echo "  📶 Network: 实时网速、WiFi信息(SSID/信号强度)、数据使用量"
    echo "  📊 Processes: Top进程列表(CPU/内存排序)"
    echo "  📱 Sensors: 传感器列表"
    echo "  🔦 Torch: 手电筒控制"
    echo "  📈 System: 负载平均值、运行时间、主机名"
    echo ""
}

run_monitor() {
    echo "[4/4] 启动监控..."
    echo ""
    echo "========================================"
    echo "  快捷键:"
    echo "  Ctrl+C - 退出程序"
    echo "  V       - 振动"
    echo "  T       - 开关手电筒"
    echo "  L       - 显示负载平均值"
    echo "  Q       - 退出"
    echo "========================================"
    echo ""

    termux-monitor
}

check_dependencies
download_binary
show_features
run_monitor
