# 📊 System Monitor

<p align="center">
  <a href="https://github.com/2233qazwsx0/linux-sys-monitor/releases">
    <img src="https://img.shields.io/badge/Version-2.5.0-6366f1?style=flat-square" alt="Version">
  </a>
  <a href="https://github.com/2233qazwsx0/linux-sys-monitor/blob/main/LICENSE">
    <img src="https://img.shields.io/badge/License-MIT-blue?style=flat-square" alt="License">
  </a>
  <a href="https://github.com/2233qazwsx0/linux-sys-monitor/stargazers">
    <img src="https://img.shields.io/github/stars/2233qazwsx0/linux-sys-monitor?style=flat-square" alt="Stars">
  </a>
  <a href="https://github.com/2233qazwsx0/linux-sys-monitor/network/members">
    <img src="https://img.shields.io/github/forks/2233qazwsx0/linux-sys-monitor?style=flat-square" alt="Forks">
  </a>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/Rust-1.75+-orange?style=flat-square&logo=rust" alt="Rust">
  <img src="https://img.shields.io/badge/Vue-3.4-green?style=flat-square&logo=vue.js" alt="Vue">
  <img src="https://img.shields.io/badge/Platform-Linux%20%7C%20Windows-blue?style=flat-square" alt="Platform">
</p>

<p align="center">
  🌍 中文 | <a href="README.md">English</a>
</p>

---

一款**超轻量级**的实时系统监控工具，支持 Linux 和 Windows。零依赖部署，一键安装，内存占用不到 10MB！

## ✨ 特性

| 特性 | 说明 |
|------|------|
| 📊 **实时监控** | CPU、内存、Swap 每秒更新 |
| 💾 **磁盘监控** | I/O 读写速率 + 各分区空间 |
| 🌐 **网络监控** | 下载/上传速度实时追踪 |
| 🔋 **电池状态** | 笔记本电池电量（支持自动检测）|
| 📋 **进程列表** | Top 15 进程按 CPU 排序 |
| 🚨 **告警系统** | 可配置 CPU/内存告警阈值 |
| 📤 **数据导出** | 支持 JSON/CSV 格式导出历史数据 |
| 🎨 **主题切换** | 深色/浅色主题一键切换 |
| 🌍 **中英双语** | 自动检测浏览器语言 |
| 📱 **响应式** | 完美适配桌面和移动端 |
| 🐳 **Docker** | 一行命令启动 |
| 🪟 **Windows** | 原生 PowerShell 安装器 |

## 🚀 快速开始

### Linux 一键安装

```bash
curl -sSL https://raw.githubusercontent.com/2233qazwsx0/linux-sys-monitor/main/setup.sh | bash
```

### Windows PowerShell

```powershell
irm https://raw.githubusercontent.com/2233qazwsx0/linux-sys-monitor/main/install-windows.ps1 | iex
```

### Docker

```bash
git clone https://github.com/2233qazwsx0/linux-sys-monitor.git
cd linux-sys-monitor
docker-compose up -d
```

### 直接下载

从 [Releases](https://github.com/2233qazwsx0/linux-sys-monitor/releases/latest) 下载预编译二进制：

```bash
wget https://github.com/2233qazwsx0/linux-sys-monitor/releases/download/v2.5.0/linux-system-monitor-2.5.0-linux-x64.tar.gz
tar -xzf linux-system-monitor-*.tar.gz
cd v2.5.0 && sudo ./install.sh
```

然后打开 **http://localhost:8080**

## 📷 截图

```
┌─────────────────────────────────────────────────────────────┐
│  📊 系统监控                    ubuntu  Live ●  [EN ▼]    │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐    │
│  │ ⚡ CPU    │ │ 🧠 内存   │ │ 💿 Swap  │ │ 💾 磁盘I/O│    │
│  │  25.3%   │ │  62.1%   │ │   0.0%   │ │ ↓1.2MB/s │    │
│  │ ████░░░░ │ │ ██████░░ │ │ ░░░░░░░░ │ │ ↑256KB/s │    │
│  │ 4核 25%  │ │ 8G/16G   │ │ 0G/2G    │ │           │    │
│  └──────────┘ └──────────┘ └──────────┘ └──────────┘    │
│                                                             │
│  ⚠️ 告警                                                    │
│  ┌─────────────────────────────────────────────────────┐  │
│  │ 🔴 CPU 使用率超过阈值: 90.0% (当前: 92.3%)         │  │
│  └─────────────────────────────────────────────────────┘  │
│                                                             │
│  💿 磁盘空间                                                │
│  ┌─────────────────┐ ┌─────────────────┐                   │
│  │ /dev/sda1  /   │ │ /dev/sda2  /home│                   │
│  │ ████████░░ 78% │ │ ████░░░░░ 45%  │                   │
│  │ 312G / 400G    │ │ 450G / 1TB     │                   │
│  └─────────────────┘ └─────────────────┘                   │
│                                                             │
│  ┌──────────────────┐ ┌──────────────────┐                 │
│  │ CPU 使用率         │ │ 内存使用率         │                 │
│  │ ▁▂▃▄▅▆▇█▇▆▅▄▃▂▁ │ │ ▁▁▂▂▃▃▄▄▅▅▆▆▇▇ │                 │
│  └──────────────────┘ └──────────────────┘                 │
│                                                             │
│  📋 热门进程                                                │
│  ┌────┬────────┬────────┬────────┐                         │
│  │ PID │ 名称    │ CPU %  │ 内存 % │                         │
│  ├────┼────────┼────────┼────────┤                         │
│  │1234│ chrome │  15.2  │   8.3  │                         │
│  │5678│ code   │   8.1  │   5.2  │                         │
│  └────┴────────┴────────┴────────┘                         │
└─────────────────────────────────────────────────────────────┘
```

## ⚡ 性能对比

| 指标 | System Monitor | Prometheus | Grafana |
|------|---------------|------------|---------|
| 二进制大小 | **1.5 MB** | ~120 MB | ~300 MB |
| 内存占用 | **< 10 MB** | ~200 MB | ~500 MB |
| 启动时间 | **< 1s** | ~5s | ~10s |
| 依赖 | 0 | 需要多服务 | 需要多服务 |

## 🛠️ 技术栈

```
Frontend   Vue 3 + ECharts + Vite
Backend    Rust + Axum + Tokio
Metrics    sysinfo crate
Protocol   WebSocket
```

## 📦 环境变量

| 变量 | 默认值 | 说明 |
|------|--------|------|
| `MONITOR_PORT` | `8080` | 服务端口 |
| `MONITOR_HOME` | `~/linux-system-monitor` | 安装目录 |
| `RUST_LOG` | `info` | 日志级别 |

## 🌐 API 接口

### WebSocket 实时数据

```
ws://localhost:8080/ws
```

数据格式：
```json
{
  "timestamp": 1700000000,
  "uptime": 86400,
  "hostname": "ubuntu",
  "os_version": "Ubuntu 22.04",
  "kernel": "5.15.0",
  "cpu": {
    "name": "Intel i7-12700K",
    "usage": 25.3,
    "core_count": 12,
    "per_core": [20.0, 30.0, ...],
    "frequencies": [3600, 3500, ...]
  },
  "memory": {
    "total": 16777216,
    "used": 10485760,
    "available": 6291456,
    "usage_percent": 62.5
  },
  "swap": {
    "total": 2097152,
    "used": 0,
    "usage_percent": 0.0
  },
  "disk": {
    "read_rate": 1048576,
    "write_rate": 262144
  },
  "disks": [...],
  "network": {
    "rx_bytes": 5000000,
    "tx_bytes": 2000000,
    "rx_rate": 10240,
    "tx_rate": 5120
  },
  "processes": [...],
  "battery": null
}
```

### REST API

| 端点 | 方法 | 说明 |
|------|------|------|
| `/api/health` | GET | 健康检查 |
| `/api/history` | GET | 历史数据 (最近1小时) |
| `/api/alerts` | GET | 获取当前告警状态 |
| `/api/alerts/config` | POST | 更新告警配置 |
| `/api/export` | GET | 导出历史数据 (JSON) |

### 告警配置

```json
POST /api/alerts/config
{
  "cpu_threshold": 90.0,
  "memory_threshold": 85.0,
  "disk_threshold": 95.0
}
```

## 🏗️ 项目结构

```
linux-system-monitor/
├── src/
│   ├── main.rs           # 入口
│   ├── api/              # HTTP/WebSocket
│   └── metrics/          # 系统指标采集
├── frontend/              # Vue.js 前端
│   └── src/components/   # 图表组件
├── Dockerfile
├── docker-compose.yml
├── setup.sh              # Linux 安装脚本
└── install-windows.ps1   # Windows 安装脚本
```

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

## 📱 Termux 支持

本项目支持在 Android 设备上通过 Termux 运行！

### 功能特性

| 特性 | 说明 |
|------|------|
| 🔋 **电池监控** | 电量、温度、健康状态、充电状态 |
| 🌡️ **温度监控** | CPU 温度（通过 /sys 或 termux-sensor） |
| 📊 **系统信息** | 内存、存储、CPU 型号 |
| 📈 **进程管理** | Top 进程 CPU 占用 |
| 💡 **触觉反馈** | 支持 termux-vibrate 震动反馈 |
| 🎨 **ANSI 色彩** | 适配 80x24 小终端 |

### 安装 Termux API（可选但推荐）

```bash
pkg install termux-api
```

### 构建

```bash
# 方法一：使用构建脚本
chmod +x build-termux.sh
./build-termux.sh

# 方法二：手动构建
rustup target add aarch64-linux-android
cargo build --features termux --target aarch64-linux-android --release
```

### 使用

```bash
# 在 Termux 中运行
termux-monitor

# 或通过 adb 在 Android 设备上运行
adb push target/aarch64-linux-android/release/termux-monitor /data/local/tmp/
adb shell chmod 755 /data/local/tmp/termux-monitor
adb shell /data/local/tmp/termux-monitor
```

### 截图预览

```
╔══════════════════════════════════════════════════════════════════╗
║  ◆ Termux System Monitor    android    5d 12h 34m               ║
╚══════════════════════════════════════════════════════════════════╝
┌─ Battery & Temperature ────────────────────────────────────────────┐
│ ⚡ 65%  ████████████████████░░░  │ Temperature: 32.5°C │
│ Health: good | Status: charging | Temp: 32.5°C │
└───────────────────────────────────────────────────────────────────┘
┌─ CPU (Qualcomm Snapdragon 888) ─────────────────────────────────┐
│ ⚡ Usage: 23.5%   █████░░░░░░░░░░░░░░░  │ 8 cores │
└───────────────────────────────────────────────────────────────────┘
┌─ Memory (2.1G/8.0G) ───────────────────────────────────────────────┐
│ 💾 RAM:  26.3%  █████░░░░░░░░░░░░░░░  │ Cached: 1.2G │
└───────────────────────────────────────────────────────────────────┘
┌─ Storage ─────────────────────────────────────────────────────────┐
│ 📦 Used: 45.2%  █████████░░░░░░░░░░░░  │ 92.5G/205.0G │
└───────────────────────────────────────────────────────────────────┘
┌─ Top Processes ────────────────────────────────────────────────────┐
│    PID │ Name                     │   CPU % │
│────────┼──────────────────────────┼──────────│
│   1234 │ com.android.chrome       │   15.2% │
│   5678 │ com.termux.x11           │    8.1% │
└───────────────────────────────────────────────────────────────────┘
```

### 键盘快捷键

| 快捷键 | 功能 |
|--------|------|
| `Ctrl+C` | 退出程序 |

### 注意事项

1. 部分功能需要 Termux API 支持
2. 电池信息需要 `termux-battery-status` 命令
3. 温度读取会尝试多个路径，Android 设备可能需要 root 权限
4. 进程列表可能因 Android 版本不同而有差异

## 📄 许可证

本项目基于 MIT 许可证开源 - 详见 [LICENSE](LICENSE)

## ⭐ 如果对你有帮助

如果这个项目对你有帮助，请给我一个 Star！

[![Star History](https://api.star-history.com/svg?repos=2233qazwsx0/linux-sys-monitor&type=Timeline)](https://star-history.com/#2233qazwsx0/linux-sys-monitor&Timeline)

---

<p align="center">
  用 ❤️ 和 <a href="https://www.rust-lang.org/">Rust</a> 构建
</p>
