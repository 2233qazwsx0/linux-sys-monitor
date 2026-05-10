use std::io::{self, Write, Read};
use std::thread;
use std::time::Duration;
use std::sync::atomic::{AtomicBool, Ordering};
use sysinfo::{System, Disks, CpuRefreshKind, MemoryRefreshKind, RefreshKind, ProcessRefreshKind, Networks};
use std::collections::HashMap;

static RUNNING: AtomicBool = AtomicBool::new(true);

mod cpu;
mod memory;
mod network;
mod disk;
mod process;
mod system_info;
mod services;
mod containers;
mod gpu;
mod alerts;

pub use cpu::*;
pub use memory::*;
pub use network::*;
pub use disk::*;
pub use process::*;
pub use system_info::*;
pub use services::*;
pub use containers::*;
pub use gpu::*;
pub use alerts::{Alert, AlertThresholds, check_alerts_impl};

fn format_bytes(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;
    const TB: u64 = GB * 1024;
    
    if bytes >= TB {
        format!("{:.2}T", bytes as f64 / TB as f64)
    } else if bytes >= GB {
        format!("{:.2}G", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2}M", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2}K", bytes as f64 / KB as f64)
    } else {
        format!("{}B", bytes)
    }
}

fn format_rate(bytes: u64) -> String {
    let rate = bytes as f64 / 1024.0;
    if rate >= 1024.0 * 1024.0 {
        format!("{:.2}MB/s", rate / 1024.0 / 1024.0)
    } else if rate >= 1024.0 {
        format!("{:.2}KB/s", rate / 1024.0)
    } else {
        format!("{:.2}B/s", rate)
    }
}

fn format_percent(value: f32) -> String {
    format!("{:.1}%", value)
}

fn draw_bar(percent: f32, width: usize) -> String {
    let filled = ((percent.clamp(0.0, 100.0) / 100.0) * width as f32) as usize;
    let empty = width.saturating_sub(filled);
    format!("{}{}", "█".repeat(filled), "░".repeat(empty))
}

fn color_by_load(percent: f32) -> &'static str {
    if percent >= 90.0 {
        "\x1b[91m"
    } else if percent >= 70.0 {
        "\x1b[93m"
    } else if percent >= 50.0 {
        "\x1b[33m"
    } else {
        "\x1b[92m"
    }
}

fn get_terminal_width() -> usize {
    terminal_size::terminal_size().map(|(w, _)| w.0 as usize).unwrap_or(120).min(140).max(80)
}

fn get_terminal_height() -> usize {
    terminal_size::terminal_size().map(|(_, h)| h.0 as usize).unwrap_or(40).min(60).max(24)
}

struct CliMonitor {
    system: System,
    disks: Disks,
    networks: Networks,
    prev_net_stats: HashMap<String, (u64, u64)>,
    prev_disk_stats: HashMap<String, (u64, u64)>,
    prev_cpu_times: Vec<u64>,
    alert_thresholds: AlertThresholds,
}

impl CliMonitor {
    fn new() -> Self {
        let system = System::new_with_specifics(
            RefreshKind::new()
                .with_cpu(CpuRefreshKind::everything())
                .with_memory(MemoryRefreshKind::everything())
                .with_processes(ProcessRefreshKind::everything())
        );
        Self {
            system,
            disks: Disks::new_with_refreshed_list(),
            networks: Networks::new_with_refreshed_list(),
            prev_net_stats: HashMap::new(),
            prev_disk_stats: HashMap::new(),
            prev_cpu_times: Vec::new(),
            alert_thresholds: AlertThresholds::default(),
        }
    }

    fn refresh(&mut self) {
        self.system.refresh_cpu_specifics(CpuRefreshKind::everything());
        self.system.refresh_memory();
        self.system.refresh_processes();
        self.disks.refresh();
        self.networks.refresh();
    }

    fn get_cpu_info(&self) -> CpuInfo {
        get_cpu_info_impl(&self.system)
    }

    fn get_per_core_usage(&self) -> Vec<(String, f32)> {
        get_per_core_usage_impl(&self.system)
    }

    fn get_cpu_frequency(&self) -> CpuFrequency {
        get_cpu_frequency_impl(&self.system)
    }

    fn get_cpu_temperature(&self) -> CpuTemperature {
        get_cpu_temperature_impl()
    }

    fn get_memory_info(&self) -> MemoryInfo {
        get_memory_info_impl(&self.system)
    }

    fn get_swap_info(&self) -> SwapInfo {
        get_swap_info_impl()
    }

    fn get_network_info(&mut self) -> NetworkInfo {
        get_network_info_impl(&self.networks, &mut self.prev_net_stats)
    }

    fn get_disk_info(&mut self) -> DiskInfo {
        get_disk_info_impl(&self.disks, &mut self.prev_disk_stats)
    }

    fn get_processes(&self, sort_by: ProcessSort) -> Vec<ProcessInfo> {
        get_processes_impl(&self.system, sort_by)
    }

    fn get_process_tree(&self) -> Vec<ProcessTreeNode> {
        get_process_tree_impl(&self.system)
    }

    fn get_system_info(&self) -> SystemInfoData {
        get_system_info_impl()
    }

    fn get_load_average(&self) -> LoadAverage {
        get_load_average_impl()
    }

    fn get_users(&self) -> Vec<UserInfo> {
        get_users_impl()
    }

    fn get_services(&self) -> ServicesInfo {
        get_services_impl()
    }

    fn get_containers(&self) -> ContainersInfo {
        get_containers_impl()
    }

    fn get_gpu_info(&self) -> GpuInfo {
        get_gpu_info_impl()
    }

    fn check_alerts(&self) -> Vec<Alert> {
        check_alerts_impl(&self.system, &self.disks, &self.alert_thresholds)
    }

    fn set_alert_threshold(&mut self, name: &str, value: f32) {
        match name {
            "cpu" => self.alert_thresholds.cpu = value,
            "memory" => self.alert_thresholds.memory = value,
            "disk" => self.alert_thresholds.disk = value,
            "temp" => self.alert_thresholds.temperature = value,
            _ => {}
        }
    }
}

fn setup_signal_handler() {
    ctrlc::set_handler(|| {
        RUNNING.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");
}

fn render_dashboard(monitor: &mut CliMonitor, width: usize, height: usize, view: &str) {
    monitor.refresh();
    
    let alerts = monitor.check_alerts();
    
    print!("\x1b[2J\x1b[H");
    
    let sep = "─".repeat(width.min(138));
    let double_sep = "═".repeat(width.min(138));
    
    println!("\x1b[1;36m╔{}╗\x1b[0m", double_sep);
    println!("\x1b[1;36m║\x1b[0m \x1b[1;33m◆ Linux System Monitor CLI v5.0.0\x1b[0m");
    println!("\x1b[1;36m║\x1b[0m \x1b[90mView: \x1b[94m{}\x1b[0m \x1b[90m| Terminal: {}x{} \x1b[90m| Alerts: {} \x1b[0m \x1b[36m║", 
        view, width, height, alerts.len());
    println!("\x1b[1;36m╚{}╝\x1b[0m", double_sep);
    
    if !alerts.is_empty() {
        println!("\x1b[1;31m⚠ Alerts:\x1b[0m");
        for alert in alerts.iter().take(3) {
            println!("  \x1b[91m{}\x1b[0m [{}] {}", alert.level, alert.category, alert.message);
        }
        println!();
    }
    
    match view {
        "full" | "cpu" => render_cpu_view(monitor, width),
        "memory" => render_memory_view(monitor, width),
        "network" => render_network_view(monitor, width),
        "disk" => render_disk_view(monitor, width),
        "process" => render_process_view(monitor, width, height),
        "system" => render_system_view(monitor, width),
        "services" => render_services_view(monitor, width),
        "containers" => render_containers_view(monitor, width),
        "gpu" => render_gpu_view(monitor, width),
        _ => render_full_dashboard(monitor, width, height),
    }
    
    println!("\x1b[90m┌─ {} ─┐\x1b[0m", sep);
    println!("\x1b[90m│ \x1b[33m1\x1b[0m Full \x1b[33m2\x1b[0m CPU \x1b[33m3\x1b[0m Memory \x1b[33m4\x1b[0m Network \x1b[33m5\x1b[0m Disk \x1b[33m6\x1b[0m Process \x1b[90m│");
    println!("\x1b[90m│ \x1b[33m7\x1b[0m System \x1b[33m8\x1b[0m Services \x1b[33m9\x1b[0m Containers \x1b[33m0\x1b[0m GPU \x1b[90m│");
    println!("\x1b[90m│ Press \x1b[33mCtrl+C\x1b[0m to exit  |  \x1b[33mq\x1b[0m Quit \x1b[90m│\x1b[0m");
    println!("\x1b[90m└────────────────────────────────────────────────────────────────────────────┘\x1b[0m");
    
    io::stdout().flush().unwrap();
}

fn render_full_dashboard(monitor: &mut CliMonitor, width: usize, height: usize) {
    let cpu_info = monitor.get_cpu_info();
    let mem_info = monitor.get_memory_info();
    let swap_info = monitor.get_swap_info();
    let net_info = monitor.get_network_info();
    let disk_info = monitor.get_disk_info();
    let sys_info = monitor.get_system_info();
    let load = monitor.get_load_average();
    let gpu_info = monitor.get_gpu_info();
    
    let bar_w = ((width - 40) / 2).max(20);
    
    println!("\x1b[1m┌─ CPU ──────────────────────────────────────┬─ Memory ────────────────────────────────┐\x1b[0m");
    println!("│ \x1b[33m⚡\x1b[0m Usage: {}{:>6.1}%{}  {} │ \x1b[35m💾\x1b[0m RAM:   {}{:>6.1}%{}  {} │", 
        color_by_load(cpu_info.usage), cpu_info.usage, "\x1b[0m", draw_bar(cpu_info.usage, bar_w / 2),
        color_by_load(mem_info.usage_percent), mem_info.usage_percent, "\x1b[0m", draw_bar(mem_info.usage_percent, bar_w / 2));
    println!("│ \x1b[36mCPU:\x1b[0m {} ({} cores)        │ \x1b[90mTotal: {}  Used: {}  Free: {}\x1b[0m │", 
        cpu_info.model.chars().take(25).collect::<String>(), cpu_info.core_count,
        format_bytes(mem_info.total), format_bytes(mem_info.used), format_bytes(mem_info.free));
    if let Some(temp) = cpu_info.temperature {
        let temp_color = if temp > 80.0 { "\x1b[91m" } else if temp > 60.0 { "\x1b[93m" } else { "\x1b[92m" };
        println!("│ \x1b[90mTemp: {}{:.1}°C\x1b[0m | Freq: {:.0}MHz         │ \x1b[90mCached: {}  Avail: {}\x1b[0m                │", 
            temp_color, temp, cpu_info.frequency.unwrap_or(0.0), format_bytes(mem_info.cached), format_bytes(mem_info.available));
    } else {
        println!("│ \x1b[90mFreq: N/A\x1b[0m                          │ \x1b[90m\x1b[0m                                             │");
    }
    println!("\x1b[1m└─────────────────────────────────────────────┴───────────────────────────────────────┘\x1b[0m");
    
    if swap_info.total > 0 {
        let swap_pct = if swap_info.total > 0 { swap_info.used as f32 / swap_info.total as f32 * 100.0 } else { 0.0 };
        println!("\x1b[1m┌─ Swap ─────────────────────────────────────────────────────────────────────────────┐\x1b[0m");
        println!("│ \x1b[34m↔\x1b[0m Swap: {}{:>6.1}%{}  {} │ Used: {}/{} │", 
            color_by_load(swap_pct), swap_pct, "\x1b[0m", draw_bar(swap_pct, bar_w / 2),
            format_bytes(swap_info.used), format_bytes(swap_info.total));
        println!("\x1b[1m└────────────────────────────────────────────────────────────────────────────────────┘\x1b[0m");
    }
    
    println!("\x1b[1m┌─ Network ─────────────────────────────────┬─ Disk ─────────────────────────────────────┐\x1b[0m");
    println!("│ \x1b[32m↓\x1b[0m RX: {:>10}  \x1b[31m↑\x1b[0m TX: {:>10} │ \x1b[36mRead:\x1b[0m {:>10}  \x1b[33mWrite:\x1b[0m {:>10} │", 
        format_rate(net_info.rx_rate), format_rate(net_info.tx_rate),
        format_rate(disk_info.read_rate), format_rate(disk_info.write_rate));
    println!("│ \x1b[90mInterfaces: {}  Conn: {}\x1b[0m              │ \x1b[90mUsed: {}/{} ({:.1}%)\x1b[0m │", 
        net_info.interface_count, net_info.connection_count,
        format_bytes(disk_info.used), format_bytes(disk_info.total), disk_info.usage_percent);
    println!("\x1b[1m└─────────────────────────────────────────────┴───────────────────────────────────────┘\x1b[0m");
    
    println!("\x1b[1m┌─ System ─────────────────────────────────┬─ GPU ───────────────────────────────────────┐\x1b[0m");
    println!("│ \x1b[35m⎈\x1b[0m Host: {}           \x1b[90m\x1b[0m      │ \x1b[33m🎮\x1b[0m {}                      │", 
        sys_info.hostname.chars().take(20).collect::<String>(),
        gpu_info.name.chars().take(25).collect::<String>());
    println!("│ \x1b[90mUptime: {}  Load: {:.2}\x1b[0m             │ \x1b[90mMemory: {:.1}%  Temp: {:.0}°C\x1b[0m            │", 
        sys_info.uptime, load.one_minute,
        gpu_info.memory_usage.unwrap_or(0.0), gpu_info.temperature.unwrap_or(0.0));
    println!("│ \x1b[90mOS: {}  Kernel: {}\x1b[0m    │ \x1b[90mUtil: {:.1}%  VRAM: {}\x1b[0m                      │", 
        sys_info.os_name.chars().take(15).collect::<String>(), sys_info.kernel_version.chars().take(15).collect::<String>(),
        gpu_info.utilization.unwrap_or(0.0), format_bytes(gpu_info.vram_used.unwrap_or(0)));
    println!("\x1b[1m└─────────────────────────────────────────────┴───────────────────────────────────────┘\x1b[0m");
    
    let processes = monitor.get_processes(ProcessSort::Cpu);
    println!("\x1b[1m┌─ Top Processes (by CPU) ──────────────────────────────────────────────────────────┐\x1b[0m");
    println!("│ {:>6} │ {:<24} │ {:>6} │ {:>8} │ {:>10} │", "PID", "Name", "CPU %", "Memory %", "Memory");
    println!("│────────┼──────────────────────────┼──────────┼───────────┼────────────│");
    for proc in processes.iter().take(5) {
        let name = if proc.name.len() > 24 { format!("{}..", &proc.name[..22]) } else { proc.name.clone() };
        let mem_color = color_by_load(proc.memory_percent);
        let cpu_color = color_by_load(proc.cpu_usage);
        println!("│ {:>6} │ \x1b[36m{:<24}\x1b[0m │ {} {:>5.1}% │ {} {:>6.1}% │ {:>10} │", 
            proc.pid, name, cpu_color, proc.cpu_usage, mem_color, proc.memory_percent, format_bytes(proc.memory));
    }
    println!("\x1b[1m└────────────────────────────────────────────────────────────────────────────────────┘\x1b[0m");
}

fn render_cpu_view(monitor: &mut CliMonitor, width: usize) {
    let cpu_info = monitor.get_cpu_info();
    let per_core = monitor.get_per_core_usage();
    let freq = monitor.get_cpu_frequency();
    let temp = monitor.get_cpu_temperature();
    
    let bar_w = ((width - 40) / 2).max(25);
    
    println!("\x1b[1m┌─ CPU Overview ─────────────────────────────────────────────────────────────────────┐\x1b[0m");
    println!("│ \x1b[33mModel:\x1b[0m {}                                                     │", 
        cpu_info.model.chars().take(70).collect::<String>());
    println!("│ \x1b[33mCores:\x1b[0m {} Physical, {} Logical  |  \x1b[33mArchitecture:\x1b[0m {}              │", 
        cpu_info.physical_cores, cpu_info.core_count, cpu_info.arch);
    println!("│ \x1b[33mUsage:\x1b[0m {}{:>6.1}%{}  {}  |  \x1b[33mUser:\x1b[0m {:.1}%  \x1b[33mSystem:\x1b[0m {:.1}%  \x1b[33mIdle:\x1b[0m {:.1}% │", 
        color_by_load(cpu_info.usage), cpu_info.usage, "\x1b[0m", draw_bar(cpu_info.usage, bar_w / 2),
        cpu_info.user_usage, cpu_info.system_usage, cpu_info.idle_usage);
    println!("\x1b[1m└────────────────────────────────────────────────────────────────────────────────────┘\x1b[0m");
    
    println!("\x1b[1m┌─ Per-Core Usage ─────────────────────────────────────────────────────────────────────┐\x1b[0m");
    let cols = 2;
    let rows = (per_core.len() + cols - 1) / cols;
    for row in 0..rows {
        for col in 0..cols {
            let idx = row + col * rows;
            if idx < per_core.len() {
                let (name, usage) = &per_core[idx];
                let bar = draw_bar(*usage, bar_w / 2);
                print!("│ \x1b[36m{:>4}:\x1b[0m {}{:>5.1}%{} {} ", name, color_by_load(*usage), usage, "\x1b[0m", bar);
            }
        }
        println!("│");
    }
    println!("\x1b[1m└────────────────────────────────────────────────────────────────────────────────────┘\x1b[0m");
    
    println!("\x1b[1m┌─ CPU Frequency ─────────────────────────────┬─ CPU Temperature ──────────────────────┐\x1b[0m");
    if let Some(min_freq) = freq.min_freq {
        println!("│ \x1b[33mMin:\x1b[0m {:.0} MHz  \x1b[33mMax:\x1b[0m {:.0} MHz  \x1b[33mAvg:\x1b[0m {:.0} MHz │", min_freq, freq.max_freq.unwrap_or(0.0), freq.current_freq.unwrap_or(0.0));
    } else {
        println!("│ \x1b[90mFrequency info not available\x1b[0m                        │");
    }
    
    if let Some(t) = temp.cpu_temp {
        let temp_color = if t > 80.0 { "\x1b[91m" } else if t > 60.0 { "\x1b[93m" } else { "\x1b[92m" };
        println!("│ \x1b[33mCore:\x1b[0m {}{:.1}°C\x1b[0m                                  │ \x1b[33mMax:\x1b[0m {}{:.1}°C\x1b[0m                      │", 
            temp_color, t, temp_color, temp.max_temp.unwrap_or(0.0));
    } else {
        println!("│ \x1b[90mTemperature info not available\x1b[0m                │ \x1b[90m\x1b[0m                                        │");
    }
    
    if !temp.zone_temps.is_empty() {
        print!("│ \x1b[33mZones:\x1b[0m ");
        for (zone, t) in temp.zone_temps.iter().take(4) {
            print!("{}: {:.1}°C ", zone, t);
        }
        println!("  │");
    }
    println!("\x1b[1m└─────────────────────────────────────────────┴───────────────────────────────────────┘\x1b[0m");
}

fn render_memory_view(monitor: &mut CliMonitor, width: usize) {
    let mem_info = monitor.get_memory_info();
    let swap_info = monitor.get_swap_info();
    
    let bar_w = ((width - 40) / 2).max(30);
    
    println!("\x1b[1m┌─ Memory Overview ────────────────────────────────────────────────────────────────────┐\x1b[0m");
    println!("│ \x1b[35mTotal\x1b[0m: {}    \x1b[92mUsed\x1b[0m: {}    \x1b[94mFree\x1b[0m: {}    \x1b[33mAvailable\x1b[0m: {}        │", 
        format_bytes(mem_info.total), format_bytes(mem_info.used), format_bytes(mem_info.free), format_bytes(mem_info.available));
    println!("│ \x1b[33mUsage:\x1b[0m {}{:>6.1}%{}  {}                                              │", 
        color_by_load(mem_info.usage_percent), mem_info.usage_percent, "\x1b[0m", draw_bar(mem_info.usage_percent, width - 50));
    println!("\x1b[1m└────────────────────────────────────────────────────────────────────────────────────┘\x1b[0m");
    
    println!("\x1b[1m┌─ Memory Details ────────────────────────────────────────────────────────────────────┐\x1b[0m");
    println!("│ \x1b[90mBuffers:\x1b[0m {:>10}  \x1b[90mCached:\x1b[0m {:>10}  \x1b[90mActive:\x1b[0m {:>10}  \x1b[90mInactive:\x1b[0m {:>10} │", 
        format_bytes(mem_info.buffers), format_bytes(mem_info.cached), format_bytes(mem_info.active), format_bytes(mem_info.inactive));
    println!("│ \x1b[90mSReclaimable:\x1b[0m {:>10}  \x1b[90mShmem:\x1b[0m {:>10}  \x1b[90mDirty:\x1b[0m {:>10}  \x1b[90mWriteback:\x1b[0m {:>10} │", 
        format_bytes(mem_info.s_reclaimable), format_bytes(mem_info.shmem), format_bytes(mem_info.dirty), format_bytes(mem_info.writeback));
    println!("\x1b[1m└────────────────────────────────────────────────────────────────────────────────────┘\x1b[0m");
    
    let swap_bar = draw_bar(if swap_info.total > 0 { swap_info.used as f32 / swap_info.total as f32 * 100.0 } else { 0.0 }, width - 50);
    println!("\x1b[1m┌─ Swap Space ─────────────────────────────────────────────────────────────────────────┐\x1b[0m");
    if swap_info.total > 0 {
        let swap_pct = swap_info.used as f32 / swap_info.total as f32 * 100.0;
        println!("│ \x1b[33mTotal\x1b[0m: {}    \x1b[92mUsed\x1b[0m: {}    \x1b[94mFree\x1b[0m: {}                                    │", 
            format_bytes(swap_info.total), format_bytes(swap_info.used), format_bytes(swap_info.free));
        println!("│ \x1b[33mUsage:\x1b[0m {}{:>6.1}%{}  {}                                            │", 
            color_by_load(swap_pct), swap_pct, "\x1b[0m", swap_bar);
    } else {
        println!("│ \x1b[90mNo swap space configured\x1b[0m                                                       │");
    }
    println!("\x1b[1m└────────────────────────────────────────────────────────────────────────────────────┘\x1b[0m");
}

fn render_network_view(monitor: &mut CliMonitor, width: usize) {
    let net_info = monitor.get_network_info();
    
    let bar_w = ((width - 40) / 2).max(30);
    
    println!("\x1b[1m┌─ Network Overview ───────────────────────────────────────────────────────────────────┐\x1b[0m");
    println!("│ \x1b[32m↓ RX\x1b[0m: {:>12}/s  \x1b[31m↑ TX\x1b[0m: {:>12}/s  \x1b[90mInterfaces: {}\x1b[0m                            │", 
        format_rate(net_info.rx_rate), format_rate(net_info.tx_rate), net_info.interface_count);
    println!("│ \x1b[33mTotal RX:\x1b[0m {:>12}  \x1b[33mTotal TX:\x1b[0m {:>12}  \x1b[90mConnections: {}\x1b[0m                     │", 
        format_bytes(net_info.total_rx), format_bytes(net_info.total_tx), net_info.connection_count);
    println!("\x1b[1m└────────────────────────────────────────────────────────────────────────────────────┘\x1b[0m");
    
    println!("\x1b[1m┌─ Network Interfaces ─────────────────────────────────────────────────────────────────┐\x1b[0m");
    println!("│ {:<12} │ {:>12} │ {:>12} │ {:>12} │ {:>12} │", "Interface", "RX Rate", "TX Rate", "Total RX", "Total TX");
    println!("│────────────┼──────────────┼──────────────┼──────────────┼──────────────│");
    for iface in net_info.interfaces.iter().take(8) {
        let rx_rate = format_rate(iface.rx_rate);
        let tx_rate = format_rate(iface.tx_rate);
        let total_rx = format_bytes(iface.total_rx);
        let total_tx = format_bytes(iface.total_tx);
        println!("│ \x1b[36m{:<12}\x1b[0m │ {:>12} │ {:>12} │ {:>12} │ {:>12} │", 
            iface.name.chars().take(12).collect::<String>(), rx_rate, tx_rate, total_rx, total_tx);
    }
    println!("\x1b[1m└────────────────────────────────────────────────────────────────────────────────────┘\x1b[0m");
    
    println!("\x1b[1m┌─ Connection Summary ─────────────────────────────────────────────────────────────────┐\x1b[0m");
    println!("│ \x1b[33mTCP:\x1b[0m {}  \x1b[33mUDP:\x1b[0m {}  \x1b[33mLISTEN:\x1b[0m {}  \x1b[33mESTABLISHED:\x1b[0m {}  \x1b[33mTIME_WAIT:\x1b[0m {}  \x1b[33mCLOSED:\x1b[0m {} │", 
        net_info.tcp_count, net_info.udp_count, net_info.listen_count, net_info.established_count, 
        net_info.time_wait_count, net_info.closed_count);
    println!("\x1b[1m└────────────────────────────────────────────────────────────────────────────────────┘\x1b[0m");
}

fn render_disk_view(monitor: &mut CliMonitor, width: usize) {
    let disk_info = monitor.get_disk_info();
    
    let bar_w = ((width - 40) / 2).max(30);
    
    println!("\x1b[1m┌─ Disk Overview ─────────────────────────────────────────────────────────────────────┐\x1b[0m");
    println!("│ \x1b[32m↓ Read\x1b[0m: {:>12}/s  \x1b[31m↑ Write\x1b[0m: {:>12}/s                                     │", 
        format_rate(disk_info.read_rate), format_rate(disk_info.write_rate));
    println!("│ \x1b[33mRead Ops:\x1b[0m {:>8}/s  \x1b[33mWrite Ops:\x1b[0m {:>8}/s  \x1b[90mUtilization: {:.1}%\x1b[0m                      │", 
        disk_info.read_ops_sec, disk_info.write_ops_sec, disk_info.utilization);
    println!("\x1b[1m└────────────────────────────────────────────────────────────────────────────────────┘\x1b[0m");
    
    println!("\x1b[1m┌─ Disk Usage ─────────────────────────────────────────────────────────────────────────┐\x1b[0m");
    println!("│ {:<40} │ {:>10} │ {:>10} │ {:>8} │ {:>12} │", "Mount", "Total", "Used", "Use %", "Available");
    println!("│────────────────────────────────────────┼────────────┼────────────┼─────────┼──────────────│");
    for partition in disk_info.partitions.iter().take(6) {
        let mount = partition.mount_point.chars().take(40).collect::<String>();
        let usage_pct = if partition.total > 0 { partition.used as f32 / partition.total as f32 * 100.0 } else { 0.0 };
        let color = color_by_load(usage_pct);
        println!("│ \x1b[36m{:<40}\x1b[0m │ {:>10} │ {:>10} │ {} {:>5.1}% │ {:>12} │", 
            mount, format_bytes(partition.total), format_bytes(partition.used), color, usage_pct, format_bytes(partition.available));
    }
    println!("\x1b[1m└────────────────────────────────────────────────────────────────────────────────────┘\x1b[0m");
    
    println!("\x1b[1m┌─ Inodes ─────────────────────────────────────────────────────────────────────────────┐\x1b[0m");
    println!("│ \x1b[90mTotal Inodes: {}  |  Used Inodes: {}  |  Available Inodes: {}\x1b[0m              │", 
        disk_info.total_inodes, disk_info.used_inodes, disk_info.available_inodes);
    println!("\x1b[1m└────────────────────────────────────────────────────────────────────────────────────┘\x1b[0m");
}

fn render_process_view(monitor: &mut CliMonitor, width: usize, height: usize) {
    let processes = monitor.get_processes(ProcessSort::Cpu);
    
    let rows = (height - 15).min(20).max(10);
    
    println!("\x1b[1m┌─ Top Processes (by CPU) ─────────────────────────────────────────────────────────────┐\x1b[0m");
    println!("│ {:>6} │ {:<24} │ {:>8} │ {:>10} │ {:>8} │ {:>10} │", "PID", "Name", "CPU %", "Memory", "Mem %", "Status");
    println!("│────────┼───────────────────────────┼──────────┼────────────┼─────────┼────────────│");
    
    for proc in processes.iter().take(rows) {
        let name = if proc.name.len() > 24 { format!("{}..", &proc.name[..22]) } else { proc.name.clone() };
        let cpu_color = color_by_load(proc.cpu_usage);
        let mem_color = color_by_load(proc.memory_percent);
        println!("│ {:>6} │ \x1b[36m{:<24}\x1b[0m │ {} {:>5.1}% │ {:>10} │ {} {:>5.1}% │ {:>10} │", 
            proc.pid, name, cpu_color, proc.cpu_usage, format_bytes(proc.memory), mem_color, proc.memory_percent, proc.status);
    }
    println!("\x1b[1m└────────────────────────────────────────────────────────────────────────────────────┘\x1b[0m");
    
    println!("\x1b[1m┌─ Process Tree (partial) ────────────────────────────────────────────────────────────┐\x1b[0m");
    let tree = monitor.get_process_tree();
    for node in tree.iter().take(rows) {
        let indent = "  ".repeat(node.depth.min(5));
        let name = if node.name.len() > 40 { format!("{}..", &node.name[..38]) } else { node.name.clone() };
        println!("│ \x1b[90m{}{}─ {} ({}) [{}% CPU]\x1b[0m", indent, "├", name, node.pid, node.cpu_usage as u32);
    }
    println!("\x1b[1m└────────────────────────────────────────────────────────────────────────────────────┘\x1b[0m");
}

fn render_system_view(monitor: &mut CliMonitor, width: usize) {
    let sys_info = monitor.get_system_info();
    let load = monitor.get_load_average();
    let users = monitor.get_users();
    
    println!("\x1b[1m┌─ System Information ─────────────────────────────────────────────────────────────────┐\x1b[0m");
    println!("│ \x1b[33mHostname:\x1b[0m {}                                                             │", sys_info.hostname);
    println!("│ \x1b[33mOS:\x1b[0m {}                                                                │", sys_info.os_name);
    println!("│ \x1b[33mKernel:\x1b[0m {}                                                           │", sys_info.kernel_version);
    println!("│ \x1b[33mArchitecture:\x1b[0m {}                                                          │", sys_info.arch);
    println!("│ \x1b[33mUptime:\x1b[0m {}                                                              │", sys_info.uptime);
    println!("\x1b[1m└────────────────────────────────────────────────────────────────────────────────────┘\x1b[0m");
    
    println!("\x1b[1m┌─ Load Average ─────────────────────────────────────────────────────────────────────┐\x1b[0m");
    println!("│ \x1b[33m1 min:\x1b[0m {:.2}  \x1b[33m5 min:\x1b[0m {:.2}  \x1b[33m15 min:\x1b[0m {:.2}  \x1b[90mRunning: {}  |  Total: {}\x1b[0m                    │", 
        load.one_minute, load.five_minute, load.fifteen_minute, load.running_procs, load.total_procs);
    println!("\x1b[1m└────────────────────────────────────────────────────────────────────────────────────┘\x1b[0m");
    
    println!("\x1b[1m┌─ Logged In Users ───────────────────────────────────────────────────────────────────┐\x1b[0m");
    if users.is_empty() {
        println!("│ \x1b[90mNo users logged in\x1b[0m                                                                │");
    } else {
        for user in users.iter().take(5) {
            println!("│ \x1b[36m{:<20}\x1b[0m from {}  |  Session: {}  |  Since: {}                        │", 
                user.username, user.host, user.tty, user.login_time);
        }
    }
    println!("\x1b[1m└────────────────────────────────────────────────────────────────────────────────────┘\x1b[0m");
}

fn render_services_view(monitor: &mut CliMonitor, width: usize) {
    let services = monitor.get_services();
    
    println!("\x1b[1m┌─ Systemd Services ───────────────────────────────────────────────────────────────────┐\x1b[0m");
    println!("│ \x1b[92mRunning:\x1b[0m {}  \x1b[33mFailed:\x1b[0m {}  \x1b[90mInactive:\x1b[0m {}  \x1b[90mTotal:\x1b[0m {}                                │", 
        services.running_count, services.failed_count, services.inactive_count, services.total_count);
    println!("\x1b[1m└────────────────────────────────────────────────────────────────────────────────────┘\x1b[0m");
    
    if !services.running_services.is_empty() {
        println!("\x1b[1m┌─ Running Services ─────────────────────────────────────────────────────────────────┐\x1b[0m");
        for svc in services.running_services.iter().take(10) {
            println!("│ \x1b[92m●\x1b[0m {}                                                             │", svc);
        }
        println!("\x1b[1m└────────────────────────────────────────────────────────────────────────────────────┘\x1b[0m");
    }
    
    if !services.failed_services.is_empty() {
        println!("\x1b[1m┌─ Failed Services ──────────────────────────────────────────────────────────────────┐\x1b[0m");
        for svc in services.failed_services.iter().take(10) {
            println!("│ \x1b[91m✗\x1b[0m {}                                                             │", svc);
        }
        println!("\x1b[1m└────────────────────────────────────────────────────────────────────────────────────┘\x1b[0m");
    }
}

fn render_containers_view(monitor: &mut CliMonitor, width: usize) {
    let containers = monitor.get_containers();
    
    println!("\x1b[1m┌─ Containers ─────────────────────────────────────────────────────────────────────────┐\x1b[0m");
    println!("│ \x1b[33mDocker:\x1b[0m {}  \x1b[90mRunning: {}\x1b[0m  \x1b[33mLXD:\x1b[0m {}  \x1b[90mRunning: {}\x1b[0m                                    │", 
        if containers.docker_available { "Available" } else { "Not found" }, containers.docker_running,
        if containers.lxd_available { "Available" } else { "Not found" }, containers.lxd_running);
    println!("\x1b[1m└────────────────────────────────────────────────────────────────────────────────────┘\x1b[0m");
    
    if !containers.docker_containers.is_empty() {
        println!("\x1b[1m┌─ Docker Containers ────────────────────────────────────────────────────────────────┐\x1b[0m");
        println!("│ {:<20} │ {:>12} │ {:>10} │ {:>10} │", "Name", "Status", "CPU %", "Memory");
        println!("│─────────────────────┼──────────────┼────────────┼────────────│");
        for c in containers.docker_containers.iter().take(8) {
            println!("│ \x1b[36m{:<20}\x1b[0m │ {:>12} │ {:>10.1} │ {:>10} │", 
                c.name.chars().take(20).collect::<String>(), c.status, c.cpu_percent, format_bytes(c.memory));
        }
        println!("\x1b[1m└────────────────────────────────────────────────────────────────────────────────────┘\x1b[0m");
    }
    
    if !containers.lxd_instances.is_empty() {
        println!("\x1b[1m┌─ LXD Instances ───────────────────────────────────────────────────────────────────┐\x1b[0m");
        for inst in containers.lxd_instances.iter().take(6) {
            println!("│ \x1b[36m●\x1b[0m {} ({}) - {}                                              │", inst.name, inst.status, inst.addresses.join(", "));
        }
        println!("\x1b[1m└────────────────────────────────────────────────────────────────────────────────────┘\x1b[0m");
    }
}

fn render_gpu_view(monitor: &mut CliMonitor, width: usize) {
    let gpu = monitor.get_gpu_info();
    
    println!("\x1b[1m┌─ GPU Information ─────────────────────────────────────────────────────────────────────┐\x1b[0m");
    if gpu.name != "N/A" {
        println!("│ \x1b[33mName:\x1b[0m {}                                                               │", gpu.name);
        if let Some(mem) = gpu.memory_total {
            println!("│ \x1b[33mMemory:\x1b[0m Total: {}  Used: {}  Free: {}                                     │", 
                format_bytes(mem), format_bytes(gpu.vram_used.unwrap_or(0)), format_bytes(gpu.vram_free.unwrap_or(mem)));
        }
        if let Some(util) = gpu.utilization {
            println!("│ \x1b[33mUtilization:\x1b[0m GPU: {:.1}%  |  Memory: {:.1}%                                   │", 
                util, gpu.memory_usage.unwrap_or(0.0));
        }
        if let Some(temp) = gpu.temperature {
            let temp_color = if temp > 80.0 { "\x1b[91m" } else if temp > 60.0 { "\x1b[93m" } else { "\x1b[92m" };
            println!("│ \x1b[33mTemperature:\x1b[0m {}{:.0}°C\x1b[0m  |  \x1b[33mPower:\x1b[0m {:.1}W / {:.1}W                                │", 
                temp_color, temp, gpu.power_draw.unwrap_or(0.0), gpu.power_limit.unwrap_or(0.0));
        }
        if let Some(freq) = gpu.clock_speed {
            println!("│ \x1b[33mClock Speed:\x1b[0m {} MHz  |  \x1b[33mMax:\x1b[0m {} MHz                                     │", 
                freq, gpu.max_clock_speed.unwrap_or(0.0));
        }
        if !gpu.fan_speeds.is_empty() {
            println!("│ \x1b[33mFan Speed:\x1b[0m {}%                                                               │", 
                gpu.fan_speeds[0]);
        }
        if let Some(pcie) = gpu.pcie_utilization {
            println!("│ \x1b[33mPCIe:\x1b[0m {:.1}%  |  \x1b[33mBandwidth:\x1b[0m Gen {}x{}                                   │", 
                pcie, gpu.pcie_gen.unwrap_or(4), gpu.pcie_width.unwrap_or(16));
        }
    } else {
        println!("│ \x1b[90mNo GPU detected or nvidia-smi/rocm-smi not available\x1b[0m                             │");
    }
    println!("\x1b[1m└────────────────────────────────────────────────────────────────────────────────────┘\x1b[0m");
}

fn main() {
    setup_signal_handler();
    
    let mut monitor = CliMonitor::new();
    monitor.refresh();
    thread::sleep(Duration::from_millis(200));
    
    println!("\x1b[2J\x1b[H");
    println!("\x1b[1;32m╔═══════════════════════════════════════════════════════════════╗\x1b[0m");
    println!("\x1b[1;32m║\x1b[0m       \x1b[1;33mLinux System Monitor CLI v5.0.0\x1b[0m                        \x1b[1;32m║\x1b[0m");
    println!("\x1b[1;32m║\x1b[0m       \x1b[36m100 Terminal-Optimized Features\x1b[0m                         \x1b[1;32m║\x1b[0m");
    println!("\x1b[1;32m╚═══════════════════════════════════════════════════════════════╝\x1b[0m");
    println!();
    println!("\x1b[90mCollecting system information...\x1b[0m");
    
    thread::sleep(Duration::from_millis(500));
    
    let mut current_view = String::from("full");
    
    println!("\x1b[2J\x1b[H");
    println!("\x1b[32mStarting monitoring... Use number keys 1-0 to switch views.\x1b[0m\n");
    
    while RUNNING.load(Ordering::SeqCst) {
        let width = get_terminal_width();
        let height = get_terminal_height();
        render_dashboard(&mut monitor, width, height, &current_view);
        
        use std::io::Read;
        let mut buffer = [0u8; 1];
        if std::io::stdin().read(&mut buffer).is_ok() {
            let c = buffer[0] as char;
            match c {
                '1' => current_view = "full".to_string(),
                '2' => current_view = "cpu".to_string(),
                '3' => current_view = "memory".to_string(),
                '4' => current_view = "network".to_string(),
                '5' => current_view = "disk".to_string(),
                '6' => current_view = "process".to_string(),
                '7' => current_view = "system".to_string(),
                '8' => current_view = "services".to_string(),
                '9' => current_view = "containers".to_string(),
                '0' => current_view = "gpu".to_string(),
                'q' | 'Q' => break,
                _ => {}
            }
        }
        
        thread::sleep(Duration::from_secs(1));
    }
    
    println!("\x1b[2J\x1b[H");
    println!("\x1b[32m╔═══════════════════════════════════════╗\x1b[0m");
    println!("\x1b[32m║\x1b[0m   Thank you for using Linux System Monitor!   \x1b[32m║\x1b[0m");
    println!("\x1b[32m╚═══════════════════════════════════════╝\x1b[0m\n");
}
