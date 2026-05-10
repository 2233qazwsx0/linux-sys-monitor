#!/data/data/com.termux/files/usr/bin/bash
# Termux System Monitor v5.0.0 - 50 Mobile Features
# 简化版一键安装脚本

set -e

VERSION="5.0.0"
BINARY_NAME="termux-monitor"

echo "========================================"
echo "  📱 Termux System Monitor v${VERSION}"
echo "  🔧 50 Mobile-Optimized Features"
echo "========================================"
echo ""

check_dependencies() {
    echo "[1/4] 检查依赖..."
    
    if ! command -v cargo &> /dev/null; then
        echo "  安装 Rust 工具链..."
        pkg update && pkg install -y rust
    fi
    
    if ! command -v termux-battery-status &> /dev/null; then
        echo "  安装 Termux API (电池/传感器等功能需要)..."
        pkg update && pkg install -y termux-api
    fi
    
    if ! command -v git &> /dev/null; then
        echo "  安装 Git..."
        pkg update && pkg install -y git
    fi
    
    echo "  依赖检查完成 ✓"
}

clone_and_build() {
    echo "[2/4] 构建程序..."
    
    WORK_DIR="$HOME/linux-sys-monitor-build"
    mkdir -p "$WORK_DIR"
    cd "$WORK_DIR"
    
    if [ ! -d ".git" ]; then
        rm -rf "$WORK_DIR"
        echo "  克隆源码仓库 (termux 分支)..."
        git clone -b termux --depth 1 https://github.com/2233qazwsx0/linux-sys-monitor.git "$WORK_DIR"
    fi
    
    cd "$WORK_DIR"
    git fetch origin termux
    git reset --hard origin/termux
    
    echo "  编译中 (首次可能需要几分钟)..."
    cargo build --release --features termux 2>/dev/null || \
    cargo build --features termux
    
    mkdir -p "$PREFIX/bin"
    cp target/*/release/$BINARY_NAME "$PREFIX/bin/" 2>/dev/null || \
    cp target/release/$BINARY_NAME "$PREFIX/bin/"
    chmod +x "$PREFIX/bin/$BINARY_NAME"
    
    echo "  构建完成 ✓"
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
clone_and_build
show_features
run_monitor
