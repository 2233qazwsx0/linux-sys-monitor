# 📊 System Monitor

<p align="center">
  <a href="https://github.com/2233qazwsx0/linux-sys-monitor/releases">
    <img src="https://img.shields.io/badge/Version-4.0.0-6366f1?style=flat-square" alt="Version">
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
  <img src="https://img.shields.io/badge/Binary-1.3MB-00ff00?style=flat-square" alt="Binary Size">
  <img src="https://img.shields.io/badge/Features-100+-ff6b6b?style=flat-square" alt="Features">
</p>

---

一款**超功能型**实时系统监控工具，支持 Linux/Windows/Android (Termux)！

## ✨ 100+ 功能特性

### 系统监控 (40+)
CPU governor | 上下文切换 | 中断统计 | 软中断 | 内存压力 | Swap速率 | CPU steal | 磁盘I/O | 队列深度 | 文件系统 | inode | 打开文件数 | 负载归一化 | 内存区域 | 大页 | 内核线程 | 用户线程 | 僵尸进程 | 温度监控 | 电池状态...

### 进程管理 (40+)
父进程 | 线程数 | Nice值 | 进程状态 | 命令行参数 | 环境变量 | 文件描述符 | Socket数 | 子进程树 | CPU时间 | 内存映射 | 资源限制 | OOM评分 | 启动时间 | CPU亲和性 | Seccomp | Capabilities | 信号发送 | 进程告警...

### 网络监控 (40+)
IPv4/IPv6 | MTU | 接口标志 | TCP状态 | UDP端点 | 监听端口 | 连接详情 | 连接时长 | 带宽统计 | 数据包计数 | 错误统计 | 双工模式 | WiFi信号 | 蜂窝数据 | DNS | 路由表 | ARP表 | 网络命名空间...

### 数据导出 (20+)
JSON | CSV | Prometheus | InfluxDB | Graphite | 自定义时间 | 聚合统计 | 异常检测 | 趋势分析 | 峰值检测 | 报告生成 | 定时导出 | 邮件告警 | Webhook | Slack | Telegram...

### UI/UX (20+)
深色/浅色主题 | 跟随系统 | 可折叠面板 | 最小化托盘 | 通知徽章 | 实时搜索 | 键盘快捷键 | 可调刷新率 | 图表缩放 | 图片导出 | 拖拽排序 | 预设布局 | 阈值颜色 | 趋势线 | 热力图 | 仪表盘 | 数字动画...

## 🚀 快速开始

```bash
# 一键安装
wget https://github.com/2233qazwsx0/linux-sys-monitor/releases/download/v4.0.0/linux-system-monitor-4.0.0-linux-x64.tar.gz
tar -xzf linux-system-monitor-4.0.0-linux-x64.tar.gz
cd v4.0.0 && sudo ./install.sh
```

打开 **http://localhost:8080**

## 📦 性能

| 指标 | v4.0.0 |
|------|--------|
| 二进制大小 | **1.3 MB** |
| 功能数量 | **100+** |
| 内存占用 | **< 10 MB** |
| 启动时间 | **< 1s** |

## 🌐 API

| 端点 | 说明 |
|------|------|
| `/api/history` | 历史数据 |
| `/api/alerts` | 告警状态 |
| `/api/export/*` | 多格式导出 |
| `/api/analyze/*` | 数据分析 |
| `/api/compare` | 指标对比 |
| `/ws` | WebSocket |

## 📁 分支

| 分支 | 说明 |
|------|------|
| `main` | Web 版本 |
| `cli` | 终端 CLI |
| `termux` | Android Termux |

---

MIT License | 用 ❤️ 和 Rust 构建
