use serde::{Deserialize, Serialize};
use sysinfo::{System, Disks, CpuRefreshKind, MemoryRefreshKind, RefreshKind, Networks, ProcessRefreshKind};
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
    #[serde(skip_serializing_if = "Vec::is_empty")]
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
    pub core_count: u8,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub per_core: Vec<f32>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
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
    hostname_buf: String,
    os_version_buf: String,
    kernel_buf: String,
    cpu_name_buf: String,
    disk_line_buf: String,
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
            hostname_buf: String::new(),
            os_version_buf: String::new(),
            kernel_buf: String::new(),
            cpu_name_buf: String::new(),
            disk_line_buf: String::new(),
        }
    }

    pub fn collect(&mut self) -> SystemMetrics {
        self.system.refresh_cpu_specifics(CpuRefreshKind::everything());
        self.system.refresh_memory();
        self.system.refresh_processes_specifics(ProcessRefreshKind::everything());
        self.disks.refresh();
        self.networks.refresh();

        let timestamp = chrono::Utc::now().timestamp();
        let uptime = timestamp - self.boot_time;

        self.hostname_buf.clear();
        self.hostname_buf.push_str(System::host_name().unwrap_or_else(|| "Unknown".into()).as_str());

        self.os_version_buf.clear();
        self.os_version_buf.push_str(System::os_version().unwrap_or_else(|| "Unknown".into()).as_str());

        self.kernel_buf.clear();
        self.kernel_buf.push_str(System::kernel_version().unwrap_or_else(|| "Unknown".into()).as_str());

        let cpu = self.collect_cpu();
        let memory = self.collect_memory();
        let swap = self.collect_swap();
        let disk = self.collect_disk_io();
        let disks = self.collect_disk_info();
        let network = self.collect_network();
        let processes = self.collect_processes();
        let battery = self.collect_battery();

        SystemMetrics {
            timestamp,
            uptime,
            hostname: core::mem::take(&mut self.hostname_buf),
            os_version: core::mem::take(&mut self.os_version_buf),
            kernel: core::mem::take(&mut self.kernel_buf),
            cpu,
            memory,
            swap,
            disk,
            disks,
            network,
            processes,
            battery,
        }
    }

    fn collect_cpu(&mut self) -> CpuMetrics {
        let cpus = self.system.cpus();
        let mut per_core = Vec::with_capacity(cpus.len());
        let mut sum: f32 = 0.0;

        for c in cpus {
            let usage = c.cpu_usage();
            sum += usage;
            per_core.push(usage);
        }

        let usage = if per_core.is_empty() {
            0.0
        } else {
            sum / per_core.len() as f32
        };

        let mut frequencies = Vec::with_capacity(cpus.len());
        for c in cpus {
            frequencies.push(c.frequency());
        }

        self.cpu_name_buf.clear();
        if let Some(c) = cpus.first() {
            self.cpu_name_buf.push_str(c.brand());
        } else {
            self.cpu_name_buf.push_str("Unknown CPU");
        }

        CpuMetrics {
            name: core::mem::take(&mut self.cpu_name_buf),
            usage,
            core_count: cpus.len() as u8,
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
                self.disk_line_buf.clear();
                self.disk_line_buf.push_str(line);
                let mut parts = self.disk_line_buf.split_whitespace();
                let mut sector_count = 0u8;
                let mut sectors_read: u64 = 0;
                let mut sectors_written: u64 = 0;

                for part in parts.by_ref() {
                    match sector_count {
                        5 => { sectors_read = part.parse().unwrap_or(0); }
                        9 => { sectors_written = part.parse().unwrap_or(0); }
                        _ => {}
                    }
                    sector_count += 1;
                    if sector_count >= 14 {
                        break;
                    }
                }

                total_read = total_read.saturating_add(sectors_read.wrapping_mul(512));
                total_write = total_write.saturating_add(sectors_written.wrapping_mul(512));
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
        let disk_count = self.disks.iter().count().min(8);
        let mut disks = Vec::with_capacity(disk_count);

        for disk in self.disks.iter() {
            let total = disk.total_space();
            let available = disk.available_space();
            let used = total.saturating_sub(available);
            let usage_percent = if total > 0 {
                (used as f32 / total as f32) * 100.0
            } else {
                0.0
            };

            disks.push(DiskInfo {
                name: disk.name().to_string_lossy().into_owned(),
                mount_point: disk.mount_point().to_string_lossy().into_owned(),
                total,
                used,
                available,
                usage_percent,
            });
        }
        disks
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

        let mut processes: Vec<ProcessInfo> = Vec::with_capacity(100);

        for p in self.system.processes().values() {
            let mem_pct = (p.memory() as f32 / total_memory) * 100.0;
            if p.cpu_usage() < 0.1 && mem_pct < 0.1 {
                continue;
            }

            processes.push(ProcessInfo {
                pid: p.pid().as_u32(),
                name: p.name().to_string(),
                cpu: p.cpu_usage(),
                memory: mem_pct,
            });
        }

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
