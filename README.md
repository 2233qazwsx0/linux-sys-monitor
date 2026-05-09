# 📊 Linux System Monitor

<p align="center">
  <a href="https://github.com/2233qazwsx0/linux-sys-monitor/releases">
    <img src="https://img.shields.io/badge/Version-4.2.0-6366f1?style=flat-square" alt="Version">
  </a>
  <a href="https://github.com/2233qazwsx0/linux-sys-monitor/blob/main/LICENSE">
    <img src="https://img.shields.io/badge/License-MIT-blue?style=flat-square" alt="License">
  </a>
  <a href="https://github.com/2233qazwsx0/linux-sys-monitor/stargazers">
    <img src="https://img.shields.io/github/stars/2233qazwsx0/linux-sys-monitor?style=flat-square" alt="Stars">
  </a>
  <a href="https://github.com/2233qazwsx0/linux-sys-monitor/releases">
    <img src="https://img.shields.io/github/downloads/2233qazwsx0/linux-sys-monitor/total?style=flat-square" alt="Downloads">
  </a>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/Rust-1.75+-orange?style=flat-square&logo=rust" alt="Rust">
  <img src="https://img.shields.io/badge/Vue-3.4-green?style=flat-square&logo=vue.js" alt="Vue">
  <img src="https://img.shields.io/badge/Platform-Linux%20%7C%20Windows%20%7C%20Android-9cf?style=flat-square" alt="Platform">
</p>

<p align="center">
  🇨🇳 中文 | <a href="README_EN.md">English</a>
</p>

---

一款**超轻量级**实时系统监控工具，支持 Linux、Windows 和 Android (Termux)！

## ✨ 核心特性

| 特性 | 说明 |
|------|------|
| 📊 **实时监控** | CPU、内存、Swap、网络、磁盘 I/O |
| 🌡️ **温度监控** | CPU/GPU 温度实时监测 |
| 🔥 **进程管理** | 进程列表、终止、排序、过滤 |
| 🚨 **告警系统** | 可配置阈值，实时告警 |
| 📤 **数据导出** | JSON/CSV/Prometheus/InfluxDB |
| 🎨 **主题切换** | 深色/浅色主题 |
| 📱 **多平台** | Linux / Windows / Android |
| ⚡ **超轻量** | 二进制仅 1.3MB |

## 🚀 快速开始

### Linux

```bash
# 一键安装
curl -fsSL https://raw.githubusercontent.com/2233qazwsx0/linux-sys-monitor/main/setup.sh | bash

# 或下载解压
wget https://github.com/2233qazwsx0/linux-sys-monitor/releases/download/v4.2.0/linux-system-monitor-4.2.0-linux-x64.tar.gz
tar -xzf linux-system-monitor-4.2.0-linux-x64.tar.gz
cd v4.2.0 && sudo ./install.sh
```

打开 **http://localhost:8080**

### Windows

```powershell
# 方法一：一键安装 (推荐)
irm https://raw.githubusercontent.com/2233qazwsx0/linux-sys-monitor/main/install-windows.ps1 | iex

# 方法二：下载安装器
# 下载 system-monitor-installer.exe 并运行
```

### Android (Termux)

```bash
curl -fsSL https://raw.githubusercontent.com/2233qazwsx0/linux-sys-monitor/termux/install-termux.sh | bash
```

### Docker

```bash
git clone https://github.com/2233qazwsx0/linux-sys-monitor.git
cd linux-sys-monitor
docker-compose up -d
```

## 📷 界面预览

```
┌─────────────────────────────────────────────────────────────┐
│  📊 System Monitor v4.2.0          [🌙/☀️] [⚙️] [📤]      │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  ┌──────────────┐ ┌──────────────┐ ┌──────────────┐       │
│  │ ⚡ CPU 45.2% │ │ 🧠 内存 62%  │ │ 🌡️ 温度 58°C│       │
│  │ ██████░░░░░ │ │ ████████░░ │ │ ████████░░░ │       │
│  │ 8核 @3.5GHz  │ │ 8G/16G      │ │ CPU: 58°C    │       │
│  └──────────────┘ └──────────────┘ └──────────────┘       │
│                                                             │
│  ┌─────────────────────────────────────────────────────┐   │
│  │ 📈 趋势图                                             │   │
│  │ ▁▂▃▄▅▆▇█▇▆▅▄▃▂▁▂▃▄▅▆▇█▇▆▅▄▃▂▁                   │   │
│  └─────────────────────────────────────────────────────┘   │
│                                                             │
│  ┌─────────────────────────────────────────────────────┐   │
│  │ 📋 进程列表                        [搜索...] [🔍]  │   │
│  │ PID    名称        CPU%    内存%    状态           │   │
│  │ 1234   chrome     15.2    8.3      运行中          │   │
│  │ 5678   code       8.1     5.2      运行中          │   │
│  └─────────────────────────────────────────────────────┘   │
│                                                             │
│  🌐 网络: ↓1.2MB/s  ↑256KB/s  |  📦 磁盘: 78% 已用     │
└─────────────────────────────────────────────────────────────┘
```

## 📦 功能列表 (100+)

### 系统监控 (40+)
| 功能 | 说明 |
|------|------|
| CPU 使用率 | 整体 + 每核心 |
| CPU Governor | 调度策略显示 |
| 上下文切换 | 系统上下文切换计数 |
| 中断统计 | 硬件中断数量 |
| 内存监控 | 总量/可用/Buffers/Cached |
| 内存压力 | /proc/pressure 状态 |
| Swap 监控 | Swap 总量和使用率 |
| 磁盘 I/O | 读写速率 |
| 磁盘空间 | 各分区使用情况 |
| 文件系统 | 文件系统类型和 inode |
| 温度监控 | CPU/GPU 温度 |
| 电池状态 | 电量、状态、温度 |
| 系统负载 | 1/5/15 分钟平均 |
| 运行时间 | 系统启动时长 |
| 进程统计 | 总进程数/线程数 |
| 僵尸进程 | 僵尸进程检测 |

### 进程管理 (40+)
| 功能 | 说明 |
|------|------|
| 进程列表 | Top 进程按 CPU/内存排序 |
| 进程详情 | PID/名称/用户 |
| 线程数 | 进程的线程数 |
| Nice 值 | 进程优先级 |
| 进程状态 | R/S/Z/D 状态 |
| 内存使用 | 虚拟/物理内存 |
| CPU 时间 | 累计 CPU 时间 |
| 父进程 | 父子进程关系 |
| 子进程树 | 进程树结构 |
| 文件描述符 | 打开的 FD 数量 |
| Socket 统计 | 进程 socket 数 |
| 环境变量 | 进程环境（脱敏）|
| OOM 评分 | 内存压力评分 |
| CPU 亲和性 | CPU 核心绑定 |
| 进程终止 | 发送信号终止 |
| 进程过滤 | 按名称搜索 |
| 排序功能 | 多列排序 |

### 网络监控 (40+)
| 功能 | 说明 |
|------|------|
| 网络接口 | 各接口统计 |
| IP 地址 | IPv4/IPv6 |
| 接口状态 | UP/DOWN/MTU |
| TCP 连接 | 各状态统计 |
| UDP 端点 | UDP 连接数 |
| 监听端口 | 服务端口列表 |
| 连接详情 | 连接地址和端口 |
| 带宽统计 | 实时速率 |
| 数据包计数 | 收发包统计 |
| WiFi 信号 | 信号强度 |
| DNS 配置 | DNS 服务器 |
| 路由表 | 路由信息 |
| ARP 表 | ARP 缓存 |

### 数据导出 (20+)
| 功能 | 说明 |
|------|------|
| JSON 导出 | 完整历史数据 |
| CSV 导出 | 表格格式 |
| Prometheus | Prometheus 格式 |
| InfluxDB | Line Protocol |
| Graphite | Carbon 格式 |
| 趋势分析 | 指标趋势预测 |
| 异常检测 | 异常数据点 |
| 告警配置 | 阈值设置 |
| 邮件告警 | 邮件通知 |
| Webhook | HTTP 回调 |
| Slack 集成 | Slack 通知 |
| Telegram | Bot 通知 |

### UI/UX (20+)
| 功能 | 说明 |
|------|------|
| 深色主题 | 暗色模式 |
| 浅色主题 | 亮色模式 |
| 自动主题 | 跟随系统 |
| 可折叠面板 | 节省空间 |
| 实时搜索 | 快速过滤 |
| 键盘快捷键 | 提升效率 |
| 自定义刷新 | 刷新间隔可调 |
| 图表缩放 | 交互式图表 |
| 数据导出 | 图表导出图片 |
| 拖拽排序 | 自定义布局 |
| 预设布局 | 快速切换 |
| 通知中心 | 告警通知 |
| 设置面板 | 配置管理 |

## ⚡ 性能对比

| 指标 | System Monitor | Prometheus | Grafana |
|------|---------------|------------|---------|
| 二进制大小 | **1.3 MB** | ~120 MB | ~300 MB |
| 内存占用 | **< 10 MB** | ~200 MB | ~500 MB |
| 启动时间 | **< 1s** | ~5s | ~10s |
| 依赖数量 | **0** | 需要多服务 | 需要多服务 |
| 功能数量 | **100+** | 有限 | 有限 |

## 🌐 API 接口

### REST API

| 端点 | 方法 | 说明 |
|------|------|------|
| `/api/health` | GET | 健康检查 |
| `/api/history` | GET | 历史数据 |
| `/api/alerts` | GET | 告警状态 |
| `/api/alerts/config` | POST | 更新告警配置 |
| `/api/export/json` | GET | JSON 导出 |
| `/api/export/csv` | GET | CSV 导出 |
| `/api/export/prometheus` | GET | Prometheus 格式 |
| `/api/analyze/trends` | GET | 趋势分析 |
| `/api/analyze/anomalies` | GET | 异常检测 |
| `/api/compare` | GET | 指标对比 |
| `/api/process/kill` | POST | 终止进程 |
| `/api/network-security` | GET | 网络安全信息 |
| `/api/retention/config` | GET/POST | 数据保留策略 |

### WebSocket

```
ws://localhost:8080/ws
```

实时推送系统指标数据。

## 🛠️ 技术栈

```
┌─────────────────────────────────────┐
│           Frontend                  │
│   Vue 3 + ECharts + Vite           │
├─────────────────────────────────────┤
│           Backend                  │
│   Rust + Axum + Tokio              │
├─────────────────────────────────────┤
│           Metrics                  │
│   sysinfo crate                    │
├─────────────────────────────────────┤
│           Protocol                 │
│   WebSocket + REST API             │
└─────────────────────────────────────┘
```

## 📁 项目分支

| 分支 | 说明 | 下载 |
|------|------|------|
| `main` | Web 版本 | [.tar.gz](https://github.com/2233qazwsx0/linux-sys-monitor/releases) |
| `cli` | 终端 CLI | 编译获取 |
| `termux` | Android Termux | [安装脚本](https://raw.githubusercontent.com/2233qazwsx0/linux-sys-monitor/termux/install-termux.sh) |

## 🏗️ 项目结构

```
linux-system-monitor/
├── src/
│   ├── main.rs              # Web 版本入口
│   ├── api/                 # HTTP/WebSocket API
│   ├── metrics/             # 系统指标采集
│   │   ├── mod.rs           # 指标定义
│   │   ├── collector.rs     # 指标收集器
│   │   ├── analysis.rs      # 数据分析
│   │   ├── alerts.rs        # 告警系统
│   │   └── export.rs        # 数据导出
│   └── cli/                 # CLI 版本
│       └── main.rs
├── frontend/                 # Vue.js 前端
│   └── src/
│       ├── App.vue
│       ├── components/       # UI 组件
│       └── composables/      # 组合式函数
├── installer/               # Windows 安装器
│   └── src/main.rs
├── Dockerfile
├── docker-compose.yml
├── setup.sh                # Linux 安装脚本
└── install-windows.ps1     # Windows 安装脚本
```

## 📊 版本历史

| 版本 | 日期 | 主要更新 |
|------|------|----------|
| v4.2.0 | 2024-05 | 100+ 功能，Windows 原生支持 |
| v3.0.0 | 2024-05 | CLI 模式，Termux 支持 |
| v2.5.0 | 2024-05 | 告警系统，数据导出，主题切换 |
| v2.1.0 | 2024-05 | 温度监控，负载平均，网络连接 |
| v2.0.0 | 2024-05 | 中文界面，UI 优化 |
| v1.0.0 | 2024-05 | 初始版本 |

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

1. Fork 本项目
2. 创建特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送分支 (`git push origin feature/AmazingFeature`)
5. 创建 Pull Request

## 📄 许可证

本项目基于 MIT 许可证开源 - 详见 [LICENSE](LICENSE)

## ⭐ 如果对你有帮助

请给我一个 Star！

[![Star History](https://api.star-history.com/svg?repos=2233qazwsx0/linux-sys-monitor&type=Timeline)](https://star-history.com/#2233qazwsx0/linux-sys-monitor&Timeline)

---

<p align="center">
  用 ❤️ 和 <a href="https://www.rust-lang.org/">Rust</a> 构建
</p>
