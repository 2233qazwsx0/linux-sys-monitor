# 📊 System Monitor

<p align="center">
  <img src="https://img.shields.io/badge/Rust-1.75+-orange.svg" alt="Rust">
  <img src="https://img.shields.io/badge/Vue-3.4-green.svg" alt="Vue">
  <img src="https://img.shields.io/badge/Platform-Linux%20%7C%20Windows-blue.svg" alt="Platform">
  <img src="https://img.shields.io/badge/License-MIT-blue.svg" alt="License">
  <img src="https://img.shields.io/github/stars/2233qazwsx0/linux-sys-monitor?style=social" alt="Stars">
  <img src="https://img.shields.io/badge/Version-2.0.0-6366f1.svg" alt="Version">
</p>

一款支持 **Linux** 和 **Windows** 的实时系统监控仪表板。监控 CPU、内存、磁盘 I/O、网络流量和进程 - 所有功能都在一个时尚的暗色主题界面中。

## ✨ 功能特性

- 📊 **实时 CPU 监控** - 总体使用率和每核心统计
- 💾 **内存使用追踪** - 已用/可用/百分比，带可视化条
- 📈 **磁盘 I/O 监控** - 读写速率（字节/秒）
- 🌐 **网络流量** - 下载/上传速度追踪
- 📋 **进程列表** - 按 CPU 使用率排序的 Top 进程
- 🔄 **WebSocket 实时推送** - 每秒更新
- 🎨 **精美暗色主题** - 现代响应式界面
- 🌍 **中英文支持** - 自动检测浏览器语言
- 📱 **移动端适配** - 响应式设计
- 🐳 **Docker 支持** - 一键容器部署
- 🪟 **Windows 支持** - 原生 Windows 安装器

## 🚀 快速开始

### Linux 一键安装

```bash
curl -sSL https://raw.githubusercontent.com/2233qazwsx0/linux-sys-monitor/main/setup.sh | bash
```

### Windows PowerShell 安装

```powershell
irm https://raw.githubusercontent.com/2233qazwsx0/linux-sys-monitor/main/install-windows.ps1 | iex
```

### Docker 部署

```bash
docker-compose up -d
```

然后打开 http://localhost:8080

## 🏗️ 技术架构

```
┌─────────────────────────────────────────────────────────┐
│                      Browser                            │
│  ┌─────────────────────────────────────────────────┐  │
│  │           Vue.js + ECharts UI                    │  │
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
```

## 🛠️ 技术栈

- **后端:** Rust, Axum, Tokio, sysinfo
- **前端:** Vue 3, ECharts, Vite
- **实时:** WebSocket
- **容器:** Docker

## 📝 许可证

MIT License

---

<p align="center">
  Made with ❤️ in Rust
</p>
