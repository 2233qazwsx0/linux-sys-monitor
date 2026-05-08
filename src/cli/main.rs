use std::io::{self, Write};
use std::thread;
use std::time::Duration;
use std::sync::atomic::{AtomicBool, Ordering};
use sysinfo::{System, Disks, Networks, CpuRefreshKind, MemoryRefreshKind, RefreshKind};
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

struct Monitor {
    system: System,
    disks: Disks,
    networks: Networks,
    last_net_rx: u64,
    last_net_tx: u64,
    last_disk_read: u64,
    last_disk_write: u64,
}

impl Monitor {
    fn new() -> Self {
        let system = System::new_with_specifics(
            RefreshKind::new()
                .with_cpu(CpuRefreshKind::everything())
                .with_memory(MemoryRefreshKind::everything())
        );
        Self {
            system,
            disks: Disks::new_with_refreshed_list(),
            networks: Networks::new_with_refreshed_list(),
            last_net_rx: 0,
            last_net_tx: 0,
            last_disk_read: 0,
            last_disk_write: 0,
        }
    }

    fn refresh(&mut self) {
        self.system.refresh_cpu_specifics(CpuRefreshKind::everything());
        self.system.refresh_memory();
        self.disks.refresh();
        self.networks.refresh();
    }

    fn get_cpu_usage(&self) -> (f32, Vec<f32>) {
        let cpus = self.system.cpus();
        let per_core: Vec<f32> = cpus.iter().map(|c| c.cpu_usage()).collect();
        let avg = if per_core.is_empty() {
            0.0
        } else {
            per_core.iter().sum::<f32>() / per_core.len() as f32
        };
        (avg, per_core)
    }

    fn get_memory(&self) -> (u64, u64, u64) {
        let total = self.system.total_memory();
        let used = self.system.used_memory();
        let available = self.system.available_memory();
        (total, used, available)
    }

    fn get_swap(&self) -> (u64, u64) {
        (self.system.total_swap(), self.system.used_swap())
    }

    fn get_network_rate(&mut self) -> (u64, u64) {
        let mut total_rx: u64 = 0;
        let mut total_tx: u64 = 0;
        for (_, data) in self.networks.iter() {
            total_rx += data.received();
            total_tx += data.transmitted();
        }
        let rx_rate = total_rx.saturating_sub(self.last_net_rx);
        let tx_rate = total_tx.saturating_sub(self.last_net_tx);
        self.last_net_rx = total_rx;
        self.last_net_tx = total_tx;
        (rx_rate, tx_rate)
    }

    fn get_disk_io(&mut self) -> (u64, u64) {
        let mut read_total: u64 = 0;
        let mut write_total: u64 = 0;
        if let Ok(content) = fs::read_to_string("/proc/diskstats") {
            for line in content.lines() {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 14 {
                    if let (Ok(sectors_read), Ok(sectors_written)) = (
                        parts[5].parse::<u64>(),
                        parts[9].parse::<u64>()
                    ) {
                        read_total += sectors_read * 512;
                        write_total += sectors_written * 512;
                    }
                }
            }
        }
        let read_rate = read_total.saturating_sub(self.last_disk_read);
        let write_rate = write_total.saturating_sub(self.last_disk_write);
        self.last_disk_read = read_total;
        self.last_disk_write = write_total;
        (read_rate, write_rate)
    }

    fn get_disks(&self) -> Vec<(String, String, u64, u64)> {
        self.disks.iter().map(|d| {
            let total = d.total_space();
            let avail = d.available_space();
            let used = total.saturating_sub(avail);
            (
                d.name().to_string_lossy().into_owned(),
                d.mount_point().to_string_lossy().into_owned(),
                used,
                total,
            )
        }).collect()
    }

    fn get_top_processes(&self, count: usize) -> Vec<(String, u32, f32, f32)> {
        let total_mem = self.system.total_memory() as f32;
        let mut processes: Vec<_> = self.system.processes().iter()
            .map(|(pid, p)| {
                (
                    p.name().to_string(),
                    pid.as_u32(),
                    p.cpu_usage(),
                    if total_mem > 0.0 { p.memory() as f32 / total_mem * 100.0 } else { 0.0 },
                )
            })
            .collect();
        processes.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());
        processes.into_iter().take(count).collect()
    }

    fn get_hostname(&self) -> String {
        System::host_name().unwrap_or_else(|| "unknown".to_string())
    }

    fn get_uptime(&self) -> i64 {
        (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)
            .unwrap().as_secs() as i64) - (sysinfo::System::boot_time() as i64)
    }
}

fn setup_signal_handler() {
    ctrlc::set_handler(|| {
        RUNNING.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");
}

fn get_terminal_width() -> usize {
    terminal_size::terminal_size().map(|(w, _)| w.0 as usize).unwrap_or(80).max(60)
}

fn render(monitor: &mut Monitor, width: usize) {
    monitor.refresh();
    
    let (cpu_avg, cpu_cores) = monitor.get_cpu_usage();
    let (mem_total, mem_used, _mem_avail) = monitor.get_memory();
    let (swap_total, swap_used) = monitor.get_swap();
    let (net_rx, net_tx) = monitor.get_network_rate();
    let (disk_read, disk_write) = monitor.get_disk_io();
    let disks = monitor.get_disks();
    let processes = monitor.get_top_processes(8);
    let hostname = monitor.get_hostname();
    let uptime = monitor.get_uptime();
    
    let mem_pct = if mem_total > 0 { mem_used as f32 / mem_total as f32 * 100.0 } else { 0.0 };
    let swap_pct = if swap_total > 0 { swap_used as f32 / swap_total as f32 * 100.0 } else { 0.0 };
    
    print!("\x1b[2J\x1b[H");
    
    let separator = "─".repeat(width);
    let double_sep = "═".repeat(width);
    
    println!("\x1b[1;36m╔{}╗\x1b[0m", double_sep);
    println!("\x1b[1;36m║\x1b[0m \x1b[1;33m◆ Linux System Monitor\x1b[0m  \x1b[32m{}\x1b[0m  \x1b[35mUptime: {}\x1b[0m \x1b[1;36m║", hostname, format_uptime(uptime));
    println!("\x1b[1;36m╚{}╝\x1b[0m", double_sep);
    
    let bar_w = (width - 40) / 3;
    let cpu_bar = draw_bar(cpu_avg, bar_w);
    let mem_bar = draw_bar(mem_pct, bar_w);
    let swap_bar = draw_bar(swap_pct, bar_w);
    
    println!("\x1b[1m┌─ System ────────────────────────────────────────────────┐\x1b[0m");
    println!("│ \x1b[33m⚡ CPU\x1b[0m  │ Usage: \x1b[32m{:>5.1}%  \x1b[0m│ {} │ \x1b[36m{:>3} cores\x1b[0m │", 
        cpu_avg, cpu_bar, cpu_cores.len());
    
    let core_display: String = cpu_cores.iter().take(12).map(|c| {
        let pct = *c as i32;
        if pct >= 80 { "\x1b[31m█".to_string() }
        else if pct >= 50 { "\x1b[33m▓".to_string() }
        else if pct >= 20 { "\x1b[32m▒".to_string() }
        else { "\x1b[90m░".to_string() }
    }).collect::<String>() + "\x1b[0m";
    println!("│         │ Cores: {} │", core_display);
    
    println!("│ \x1b[35m💾 RAM\x1b[0m │ Usage: \x1b[32m{:>5.1}%  \x1b[0m│ {} │ {}/{} │", 
        mem_pct, mem_bar, format_bytes(mem_used), format_bytes(mem_total));
    println!("│ \x1b[34m↔ Swap\x1b[0m │ Usage: \x1b[32m{:>5.1}%  \x1b[0m│ {} │ {}/{} │", 
        swap_pct, swap_bar, format_bytes(swap_used), format_bytes(swap_total));
    println!("\x1b[1m└───────────────────────────────────────────────────────────┘\x1b[0m");
    
    let io_bar_w = (width - 55) / 2;
    let max_io_rate: f32 = 1024.0 * 1024.0 * 100.0;
    let read_bar = draw_bar((disk_read as f32 / max_io_rate).min(100.0), io_bar_w);
    let write_bar = draw_bar((disk_write as f32 / max_io_rate).min(100.0), io_bar_w);
    
    println!("\x1b[1m┌─ Disk I/O ───────────────────────────────────────────────┐\x1b[0m");
    println!("│ \x1b[32m↓ Read\x1b[0m:  \x1b[32m{:>8}\x1b[0m/s  │ {} │", format_rate(disk_read), read_bar);
    println!("│ \x1b[31m↑ Write\x1b[0m: \x1b[32m{:>8}\x1b[0m/s  │ {} │", format_rate(disk_write), write_bar);
    println!("\x1b[1m└───────────────────────────────────────────────────────────┘\x1b[0m");
    
    if !disks.is_empty() {
        println!("\x1b[1m┌─ Disks ───────────────────────────────────────────────────┐\x1b[0m");
        for (name, mount, used, total) in disks.iter().take(4) {
            let pct = if *total > 0 { *used as f32 / *total as f32 * 100.0 } else { 0.0 };
            let bar = draw_bar(pct, bar_w);
            let color = if pct > 90.0 { "\x1b[31m" } else if pct > 70.0 { "\x1b[33m" } else { "\x1b[32m" };
            println!("│ {} {:<8} {:>12} {} │ {} {:>5.1}% │", 
                color, name, mount, "\x1b[0m", bar, pct);
        }
        println!("\x1b[1m└───────────────────────────────────────────────────────────┘\x1b[0m");
    }
    
    let net_bar_w = (width - 50) / 2;
    let max_net_rate: f32 = 1024.0 * 1024.0 * 100.0;
    let rx_bar = draw_bar((net_rx as f32 / max_net_rate).min(100.0), net_bar_w);
    let tx_bar = draw_bar((net_tx as f32 / max_net_rate).min(100.0), net_bar_w);
    
    println!("\x1b[1m┌─ Network ─────────────────────────────────────────────────┐\x1b[0m");
    println!("│ \x1b[32m↓ Rx\x1b[0m: \x1b[32m{:>8}\x1b[0m/s  │ {} │", format_rate(net_rx), rx_bar);
    println!("│ \x1b[31m↑ Tx\x1b[0m: \x1b[32m{:>8}\x1b[0m/s  │ {} │", format_rate(net_tx), tx_bar);
    println!("\x1b[1m└───────────────────────────────────────────────────────────┘\x1b[0m");
    
    println!("\x1b[1m┌─ Top Processes ───────────────────────────────────────────┐\x1b[0m");
    println!("│ {:>6} │ {:<20} │ {:>8} │ {:>8} │", "PID", "Name", "CPU %", "MEM %");
    println!("│────────┼─────────────────────┼──────────┼──────────│");
    for (name, pid, cpu, mem) in processes.iter() {
        let name = if name.len() > 20 { format!("{}..", &name[..18]) } else { name.clone() };
        let cpu_color = if *cpu > 80.0 { "\x1b[31m" } else if *cpu > 50.0 { "\x1b[33m" } else { "\x1b[0m" };
        println!("│ {:>6} │ \x1b[36m{:<20}\x1b[0m │ {} {:>5.1}% │ {:>5.1}% │", 
            pid, name, cpu_color, cpu, mem);
    }
    println!("\x1b[1m└───────────────────────────────────────────────────────────┘\x1b[0m");
    
    println!("\x1b[90m┌─ {} ────────────────────────────────────────────────────┐\x1b[0m", separator);
    println!("\x1b[90m│ Press \x1b[33mCtrl+C\x1b[0m to exit                                          │\x1b[0m");
    println!("\x1b[90m└───────────────────────────────────────────────────────────┘\x1b[0m");
    
    io::stdout().flush().unwrap();
}

fn main() {
    setup_signal_handler();
    
    let mut monitor = Monitor::new();
    monitor.refresh();
    thread::sleep(Duration::from_millis(200));
    
    println!("\x1b[2J\x1b[H");
    println!("\x1b[1;32mStarting Linux System Monitor CLI...\x1b[0m");
    println!("\x1b[90mCollecting initial data...\x1b[0m");
    thread::sleep(Duration::from_millis(500));
    
    while RUNNING.load(Ordering::SeqCst) {
        let width = get_terminal_width();
        render(&mut monitor, width);
        thread::sleep(Duration::from_secs(1));
    }
    
    println!("\x1b[2J\x1b[H");
    println!("\x1b[32mExiting... Thank you for using Linux System Monitor!\x1b[0m\n");
}
