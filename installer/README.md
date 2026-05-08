# Windows Installer

Rust 写的 Windows 安装器，用于自动安装 System Monitor。

## 功能

- ✅ 自动检测并安装 Rust（如未安装）
- ✅ 从 GitHub 下载最新预编译版本
- ✅ 自动解压和配置
- ✅ 创建桌面快捷方式
- ✅ 生成启动脚本
- ✅ 彩色终端输出

## 构建

```bash
# 在 Windows 上
cargo build --release

# 生成的文件
# target/release/system-monitor-installer.exe
```

## 使用

```powershell
# 直接运行
.\system-monitor-installer.exe

# 或双击运行
```

## 环境变量

| 变量 | 默认值 | 说明 |
|------|--------|------|
| `MONITOR_HOME` | `~/system-monitor` | 安装目录 |
