# Linux System Monitor - Windows 一键安装脚本
# 使用方法: 右键点击 -> 用 PowerShell 运行
# 或: powershell -ExecutionPolicy Bypass -File install-windows.ps1

param(
    [string]$InstallPath = "$env:LOCALAPPDATA\SystemMonitor",
    [switch]$AutoStart,
    [switch]$Uninstall
)

$ErrorActionPreference = "Stop"
$RepoOwner = "2233qazwsx0"
$RepoName = "linux-sys-monitor"
$AppName = "Linux System Monitor"
$ExeName = "system-monitor.exe"

function Write-Step($message) {
    Write-Host "[*] $message" -ForegroundColor Cyan
}

function Write-Success($message) {
    Write-Host "[+] $message" -ForegroundColor Green
}

function Write-Error($message) {
    Write-Host "[-] $message" -ForegroundColor Red
}

function Get-GitHubLatestRelease {
    $url = "https://api.github.com/repos/$RepoOwner/$RepoName/releases/latest"
    try {
        $response = Invoke-RestMethod -Uri $url -UseBasicParsing
        return $response
    } catch {
        return $null
    }
}

function Get-SystemArchitecture {
    $arch = $env:PROCESSOR_ARCHITECTURE
    if ($arch -eq "ARM64") {
        return "arm64"
    } elseif ($arch -eq "AMD64") {
        return "x64"
    } else {
        return "x86"
    }
}

function Install-RustIfNeeded {
    Write-Step "检查 Rust..."
    
    $rustc = Get-Command rustc -ErrorAction SilentlyContinue
    if ($rustc) {
        Write-Success "Rust 已安装: $(rustc --version)"
        return $true
    }
    
    Write-Step "正在安装 Rust..."
    
    $rustupPath = "$env:TEMP\rustup-init.exe"
    
    try {
        Write-Step "下载 rustup-init.exe..."
        Invoke-WebRequest -Uri "https://win.rustup.rs" -OutFile $rustupPath -UseBasicParsing
        
        Write-Step "运行 rustup-init.exe..."
        Start-Process -FilePath $rustupPath -ArgumentList "-y", "--default-host", "x86_64-pc-windows-msvc" -Wait -NoNewWindow
        
        # 刷新环境变量
        $env:Path = [System.Environment]::GetEnvironmentVariable("Path", "Machine") + ";" + [System.Environment]::GetEnvironmentVariable("Path", "User")
        
        Remove-Item $rustupPath -Force -ErrorAction SilentlyContinue
        
        Write-Success "Rust 安装完成"
        return $true
    } catch {
        Write-Error "Rust 安装失败: $_"
        return $false
    }
}

function Install-SystemMonitor {
    Write-Step "创建安装目录..."
    if (!(Test-Path $InstallPath)) {
        New-Item -ItemType Directory -Path $InstallPath -Force | Out-Null
    }
    
    Write-Step "获取最新版本信息..."
    $release = Get-GitHubLatestRelease
    
    if (!$release) {
        Write-Error "无法获取最新版本信息"
        return $false
    }
    
    $version = $release.tag_name -replace "v", ""
    Write-Step "最新版本: $version"
    
    # 构建下载 URL (使用 release asset)
    $arch = Get-SystemArchitecture
    $downloadName = "system-monitor-$version-windows-$arch.zip"
    $downloadUrl = "https://github.com/$RepoOwner/$RepoName/releases/download/v$version/$downloadName"
    
    Write-Step "下载 $downloadName..."
    
    $zipPath = "$env:TEMP\$downloadName"
    
    try {
        Invoke-WebRequest -Uri $downloadUrl -OutFile $zipPath -UseBasicParsing
    } catch {
        # 如果特定架构版本不存在，尝试通用版本
        $downloadName = "system-monitor-$version-windows.zip"
        $downloadUrl = "https://github.com/$RepoOwner/$RepoName/releases/download/v$version/$downloadName"
        Invoke-WebRequest -Uri $downloadUrl -OutFile $zipPath -UseBasicParsing
    }
    
    Write-Step "解压文件..."
    Expand-Archive -Path $zipPath -DestinationPath $InstallPath -Force
    
    # 查找并重命名 exe
    $extractedExe = Get-ChildItem -Path $InstallPath -Filter "*.exe" -Recurse | Select-Object -First 1
    if ($extractedExe) {
        $targetPath = Join-Path $InstallPath $ExeName
        Move-Item -Path $extractedExe.FullName -Destination $targetPath -Force
        
        # 清理 zip
        Remove-Item $zipPath -Force
        
        # 清理空文件夹
        Get-ChildItem -Path $InstallPath -Recurse -Directory | Where-Object { (Get-ChildItem $_.FullName -Force | Measure-Object).Count -eq 0 } | Remove-Item -Force -Recurse
        
        Write-Success "安装完成!"
        
        # 创建卸载脚本
        Create-UninstallScript
        
        # 设置开机自启动
        if ($AutoStart) {
            Set-AutoStart
        }
        
        # 运行程序
        Write-Step "启动程序..."
        Start-Process -FilePath $targetPath
        
        return $true
    } else {
        Write-Error "未找到可执行文件"
        return $false
    }
}

function Create-UninstallScript {
    $uninstallScript = @"
# System Monitor 卸载脚本
`$Confirm = Read-Host "确定要卸载 $AppName 吗? (y/N)"
if (`$Confirm -ne "y" -and `$Confirm -ne "Y") {
    exit
}

# 移除开机自启动
`$regPath = "HKCU:\Software\Microsoft\Windows\CurrentVersion\Run"
Remove-ItemProperty -Path `$regPath -Name "SystemMonitor" -ErrorAction SilentlyContinue

# 移除安装目录
`$installPath = "$InstallPath"
if (Test-Path `$installPath) {
    Remove-Item -Path `$installPath -Recurse -Force
    Write-Host "[+] 卸载完成" -ForegroundColor Green
} else {
    Write-Host "[*] 安装目录不存在" -ForegroundColor Yellow
}
"@
    
    $uninstallPath = Join-Path $InstallPath "Uninstall.ps1"
    $uninstallScript | Out-File -FilePath $uninstallPath -Encoding UTF8
    Write-Step "卸载脚本已创建: $uninstallPath"
}

function Set-AutoStart {
    $regPath = "HKCU:\Software\Microsoft\Windows\CurrentVersion\Run"
    $exePath = Join-Path $InstallPath $ExeName
    
    Set-ItemProperty -Path $regPath -Name "SystemMonitor" -Value "`"$exePath`""
    Write-Success "已设置开机自启动"
}

function Start-Uninstall {
    Write-Host ""
    Write-Host "========================================" -ForegroundColor Yellow
    Write-Host "  $AppName 卸载程序" -ForegroundColor Yellow
    Write-Host "========================================" -ForegroundColor Yellow
    Write-Host ""
    
    if (Test-Path $InstallPath) {
        $uninstallScript = Join-Path $InstallPath "Uninstall.ps1"
        if (Test-Path $uninstallScript) {
            & $uninstallScript
        } else {
            $Confirm = Read-Host "确定要卸载 $AppName 吗? (y/N)"
            if ($Confirm -eq "y" -or $Confirm -eq "Y") {
                # 移除开机自启动
                $regPath = "HKCU:\Software\Microsoft\Windows\CurrentVersion\Run"
                Remove-ItemProperty -Path $regPath -Name "SystemMonitor" -ErrorAction SilentlyContinue
                
                # 移除安装目录
                Remove-Item -Path $InstallPath -Recurse -Force
                Write-Success "卸载完成"
            }
        }
    } else {
        Write-Host "[*] 未找到安装目录" -ForegroundColor Yellow
    }
}

# 主程序
Write-Host ""
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  $AppName Windows 安装程序" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

if ($Uninstall) {
    Start-Uninstall
    exit 0
}

Write-Host "安装路径: $InstallPath" -ForegroundColor Gray
if ($AutoStart) {
    Write-Host "选项: 开机自启动" -ForegroundColor Gray
}
Write-Host ""

try {
    $rustInstalled = Install-RustIfNeeded
    if (!$rustInstalled) {
        throw "Rust 安装失败"
    }
    
    $installed = Install-SystemMonitor
    if ($installed) {
        Write-Host ""
        Write-Host "========================================" -ForegroundColor Green
        Write-Host "  安装成功!" -ForegroundColor Green
        Write-Host "========================================" -ForegroundColor Green
        Write-Host ""
        Write-Host "使用说明:" -ForegroundColor White
        Write-Host "  运行: $InstallPath\$ExeName" -ForegroundColor Gray
        Write-Host "  卸载: $InstallPath\Uninstall.ps1" -ForegroundColor Gray
        Write-Host ""
    }
} catch {
    Write-Error "安装失败: $_"
    exit 1
}
