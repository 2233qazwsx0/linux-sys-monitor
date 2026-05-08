#!/data/data/com.termux/files/usr/bin/bash
# Termux System Monitor - 简化版一键安装
# 支持直接从 GitHub Release 下载预编译二进制

set -e

VERSION="3.0.0"
BINARY_NAME="termux-monitor"
DOWNLOAD_URL="https://github.com/2233qazwsx0/linux-sys-monitor/releases/download/v${VERSION}/${BINARY_NAME}-${VERSION}-aarch64.apk" 2>/dev/null || true

echo "========================================"
echo "  📱 Termux System Monitor v${VERSION}"
echo "========================================"
echo ""

install_dependencies() {
    echo "[1/3] 检查依赖..."
    
    if ! command -v cargo &> /dev/null; then
        echo "  安装 Rust..."
        pkg update && pkg install -y rust
    fi
    
    if ! command -v termux-battery-status &> /dev/null; then
        echo "  安装 Termux API..."
        pkg update && pkg install -y termux-api
    fi
    
    echo "  依赖检查完成 ✓"
}

clone_and_build() {
    echo "[2/3] 构建程序..."
    
    WORK_DIR="$HOME/linux-sys-monitor-build"
    mkdir -p "$WORK_DIR"
    cd "$WORK_DIR"
    
    if [ ! -d ".git" ]; then
        rm -rf "$WORK_DIR"
        echo "  克隆源码..."
        git clone -b termux --depth 1 https://github.com/2233qazwsx0/linux-sys-monitor.git "$WORK_DIR"
    fi
    
    cd "$WORK_DIR"
    git pull origin termux 2>/dev/null || true
    
    echo "  编译中 (首次可能需要几分钟)..."
    cargo build --release --features termux 2>/dev/null || \
    cargo build --features termux
    
    mkdir -p "$PREFIX/bin"
    cp target/*/release/$BINARY_NAME "$PREFIX/bin/" 2>/dev/null || \
    cp target/release/$BINARY_NAME "$PREFIX/bin/"
    chmod +x "$PREFIX/bin/$BINARY_NAME"
    
    echo "  构建完成 ✓"
}

run_monitor() {
    echo "[3/3] 启动监控..."
    echo ""
    echo "========================================"
    echo "  按 Ctrl+C 退出"
    echo "========================================"
    echo ""
    
    termux-monitor
}

install_dependencies
clone_and_build
run_monitor
