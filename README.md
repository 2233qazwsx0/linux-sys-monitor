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
</p>

<p align="center">
  <img src="https://img.shields.io/badge/Rust-1.75+-orange?style=flat-square&logo=rust" alt="Rust">
  <img src="https://img.shields.io/badge/Vue-3.4-green?style=flat-square&logo=vue.js" alt="Vue">
  <img src="https://img.shields.io/badge/Binary-649KB-00ff00?style=flat-square" alt="Binary Size">
</p>

---

一款**超轻量级**的实时系统监控工具，支持 Linux/Windows + Web/CLI 两种模式！

## ✨ 核心特性

| 特性 | 说明 |
|------|------|
| 📊 **实时监控** | CPU、内存、Swap、网络、磁盘 I/O |
| 🌡️ **温度监控** | CPU/GPU 温度实时监测 |
| 📈 **负载平均** | 1/5/15 分钟系统负载 |
| 🔥 **进程管理** | Top 进程、进程终止、排序过滤 |
| 🚨 **告警系统** | 可配置阈值，实时告警通知 |
| 📤 **数据导出** | JSON/CSV 历史数据导出 |
| 🎨 **主题切换** | 深色/浅色主题 |
| 🌐 **网络详情** | 端口、连接状态、DNS、网关 |
| 📱 **响应式** | 完美适配桌面和移动端 |

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

### CLI 模式（轻量终端）

```bash
# 构建 CLI 版本
cargo build --release --features cli -p linux-system-monitor --bin cli

# 运行
./target/release/cli
```

```
┌────────────────────────────────────────────────────┐
│  Linux System Monitor CLI v3.0.0                   │
├────────────────────────────────────────────────────┤
│  CPU: ████████░░ 45.2%  │  Memory: ██████░░░ 62%  │
│  Load: 1.23 0.98 0.85   │  Uptime: 5d 12h 34m     │
│  Temp: 58°C             │  Net: ↓1.2MB/s ↑256KB/s │
├────────────────────────────────────────────────────┤
│  Top Processes                                    │
│  PID    Name           CPU%    Mem%                │
│  1234   chrome         15.2    8.3                 │
│  5678   code           8.1     5.2                 │
└────────────────────────────────────────────────────┘
```

## 📦 性能指标

| 指标 | v3.0.0 | 竞品对比 |
|------|--------|----------|
| 二进制大小 | **649 KB** | Prometheus ~120MB |
| 内存占用 | **< 5 MB** | Grafana ~500MB |
| 启动时间 | **< 1s** | Prometheus ~5s |
| 依赖数量 | **0** | 需要多服务 |

## 🌐 API 接口

| 端点 | 方法 | 说明 |
|------|------|------|
| `/api/health` | GET | 健康检查 |
| `/api/history` | GET | 历史数据 |
| `/api/alerts` | GET | 告警状态 |
| `/api/alerts/config` | POST | 更新告警配置 |
| `/api/export` | GET | 导出数据 |
| `/api/historical` | GET | 历史对比 |
| `/api/trends` | GET | 趋势数据 |
| `/api/process/kill` | POST | 终止进程 |
| `/api/network-security` | GET | 网络安全信息 |
| `/ws` | WebSocket | 实时数据流 |

## 🛠️ 技术栈

```
Frontend   Vue 3 + ECharts + Vite
Backend    Rust + Axum + Tokio
Metrics    sysinfo crate
Protocol   WebSocket
```

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

## 📄 许可证

MIT License - 详见 [LICENSE](LICENSE)

---

<p align="center">
  用 ❤️ 和 <a href="https://www.rust-lang.org/">Rust</a> 构建
</p>
