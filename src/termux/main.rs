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
        format!("{:.2}G", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2}M", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.1}K", bytes as f64 / KB as f64)
    } else {
        format!("{}B", bytes)
    }
}

fn format_rate(bytes: u64) -> String {
    let rate = bytes as f64 / 1024.0;
    if rate >= 1024.0 * 1024.0 {
        format!("{:.1}MB/s", rate / 1024.0 / 1024.0)
    } else if rate >= 1024.0 {
        format!("{:.1}KB/s", rate / 1024.0)
    } else {
        format!("{:.0}B/s", rate)
    }
}

fn draw_bar(percent: f32, width: usize) -> String {
    let filled = ((percent.clamp(0.0, 100.0) / 100.0) * width as f32) as usize;
    let empty = width.saturating_sub(filled);
    format!("{}{}", "█".repeat(filled), "░".repeat(empty))
}

fn get_terminal_width() -> usize {
    terminal_size::terminal_size().map(|(w, _)| w.0 as usize).unwrap_or(80).max(60)
}

struct TermuxMonitor {
    system: System,
    disks: Disks,
    prev_net_rx: u64,
    prev_net_tx: u64,
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
            prev_net_rx: 0,
            prev_net_tx: 0,
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
                        voltage: data["voltage"].as_f64().unwrap_or(0.0),
                        current: self.get_battery_current(),
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
            voltage: 0.0,
            current: 0,
        }
    }

    fn get_battery_current(&self) -> i32 {
        if let Ok(content) = fs::read_to_string("/sys/class/power_supply/battery/current_now") {
            if let Ok(current) = content.trim().parse::<i32>() {
                return current / 1000;
            }
        }
        0
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

    fn get_external_storage(&self) -> (u64, u64) {
        if let Ok(output) = Command::new("df").arg("/sdcard").output() {
            let content = String::from_utf8_lossy(&output.stdout);
            for line in content.lines().skip(1) {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 4 {
                    if let (Ok(total), Ok(used)) = (
                        parts[1].parse::<u64>(),
                        parts[2].parse::<u64>(),
                    ) {
                        let block_size: u64 = 1024;
                        return (total * block_size, used * block_size);
                    }
                }
            }
        }
        (0, 0)
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
                        "Active" => info.active = value,
                        "Inactive" => info.inactive = value,
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

    fn get_cpu_info(&self) -> String {
        if let Ok(content) = fs::read_to_string("/proc/cpuinfo") {
            let mut model = String::new();
            let mut hardware = String::new();
            let mut features = String::new();
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
                if line.starts_with("Features") || line.starts_with("flags") {
                    if let Some(val) = line.split(':').nth(1) {
                        features = val.trim().to_string();
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

    fn get_cpu_cores(&self) -> usize {
        self.system.cpus().len()
    }

    fn get_cpu_frequency(&self) -> (u64, u64) {
        let mut min_freq: u64 = u64::MAX;
        let mut max_freq: u64 = 0;
        
        for i in 0..8 {
            let path = format!("/sys/devices/system/cpu/cpu{}/cpufreq/cpuinfo_max_freq", i);
            if let Ok(content) = fs::read_to_string(&path) {
                if let Ok(freq) = content.trim().parse::<u64>() {
                    max_freq = max_freq.max(freq);
                }
            }
        }
        
        if max_freq == 0 {
            if let Ok(content) = fs::read_to_string("/sys/devices/system/cpu/cpu0/cpufreq/cpuinfo_max_freq") {
                if let Ok(freq) = content.trim().parse::<u64>() {
                    max_freq = freq;
                    min_freq = freq;
                }
            }
        }
        
        if min_freq == u64::MAX {
            min_freq = 0;
        }
        
        (min_freq / 1000, max_freq / 1000)
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

    fn get_top_processes(&self, count: usize) -> Vec<(String, u32, f32, u64)> {
        let total_mem = self.system.total_memory() as f32;
        let mut processes: Vec<_> = self.system.processes().iter()
            .map(|(pid, p)| {
                (
                    p.name().to_string(),
                    pid.as_u32(),
                    p.cpu_usage(),
                    p.memory(),
                )
            })
            .collect();
        processes.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap_or(std::cmp::Ordering::Equal));
        processes.into_iter().take(count).collect()
    }

    fn get_network_stats(&mut self) -> (u64, u64) {
        let mut rx: u64 = 0;
        let mut tx: u64 = 0;
        
        if let Ok(content) = fs::read_to_string("/proc/net/dev") {
            for line in content.lines().skip(2) {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 10 {
                    if let (Ok(rx_bytes), Ok(tx_bytes)) = (
                        parts[1].parse::<u64>(),
                        parts[9].parse::<u64>(),
                    ) {
                        rx += rx_bytes;
                        tx += tx_bytes;
                    }
                }
            }
        }
        
        let rx_rate = rx.saturating_sub(self.prev_net_rx);
        let tx_rate = tx.saturating_sub(self.prev_net_tx);
        self.prev_net_rx = rx;
        self.prev_net_tx = tx;
        
        (rx_rate, tx_rate)
    }

    fn get_wifi_info(&self) -> WifiInfo {
        let mut info = WifiInfo::default();
        
        if let Ok(output) = Command::new("termux-wifi-connectioninfo").output() {
            if output.status.success() {
                let json_str = String::from_utf8_lossy(&output.stdout);
                if let Ok(data) = serde_json::from_str::<serde_json::Value>(&json_str) {
                    info.connected = true;
                    info.ssid = data["SSID"].as_str().unwrap_or("Unknown").to_string();
                    info.bssid = data["BSSID"].as_str().unwrap_or("Unknown").to_string();
                    info.ip = data["IP"].as_str().unwrap_or("Unknown").to_string();
                    info.link_speed = data["link_speed"].as_i64().unwrap_or(0) as u32;
                    info.signal_strength = data["rssi"].as_i64().unwrap_or(0) as i32;
                    info.frequency = data["frequency"].as_i64().unwrap_or(0) as u32;
                }
            }
        }
        
        if let Ok(content) = fs::read_to_string("/proc/net/route") {
            let default_interface = content.lines()
                .skip(1)
                .filter_map(|line| {
                    let parts: Vec<&str> = line.split('\t').collect();
                    if parts.len() >= 3 && parts[1] == "00000000" {
                        Some(parts[0].to_string())
                    } else {
                        None
                    }
                })
                .next();
            
            if let Some(iface) = default_interface {
                info.interface = iface;
            }
        }
        
        info
    }

    fn get_data_usage(&self) -> DataUsage {
        let mut usage = DataUsage::default();
        
        if let Ok(content) = fs::read_to_string("/proc/net/xt_qtaguid/iface_stat_all") {
            for line in content.lines() {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 5 {
                    usage.rx_bytes = parts[1].parse().unwrap_or(0);
                    usage.tx_bytes = parts[3].parse().unwrap_or(0);
                }
            }
        }
        
        if let Ok(content) = fs::read_to_string("/sys/class/net/wlan0/statistics/rx_bytes") {
            if let Ok(bytes) = content.trim().parse::<u64>() {
                usage.wifi_rx = bytes;
            }
        }
        if let Ok(content) = fs::read_to_string("/sys/class/net/wlan0/statistics/tx_bytes") {
            if let Ok(bytes) = content.trim().parse::<u64>() {
                usage.wifi_tx = bytes;
            }
        }
        if let Ok(content) = fs::read_to_string("/sys/class/net/rmnet0/statistics/rx_bytes") {
            if let Ok(bytes) = content.trim().parse::<u64>() {
                usage.mobile_rx = bytes;
            }
        }
        if let Ok(content) = fs::read_to_string("/sys/class/net/rmnet0/statistics/tx_bytes") {
            if let Ok(bytes) = content.trim().parse::<u64>() {
                usage.mobile_tx = bytes;
            }
        }
        
        usage
    }

    fn get_sensors(&self) -> Vec<SensorInfo> {
        let mut sensors = Vec::new();
        
        if let Ok(output) = Command::new("termux-sensor")
            .arg("-l")
            .output()
        {
            if output.status.success() {
                let content = String::from_utf8_lossy(&output.stdout);
                for line in content.lines() {
                    if !line.is_empty() && !line.starts_with("Available") {
                        sensors.push(SensorInfo {
                            name: line.trim().to_string(),
                            value: String::new(),
                        });
                    }
                }
            }
        }
        
        if sensors.len() > 5 {
            sensors.truncate(5);
        }
        
        sensors
    }

    fn get_torch_status(&self) -> bool {
        if let Ok(content) = fs::read_to_string("/sys/class/leds/flashlight/brightness") {
            if let Ok(brightness) = content.trim().parse::<u32>() {
                return brightness > 0;
            }
        }
        false
    }

    fn get_load_average(&self) -> (f64, f64, f64) {
        if let Ok(content) = fs::read_to_string("/proc/loadavg") {
            let parts: Vec<&str> = content.split_whitespace().collect();
            if parts.len() >= 3 {
                return (
                    parts[0].parse().unwrap_or(0.0),
                    parts[1].parse().unwrap_or(0.0),
                    parts[2].parse().unwrap_or(0.0),
                );
            }
        }
        (0.0, 0.0, 0.0)
    }

    fn get_inodes(&self) -> (u64, u64, u64) {
        if let Ok(content) = fs::read_to_string("/proc/sys/fs/inotify") {
            if let Ok(max) = content.trim().parse::<u64>() {
                return (0, 0, max);
            }
        }
        (0, 0, 0)
    }
}

struct BatteryInfo {
    level: u32,
    temperature: f64,
    health: String,
    status: String,
    plugged: String,
    voltage: f64,
    current: i32,
}

#[derive(Default)]
struct ProcMemInfo {
    total: u64,
    free: u64,
    available: u64,
    buffers: u64,
    cached: u64,
    active: u64,
    inactive: u64,
    used: u64,
    swap_total: u64,
    swap_free: u64,
    swap_used: u64,
}

struct WifiInfo {
    connected: bool,
    ssid: String,
    bssid: String,
    ip: String,
    link_speed: u32,
    signal_strength: i32,
    frequency: u32,
    interface: String,
}

impl Default for WifiInfo {
    fn default() -> Self {
        Self {
            connected: false,
            ssid: "未连接".to_string(),
            bssid: "00:00:00:00:00:00".to_string(),
            ip: "0.0.0.0".to_string(),
            link_speed: 0,
            signal_strength: 0,
            frequency: 0,
            interface: "wlan0".to_string(),
        }
    }
}

struct DataUsage {
    rx_bytes: u64,
    tx_bytes: u64,
    wifi_rx: u64,
    wifi_tx: u64,
    mobile_rx: u64,
    mobile_tx: u64,
}

impl Default for DataUsage {
    fn default() -> Self {
        Self {
            rx_bytes: 0,
            tx_bytes: 0,
            wifi_rx: 0,
            wifi_tx: 0,
            mobile_rx: 0,
            mobile_tx: 0,
        }
    }
}

struct SensorInfo {
    name: String,
    value: String,
}

fn setup_signal_handler() {
    ctrlc::set_handler(|| {
        RUNNING.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");
}

fn vibrate(duration_ms: u16) {
    let _ = Command::new("termux-vibrate")
        .arg("-d")
        .arg(duration_ms.to_string())
        .spawn();
}

fn vibrate_pattern() {
    let _ = Command::new("termux-vibrate")
        .arg("-p")
        .arg("500")
        .spawn();
}

fn toggle_torch() {
    let current_state = fs::read_to_string("/sys/class/leds/flashlight/brightness")
        .ok()
        .and_then(|s| s.trim().parse::<u32>().ok())
        .unwrap_or(0);
    
    let new_state = if current_state > 0 { "0" } else { "255" };
    let _ = fs::write("/sys/class/leds/flashlight/brightness", new_state);
}

fn render(monitor: &mut TermuxMonitor, width: usize) {
    monitor.refresh();
    
    let (cpu_avg, cpu_count) = monitor.get_cpu_usage();
    let (mem_total, mem_used, _mem_avail) = monitor.get_memory();
    let battery = monitor.get_battery_info();
    let cpu_temp = monitor.get_cpu_temp();
    let (storage_total, storage_avail) = monitor.get_storage_info();
    let (ext_total, ext_used) = monitor.get_external_storage();
    let meminfo = monitor.get_proc_meminfo();
    let cpu_model = monitor.get_cpu_info();
    let hostname = monitor.get_hostname();
    let uptime = monitor.get_uptime();
    let processes = monitor.get_top_processes(5);
    let (net_rx, net_tx) = monitor.get_network_stats();
    let wifi = monitor.get_wifi_info();
    let data_usage = monitor.get_data_usage();
    let (cpu_min_freq, cpu_max_freq) = monitor.get_cpu_frequency();
    let sensors = monitor.get_sensors();
    let torch_on = monitor.get_torch_status();
    let (load1, load5, load15) = monitor.get_load_average();
    let (min_inodes, used_inodes, max_inodes) = monitor.get_inodes();
    
    let mem_pct = if mem_total > 0 { mem_used as f32 / mem_total as f32 * 100.0 } else { 0.0 };
    let storage_pct = if storage_total > 0 { 
        (storage_total - storage_avail) as f32 / storage_total as f32 * 100.0 
    } else { 0.0 };
    let ext_pct = if ext_total > 0 { ext_used as f32 / ext_total as f32 * 100.0 } else { 0.0 };
    
    print!("\x1b[2J\x1b[H");
    
    let double_sep = "═".repeat(width.min(78));
    
    println!("\x1b[1;36m╔{}╗\x1b[0m", double_sep);
    println!("\x1b[1;36m║\x1b[0m \x1b[1;33m◆ Termux 系统监控 v5.0.0\x1b[0m");
    println!("\x1b[1;36m║\x1b[0m \x1b[90m{}\x1b[0m | \x1b[35m运行: {}\x1b[0m | \x1b[33m负载: {:.2}\x1b[0m | \x1b[32m手电: {}\x1b[0m \x1b[36m║", 
        hostname, format_uptime(uptime), load1, if torch_on { "开" } else { "关" });
    println!("\x1b[1;36m╚{}╝\x1b[0m", double_sep);
    
    let bar_w = (width - 30).min(25);
    
    let battery_icon = if battery.level > 80 { "⚡" } else if battery.level > 20 { "🔋" } else { "🪫" };
    let battery_color = if battery.level > 50 { "\x1b[32m" } else if battery.level > 20 { "\x1b[33m" } else { "\x1b[31m" };
    let battery_bar = draw_bar(battery.level as f32, bar_w / 2);
    let temp_color = if cpu_temp > 50.0 { "\x1b[31m" } else if cpu_temp > 35.0 { "\x1b[33m" } else { "\x1b[32m" };
    
    println!("\x1b[1m┌─ 电池与电量 ────────────────────────────────────────────────────────────┐\x1b[0m");
    println!("│ {} {} {:>3}% {}  │ 健康状态: {} │", 
        battery_icon, battery_color, battery.level, "\x1b[0m", battery.health);
    println!("│ \x1b[90m温度: {}{:.1}°C\x1b[0m | 状态: {} | 充电: {} | 电压: {:.2}V | 电流: {}mA │", 
        temp_color, cpu_temp, battery.status, battery.plugged, battery.voltage / 1000.0, battery.current);
    println!("\x1b[1m└─────────────────────────────────────────────────────────────────────────────┘\x1b[0m");
    
    let cpu_bar = draw_bar(cpu_avg, bar_w);
    let cpu_color = if cpu_avg > 80.0 { "\x1b[31m" } else if cpu_avg > 50.0 { "\x1b[33m" } else { "\x1b[32m" };
    
    println!("\x1b[1m┌─ CPU ({}) ───────────────────────────────────────────────┐\x1b[0m", 
        cpu_model.chars().take(25).collect::<String>());
    println!("│ \x1b[33m⚡\x1b[0m 使用率: {}{:>5.1}%{}  {} │ \x1b[36m{} 核\x1b[0m │", 
        cpu_color, cpu_avg, "\x1b[0m", cpu_bar, cpu_count);
    println!("│ \x1b[90m频率: {}-{} MHz\x1b[0m                                           │", 
        cpu_min_freq, cpu_max_freq);
    println!("\x1b[1m└───────────────────────────────────────────────────────────┘\x1b[0m");
    
    let mem_bar = draw_bar(mem_pct, bar_w);
    let mem_color = if mem_pct > 90.0 { "\x1b[31m" } else if mem_pct > 70.0 { "\x1b[33m" } else { "\x1b[32m" };
    
    println!("\x1b[1m┌─ 内存 ({}/{}) ────────────────────────────────────────┐\x1b[0m", 
        format_bytes(mem_used), format_bytes(mem_total));
    println!("│ \x1b[35m💾\x1b[0m 内存: {}{:>5.1}%{}  {} │ \x1b[90m缓存: {}\x1b[0m │", 
        mem_color, mem_pct, "\x1b[0m", mem_bar, format_bytes(meminfo.cached));
    println!("│ \x1b[90m活跃: {} | 非活跃: {}\x1b[0m                        │", 
        format_bytes(meminfo.active), format_bytes(meminfo.inactive));
    if meminfo.swap_total > 0 {
        let swap_pct = meminfo.swap_used as f32 / meminfo.swap_total as f32 * 100.0;
        let swap_bar = draw_bar(swap_pct, bar_w / 2);
        println!("│ \x1b[34m↔\x1b[0m 交换区: {:>5.1}%  {} │ {}/{} │", 
            swap_pct, swap_bar, format_bytes(meminfo.swap_used), format_bytes(meminfo.swap_total));
    }
    println!("\x1b[1m└───────────────────────────────────────────────────────────┘\x1b[0m");
    
    let storage_bar = draw_bar(storage_pct, bar_w);
    let storage_color = if storage_pct > 90.0 { "\x1b[31m" } else if storage_pct > 70.0 { "\x1b[33m" } else { "\x1b[32m" };
    let storage_used = storage_total.saturating_sub(storage_avail);
    
    println!("\x1b[1m┌─ 存储 (内置) ──────────────────────────────────────┐\x1b[0m");
    println!("│ \x1b[32m📦\x1b[0m 已用: {}{:>5.1}%{}  {} │ {}/{} │", 
        storage_color, storage_pct, "\x1b[0m", storage_bar, format_bytes(storage_used), format_bytes(storage_total));
    if ext_total > 0 {
        let ext_bar = draw_bar(ext_pct, bar_w / 2);
        println!("│ \x1b[34m📁\x1b[0m 外部: {:>5.1}%  {} │ {}/{} │", 
            ext_pct, ext_bar, format_bytes(ext_used), format_bytes(ext_total));
    }
    println!("│ \x1b[90mInode: {}/{} (最大: {})\x1b[0m                              │", 
        used_inodes, min_inodes, max_inodes);
    println!("\x1b[1m└───────────────────────────────────────────────────────────┘\x1b[0m");
    
    println!("\x1b[1m┌─ 网络 ──────────────────────────────────────────────────┐\x1b[0m");
    println!("│ \x1b[32m↓\x1b[0m 下载: {:>10}/s  \x1b[31m↑\x1b[0m 上传: {:>10}/s │", 
        format_rate(net_rx), format_rate(net_tx));
    println!("│ \x1b[90m总计: ↓{} ↑{}\x1b[0m                                   │", 
        format_bytes(data_usage.wifi_rx + data_usage.mobile_rx),
        format_bytes(data_usage.wifi_tx + data_usage.mobile_tx));
    if wifi.connected {
        println!("│ \x1b[36m📶\x1b[0m WiFi: {} | {} Mbps | {} dBm | {} MHz │", 
            wifi.ssid, wifi.link_speed, wifi.signal_strength, wifi.frequency);
        println!("│ \x1b[90mIP: {} | BSSID: {}\x1b[0m                       │", wifi.ip, wifi.bssid);
    } else {
        println!("│ \x1b[33m📶\x1b[0m WiFi: 未连接\x1b[0m                                 │");
    }
    println!("\x1b[1m└───────────────────────────────────────────────────────────┘\x1b[0m");
    
    if !sensors.is_empty() {
        println!("\x1b[1m┌─ 传感器 ──────────────────────────────────────────────────┐\x1b[0m");
        for sensor in sensors.iter().take(3) {
            println!("│ \x1b[36m◉\x1b[0m {}                                               │", sensor.name.chars().take(50).collect::<String>());
        }
        println!("\x1b[1m└───────────────────────────────────────────────────────────┘\x1b[0m");
    }
    
    println!("\x1b[1m┌─ 进程排行 ────────────────────────────────────────────┐\x1b[0m");
    println!("│ {:>6} │ {:<20} │ {:>6} │ {:>8} │", "PID", "名称", "CPU %", "内存");
    println!("│────────┼──────────────────────┼──────────┼──────────│");
    for (name, pid, cpu, mem) in processes.iter() {
        let name = if name.len() > 20 { format!("{}..", &name[..18]) } else { name.clone() };
        let cpu_color = if *cpu > 50.0 { "\x1b[31m" } else if *cpu > 25.0 { "\x1b[33m" } else { "\x1b[0m" };
        println!("│ {:>6} │ \x1b[36m{:<20}\x1b[0m │ {} {:>5.1}% │ {:>8} │", 
            pid, name, cpu_color, cpu, format_bytes(*mem));
    }
    println!("\x1b[1m└───────────────────────────────────────────────────────────┘\x1b[0m");
    
    let sep = "─".repeat(width.min(78));
    println!("\x1b[90m┌─ {} ─┐\x1b[0m", sep);
    println!("\x1b[90m│ \x1b[33mCtrl+C\x1b[0m 退出  |  \x1b[33mCtrl+V\x1b[0m 震动  |  \x1b[33mCtrl+T\x1b[0m 手电  |  \x1b[33mCtrl+L\x1b[0m 负载  \x1b[90m│\x1b[0m");
    println!("\x1b[90m└───────────────────────────────────────────────────────────┘\x1b[0m");
    
    io::stdout().flush().unwrap();
}

fn format_uptime(seconds: i64) -> String {
    let days = seconds / 86400;
    let hours = (seconds % 86400) / 3600;
    let mins = (seconds % 3600) / 60;
    if days > 0 {
        format!("{}天 {}小时 {}分", days, hours, mins)
    } else if hours > 0 {
        format!("{}小时 {}分", hours, mins)
    } else {
        format!("{}分", mins)
    }
}

fn main() {
    setup_signal_handler();
    
    let mut monitor = TermuxMonitor::new();
    monitor.refresh();
    thread::sleep(Duration::from_millis(200));
    
    println!("\x1b[2J\x1b[H");
    println!("\x1b[1;32m╔═══════════════════════════════════════════╗\x1b[0m");
    println!("\x1b[1;32m║\x1b[0m     \x1b[1;33mTermux 系统监控 v5.0.0\x1b[0m      \x1b[1;32m║\x1b[0m");
    println!("\x1b[1;32m║\x1b[0m     \x1b[36m50个手机优化功能\x1b[0m       \x1b[1;32m║\x1b[0m");
    println!("\x1b[1;32m╚═══════════════════════════════════════════╝\x1b[0m");
    println!("\x1b[90m正在收集初始数据...\x1b[0m");
    
    let termux_available = Command::new("termux-battery-status").output().is_ok();
    if !termux_available {
        println!("\x1b[33m警告: 未找到Termux API。部分功能可能不可用。\x1b[0m");
        println!("\x1b[90m安装termux-api包以获得完整功能。\x1b[0m");
    }
    
    thread::sleep(Duration::from_millis(500));
    
    let mut torch_enabled = false;
    
    while RUNNING.load(Ordering::SeqCst) {
        let width = get_terminal_width();
        render(&mut monitor, width);
        
        use std::io::Read;
        let mut buffer = [0u8; 1];
        if std::io::stdin().read(&mut buffer).is_ok() {
            match buffer[0] as char {
                'v' | 'V' => {
                    vibrate_pattern();
                }
                't' | 'T' => {
                    torch_enabled = !torch_enabled;
                    toggle_torch();
                }
                'l' | 'L' => {
                    let (load1, load5, load15) = monitor.get_load_average();
                    println!("\x1b[2J\x1b[H");
                    println!("\x1b[32m负载平均值: {:.2} (1分) {:.2} (5分) {:.2} (15分)\x1b[0m", load1, load5, load15);
                    thread::sleep(Duration::from_secs(2));
                }
                'q' | 'Q' => break,
                _ => {}
            }
        }
        
        thread::sleep(Duration::from_secs(1));
    }
    
    println!("\x1b[2J\x1b[H");
    println!("\x1b[32m╔═══════════════════════════════════╗\x1b[0m");
    println!("\x1b[32m║\x1b[0m  感谢使用Termux系统监控！  \x1b[32m║\x1b[0m");
    println!("\x1b[32m╚═══════════════════════════════════╝\x1b[0m\n");
}
