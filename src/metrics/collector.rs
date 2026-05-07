use serde::{Deserialize, Serialize};
use sysinfo::{System, Disks, CpuRefreshKind, MemoryRefreshKind, RefreshKind};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    pub timestamp: i64,
    pub cpu: CpuMetrics,
    pub memory: MemoryMetrics,
    pub disk: DiskMetrics,
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

pub struct MetricsCollector {
    system: System,
    disks: Disks,
    last_disk_stats: (u64, u64),
    last_time: std::time::Instant,
}

impl MetricsCollector {
    pub fn new() -> Self {
        let system = System::new_with_specifics(
            RefreshKind::new()
                .with_cpu(CpuRefreshKind::everything())
                .with_memory(MemoryRefreshKind::everything())
        );
        let disks = Disks::new_with_refreshed_list();
        Self {
            system,
            disks,
            last_disk_stats: (0, 0),
            last_time: std::time::Instant::now(),
        }
    }

    pub fn collect(&mut self) -> SystemMetrics {
        self.system.refresh_cpu_all();
        self.system.refresh_memory();
        self.disks.refresh();
        
        let timestamp = chrono::Utc::now().timestamp();
        
        let cpu = self.collect_cpu();
        let memory = self.collect_memory();
        let disk = self.collect_disk();
        
        SystemMetrics { timestamp, cpu, memory, disk }
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
        
        let elapsed = self.last_time.elapsed().as_secs_f64().max(1.0);
        let read_rate = ((total_read.saturating_sub(self.last_disk_stats.0)) as f64 / elapsed) as u64;
        let write_rate = ((total_write.saturating_sub(self.last_disk_stats.1)) as f64 / elapsed) as u64;
        
        self.last_disk_stats = (total_read, total_write);
        self.last_time = std::time::Instant::now();
        
        DiskMetrics {
            read_bytes: total_read,
            write_bytes: total_write,
            read_rate,
            write_rate,
        }
    }
}
