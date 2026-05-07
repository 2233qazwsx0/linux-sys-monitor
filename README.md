# 🐧 Linux System Monitor

<p align="center">
  <img src="https://img.shields.io/badge/Rust-1.75+-orange.svg" alt="Rust">
  <img src="https://img.shields.io/badge/Vue-3.4-green.svg" alt="Vue">
  <img src="https://img.shields.io/badge/Platform-Linux%20%7C%20Windows-blue.svg" alt="Platform">
  <img src="https://img.shields.io/badge/License-MIT-blue.svg" alt="License">
  <img src="https://img.shields.io/github/stars/2233qazwsx0/linux-sys-monitor?style=social" alt="Stars">
</p>

A beautiful, real-time system monitoring dashboard for both Linux and Windows. Monitor CPU, memory, disk I/O, network traffic, and running processes - all in a sleek dark-themed interface.

## ✨ Features

- 📊 **Real-time CPU Monitoring** - Overall usage and per-core statistics
- 💾 **Memory Usage Tracking** - Used, available, and percentage with visual bar
- 📈 **Disk I/O Monitoring** - Read/write rates in bytes per second
- 🌐 **Network Traffic** - Download/upload speed tracking
- 📋 **Process List** - Top 10 processes by CPU usage
- 🔄 **WebSocket Streaming** - Live updates every second
- 🎨 **Beautiful Dark Theme** - Modern, responsive UI
- 📱 **Mobile Friendly** - Works on all screen sizes
- 🐳 **Docker Support** - One-click container deployment
- 🪟 **Windows Support** - Native Windows installer available

## 🚀 Quick Start

### Linux

```bash
# One-line install
curl -sSL https://raw.githubusercontent.com/2233qazwsx0/linux-sys-monitor/main/setup.sh | bash

# Or clone and install
git clone https://github.com/2233qazwsx0/linux-sys-monitor.git
cd linux-system-monitor
chmod +x setup.sh
./setup.sh
./target/release/linux-system-monitor
```

### Windows

#### Option 1: PowerShell Installer (Recommended)

Run in PowerShell as Administrator:

```powershell
irm https://raw.githubusercontent.com/2233qazwsx0/linux-sys-monitor/main/install-windows.ps1 | iex
```

#### Option 2: Manual Installation

1. Download the latest release from [GitHub Releases](https://github.com/2233qazwsx0/linux-sys-monitor/releases)
2. Extract the ZIP file
3. Run `start.bat` or the executable directly
4. Open http://localhost:8080

### Docker

```bash
docker-compose up -d
```

Open [http://localhost:8080](http://localhost:8080) in your browser.

## 🏗️ Architecture

```
┌─────────────────────────────────────────────────────────┐
│                      Browser                            │
│  ┌─────────────────────────────────────────────────┐  │
│  │              Vue.js + ECharts UI                  │  │
│  └─────────────────────────────────────────────────┘  │
│                         ▲                               │
│                         │ WebSocket                     │
└─────────────────────────┼───────────────────────────────┘
                          │
┌─────────────────────────┼───────────────────────────────┐
│                    Rust Server                          │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐   │
│  │   Axum HTTP │  │   WebSocket  │  │   Metrics     │   │
│  │   Server    │  │   Handler    │  │   Collector   │   │
│  └──────────────┘  └──────────────┘  └──────────────┘   │
└─────────────────────────────────────────────────────────┘
                          │
          ┌───────────────┴───────────────┐
          ▼                               ▼
   ┌─────────────┐                ┌─────────────┐
   │    Linux    │                │   Windows   │
   │   /proc     │                │   PDH API   │
   └─────────────┘                └─────────────┘
```

## 🛠️ Tech Stack

- **Backend:** Rust, Axum, Tokio, sysinfo
- **Frontend:** Vue 3, ECharts, Vite
- **Real-time:** WebSocket
- **Container:** Docker

## 📦 Requirements

- **Linux:** Uses /proc filesystem
- **Windows:** Windows 10/11, PowerShell 5.0+
- **Build:** Rust 1.75+ (for building from source)
- **Frontend Build:** Node.js 18+ (optional with pre-built binary)

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 📝 License

This project is licensed under the MIT License.

## ⭐ Show your support

Give a ⭐ if this project helped you!

---

<p align="center">
  Made with ❤️ in Rust
</p>
