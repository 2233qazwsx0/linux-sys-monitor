use serde::{Deserialize, Serialize};
use sysinfo::{System, Disks, CpuRefreshKind, MemoryRefreshKind, RefreshKind, Networks};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    pub timestamp: i64,
    pub uptime: i64,
    pub cpu: CpuMetrics,
    pub memory: MemoryMetrics,
    pub disk: DiskMetrics,
    pub network: NetworkMetrics,
    pub processes: Vec<ProcessInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuMetrics {
    pub usage: f32,
    pub core_count: usize,
    pub per_core: Vec<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryMetrics {
    pub total: u64,
    pub used: u64,
    pub available: u64,
    pub usage_percent: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskMetrics {
    pub read_bytes: u64,
    pub write_bytes: u64,
    pub read_rate: u64,
    pub write_rate: u64,
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
        
        let cpu = self.collect_cpu();
        let memory = self.collect_memory();
        let disk = self.collect_disk();
        let network = self.collect_network();
        let processes = self.collect_processes();
        
        SystemMetrics {
            timestamp,
            uptime,
            cpu,
            memory,
            disk,
            network,
            processes,
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
        
        CpuMetrics {
            usage,
            core_count: cpus.len(),
            per_core,
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

    fn collect_disk(&mut self) -> DiskMetrics {
        let mut total_read: u64 = 0;
        let mut total_write: u64 = 0;
        
        for disk in self.disks.iter() {
            total_read += disk.total_space();
            total_write += disk.available_space();
        }
        
        let read_rate = total_read.saturating_sub(self.last_disk_stats.0);
        let write_rate = total_write.saturating_sub(self.last_disk_stats.1);
        
        self.last_disk_stats = (total_read, total_write);
        
        DiskMetrics {
            read_bytes: total_read,
            write_bytes: total_write,
            read_rate,
            write_rate,
        }
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

    fn collect_processes(&self) -> Vec<ProcessInfo> {
        let total_memory = self.system.total_memory() as f32;
        
        let mut processes: Vec<ProcessInfo> = self.system.processes()
            .values()
            .map(|p| {
                let memory = p.memory() as f32;
                ProcessInfo {
                    pid: p.pid().as_u32(),
                    name: p.name().to_string(),
                    cpu: p.cpu_usage(),
                    memory: if total_memory > 0.0 { (memory / total_memory) * 100.0 } else { 0.0 },
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
        
        processes.truncate(20);
        processes
    }
}
