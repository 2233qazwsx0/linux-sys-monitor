use std::env;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use anyhow::{Context, Result};
use console::Style;

#[cfg(windows)]
use std::os::windows::process::CommandExt;

const GITHUB_REPO: &str = "2233qazwsx0/linux-sys-monitor";

#[cfg(windows)]
const CREATE_NO_WINDOW: u32 = 0x08000000;

#[cfg(not(windows))]
const CREATE_NO_WINDOW: u32 = 0;

fn main() -> Result<()> {
    println!();
    println!("{}", Style::new().cyan().apply_to("╔══════════════════════════════════════════════╗"));
    println!("{}", Style::new().cyan().apply_to("║   Linux System Monitor Installer v1.0.0     ║"));
    println!("{}", Style::new().cyan().apply_to("║   Windows Edition                             ║"));
    println!("{}", Style::new().cyan().apply_to("╚══════════════════════════════════════════════╝"));
    println!();

    let install_dir = get_install_dir();
    
    println!("{} {}", Style::new().yellow().apply_to("📦 Installation Directory:"), install_dir.display());
    println!();

    check_permissions()?;
    
    let arch = detect_architecture();
    println!("{} {}", Style::new().cyan().apply_to("💻 System Architecture:"), arch);
    println!();

    install_rust()?;
    
    download_and_install(&install_dir, &arch)?;
    
    create_start_menu_shortcut(&install_dir)?;
    
    create_scheduled_task(&install_dir)?;
    
    create_uninstall_script(&install_dir)?;
    
    println!();
    println!("{}", Style::new().green().apply_to("✅ Installation Complete!"));
    println!();
    println!("To start the application:");
    println!("  {}", Style::new().cyan().apply_to(format!("{} \\linux-system-monitor.exe", install_dir.display())));
    println!();
    println!("Then open {}", Style::new().cyan().apply_to("http://localhost:8080"));
    println!();
    
    if ask_yes_no("Start now? (Y/n): ")? {
        println!("Starting System Monitor...");
        #[cfg(windows)]
        {
            Command::new("cmd")
                .creation_flags(CREATE_NO_WINDOW)
                .args(["/C", "start", "", &install_dir.join("linux-system-monitor.exe").display().to_string()])
                .spawn()
                .context("Failed to start application")?;
        }
        #[cfg(not(windows))]
        {
            Command::new(&install_dir.join("linux-system-monitor"))
                .spawn()
                .context("Failed to start application")?;
        }
    }
    
    Ok(())
}

fn get_install_dir() -> PathBuf {
    if let Ok(dir) = env::var("MONITOR_HOME") {
        PathBuf::from(dir)
    } else {
        PathBuf::from("C:\\Program Files\\Linux System Monitor")
    }
}

fn check_permissions() -> Result<()> {
    println!("{}", Style::new().cyan().apply_to("🔍 Checking permissions..."));
    
    #[cfg(windows)]
    {
        let output = Command::new("net")
            .creation_flags(CREATE_NO_WINDOW)
            .args(["session"])
            .output();
        
        match output {
            Ok(o) if o.status.success() => {
                println!("{}", Style::new().green().apply_to("✓ Running as Administrator"));
            }
            _ => {
                println!("{}", Style::new().yellow().apply_to("⚠ Not running as Administrator"));
                println!("{}", Style::new().yellow().apply_to("  Some features may be limited. Run as admin for full installation."));
            }
        }
    }
    
    #[cfg(not(windows))]
    {
        if env::var("USER").map(|u| u == "root").unwrap_or(false) {
            println!("{}", Style::new().green().apply_to("✓ Running as root"));
        } else {
            println!("{}", Style::new().yellow().apply_to("⚠ Not running as root"));
        }
    }
    
    println!();
    Ok(())
}

fn detect_architecture() -> String {
    #[cfg(windows)]
    {
        let output = Command::new("powershell")
            .creation_flags(CREATE_NO_WINDOW)
            .args(["-Command", "(Get-WmiObject Win32_ComputerSystem).SystemType"])
            .output();
        
        if let Ok(output) = output {
            let result = String::from_utf8_lossy(&output.stdout);
            if result.contains("ARM64") {
                return "arm64".to_string();
            }
        }
        
        if cfg!(target_arch = "x86_64") {
            return "x64".to_string();
        } else if cfg!(target_arch = "aarch64") {
            return "arm64".to_string();
        }
    }
    
    "x64".to_string()
}

fn check_rust_installed() -> bool {
    #[cfg(windows)]
    {
        Command::new("rustc")
            .creation_flags(CREATE_NO_WINDOW)
            .arg("--version")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }
    
    #[cfg(not(windows))]
    {
        Command::new("rustc")
            .arg("--version")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }
}

fn install_rust() -> Result<()> {
    println!("{}", Style::new().cyan().apply_to("🔧 Checking Rust..."));
    
    if check_rust_installed() {
        let output = Command::new("rustc")
            .arg("--version")
            .output()?;
        let version = String::from_utf8_lossy(&output.stdout);
        println!("{}", Style::new().green().apply_to(format!("✓ Rust found: {}", version.trim())));
    } else {
        println!("{}", Style::new().yellow().apply_to("📥 Rust not found. Installing Rust..."));
        
        let temp_dir = env::temp_dir();
        let rustup_path = temp_dir.join("rustup-init.exe");
        
        let mut resp = reqwest::blocking::get("https://win.rustup.rs")
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        let mut file = File::create(&rustup_path)?;
        
        let mut buffer = [0u8; 8192];
        loop {
            let bytes_read = resp.read(&mut buffer)?;
            if bytes_read == 0 {
                break;
            }
            file.write_all(&buffer[..bytes_read])?;
        }
        
        println!();
        println!("{}", Style::new().cyan().apply_to("⏳ Running rustup installer..."));
        
        #[cfg(windows)]
        let status = Command::new(&rustup_path)
            .creation_flags(CREATE_NO_WINDOW)
            .args(["-y", "--default-toolchain", "stable", "--profile", "minimal"])
            .status()
            .context("Failed to run rustup installer");
        
        #[cfg(not(windows))]
        let status = Command::new("sh")
            .arg("-c")
            .arg(&format!("curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable --profile minimal"))
            .status()
            .context("Failed to run rustup installer");
        
        match status {
            Ok(s) if s.success() => {
                let _ = fs::remove_file(&rustup_path);
                println!("{}", Style::new().green().apply_to("✓ Rust installed successfully"));
            }
            Ok(_) => {
                return Err(anyhow::anyhow!("Rust installation failed"));
            }
            Err(e) => {
                return Err(e).context("Rust installation error");
            }
        }
        
        #[cfg(windows)]
        {
            let cargo_path = PathBuf::from("C:\\Users\\")
                .join(env::var("USERNAME").unwrap_or_default())
                .join(".cargo\\bin");
            env::set_var("PATH", format!("{};{}", cargo_path.display(), env::var("PATH").unwrap_or_default()));
        }
        
        #[cfg(not(windows))]
        {
            let home = env::var("HOME").unwrap_or_default();
            let cargo_bin = PathBuf::from(&home).join(".cargo").join("bin");
            env::set_var("PATH", format!("{}:{}", cargo_bin.display(), env::var("PATH").unwrap_or_default()));
        }
    }
    
    println!();
    Ok(())
}

fn download_and_install(install_dir: &Path, arch: &str) -> Result<()> {
    println!("{}", Style::new().cyan().apply_to("📥 Checking for pre-built binaries..."));
    
    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(60))
        .build()?;
    
    let releases: serde_json::Value = client
        .get(&format!("https://api.github.com/repos/{}/releases/latest", GITHUB_REPO))
        .header("User-Agent", "System-Monitor-Installer/1.0")
        .send()
        .context("Failed to fetch release info from GitHub")?
        .json()
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    
    let tag_name = releases["tag_name"].as_str().unwrap_or("latest");
    println!("{} Latest version: {}", Style::new().green().apply_to("✓"), tag_name);
    
    let asset_name = if arch == "arm64" {
        format!("linux-system-monitor-{}-windows-arm64.zip", tag_name.trim_start_matches('v'))
    } else {
        format!("linux-system-monitor-{}-windows-x64.zip", tag_name.trim_start_matches('v'))
    };
    
    let download_url = if let Some(assets) = releases["assets"].as_array() {
        assets.iter()
            .find(|a: &&serde_json::Value| {
                let name = a["name"].as_str().unwrap_or("");
                name.to_lowercase().contains("windows") && 
                (name.contains("x64") || name.contains("arm64"))
            })
            .map(|a: &serde_json::Value| a["browser_download_url"].as_str().unwrap_or(""))
            .unwrap_or("")
    } else {
        ""
    };
    
    if download_url.is_empty() {
        println!("{}", Style::new().yellow().apply_to("⚠ No pre-built Windows binary found"));
        println!("{}", Style::new().cyan().apply_to("📦 Building from source..."));
        build_from_source(install_dir)?;
    } else {
        println!("{} Binary: {}", Style::new().green().apply_to("✓"), asset_name);
        println!();
        
        fs::create_dir_all(install_dir)?;
        
        let temp_zip = env::temp_dir().join("linux-system-monitor.zip");
        
        println!("{}", Style::new().cyan().apply_to("📥 Downloading binary..."));
        
        let mut resp = client.get(download_url).send()?;
        let total = resp.content_length().unwrap_or(0);
        
        let mut file = File::create(&temp_zip)?;
        let mut downloaded: u64 = 0;
        let mut buffer = [0u8; 65536];
        
        loop {
            let bytes_read = resp.read(&mut buffer)?;
            if bytes_read == 0 {
                break;
            }
            file.write_all(&buffer[..bytes_read])?;
            downloaded += bytes_read as u64;
            if total > 0 {
                let pct = (downloaded * 100) / total;
                print!("\r{} {}%", Style::new().cyan().apply_to("📥 Downloading..."), pct);
                io::stdout().flush().ok();
            }
        }
        
        println!();
        println!("{}", Style::new().green().apply_to("✓ Download complete"));
        println!();
        
        println!("{}", Style::new().cyan().apply_to("📦 Extracting archive..."));
        extract_zip(&temp_zip, install_dir)?;
        
        let _ = fs::remove_file(&temp_zip);
        println!("{}", Style::new().green().apply_to("✓ Extraction complete"));
    }
    
    let exe_path = install_dir.join("linux-system-monitor.exe");
    if exe_path.exists() {
        println!();
        println!("{} {}", Style::new().green().apply_to("✓ Binary installed:"), exe_path.display());
    }
    
    println!();
    Ok(())
}

fn extract_zip(zip_path: &Path, dest_dir: &Path) -> Result<()> {
    let file = File::open(zip_path)?;
    let mut archive = zip::ZipArchive::new(file)?;
    
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = dest_dir.join(file.name());
        
        if file.name().ends_with('/') {
            fs::create_dir_all(&outpath)?;
        } else {
            if let Some(parent) = outpath.parent() {
                if !parent.exists() {
                    fs::create_dir_all(parent)?;
                }
            }
            
            let mut outfile = File::create(&outpath)?;
            io::copy(&mut file, &mut outfile)?;
        }
    }
    
    Ok(())
}

fn build_from_source(install_dir: &Path) -> Result<()> {
    let temp_dir = env::temp_dir().join("linux-system-monitor-src");
    
    println!();
    println!("{}", Style::new().cyan().apply_to("📦 Cloning repository..."));
    
    if temp_dir.exists() {
        let _ = fs::remove_dir_all(&temp_dir);
    }
    
    #[cfg(windows)]
    let git_status = Command::new("git")
        .creation_flags(CREATE_NO_WINDOW)
        .args(["clone", "--depth", "1", &format!("https://github.com/{}.git", GITHUB_REPO), &temp_dir.display().to_string()])
        .status();
    
    #[cfg(not(windows))]
    let git_status = Command::new("git")
        .args(["clone", "--depth", "1", &format!("https://github.com/{}.git", GITHUB_REPO), &temp_dir.display().to_string()])
        .status();
    
    match git_status {
        Ok(status) if status.success() => {
            println!("{}", Style::new().green().apply_to("✓ Repository cloned"));
        }
        _ => {
            return Err(anyhow::anyhow!("Failed to clone repository. Please ensure git is installed."));
        }
    }
    
    fs::create_dir_all(install_dir)?;
    
    println!();
    println!("{}", Style::new().cyan().apply_to("🔨 Building release binary..."));
    println!("{}", Style::new().yellow().apply_to("  This may take several minutes..."));
    
    #[cfg(windows)]
    let build_result = Command::new("cargo")
        .creation_flags(CREATE_NO_WINDOW)
        .args(["build", "--release"])
        .current_dir(&temp_dir)
        .status();
    
    #[cfg(not(windows))]
    let build_result = Command::new("cargo")
        .args(["build", "--release"])
        .current_dir(&temp_dir)
        .status();
    
    match build_result {
        Ok(status) if status.success() => {
            println!("{}", Style::new().green().apply_to("✓ Build complete"));
            
            let src_exe = temp_dir.join("target").join("release").join("linux-system-monitor.exe");
            if src_exe.exists() {
                fs::copy(&src_exe, install_dir.join("linux-system-monitor.exe"))?;
            }
        }
        _ => {
            let _ = fs::remove_dir_all(&temp_dir);
            return Err(anyhow::anyhow!("Build failed. Please check Rust dependencies."));
        }
    }
    
    let _ = fs::remove_dir_all(&temp_dir);
    
    Ok(())
}

fn create_start_menu_shortcut(install_dir: &Path) -> Result<()> {
    println!("{}", Style::new().cyan().apply_to("🔗 Creating Start Menu shortcut..."));
    
    let start_menu = env::var("APPDATA")
        .map(|p| PathBuf::from(p).join("Microsoft\\Windows\\Start Menu\\Programs"))
        .unwrap_or_else(|_| PathBuf::from("C:\\ProgramData\\Microsoft\\Windows\\Start Menu\\Programs"));
    
    let shortcut_dir = start_menu.join("Linux System Monitor");
    fs::create_dir_all(&shortcut_dir)?;
    
    #[cfg(windows)]
    {
        let shortcut_path = shortcut_dir.join("System Monitor.lnk");
        let exe_path = install_dir.join("linux-system-monitor.exe");
        
        let vbscript = format!(r#"Set WshShell = CreateObject("WScript.Shell")
Set Shortcut = WshShell.CreateShortcut("{}")
Shortcut.TargetPath = "{}"
Shortcut.WorkingDirectory = "{}"
Shortcut.Description = "Linux System Monitor - System Performance Monitoring Tool"
Shortcut.WindowStyle = 1
Shortcut.Save"#, 
            shortcut_path.display(), 
            exe_path.display(), 
            install_dir.display()
        );
        
        let vbs_path = env::temp_dir().join("create_shortcut.vbs");
        fs::write(&vbs_path, vbscript)?;
        
        let _ = Command::new("cscript")
            .creation_flags(CREATE_NO_WINDOW)
            .args(["//Nologo", &vbs_path.display().to_string()])
            .status();
        
        let _ = fs::remove_file(&vbs_path);
    }
    
    #[cfg(windows)]
    {
        let desktop_shortcut = dirs::desktop_dir()
            .map(|d| d.join("Linux System Monitor.lnk"))
            .unwrap_or_else(|| install_dir.join("Linux System Monitor.lnk"));
        
        let exe_path = install_dir.join("linux-system-monitor.exe");
        
        let vbscript = format!(r#"Set WshShell = CreateObject("WScript.Shell")
Set Shortcut = WshShell.CreateShortcut("{}")
Shortcut.TargetPath = "{}"
Shortcut.WorkingDirectory = "{}"
Shortcut.Description = "Linux System Monitor"
Shortcut.Save"#, 
            desktop_shortcut.display(), 
            exe_path.display(), 
            install_dir.display()
        );
        
        let vbs_path = env::temp_dir().join("create_desktop_shortcut.vbs");
        fs::write(&vbs_path, vbscript)?;
        
        let _ = Command::new("cscript")
            .creation_flags(CREATE_NO_WINDOW)
            .args(["//Nologo", &vbs_path.display().to_string()])
            .status();
        
        let _ = fs::remove_file(&vbs_path);
    }
    
    println!("{}", Style::new().green().apply_to("✓ Shortcuts created"));
    println!();
    
    Ok(())
}

fn create_scheduled_task(install_dir: &Path) -> Result<()> {
    println!("{}", Style::new().cyan().apply_to("📅 Setting up auto-start task..."));
    
    #[cfg(windows)]
    {
        let task_name = "LinuxSystemMonitor";
        let exe_path = install_dir.join("linux-system-monitor.exe");
        
        let delete_result = Command::new("schtasks")
            .creation_flags(CREATE_NO_WINDOW)
            .args(["/Delete", "/TN", task_name, "/F"])
            .status();
        
        let _ = delete_result;
        
        let create_result = Command::new("schtasks")
            .creation_flags(CREATE_NO_WINDOW)
            .args([
                "/Create",
                "/TN", task_name,
                "/TR", &exe_path.display().to_string(),
                "/SC", "ONLOGON",
                "/RL", "LIMITED",
                "/F"
            ])
            .status();
        
        match create_result {
            Ok(status) if status.success() => {
                println!("{}", Style::new().green().apply_to("✓ Auto-start task created"));
            }
            _ => {
                println!("{}", Style::new().yellow().apply_to("⚠ Could not create auto-start task (may need admin rights)"));
            }
        }
    }
    
    #[cfg(not(windows))]
    {
        println!("{}", Style::new().yellow().apply_to("⚠ Auto-start configuration not supported on this platform"));
    }
    
    println!();
    Ok(())
}

fn create_uninstall_script(install_dir: &Path) -> Result<()> {
    let uninstall_script = install_dir.join("uninstall.bat");
    
    let script_content = format!(r#"@echo off
echo Uninstalling Linux System Monitor...
echo.

schtasks /Delete /TN "LinuxSystemMonitor" /F 2>nul

del "%USERPROFILE%\Desktop\Linux System Monitor.lnk" 2>nul

echo Removing installation directory...
rd /s /q "{}"

echo.
echo Linux System Monitor has been uninstalled.
echo.
pause
"#, install_dir.display());
    
    fs::write(&uninstall_script, script_content)?;
    
    println!("{}", Style::new().cyan().apply_to("🗑️ Uninstall script created"));
    println!("  To uninstall, run: {}", uninstall_script.display());
    println!();
    
    Ok(())
}

#[allow(dead_code)]
fn ask_yes_no(prompt: &str) -> Result<bool> {
    print!("{}", prompt);
    io::stdout().flush()?;
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    
    let answer = input.trim().to_lowercase();
    Ok(answer.is_empty() || answer == "y" || answer == "yes")
}
