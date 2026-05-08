use serde::{Deserialize, Serialize};
use sysinfo::{System, Disks, CpuRefreshKind, MemoryRefreshKind, RefreshKind, Networks};
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    pub timestamp: i64,
    pub uptime: i64,
    pub hostname: String,
    pub os_version: String,
    pub kernel: String,
    pub cpu: CpuMetrics,
    pub memory: MemoryMetrics,
    pub swap: SwapMetrics,
    pub disk: DiskMetrics,
    pub disks: Vec<DiskInfo>,
    pub network: NetworkMetrics,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processes: Option<Vec<ProcessInfo>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub battery: Option<BatteryInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuMetrics {
    pub name: String,
    pub usage: f32,
    pub core_count: usize,
    pub per_core: Vec<f32>,
    pub frequencies: Vec<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryMetrics {
    pub total: u64,
    pub used: u64,
    pub available: u64,
    pub usage_percent: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwapMetrics {
    pub total: u64,
    pub used: u64,
    pub usage_percent: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskMetrics {
    pub read_rate: u64,
    pub write_rate: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskInfo {
    pub name: String,
    pub mount_point: String,
    pub total: u64,
    pub used: u64,
    pub available: u64,
    pub usage_percent: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkMetrics {
    pub rx_bytes: u64,
    pub tx_bytes: u64,
    pub rx_rate: u64,
    pub tx_rate: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub cpu: f32,
    pub memory: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatteryInfo {
    pub name: String,
    pub charge_percent: f32,
    pub is_charging: bool,
    pub time_remaining: i32,
}

pub struct MetricsCollector {
    system: System,
    disks: Disks,
    networks: Networks,
    last_disk_stats: (u64, u64),
    last_network_stats: (u64, u64),
    boot_time: i64,
}

impl MetricsCollector {
    pub fn new() -> Self {
        let system = System::new_with_specifics(
            RefreshKind::new()
                .with_cpu(CpuRefreshKind::everything())
                .with_memory(MemoryRefreshKind::everything())
        );
        let disks = Disks::new_with_refreshed_list();
        let networks = Networks::new_with_refreshed_list();
        let boot_time = chrono::Utc::now().timestamp() - System::boot_time() as i64;
        
        Self {
            system,
            disks,
            networks,
            last_disk_stats: (0, 0),
            last_network_stats: (0, 0),
            boot_time,
        }
    }

    pub fn collect(&mut self) -> SystemMetrics {
        self.system.refresh_cpu_specifics(CpuRefreshKind::everything());
        self.system.refresh_memory();
        self.disks.refresh();
        self.networks.refresh();
        
        let timestamp = chrono::Utc::now().timestamp();
        let uptime = timestamp - self.boot_time;
        
        let hostname = System::host_name().unwrap_or_else(|| "Unknown".to_string());
        let os_version = System::os_version().unwrap_or_else(|| "Unknown".to_string());
        let kernel = System::kernel_version().unwrap_or_else(|| "Unknown".to_string());
        
        let cpu = self.collect_cpu();
        let memory = self.collect_memory();
        let swap = self.collect_swap();
        let disk = self.collect_disk_io();
        let disks = self.collect_disk_info();
        let network = self.collect_network();
        let network_details = self.collect_network_details();
        let processes = self.collect_processes();
        let battery = self.collect_battery();
        
        SystemMetrics {
            timestamp,
            uptime,
            hostname,
            os_version,
            kernel,
            cpu,
            memory,
            swap,
            disk,
            disks,
            network,
            network_details,
            processes,
            battery,
        }
    }

    fn collect_cpu(&self) -> CpuMetrics {
        let cpus = self.system.cpus();
        let per_core: Vec<f32> = cpus.iter().map(|c| c.cpu_usage()).collect();
        let usage = if per_core.is_empty() {
            0.0
        } else {
            per_core.iter().sum::<f32>() / per_core.len() as f32
        };
        
        let frequencies: Vec<u64> = cpus.iter().map(|c| c.frequency()).collect();
        let cpu_name = cpus.first()
            .map(|c| c.brand().to_string())
            .unwrap_or_else(|| "Unknown CPU".to_string());
        
        CpuMetrics {
            name: cpu_name,
            usage,
            core_count: cpus.len(),
            per_core,
            frequencies,
        }
    }

    fn collect_memory(&self) -> MemoryMetrics {
        let total = self.system.total_memory();
        let used = self.system.used_memory();
        let available = self.system.available_memory();
        let usage_percent = if total > 0 {
            (used as f32 / total as f32) * 100.0
        } else {
            0.0
        };
        
        MemoryMetrics {
            total,
            used,
            available,
            usage_percent,
        }
    }

    fn collect_swap(&self) -> SwapMetrics {
        let total = self.system.total_swap();
        let used = self.system.used_swap();
        let usage_percent = if total > 0 {
            (used as f32 / total as f32) * 100.0
        } else {
            0.0
        };
        
        SwapMetrics {
            total,
            used,
            usage_percent,
        }
    }

    fn collect_disk_io(&mut self) -> DiskMetrics {
        let mut total_read: u64 = 0;
        let mut total_write: u64 = 0;
        
        if let Ok(content) = fs::read_to_string("/proc/diskstats") {
            for line in content.lines() {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 14 {
                    if let (Ok(sectors_read), Ok(sectors_written)) = (
                        parts[5].parse::<u64>(),
                        parts[9].parse::<u64>()
                    ) {
                        total_read += sectors_read * 512;
                        total_write += sectors_written * 512;
                    }
                }
            }
        }
        
        let read_rate = total_read.saturating_sub(self.last_disk_stats.0);
        let write_rate = total_write.saturating_sub(self.last_disk_stats.1);
        
        self.last_disk_stats = (total_read, total_write);
        
        DiskMetrics {
            read_rate,
            write_rate,
        }
    }

    fn collect_disk_info(&self) -> Vec<DiskInfo> {
        self.disks.iter()
            .map(|disk| {
                let total = disk.total_space();
                let available = disk.available_space();
                let used = total.saturating_sub(available);
                let usage_percent = if total > 0 {
                    (used as f32 / total as f32) * 100.0
                } else {
                    0.0
                };
                
                DiskInfo {
                    name: disk.name().to_string_lossy().to_string(),
                    mount_point: disk.mount_point().to_string_lossy().to_string(),
                    total,
                    used,
                    available,
                    usage_percent,
                }
            })
            .collect()
    }

    fn collect_network(&mut self) -> NetworkMetrics {
        let mut total_rx: u64 = 0;
        let mut total_tx: u64 = 0;
        
        for (_name, data) in self.networks.iter() {
            total_rx += data.received();
            total_tx += data.transmitted();
        }
        
        let rx_rate = total_rx.saturating_sub(self.last_network_stats.0);
        let tx_rate = total_tx.saturating_sub(self.last_network_stats.1);
        
        self.last_network_stats = (total_rx, total_tx);
        
        NetworkMetrics {
            rx_bytes: total_rx,
            tx_bytes: total_tx,
            rx_rate,
            tx_rate,
        }
    }

    fn collect_processes(&self) -> Option<Vec<ProcessInfo>> {
        let total_memory = self.system.total_memory() as f32;
        
        if total_memory == 0.0 {
            return None;
        }
        
        let mut processes: Vec<ProcessInfo> = self.system.processes()
            .values()
            .map(|p| {
                let memory = p.memory() as f32;
                ProcessInfo {
                    pid: p.pid().as_u32(),
                    name: p.name().to_string(),
                    cpu: p.cpu_usage(),
                    memory: (memory / total_memory) * 100.0,
                }
            })
            .filter(|p| p.cpu > 0.1 || p.memory > 0.1)
            .collect();
        
        processes.sort_by(|a, b| {
            let cpu_cmp = b.cpu.partial_cmp(&a.cpu).unwrap();
            if cpu_cmp != std::cmp::Ordering::Equal {
                cpu_cmp
            } else {
                b.memory.partial_cmp(&a.memory).unwrap()
            }
        });
        
        processes.truncate(15);
        Some(processes)
    }

    fn collect_battery(&self) -> Option<BatteryInfo> {
        if let Ok(content) = fs::read_to_string("/sys/class/power_supply/BAT0/status") {
            let is_charging = content.trim() == "Charging";
            
            if let Ok(charge_str) = fs::read_to_string("/sys/class/power_supply/BAT0/capacity") {
                if let Ok(charge) = charge_str.trim().parse::<f32>() {
                    let time_remaining = if is_charging { 0 } else { -1 };
                    
                    return Some(BatteryInfo {
                        name: "BAT0".to_string(),
                        charge_percent: charge,
                        is_charging,
                        time_remaining,
                    });
                }
            }
        }
        None
    }
}
