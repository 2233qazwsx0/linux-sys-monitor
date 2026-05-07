# 🐧 Linux System Monitor

A beautiful, real-time Linux system monitoring tool with live charts and WebSocket streaming.

![Dashboard Preview](preview.png)

## Features

- 📊 **Real-time CPU monitoring** with per-core usage visualization
- 💾 **Memory usage tracking** with usage percentage and absolute values
- 📈 **Disk I/O monitoring** with read/write rates
- 🔄 **WebSocket streaming** for live data updates (1 second interval)
- 📜 **Historical data** stored in memory ring buffer (1 hour)
- 🎨 **Beautiful UI** with ECharts

## Quick Start

### One-line Install

```bash
curl -sSL https://raw.githubusercontent.com/monitorlinux/linux-system-monitor/main/setup.sh | bash
```

### Manual Installation

1. Install Rust (if not installed):
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. Install Node.js (for frontend build):
```bash
# Ubuntu/Debian
curl -fsSL https://deb.nodesource.com/setup_20.x | sudo bash -
sudo apt-get install -y nodejs

# Arch Linux
sudo pacman -S nodejs npm
```

3. Build and run:
```bash
git clone https://github.com/monitorlinux/linux-system-monitor.git
cd linux-system-monitor
chmod +x setup.sh
./setup.sh
./target/release/linux-system-monitor
```

4. Open http://localhost:8080

## API Reference

### WebSocket Endpoint
```
ws://localhost:8080/ws
```

Receives JSON metrics every second:
```json
{
  "timestamp": 1700000000,
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
  }
}
```

### REST Endpoints

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/api/health` | GET | Health check |
| `/api/history` | GET | Get historical metrics |

## Architecture

```
┌─────────────┐     WebSocket      ┌─────────────┐
│   Browser   │ ◄───────────────── │   Rust      │
│   (Vue.js)  │                    │   Backend   │
└─────────────┘                    └─────────────┘
                                          │
                                          ▼
                                   ┌─────────────┐
                                   │   sysinfo   │
                                   │   crate     │
                                   └─────────────┘
```

## Tech Stack

- **Backend**: Rust + Axum + Tokio
- **Frontend**: Vue 3 + ECharts
- **Real-time**: WebSocket
- **Metrics**: sysinfo crate

## License

MIT
