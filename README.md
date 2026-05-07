# 🐧 Linux System Monitor

<p align="center">
  <img src="https://img.shields.io/badge/Rust-1.75+-orange.svg" alt="Rust">
  <img src="https://img.shields.io/badge/Vue-3.4-green.svg" alt="Vue">
  <img src="https://img.shields.io/badge/License-MIT-blue.svg" alt="License">
  <img src="https://img.shields.io/github/stars/2233qazwsx0/linux-sys-monitor?style=social" alt="Stars">
</p>

A beautiful, real-time Linux system monitoring dashboard with live WebSocket streaming. Monitor CPU, memory, disk I/O, network traffic, and running processes - all in a sleek dark-themed interface.

![Dashboard Preview](https://raw.githubusercontent.com/2233qazwsx0/linux-sys-monitor/main/preview.png)

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

## 🚀 Quick Start

### One-Line Install

```bash
curl -sSL https://raw.githubusercontent.com/2233qazwsx0/linux-sys-monitor/main/setup.sh | bash
```

### Manual Installation

```bash
# Clone the repository
git clone https://github.com/2233qazwsx0/linux-sys-monitor.git
cd linux-sys-monitor

# Run installation script
chmod +x setup.sh
./setup.sh

# Start the monitor
./target/release/linux-system-monitor
```

### Docker Deployment

```bash
# Using docker-compose (recommended)
docker-compose up -d

# Or build and run manually
docker build -t linux-system-monitor .
docker run -d -p 8080:8080 --privileged --pid=host linux-system-monitor
```

Open [http://localhost:8080](http://localhost:8080) in your browser.

## 🔧 API Reference

### WebSocket Endpoint

Connect to `ws://localhost:8080/ws` to receive real-time metrics.

**Message Format:**
```json
{
  "timestamp": 1700000000,
  "uptime": 86400,
  "cpu": {
    "usage": 25.5,
    "core_count": 8,
    "per_core": [20.0, 30.0, 25.0, 28.0, 22.0, 18.0, 35.0, 20.0]
  },
  "memory": {
    "total": 16777216,
    "used": 8388608,
    "available": 8388608,
    "usage_percent": 50.0
  },
  "disk": {
    "read_bytes": 1000000,
    "write_bytes": 500000,
    "read_rate": 1024,
    "write_rate": 512
  },
  "network": {
    "rx_bytes": 5000000,
    "tx_bytes": 2000000,
    "rx_rate": 10240,
    "tx_rate": 5120
  },
  "processes": [
    {
      "pid": 1234,
      "name": "chrome",
      "cpu": 15.5,
      "memory": 8.2
    }
  ]
}
```

### REST Endpoints

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/api/health` | GET | Health check, returns "OK" |
| `/api/history` | GET | Historical data endpoint |
| `/` | GET | Web dashboard |
| `/ws` | WebSocket | Real-time metrics stream |

## 🏗️ Architecture

```
┌─────────────────────────────────────────────────────────┐
│                      Browser                            │
│  ┌─────────────────────────────────────────────────┐    │
│  │              Vue.js + ECharts UI                 │    │
│  └─────────────────────────────────────────────────┘    │
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
│                                              │          │
└──────────────────────────────────────────────┼──────────┘
                                               │
                          ┌────────────────────┴───────────┐
                          │        Linux Kernel            │
                          │   sysinfo crate /proc         │
                          └───────────────────────────────┘
```

## 🛠️ Tech Stack

- **Backend:** Rust, Axum, Tokio, sysinfo
- **Frontend:** Vue 3, ECharts, Vite
- **Real-time:** WebSocket
- **Container:** Docker

## 📦 Requirements

- Linux system (uses /proc filesystem)
- Rust 1.75+ (for building from source)
- Node.js 18+ (for frontend build, optional with pre-built binary)

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## ⭐ Show your support

Give a ⭐ if this project helped you!

[![Star History Chart](https://api.star-history.com/svg?repos=2233qazwsx0/linux-sys-monitor&type=Timeline)](https://star-history.com/#2233qazwsx0/linux-sys-monitor&Timeline)

---

<p align="center">
  Made with ❤️ in Rust
</p>
