# Linux System Monitor Windows Installer

Windows 平台的 Rust 安装程序，用于自动安装 Linux System Monitor。

## 功能特性

- ✅ 自动检测并安装 Rust（如未安装）
- ✅ 检测 Windows 架构 (x64 / ARM64)
- ✅ 从 GitHub 下载最新预编译版本
- ✅ 自动下载并运行 rustup-init.exe
- ✅ 支持从源码编译（如果无预编译版本）
- ✅ 自动解压 ZIP 压缩包
- ✅ 创建开始菜单快捷方式
- ✅ 创建桌面快捷方式
- ✅ 设置开机自启动任务
- ✅ 生成卸载脚本
- ✅ 彩色终端输出和进度显示
- ✅ 错误处理和用户交互

## 系统要求

- Windows 10/11 (x64 或 ARM64)
- 管理员权限（建议，用于完整功能）
- 网络连接（下载 Rust 和程序）

## 构建

### 在 Windows 上构建

```bash
cargo build --release
```

生成文件：`target/release/system-monitor-installer.exe`

### 使用 cargo-xwin 交叉编译

```bash
# 安装交叉编译工具
cargo install cargo-xwin

# 构建 Windows 版本
cargo xwin build --release
```

### 使用 cross 交叉编译

```bash
# 安装 cross
cargo install cross

# 创建 .cargo/config.toml
[target.x86_64-pc-windows-gnu]
linker = "x86_64-w64-mingw32-gcc"
ar = "x86_64-w64-mingw32-ar"

# 构建
cross build --release --target x86_64-pc-windows-gnu
```

## 使用方法

### 基本使用

```powershell
# 直接运行安装程序
.\system-monitor-installer.exe

# 以管理员身份运行（推荐）
Start-Process .\system-monitor-installer.exe -Verb RunAs
```

### 环境变量

| 变量 | 默认值 | 说明 |
|------|--------|------|
| `MONITOR_HOME` | `C:\Program Files\Linux System Monitor` | 自定义安装目录 |

```powershell
# 使用自定义安装目录
$env:MONITOR_HOME = "D:\MyApps\SystemMonitor"
.\system-monitor-installer.exe
```

## 安装流程

1. **权限检查** - 检查是否以管理员身份运行
2. **架构检测** - 检测系统是 x64 还是 ARM64
3. **Rust 检查** - 检测 Rust 是否已安装
4. **下载 Rust** - 如未安装，静默下载并安装 rustup
5. **获取版本** - 从 GitHub API 获取最新版本信息
6. **下载程序** - 下载预编译的 Windows 二进制文件
7. **解压文件** - 解压 ZIP 压缩包到安装目录
8. **创建快捷方式** - 在开始菜单和桌面创建快捷方式
9. **设置自启动** - 创建 Windows 计划任务，开机自启动
10. **生成卸载脚本** - 创建卸载批处理脚本

## 卸载

安装完成后，在安装目录会有 `uninstall.bat` 卸载脚本：

```powershell
cd "C:\Program Files\Linux System Monitor"
.\uninstall.bat
```

或手动删除：

1. 删除安装目录
2. 删除桌面快捷方式
3. 运行 `schtasks /Delete /TN "LinuxSystemMonitor" /F` 删除计划任务
4. 删除开始菜单中的快捷方式

## 故障排除

### 安装失败

1. 确保以管理员身份运行
2. 检查网络连接
3. 确保 C:\Program Files 可写

### Rust 安装失败

- 手动安装：[rustup.rs](https://rustup.rs)
- 或使用离线安装程序

### 快捷方式创建失败

- 需要管理员权限
- 或手动创建快捷方式

### 计划任务创建失败

- 需要管理员权限
- 可手动创建：控制面板 -> 计划任务

## 许可证

继承 Linux System Monitor 项目许可证
