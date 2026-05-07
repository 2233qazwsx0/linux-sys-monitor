# Linux System Monitor - Windows Installer
# Run as Administrator

param(
    [string]$InstallPath = "$env:LOCALAPPDATA\linux-system-monitor",
    [int]$Port = 8080
)

$ErrorActionPreference = "Stop"

function Write-Step($message) {
    Write-Host "[*] $message" -ForegroundColor Cyan
}

function Write-Success($message) {
    Write-Host "[+] $message" -ForegroundColor Green
}

function Write-Error($message) {
    Write-Host "[-] $message" -ForegroundColor Red
}

Write-Host ""
Write-Host "========================================" -ForegroundColor Magenta
Write-Host "  Linux System Monitor - Windows Setup" -ForegroundColor Magenta
Write-Host "========================================" -ForegroundColor Magenta
Write-Host ""

# Check if running as Administrator
$isAdmin = ([Security.Principal.WindowsPrincipal] [Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
if (-not $isAdmin) {
    Write-Error "Please run as Administrator for full system metrics access"
    Write-Host "Continuing anyway (some metrics may be limited)..." -ForegroundColor Yellow
}

Write-Step "Checking prerequisites..."

# Check for Rust
$rustInstalled = $null -ne (Get-Command rustc -ErrorAction SilentlyContinue)
if (-not $rustInstalled) {
    Write-Step "Installing Rust..."
    $rustup = Invoke-WebRequest -Uri https://win.rustup.rs -OutFile "$env:TEMP\rustup-init.exe"
    Start-Process -FilePath "$env:TEMP\rustup-init.exe" -ArgumentList "-y", "--default-toolchain", "stable" -Wait -NoNewWindow
    $env:Path = [System.Environment]::GetEnvironmentVariable("Path", "Machine") + ";" + [System.Environment]::GetEnvironmentVariable("Path", "User")
    
    if ($null -eq (Get-Command rustc -ErrorAction SilentlyContinue)) {
        Write-Error "Rust installation failed. Please install manually from https://rustup.rs"
        exit 1
    }
    Write-Success "Rust installed"
}

# Check for Node.js (optional for frontend build)
$nodeInstalled = $null -ne (Get-Command node -ErrorAction SilentlyContinue)
if (-not $nodeInstalled) {
    Write-Step "Node.js not found. Will use pre-built binary."
}

Write-Step "Creating installation directory: $InstallPath"
New-Item -ItemType Directory -Force -Path $InstallPath | Out-Null

# Clone or update repository
$repoPath = Join-Path $InstallPath "repo"
if (Test-Path $repoPath) {
    Write-Step "Updating existing installation..."
    Set-Location $repoPath
    git pull
} else {
    Write-Step "Downloading source code..."
    git clone https://github.com/2233qazwsx0/linux-sys-monitor.git $repoPath
    Set-Location $repoPath
}
Write-Success "Source code ready"

# Build frontend if Node.js available
if ($nodeInstalled) {
    Write-Step "Building frontend..."
    Set-Location (Join-Path $repoPath "frontend")
    npm install --silent
    npm run build
    Set-Location $repoPath
    Write-Success "Frontend built"
} else {
    Write-Step "Skipping frontend build (Node.js not installed)"
}

# Build backend
Write-Step "Building backend (this may take a few minutes)..."
Set-Location $repoPath

# Try pre-built binary first for Windows
$releaseUrl = "https://github.com/2233qazwsx0/linux-sys-monitor/releases/latest"
$binaryName = "linux-system-monitor.exe"
$downloadUrl = "https://github.com/2233qazwsx0/linux-sys-monitor/releases/download/v1.0.0/linux-system-monitor.exe"

try {
    Write-Step "Trying to download pre-built binary..."
    Invoke-WebRequest -Uri $downloadUrl -OutFile (Join-Path $repoPath "target\release\$binaryName") -UseBasicParsing
    Write-Success "Binary downloaded"
} catch {
    Write-Step "No pre-built binary available, building from source..."
    cargo build --release
    Write-Success "Binary built from source"
}

# Create start script
$startScript = @"
@echo off
cd /d "%~dp0"
echo Starting Linux System Monitor...
echo Open http://localhost:$Port in your browser
target\release\linux-system-monitor.exe
"@

$startScript | Out-File -FilePath (Join-Path $repoPath "start.bat") -Encoding ASCII
Write-Success "Start script created"

# Create desktop shortcut
$WshShell = New-Object -ComObject WScript.Shell
$Shortcut = $WshShell.CreateShortcut("$env:USERPROFILE\Desktop\System Monitor.lnk")
$Shortcut.TargetPath = (Join-Path $repoPath "start.bat")
$Shortcut.WorkingDirectory = $repoPath
$Shortcut.Description = "Linux System Monitor"
$Shortcut.Save()
Write-Success "Desktop shortcut created"

# Create Windows Firewall rule
Write-Step "Creating firewall rule..."
$ruleName = "Linux System Monitor"
Remove-NetFirewallRule -DisplayName $ruleName -ErrorAction SilentlyContinue
New-NetFirewallRule -DisplayName $ruleName -Direction Inbound -Protocol TCP -LocalPort $Port -Action Allow -Enabled True | Out-Null
Write-Success "Firewall rule created"

Write-Host ""
Write-Host "========================================" -ForegroundColor Green
Write-Host "  Installation Complete!" -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Green
Write-Host ""
Write-Host "To start the monitor:" -ForegroundColor Cyan
Write-Host "  1. Double-click 'System Monitor' on your desktop" -ForegroundColor White
Write-Host "  2. Or run: $repoPath\start.bat" -ForegroundColor White
Write-Host ""
Write-Host "Then open http://localhost:$Port in your browser" -ForegroundColor Cyan
Write-Host ""

# Ask to start now
$response = Read-Host "Start now? (Y/n)"
if ($response -ne "n" -and $response -ne "N") {
    Write-Step "Starting monitor..."
    Start-Process (Join-Path $repoPath "start.bat")
}
