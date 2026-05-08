use std::io::{self, Write};
use std::thread;
use std::time::Duration;
use std::sync::atomic::{AtomicBool, Ordering};
use std::process::Command;
use sysinfo::{System, Disks, CpuRefreshKind, MemoryRefreshKind, RefreshKind};
use std::fs;

static RUNNING: AtomicBool = AtomicBool::new(true);

fn format_bytes(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;
    
    if bytes >= GB {
        format!("{:.1}G", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.1}M", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.1}K", bytes as f64 / KB as f64)
    } else {
        format!("{}B", bytes)
    }
}

fn format_rate(bytes: u64) -> String {
    let rate = bytes as f64 / 1024.0;
    if rate >= 1024.0 {
        format!("{:.1}MB/s", rate / 1024.0)
    } else {
        format!("{:.1}KB/s", rate)
    }
}

fn draw_bar(percent: f32, width: usize) -> String {
    let filled = ((percent / 100.0) * width as f32) as usize;
    let empty = width.saturating_sub(filled);
    format!("{}{}", "█".repeat(filled), "░".repeat(empty))
}

fn format_uptime(seconds: i64) -> String {
    let days = seconds / 86400;
    let hours = (seconds % 86400) / 3600;
    let mins = (seconds % 3600) / 60;
    if days > 0 {
        format!("{}d {}h {}m", days, hours, mins)
    } else if hours > 0 {
        format!("{}h {}m", hours, mins)
    } else {
        format!("{}m", mins)
    }
}

struct TermuxMonitor {
    system: System,
    disks: Disks,
}

impl TermuxMonitor {
    fn new() -> Self {
        let system = System::new_with_specifics(
            RefreshKind::new()
                .with_cpu(CpuRefreshKind::everything())
                .with_memory(MemoryRefreshKind::everything())
        );
        Self {
            system,
            disks: Disks::new_with_refreshed_list(),
        }
    }

    fn refresh(&mut self) {
        self.system.refresh_cpu_specifics(CpuRefreshKind::everything());
        self.system.refresh_memory();
        self.disks.refresh();
    }

    fn get_cpu_usage(&self) -> (f32, usize) {
        let cpus = self.system.cpus();
        let avg = if cpus.is_empty() {
            0.0
        } else {
            let sum: f32 = cpus.iter().map(|c| c.cpu_usage()).sum();
            sum / cpus.len() as f32
        };
        (avg, cpus.len())
    }

    fn get_memory(&self) -> (u64, u64, u64) {
        let total = self.system.total_memory();
        let used = self.system.used_memory();
        let available = self.system.available_memory();
        (total, used, available)
    }

    fn get_battery_info(&self) -> BatteryInfo {
        if let Ok(output) = Command::new("termux-battery-status").output() {
            if output.status.success() {
                let json_str = String::from_utf8_lossy(&output.stdout);
                if let Ok(data) = serde_json::from_str::<serde_json::Value>(&json_str) {
                    return BatteryInfo {
                        level: data["percentage"].as_u64().unwrap_or(0) as u32,
                        temperature: data["temperature"].as_f64().unwrap_or(0.0),
                        health: data["health"].as_str().unwrap_or("unknown").to_string(),
                        status: data["status"].as_str().unwrap_or("unknown").to_string(),
                        plugged: data["plugged"].as_str().unwrap_or("unknown").to_string(),
                    };
                }
            }
        }
        BatteryInfo {
            level: 0,
            temperature: 0.0,
            health: "unknown".to_string(),
            status: "unknown".to_string(),
            plugged: "unknown".to_string(),
        }
    }

    fn get_cpu_temp(&self) -> f64 {
        let paths = [
            "/sys/class/thermal/thermal_zone0/temp",
            "/sys/devices/virtual/thermal/thermal_zone0/temp",
            "/sys/class/hwmon/hwmon0/temp1_input",
        ];
        for path in &paths {
            if let Ok(content) = fs::read_to_string(path) {
                if let Ok(temp) = content.trim().parse::<i64>() {
                    return temp as f64 / 1000.0;
                }
            }
        }
        if let Ok(output) = Command::new("termux-sensor")
            .arg("-s")
            .arg("ambient_thermometer")
            .arg("-n")
            .arg("1")
            .output()
        {
            if output.status.success() {
                let json_str = String::from_utf8_lossy(&output.stdout);
                if let Ok(data) = serde_json::from_str::<serde_json::Value>(&json_str) {
                    if let Some(values) = data["values"].as_array() {
                        if let Some(temp) = values.first() {
                            return temp["ambient_thermometer"].as_f64().unwrap_or(0.0);
                        }
                    }
                }
            }
        }
        0.0
    }

    fn get_storage_info(&self) -> (u64, u64) {
        let mut total: u64 = 0;
        let mut available: u64 = 0;
        for disk in self.disks.iter() {
            total += disk.total_space();
            available += disk.available_space();
        }
        (total, available)
    }

    fn get_proc_meminfo(&self) -> ProcMemInfo {
        let mut info = ProcMemInfo::default();
        if let Ok(content) = fs::read_to_string("/proc/meminfo") {
            for line in content.lines() {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    let value: u64 = parts[1].parse().unwrap_or(0) * 1024;
                    match parts[0].trim_end_matches(':') {
                        "MemTotal" => info.total = value,
                        "MemFree" => info.free = value,
                        "MemAvailable" => info.available = value,
                        "Buffers" => info.buffers = value,
                        "Cached" => info.cached = value,
                        "SwapTotal" => info.swap_total = value,
                        "SwapFree" => info.swap_free = value,
                        _ => {}
                    }
                }
            }
        }
        info.used = info.total.saturating_sub(info.free + info.buffers + info.cached);
        info.swap_used = info.swap_total.saturating_sub(info.swap_free);
        info
    }

    fn get_proc_cpuinfo(&self) -> String {
        if let Ok(content) = fs::read_to_string("/proc/cpuinfo") {
            let mut model = String::new();
            let mut hardware = String::new();
            for line in content.lines() {
                if line.starts_with("model name") || line.starts_with("Processor") {
                    if let Some(val) = line.split(':').nth(1) {
                        model = val.trim().to_string();
                        break;
                    }
                }
                if line.starts_with("Hardware") {
                    if let Some(val) = line.split(':').nth(1) {
                        hardware = val.trim().to_string();
                    }
                }
            }
            if !model.is_empty() {
                return model;
            }
            if !hardware.is_empty() {
                return hardware;
            }
        }
        "Unknown ARM Processor".to_string()
    }

    fn get_uptime(&self) -> i64 {
        if let Ok(content) = fs::read_to_string("/proc/uptime") {
            if let Some(uptime) = content.split_whitespace().next() {
                if let Ok(secs) = uptime.parse::<f64>() {
                    return secs as i64;
                }
            }
        }
        0
    }

    fn get_hostname(&self) -> String {
        System::host_name().unwrap_or_else(|| "android".to_string())
    }

    fn get_top_processes(&self, count: usize) -> Vec<(String, u32, f32)> {
        let total_mem = self.system.total_memory() as f32;
        let mut processes: Vec<_> = self.system.processes().iter()
            .map(|(pid, p)| {
                (
                    p.name().to_string_lossy().into_owned(),
                    pid.as_u32(),
                    p.cpu_usage(),
                    if total_mem > 0.0 { p.memory() as f32 / total_mem * 100.0 } else { 0.0 },
                )
            })
            .collect();
        processes.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap_or(std::cmp::Ordering::Equal));
        processes.into_iter().take(count).map(|(n, p, c, _)| (n, p, c)).collect()
    }
}

#[derive(Default)]
struct ProcMemInfo {
    total: u64,
    free: u64,
    available: u64,
    buffers: u64,
    cached: u64,
    used: u64,
    swap_total: u64,
    swap_free: u64,
    swap_used: u64,
}

struct BatteryInfo {
    level: u32,
    temperature: f64,
    health: String,
    status: String,
    plugged: String,
}

fn setup_signal_handler() {
    ctrlc::set_handler(|| {
        RUNNING.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");
}

fn get_terminal_width() -> usize {
    terminal_size::terminal_size().map(|(w, _)| w.0 as usize).unwrap_or(80).max(60)
}

fn vibrate(duration_ms: u16) {
    let _ = Command::new("termux-vibrate")
        .arg("-d")
        .arg(duration_ms.to_string())
        .spawn();
}

fn render(monitor: &mut TermuxMonitor, width: usize) {
    monitor.refresh();
    
    let (cpu_avg, cpu_count) = monitor.get_cpu_usage();
    let (mem_total, mem_used, _mem_avail) = monitor.get_memory();
    let battery = monitor.get_battery_info();
    let cpu_temp = monitor.get_cpu_temp();
    let (storage_total, storage_avail) = monitor.get_storage_info();
    let meminfo = monitor.get_proc_meminfo();
    let cpu_model = monitor.get_proc_cpuinfo();
    let hostname = monitor.get_hostname();
    let uptime = monitor.get_uptime();
    let processes = monitor.get_top_processes(5);
    
    let mem_pct = if mem_total > 0 { mem_used as f32 / mem_total as f32 * 100.0 } else { 0.0 };
    let storage_pct = if storage_total > 0 { 
        (storage_total - storage_avail) as f32 / storage_total as f32 * 100.0 
    } else { 0.0 };
    
    print!("\x1b[2J\x1b[H");
    
    let double_sep = "═".repeat(width.min(78));
    
    println!("\x1b[1;36m╔{}╗\x1b[0m", double_sep);
    println!("\x1b[1;36m║\x1b[0m \x1b[1;33m◆ Termux System Monitor\x1b[0m  \x1b[32m{}\x1b[0m \x1b[35m{}\x1b[0m \x1b[1;36m║", 
        hostname, format_uptime(uptime));
    println!("\x1b[1;36m╚{}╝\x1b[0m", double_sep);
    
    let bar_w = (width - 30).min(25);
    
    let battery_icon = if battery.level > 80 { "⚡" } else if battery.level > 20 { "🔋" } else { "🪫" };
    let battery_color = if battery.level > 50 { "\x1b[32m" } else if battery.level > 20 { "\x1b[33m" } else { "\x1b[31m" };
    let battery_bar = draw_bar(battery.level as f32, bar_w / 2);
    let temp_color = if cpu_temp > 50.0 { "\x1b[31m" } else if cpu_temp > 35.0 { "\x1b[33m" } else { "\x1b[32m" };
    
    println!("\x1b[1m┌─ Battery & Temperature ────────────────────────────────┐\x1b[0m");
    println!("│ {} {} {:>3}% {}  │ \x1b[90m{}\x1b[0m │", 
        battery_icon, battery_color, battery.level, "\x1b[0m", battery_bar);
    println!("│ \x1b[90mHealth: {} | Status: {} | Temp: {}{:.1}°C\x1b[0m │", 
        battery.health, battery.status, temp_color, cpu_temp);
    println!("\x1b[1m└───────────────────────────────────────────────────────────┘\x1b[0m");
    
    let cpu_bar = draw_bar(cpu_avg, bar_w);
    let cpu_color = if cpu_avg > 80.0 { "\x1b[31m" } else if cpu_avg > 50.0 { "\x1b[33m" } else { "\x1b[32m" };
    
    println!("\x1b[1m┌─ CPU ({}) ───────────────────────────────────────────────┐\x1b[0m", 
        cpu_model.chars().take(30).collect::<String>());
    println!("│ \x1b[33m⚡\x1b[0m Usage: {}{:>5.1}%{}  {} │ \x1b[36m{} cores\x1b[0m │", 
        cpu_color, cpu_avg, "\x1b[0m", cpu_bar, cpu_count);
    println!("\x1b[1m└───────────────────────────────────────────────────────────┘\x0m");
    
    let mem_bar = draw_bar(mem_pct, bar_w);
    let mem_color = if mem_pct > 90.0 { "\x1b[31m" } else if mem_pct > 70.0 { "\x1b[33m" } else { "\x1b[32m" };
    
    println!("\x1b[1m┌─ Memory ({}/{}) ────────────────────────────────────────┐\x1b[0m", 
        format_bytes(mem_used), format_bytes(mem_total));
    println!("│ \x1b[35m💾\x1b[0m RAM:  {}{:>5.1}%{}  {} │ \x1b[90mCached: {}\x1b[0m │", 
        mem_color, mem_pct, "\x1b[0m", mem_bar, format_bytes(meminfo.cached));
    if meminfo.swap_total > 0 {
        let swap_pct = meminfo.swap_used as f32 / meminfo.swap_total as f32 * 100.0;
        let swap_bar = draw_bar(swap_pct, bar_w / 2);
        println!("│ \x1b[34m↔\x1b[0m Swap: {:>5.1}%  {} │ {}/{} │", 
            swap_pct, swap_bar, format_bytes(meminfo.swap_used), format_bytes(meminfo.swap_total));
    }
    println!("\x1b[1m└───────────────────────────────────────────────────────────┘\x1b[0m");
    
    let storage_bar = draw_bar(storage_pct, bar_w);
    let storage_color = if storage_pct > 90.0 { "\x1b[31m" } else if storage_pct > 70.0 { "\x1b[33m" } else { "\x1b[32m" };
    let storage_used = storage_total.saturating_sub(storage_avail);
    
    println!("\x1b[1m┌─ Storage ────────────────────────────────────────────────┐\x1b[0m");
    println!("│ \x1b[32m📦\x1b[0m Used: {}{:>5.1}%{}  {} │ {}/{} │", 
        storage_color, storage_pct, "\x1b[0m", storage_bar, format_bytes(storage_used), format_bytes(storage_total));
    println!("\x1b[1m└───────────────────────────────────────────────────────────┘\x1b[0m");
    
    println!("\x1b[1m┌─ Top Processes ───────────────────────────────────────────┐\x1b[0m");
    println!("│ {:>6} │ {:<24} │ {:>6} │", "PID", "Name", "CPU %");
    println!("│────────┼──────────────────────────┼──────────│");
    for (name, pid, cpu) in processes.iter() {
        let name = if name.len() > 24 { format!("{}..", &name[..22]) } else { name.clone() };
        let cpu_color = if *cpu > 50.0 { "\x1b[31m" } else if *cpu > 25.0 { "\x1b[33m" } else { "\x1b[0m" };
        println!("│ {:>6} │ \x1b[36m{:<24}\x1b[0m │ {} {:>5.1}% │", 
            pid, name, cpu_color, cpu);
    }
    println!("\x1b[1m└───────────────────────────────────────────────────────────┘\x1b[0m");
    
    let sep = "─".repeat(width.min(78));
    println!("\x1b[90m┌─ {} ─┐\x1b[0m", sep);
    println!("\x1b[90m│ Press \x1b[33mCtrl+C\x1b[0m to exit  |  \x1b[33mCtrl+V\x1b[0m to vibrate  \x1b[90m│\x1b[0m");
    println!("\x1b[90m└───────────────────────────────────────────────────────────┘\x1b[0m");
    
    io::stdout().flush().unwrap();
}

fn main() {
    setup_signal_handler();
    
    let mut monitor = TermuxMonitor::new();
    monitor.refresh();
    thread::sleep(Duration::from_millis(200));
    
    println!("\x1b[2J\x1b[H");
    println!("\x1b[1;32mStarting Termux System Monitor...\x1b[0m");
    println!("\x1b[90mCollecting initial data...\x1b[0m");
    
    let termux_available = Command::new("termux-battery-status").output().is_ok();
    if !termux_available {
        println!("\x1b[33mWarning: Termux API not found. Some features may be unavailable.\x1b[0m");
        println!("\x1b[90mInstall termux-api package for full functionality.\x1b[0m");
    }
    
    thread::sleep(Duration::from_millis(500));
    
    while RUNNING.load(Ordering::SeqCst) {
        let width = get_terminal_width();
        render(&mut monitor, width);
        thread::sleep(Duration::from_secs(1));
    }
    
    println!("\x1b[2J\x1b[H");
    println!("\x1b[32mExiting... Thank you for using Termux System Monitor!\x1b[0m\n");
}
