# 📊 System Monitor

<p align="center">
  <a href="https://github.com/2233qazwsx0/linux-sys-monitor/releases">
    <img src="https://img.shields.io/badge/Version-3.0.0-6366f1?style=flat-square" alt="Version">
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
  <img src="https://img.shields.io/badge/Binary-649KB-00ff00?style=flat-square" alt="Binary Size">
  <img src="https://img.shields.io/badge/Platform-Linux%20%7C%20Windows%20%7C%20Android-9cf?style=flat-square" alt="Platform">
</p>

---

一款**超轻量级**的实时系统监控工具，支持多平台！

## ✨ 核心特性

| 特性 | Web版 | CLI版 | Termux版 |
|------|-------|-------|----------|
| CPU监控 | ✅ | ✅ | ✅ |
| 内存监控 | ✅ | ✅ | ✅ |
| 磁盘监控 | ✅ | ✅ | ✅ |
| 网络监控 | ✅ | ✅ | ❌ |
| 温度监控 | ✅ | ✅ | ✅ |
| 电池监控 | ✅ | ✅ | ✅ |
| 告警系统 | ✅ | ❌ | ❌ |
| Web界面 | ✅ | ❌ | ❌ |

## 🚀 快速开始

### Web 模式（推荐）

```bash
# 一键安装
curl -sSL https://raw.githubusercontent.com/2233qazwsx0/linux-sys-monitor/main/setup.sh | bash

# 或下载解压
wget https://github.com/2233qazwsx0/linux-sys-monitor/releases/download/v3.0.0/linux-system-monitor-3.0.0-linux-x64.tar.gz
tar -xzf linux-system-monitor-3.0.0-linux-x64.tar.gz
cd v3.0.0 && sudo ./install.sh
```

打开 **http://localhost:8080**

### CLI 模式

```bash
# 构建 CLI 版本
cargo build --release --features cli -p linux-system-monitor --bin cli

# 运行
./target/release/cli
```

### Termux 模式（Android）

```bash
# 安装 Termux API
pkg install termux-api

# 构建 Termux 版本
chmod +x build-termux.sh
./build-termux.sh

# 运行
termux-monitor
```

## 📦 性能指标

| 指标 | v3.0.0 | 竞品对比 |
|------|--------|----------|
| 二进制大小 | **649 KB** | Prometheus ~120MB |
| 内存占用 | **< 5 MB** | Grafana ~500MB |
| 启动时间 | **< 1s** | Prometheus ~5s |
| 依赖数量 | **0** | 需要多服务 |

## 🛠️ 技术栈

```
Frontend   Vue 3 + ECharts + Vite
Backend    Rust + Axum + Tokio
Metrics    sysinfo crate
Protocol   WebSocket
```

## 📁 分支说明

| 分支 | 说明 |
|------|------|
| `main` | Web 版本（推荐）|
| `cli` | 纯终端 CLI 版本 |
| `termux` | Android Termux 版本 |

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

## 📄 许可证

MIT License - 详见 [LICENSE](LICENSE)

---

<p align="center">
  用 ❤️ 和 <a href="https://www.rust-lang.org/">Rust</a> 构建
</p>
