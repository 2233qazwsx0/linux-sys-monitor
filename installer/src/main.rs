use std::env;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::PathBuf;
use std::process::Command;
use anyhow::{Context, Result};
use console::Style;

const GITHUB_REPO: &str = "2233qazwsx0/linux-sys-monitor";

fn main() -> Result<()> {
    println!();
    println!("{}", Style::new().cyan().apply_to("╔══════════════════════════════════════════╗"));
    println!("{}", Style::new().cyan().apply_to("║   System Monitor Installer v1.0.0        ║"));
    println!("{}", Style::new().cyan().apply_to("║   Rust-based Windows Installer           ║"));
    println!("{}", Style::new().cyan().apply_to("╚══════════════════════════════════════════╝"));
    println!();

    let install_dir = get_install_dir();
    
    println!("{}", Style::new().yellow().apply_to("📦 Installation Directory: ") + &install_dir.display().to_string());
    println!();

    check_admin()?;
    
    install_rust()?;
    
    download_binary(&install_dir)?;
    
    create_shortcut(&install_dir)?;
    
    create_start_script(&install_dir)?;
    
    println!();
    println!("{}", Style::new().green().apply_to("✅ Installation Complete!"));
    println!();
    println!("To start:");
    println!("  {}", Style::new().cyan().apply_to(format!("cd {} && .\\linux-system-monitor.exe", &install_dir.display())));
    println!();
    println!("Then open {}", Style::new().cyan().apply_to("http://localhost:8080"));
    println!();
    
    let start_now = ask("Start now? (Y/n): ")?;
    if start_now.to_lowercase() != "n" {
        println!("Starting...");
        Command::new("cmd")
            .args(["/C", "start", "cmd", "/K", &format!("cd {} && .\\linux-system-monitor.exe", install_dir.display())])
            .spawn()?;
    }
    
    Ok(())
}

fn get_install_dir() -> PathBuf {
    if let Ok(dir) = env::var("MONITOR_HOME") {
        PathBuf::from(dir)
    } else {
        dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("system-monitor")
    }
}

fn check_admin() -> Result<()> {
    println!("{}", Style::new().cyan().apply_to("🔍 Checking permissions..."));
    let output = Command::new("net")
        .args(["session"])
        .output();
    
    match output {
        Ok(o) if o.status.success() => {
            println!("{}", Style::new().green().apply_to("✓ Running as Administrator"));
        }
        _ => {
            println!("{}", Style::new().yellow().apply_to("⚠ Not running as Administrator (some features may be limited)"));
        }
    }
    println!();
    Ok(())
}

fn install_rust() -> Result<()> {
    println!("{}", Style::new().cyan().apply_to("🔧 Checking Rust..."));
    
    let rustc_check = Command::new("rustc")
        .arg("--version")
        .output();
    
    match rustc_check {
        Ok(output) if output.status.success() => {
            let version = String::from_utf8_lossy(&output.stdout);
            println!("{}", Style::new().green().apply_to(format!("✓ Rust found: {}", version.trim())));
        }
        _ => {
            println!("{}", Style::new().yellow().apply_to("📥 Installing Rust..."));
            
            let mut resp = reqwest::blocking::get("https://win.rustup.rs")?;
            let mut buffer = Vec::new();
            resp.read_to_end(&mut buffer)?;
            
            let mut file = File::create("rustup-init.exe")?;
            file.write_all(&buffer)?;
            
            Command::new("rustup-init.exe")
                .args(["-y", "--default-toolchain", "stable"])
                .status()
                .context("Failed to run rustup-init")?;
            
            let _ = fs::remove_file("rustup-init.exe");
            println!("{}", Style::new().green().apply_to("✓ Rust installed successfully"));
        }
    }
    println!();
    Ok(())
}

fn download_binary(install_dir: &PathBuf) -> Result<()> {
    println!("{}", Style::new().cyan().apply_to("📥 Getting latest release info..."));
    
    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()?;
    
    let response = client
        .get(&format!("https://api.github.com/repos/{}/releases/latest", GITHUB_REPO))
        .header("User-Agent", "System-Monitor-Installer")
        .send()
        .context("Failed to fetch release info")?;
    
    let json: serde_json::Value = response.json()?;
    
    let download_url = json["assets"]
        .as_array()
        .and_then(|assets| assets.iter().find(|a| a["name"].as_str().unwrap_or("").contains("windows")))
        .map(|a| a["browser_download_url"].as_str().unwrap_or(""))
        .unwrap_or("");
    
    if download_url.is_empty() {
        println!("{}", Style::new().yellow().apply_to("⚠ No Windows binary found, building from source..."));
        build_from_source(install_dir)?;
        return Ok(());
    }
    
    println!("{}", Style::new().green().apply_to(format!("✓ Found binary: {}", download_url)));
    println!();
    
    fs::create_dir_all(install_dir)?;
    
    println!("{}", Style::new().cyan().apply_to("📥 Downloading binary..."));
    
    let mut resp = client.get(download_url).send()?;
    let total = resp.content_length().unwrap_or(0);
    
    let mut file = File::create(install_dir.join("monitor.tar.gz"))?;
    let mut downloaded: u64 = 0;
    let mut buffer = [0u8; 8192];
    
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
    
    extract_and_setup(install_dir)?;
    
    Ok(())
}

fn build_from_source(install_dir: &PathBuf) -> Result<()> {
    println!("{}", Style::new().cyan().apply_to("📦 Cloning repository..."));
    
    fs::create_dir_all(install_dir)?;
    
    Command::new("git")
        .args(["clone", &format!("https://github.com/{}.git", GITHUB_REPO), &install_dir.display().to_string()])
        .status()
        .context("Failed to clone repository")?;
    
    println!("{}", Style::new().green().apply_to("✓ Repository cloned"));
    
    if !Command::new("cargo")
        .args(["build", "--release"])
        .current_dir(install_dir)
        .status()?
        .success()
    {
        anyhow::bail!("Failed to build from source");
    }
    
    println!("{}", Style::new().green().apply_to("✓ Build complete"));
    
    Ok(())
}

fn extract_and_setup(install_dir: &PathBuf) -> Result<()> {
    println!("{}", Style::new().cyan().apply_to("📦 Extracting files..."));
    
    let tarball = install_dir.join("monitor.tar.gz");
    
    Command::new("tar")
        .args(["-xzf", &tarball.display().to_string()])
        .current_dir(install_dir)
        .status()?;
    
    let _ = fs::remove_file(tarball);
    
    let extracted = install_dir.join("linux-system-monitor");
    if extracted.exists() {
        for entry in fs::read_dir(&extracted)? {
            let entry = entry?;
            let dest = install_dir.join(entry.file_name());
            fs::rename(entry.path(), dest).ok();
        }
        let _ = fs::remove_dir(extracted);
    }
    
    println!("{}", Style::new().green().apply_to("✓ Files extracted"));
    println!();
    Ok(())
}

fn create_shortcut(install_dir: &PathBuf) -> Result<()> {
    println!("{}", Style::new().cyan().apply_to("🔗 Creating desktop shortcut..."));
    
    let desktop = dirs::desktop_dir().unwrap_or_else(|| PathBuf::from("."));
    let shortcut_path = desktop.join("System Monitor.lnk");
    
    let vbscript = format!(r#"Set WshShell = CreateObject("WScript.Shell")
Set Shortcut = WshShell.CreateShortcut("{}")
Shortcut.TargetPath = "{}"
Shortcut.WorkingDirectory = "{}"
Shortcut.Description = "System Monitor"
Shortcut.Save"#, 
        shortcut_path.display(), 
        install_dir.join("linux-system-monitor.exe").display(), 
        install_dir.display()
    );
    
    fs::write("create_shortcut.vbs", vbscript)?;
    
    Command::new("cscript")
        .args(["//Nologo", "create_shortcut.vbs"])
        .status()?;
    
    let _ = fs::remove_file("create_shortcut.vbs");
    
    println!("{}", Style::new().green().apply_to("✓ Shortcut created"));
    println!();
    Ok(())
}

fn create_start_script(install_dir: &PathBuf) -> Result<()> {
    let script = format!(
        "@echo off\ncd /d \"{}\"\necho Starting System Monitor...\necho Open http://localhost:8080\nlinux-system-monitor.exe\n",
        install_dir.display()
    );
    
    fs::write(install_dir.join("start.bat"), script)?;
    
    println!("{}", Style::new().green().apply_to("✓ Start script created"));
    Ok(())
}

fn ask(prompt: &str) -> Result<String> {
    print!("{}", prompt);
    io::stdout().flush()?;
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    
    Ok(input.trim().to_string())
}
