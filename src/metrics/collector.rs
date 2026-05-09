use serde::{Deserialize, Serialize};
use sysinfo::{System, Disks, CpuRefreshKind, MemoryRefreshKind, RefreshKind, Networks};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    pub timestamp: i64,
    pub uptime: i64,
    pub hostname: String,
    pub os_version: String,
    pub kernel: String,
    pub platform: String,
    pub cpu: CpuMetrics,
    pub memory: MemoryMetrics,
    pub swap: SwapMetrics,
    pub disk: DiskMetrics,
    pub disks: Vec<DiskInfo>,
    pub network: NetworkMetrics,
    pub network_details: NetworkDetails,
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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NetworkDetails {
    pub interfaces: Vec<NetworkInterface>,
    pub tcp_states: TcpStates,
    pub udp_endpoints: UdpEndpoints,
    pub listening_ports: Vec<ListeningPort>,
    pub established_connections: Vec<ConnectionInfo>,
    pub bandwidth_total: BandwidthTotal,
    pub packet_counts: PacketCounts,
    pub error_counts: ErrorCounts,
    pub interface_duplex: HashMap<String, DuplexInfo>,
    pub wireless_info: Vec<WirelessInfo>,
    pub cellular_info: Vec<CellularInfo>,
    pub dns_stats: Option<DnsStats>,
    pub routing_table: Vec<RouteEntry>,
    pub arp_table: Vec<ArpEntry>,
    pub namespaces: Vec<NetworkNamespace>,
    pub socket_stats: SocketStats,
    pub connection_limits: ConnectionLimits,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkInterface {
    pub name: String,
    pub ipv4: Vec<String>,
    pub ipv6: Vec<String>,
    pub mtu: u32,
    pub flags: Vec<String>,
    pub mac: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TcpStates {
    pub established: u32,
    pub syn_sent: u32,
    pub syn_recv: u32,
    pub fin_wait1: u32,
    pub fin_wait2: u32,
    pub time_wait: u32,
    pub close: u32,
    pub close_wait: u32,
    pub last_ack: u32,
    pub listen: u32,
    pub closing: u32,
    pub total: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UdpEndpoints {
    pub total: u32,
    pub local_endpoints: Vec<UdpEndpoint>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UdpEndpoint {
    pub local_addr: String,
    pub inode: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListeningPort {
    pub port: u16,
    pub protocol: String,
    pub process_name: Option<String>,
    pub pid: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionInfo {
    pub protocol: String,
    pub local_addr: String,
    pub remote_addr: String,
    pub state: String,
    pub pid: Option<u32>,
    pub process_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BandwidthTotal {
    pub rx_bytes: u64,
    pub tx_bytes: u64,
    pub rx_packets: u64,
    pub tx_packets: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PacketCounts {
    pub rx_packets: u64,
    pub tx_packets: u64,
    pub rx_errors: u64,
    pub tx_errors: u64,
    pub rx_dropped: u64,
    pub tx_dropped: u64,
    pub multicast: u64,
    pub collisions: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ErrorCounts {
    pub rx_errors: u64,
    pub tx_errors: u64,
    pub rx_dropped: u64,
    pub tx_dropped: u64,
    pub rx_fifo_errors: u64,
    pub tx_fifo_errors: u64,
    pub rx_frame_errors: u64,
    pub rx_length_errors: u64,
    pub rx_crc_errors: u64,
    pub tx_aborted_errors: u64,
    pub tx_carrier_errors: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplexInfo {
    pub duplex: String,
    pub speed: u32,
    pub autoneg: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WirelessInfo {
    pub interface: String,
    pub ssid: Option<String>,
    pub signal_dbm: i32,
    pub signal_quality: u8,
    pub bitrate: f64,
    pub channel: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CellularInfo {
    pub interface: String,
    pub operator: Option<String>,
    pub technology: Option<String>,
    pub signal_strength: Option<i32>,
    pub mobile_ip: Option<String>,
    pub roaming: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DnsStats {
    pub queries_total: u64,
    pub queries_successful: u64,
    pub queries_failed: u64,
    pub cache_hits: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteEntry {
    pub destination: String,
    pub gateway: String,
    pub genmask: String,
    pub flags: String,
    pub metric: u32,
    pub interface: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArpEntry {
    pub ip_address: String,
    pub hw_address: String,
    pub flags: String,
    pub device: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkNamespace {
    pub name: String,
    pub interfaces: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SocketStats {
    pub used: u32,
    pub tcp_alloc: u32,
    pub tcp_orphan: u32,
    pub tcp_tw: u32,
    pub alloc: u32,
    pub mem: u32,
    pub memory: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConnectionLimits {
    pub max_files: u64,
    pub used_files: u64,
    pub max_sockets: u64,
    pub used_sockets: u64,
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
        
        let platform = Self::detect_platform();
        
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
            platform,
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

    fn detect_platform() -> String {
        if cfg!(target_os = "linux") {
            if cfg!(target_os = "android") {
                "android".to_string()
            } else {
                "linux".to_string()
            }
        } else if cfg!(target_os = "windows") {
            "windows".to_string()
        } else if cfg!(target_os = "macos") {
            "macos".to_string()
        } else {
            "unknown".to_string()
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
        #[cfg(target_os = "linux")]
        {
            use std::fs;
            
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
            
            return DiskMetrics {
                read_rate,
                write_rate,
            };
        }
        
        #[cfg(not(target_os = "linux"))]
        {
            let total = self.system.disks().iter()
                .map(|d| d.total_space())
                .sum::<u64>();
            let available = self.system.disks().iter()
                .map(|d| d.available_space())
                .sum::<u64>();
            let used = total.saturating_sub(available);
            
            self.last_disk_stats.0 += used;
            self.last_disk_stats.1 = 0;
            
            DiskMetrics {
                read_rate: self.last_disk_stats.0.saturating_sub(self.last_disk_stats.1),
                write_rate: 0,
            }
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
        #[cfg(target_os = "linux")]
        {
            use std::fs;
            
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
        }
        
        #[cfg(target_os = "windows")]
        {
            for (name, data) in self.system.batteries() {
                let charge = data.energy() as f32 / data.total_energy() as f32 * 100.0;
                return Some(BatteryInfo {
                    name: name.to_string(),
                    charge_percent: charge,
                    is_charging: data.is_charging(),
                    time_remaining: data.time_to_empty().unwrap_or(-1) as i32,
                });
            }
        }
        
        None
    }

    fn collect_network_details(&self) -> NetworkDetails {
        NetworkDetails {
            interfaces: self.collect_interfaces(),
            tcp_states: self.collect_tcp_states(),
            udp_endpoints: self.collect_udp_endpoints(),
            listening_ports: self.collect_listening_ports(),
            established_connections: self.collect_established_connections(),
            bandwidth_total: self.collect_bandwidth_total(),
            packet_counts: self.collect_packet_counts(),
            error_counts: self.collect_error_counts(),
            interface_duplex: self.collect_interface_duplex(),
            wireless_info: self.collect_wireless_info(),
            cellular_info: self.collect_cellular_info(),
            dns_stats: self.collect_dns_stats(),
            routing_table: self.collect_routing_table(),
            arp_table: self.collect_arp_table(),
            namespaces: self.collect_network_namespaces(),
            socket_stats: self.collect_socket_stats(),
            connection_limits: self.collect_connection_limits(),
        }
    }

    #[cfg(target_os = "linux")]
    fn collect_interfaces(&self) -> Vec<NetworkInterface> {
        use std::fs;
        
        let mut interfaces = Vec::new();
        
        if let Ok(paths) = fs::read_dir("/sys/class/net") {
            for path in paths.flatten() {
                let name = path.file_name().to_string_lossy().to_string();
                if let Some(iface) = self.read_interface_info(&name) {
                    interfaces.push(iface);
                }
            }
        }
        
        interfaces
    }

    #[cfg(not(target_os = "linux"))]
    fn collect_interfaces(&self) -> Vec<NetworkInterface> {
        Vec::new()
    }

    #[cfg(target_os = "linux")]
    fn read_interface_info(&self, name: &str) -> Option<NetworkInterface> {
        use std::fs;
        
        let sys_path = format!("/sys/class/net/{}", name);
        
        let ipv4 = self.read_ipv4_addresses(&sys_path);
        let ipv6 = self.read_ipv6_addresses(&sys_path);
        
        let mtu = fs::read_to_string(format!("{}/mtu", sys_path))
            .ok()
            .and_then(|s| s.trim().parse().ok())
            .unwrap_or(1500);
        
        let flags = self.read_interface_flags(&sys_path);
        
        let mac = fs::read_to_string(format!("{}/address", sys_path))
            .ok()
            .map(|s| s.trim().to_string());
        
        Some(NetworkInterface {
            name: name.to_string(),
            ipv4,
            ipv6,
            mtu,
            flags,
            mac,
        })
    }

    #[cfg(not(target_os = "linux"))]
    fn read_interface_info(&self, _name: &str) -> Option<NetworkInterface> {
        None
    }

    #[cfg(target_os = "linux")]
    fn read_ipv4_addresses(&self, sys_path: &str) -> Vec<String> {
        use std::fs;
        
        let mut addrs = Vec::new();
        
        if let Ok(content) = fs::read_to_string(format!("{}/ipv4/addr", sys_path)) {
            for line in content.lines() {
                if line.starts_with("inet ") {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if !parts.is_empty() {
                        addrs.push(parts[1].to_string());
                    }
                }
            }
        }
        
        addrs
    }

    #[cfg(not(target_os = "linux"))]
    fn read_ipv4_addresses(&self, _sys_path: &str) -> Vec<String> {
        Vec::new()
    }

    #[cfg(target_os = "linux")]
    fn read_ipv6_addresses(&self, sys_path: &str) -> Vec<String> {
        use std::fs;
        
        let mut addrs = Vec::new();
        
        if let Ok(content) = fs::read_to_string(format!("{}/ipv6/addr", sys_path)) {
            for line in content.lines() {
                if !line.starts_with("inet6 ") { continue; }
                let parts: Vec<&str> = line.split_whitespace().collect();
                if !parts.is_empty() {
                    addrs.push(parts[1].to_string());
                }
            }
        }
        
        addrs
    }

    #[cfg(not(target_os = "linux"))]
    fn read_ipv6_addresses(&self, _sys_path: &str) -> Vec<String> {
        Vec::new()
    }

    #[cfg(target_os = "linux")]
    fn read_interface_flags(&self, sys_path: &str) -> Vec<String> {
        use std::fs;
        
        let mut flags_list = Vec::new();
        
        let flags = fs::read_to_string(format!("{}/flags", sys_path))
            .ok()
            .and_then(|s| {
                let s = s.trim().trim_start_matches("0x");
                u32::from_str_radix(s, 16).ok()
            })
            .unwrap_or(0);
        
        if flags & 0x01 != 0 { flags_list.push("UP".to_string()); }
        if flags & 0x02 != 0 { flags_list.push("BROADCAST".to_string()); }
        if flags & 0x08 != 0 { flags_list.push("LOOPBACK".to_string()); }
        if flags & 0x10 != 0 { flags_list.push("POINTTOPOINT".to_string()); }
        if flags & 0x40 != 0 { flags_list.push("RUNNING".to_string()); }
        if flags & 0x80 != 0 { flags_list.push("NOARP".to_string()); }
        if flags & 0x100 != 0 { flags_list.push("PROMISC".to_string()); }
        if flags & 0x800 != 0 { flags_list.push("MULTICAST".to_string()); }
        
        flags_list
    }

    #[cfg(not(target_os = "linux"))]
    fn read_interface_flags(&self, _sys_path: &str) -> Vec<String> {
        Vec::new()
    }

    #[cfg(target_os = "linux")]
    fn collect_tcp_states(&self) -> TcpStates {
        use std::fs;
        
        let mut states = TcpStates::default();
        
        if let Ok(content) = fs::read_to_string("/proc/net/tcp") {
            for line in content.lines().skip(1) {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 4 {
                    if let Ok(state) = u8::from_str_radix(parts[3], 16) {
                        match state {
                            0x01 => states.established += 1,
                            0x02 => states.syn_sent += 1,
                            0x03 => states.syn_recv += 1,
                            0x04 => states.fin_wait1 += 1,
                            0x05 => states.fin_wait2 += 1,
                            0x06 => states.time_wait += 1,
                            0x07 => states.close += 1,
                            0x08 => states.close_wait += 1,
                            0x09 => states.last_ack += 1,
                            0x0A => states.listen += 1,
                            0x0B => states.closing += 1,
                            _ => {}
                        }
                    }
                }
            }
        }
        
        if let Ok(content) = fs::read_to_string("/proc/net/tcp6") {
            for line in content.lines().skip(1) {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 4 {
                    if let Ok(state) = u8::from_str_radix(parts[3], 16) {
                        match state {
                            0x01 => states.established += 1,
                            0x02 => states.syn_sent += 1,
                            0x03 => states.syn_recv += 1,
                            0x04 => states.fin_wait1 += 1,
                            0x05 => states.fin_wait2 += 1,
                            0x06 => states.time_wait += 1,
                            0x07 => states.close += 1,
                            0x08 => states.close_wait += 1,
                            0x09 => states.last_ack += 1,
                            0x0A => states.listen += 1,
                            0x0B => states.closing += 1,
                            _ => {}
                        }
                    }
                }
            }
        }
        
        states.total = states.established + states.syn_sent + states.syn_recv +
                       states.fin_wait1 + states.fin_wait2 + states.time_wait +
                       states.close + states.close_wait + states.last_ack +
                       states.listen + states.closing;
        
        states
    }

    #[cfg(not(target_os = "linux"))]
    fn collect_tcp_states(&self) -> TcpStates {
        TcpStates::default()
    }

    #[cfg(target_os = "linux")]
    fn collect_udp_endpoints(&self) -> UdpEndpoints {
        use std::fs;
        
        let mut endpoints = UdpEndpoints::default();
        let mut local_endpoints: Vec<UdpEndpoint> = Vec::new();
        
        if let Ok(content) = fs::read_to_string("/proc/net/udp") {
            for line in content.lines().skip(1) {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 10 {
                    let local_addr = parts[1].to_string();
                    let inode = parts[9].parse().unwrap_or(0);
                    endpoints.total += 1;
                    local_endpoints.push(UdpEndpoint { local_addr, inode });
                }
            }
        }
        
        if let Ok(content) = fs::read_to_string("/proc/net/udp6") {
            for line in content.lines().skip(1) {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 10 {
                    let local_addr = parts[1].to_string();
                    let inode = parts[9].parse().unwrap_or(0);
                    endpoints.total += 1;
                    local_endpoints.push(UdpEndpoint { local_addr, inode });
                }
            }
        }
        
        endpoints.local_endpoints = local_endpoints;
        endpoints
    }

    #[cfg(not(target_os = "linux"))]
    fn collect_udp_endpoints(&self) -> UdpEndpoints {
        UdpEndpoints::default()
    }

    #[cfg(target_os = "linux")]
    fn collect_listening_ports(&self) -> Vec<ListeningPort> {
        use std::fs;
        
        let mut ports = Vec::new();
        let mut pid_port_map: HashMap<u64, (u16, String)> = HashMap::new();
        
        if let Ok(content) = fs::read_to_string("/proc/net/tcp") {
            for line in content.lines().skip(1) {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 4 {
                    if let Ok(state) = u8::from_str_radix(parts[3], 16) {
                        if state == 0x0A {
                            if let Ok(inode) = parts[9].parse::<u64>() {
                                let local = &parts[1];
                                let port = Self::parse_hex_port(local);
                                let protocol = "tcp4".to_string();
                                pid_port_map.insert(inode, (port, protocol));
                            }
                        }
                    }
                }
            }
        }
        
        if let Ok(content) = fs::read_to_string("/proc/net/udp") {
            for line in content.lines().skip(1) {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 10 {
                    if let Ok(inode) = parts[9].parse::<u64>() {
                        let local = &parts[1];
                        let port = Self::parse_hex_port(local);
                        let protocol = "udp4".to_string();
                        pid_port_map.insert(inode, (port, protocol));
                    }
                }
            }
        }
        
        let process_map = self.get_process_for_inode();
        
        for (inode, (port, protocol)) in pid_port_map {
            let (process_name, pid) = process_map.get(&inode)
                .cloned()
                .unwrap_or((None, None));
            ports.push(ListeningPort {
                port,
                protocol,
                process_name,
                pid,
            });
        }
        
        ports.sort_by(|a, b| a.port.cmp(&b.port));
        ports.dedup_by(|a, b| a.port == b.port && a.protocol == b.protocol);
        ports
    }

    #[cfg(not(target_os = "linux"))]
    fn collect_listening_ports(&self) -> Vec<ListeningPort> {
        Vec::new()
    }

    #[cfg(target_os = "linux")]
    fn parse_hex_port(local: &str) -> u16 {
        if let Some((_, port_str)) = local.rsplit_once(':') {
            u16::from_str_radix(port_str, 16).unwrap_or(0)
        } else {
            0
        }
    }

    #[cfg(not(target_os = "linux"))]
    fn parse_hex_port(_local: &str) -> u16 {
        0
    }

    #[cfg(target_os = "linux")]
    fn collect_established_connections(&self) -> Vec<ConnectionInfo> {
        use std::fs;
        
        let mut connections = Vec::new();
        let process_map = self.get_process_for_inode();
        
        let state_names: HashMap<u8, &'static str> = [
            (0x01, "ESTABLISHED"),
            (0x02, "SYN_SENT"),
            (0x03, "SYN_RECV"),
            (0x04, "FIN_WAIT1"),
            (0x05, "FIN_WAIT2"),
            (0x06, "TIME_WAIT"),
            (0x07, "CLOSE"),
            (0x08, "CLOSE_WAIT"),
            (0x09, "LAST_ACK"),
            (0x0A, "LISTEN"),
            (0x0B, "CLOSING"),
        ].iter().cloned().collect();
        
        if let Ok(content) = fs::read_to_string("/proc/net/tcp") {
            for line in content.lines().skip(1) {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 10 {
                    if let Ok(state) = u8::from_str_radix(parts[3], 16) {
                        let state_name = state_names.get(&state).unwrap_or(&"UNKNOWN");
                        if state == 0x01 {
                            if let Ok(inode) = parts[9].parse::<u64>() {
                                let local = Self::format_socket_address(&parts[1]);
                                let remote = Self::format_socket_address(&parts[2]);
                                let (process_name, pid) = process_map.get(&inode)
                                    .cloned()
                                    .unwrap_or((None, None));
                                connections.push(ConnectionInfo {
                                    protocol: "tcp".to_string(),
                                    local_addr: local,
                                    remote_addr: remote,
                                    state: state_name.to_string(),
                                    pid,
                                    process_name,
                                });
                            }
                        }
                    }
                }
            }
        }
        
        connections
    }

    #[cfg(not(target_os = "linux"))]
    fn collect_established_connections(&self) -> Vec<ConnectionInfo> {
        Vec::new()
    }

    #[cfg(target_os = "linux")]
    fn format_socket_address(hex_addr: &str) -> String {
        let parts: Vec<&str> = hex_addr.split(':').collect();
        if parts.len() < 2 {
            return hex_addr.to_string();
        }
        
        let port = u16::from_str_radix(parts.last().unwrap_or(&"0"), 16).unwrap_or(0);
        
        let addr_hex = parts[0];
        if addr_hex.len() == 8 {
            let ip = format!("{}.{}.{}.{}",
                u8::from_str_radix(&addr_hex[6..8], 16).unwrap_or(0),
                u8::from_str_radix(&addr_hex[4..6], 16).unwrap_or(0),
                u8::from_str_radix(&addr_hex[2..4], 16).unwrap_or(0),
                u8::from_str_radix(&addr_hex[0..2], 16).unwrap_or(0),
            );
            return format!("{}:{}", ip, port);
        }
        
        format!("{}:{}", hex_addr, port)
    }

    #[cfg(not(target_os = "linux"))]
    fn format_socket_address(hex_addr: &str) -> String {
        hex_addr.to_string()
    }

    #[cfg(target_os = "linux")]
    fn get_process_for_inode(&self) -> HashMap<u64, (Option<String>, Option<u32>)> {
        use std::fs;
        
        let mut map = HashMap::new();
        
        if let Ok(fd_dir) = fs::read_dir("/proc") {
            for entry in fd_dir.flatten() {
                let path = entry.path();
                if let Some(name) = path.file_name() {
                    if let Ok(pid) = name.to_string_lossy().parse::<u32>() {
                        let fd_path = path.join("fd");
                        if let Ok(fd_entries) = fs::read_dir(&fd_path) {
                            for fd_entry in fd_entries.flatten() {
                                if let Ok(link) = fs::read_link(fd_entry.path()) {
                                    let link_str = link.to_string_lossy();
                                    if link_str.starts_with("socket:[") {
                                        if let Some(inode_str) = link_str.strip_prefix("socket:[").and_then(|s| s.strip_suffix(']')) {
                                            if let Ok(inode) = inode_str.parse::<u64>() {
                                                let proc_name = self.get_process_name(pid);
                                                map.insert(inode, (Some(proc_name), Some(pid)));
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        
        map
    }

    #[cfg(not(target_os = "linux"))]
    fn get_process_for_inode(&self) -> HashMap<u64, (Option<String>, Option<u32>)> {
        HashMap::new()
    }

    #[cfg(target_os = "linux")]
    fn get_process_name(&self, pid: u32) -> String {
        use std::fs;
        
        fs::read_to_string(format!("/proc/{}/comm", pid))
            .ok()
            .map(|s| s.trim().to_string())
            .unwrap_or_else(|| "unknown".to_string())
    }

    #[cfg(not(target_os = "linux"))]
    fn get_process_name(&self, _pid: u32) -> String {
        "unknown".to_string()
    }

    #[cfg(target_os = "linux")]
    fn collect_bandwidth_total(&self) -> BandwidthTotal {
        use std::fs;
        
        let mut rx_bytes: u64 = 0;
        let mut tx_bytes: u64 = 0;
        let mut rx_packets: u64 = 0;
        let mut tx_packets: u64 = 0;
        
        if let Ok(content) = fs::read_to_string("/proc/net/dev") {
            for line in content.lines().skip(2) {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 10 {
                    rx_bytes += parts[1].parse().unwrap_or(0);
                    tx_bytes += parts[9].parse().unwrap_or(0);
                    rx_packets += parts[2].parse().unwrap_or(0);
                    tx_packets += parts[10].parse().unwrap_or(0);
                }
            }
        }
        
        BandwidthTotal {
            rx_bytes,
            tx_bytes,
            rx_packets,
            tx_packets,
        }
    }

    #[cfg(not(target_os = "linux"))]
    fn collect_bandwidth_total(&self) -> BandwidthTotal {
        BandwidthTotal::default()
    }

    #[cfg(target_os = "linux")]
    fn collect_packet_counts(&self) -> PacketCounts {
        use std::fs;
        
        let mut rx_packets: u64 = 0;
        let mut tx_packets: u64 = 0;
        let mut rx_errors: u64 = 0;
        let mut tx_errors: u64 = 0;
        let mut rx_dropped: u64 = 0;
        let mut tx_dropped: u64 = 0;
        let mut multicast: u64 = 0;
        let mut collisions: u64 = 0;
        
        if let Ok(content) = fs::read_to_string("/proc/net/dev") {
            for line in content.lines().skip(2) {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 16 {
                    rx_packets += parts[2].parse().unwrap_or(0);
                    tx_packets += parts[10].parse().unwrap_or(0);
                    rx_errors += parts[3].parse().unwrap_or(0);
                    tx_errors += parts[11].parse().unwrap_or(0);
                    rx_dropped += parts[4].parse().unwrap_or(0);
                    tx_dropped += parts[12].parse().unwrap_or(0);
                    multicast += parts[8].parse().unwrap_or(0);
                    collisions += parts[14].parse().unwrap_or(0);
                }
            }
        }
        
        PacketCounts {
            rx_packets,
            tx_packets,
            rx_errors,
            tx_errors,
            rx_dropped,
            tx_dropped,
            multicast,
            collisions,
        }
    }

    #[cfg(not(target_os = "linux"))]
    fn collect_packet_counts(&self) -> PacketCounts {
        PacketCounts::default()
    }

    fn collect_error_counts(&self) -> ErrorCounts {
        ErrorCounts::default()
    }

    #[cfg(target_os = "linux")]
    fn collect_interface_duplex(&self) -> HashMap<String, DuplexInfo> {
        use std::fs;
        
        let mut duplex_map = HashMap::new();
        
        if let Ok(paths) = fs::read_dir("/sys/class/net") {
            for path in paths.flatten() {
                let name = path.file_name().to_string_lossy().to_string();
                let dev_path = path.path();
                
                let speed: u32 = fs::read_to_string(dev_path.join("speed"))
                    .ok()
                    .and_then(|s| s.trim().parse().ok())
                    .unwrap_or(0);
                
                let duplex = fs::read_to_string(dev_path.join("duplex"))
                    .ok()
                    .map(|s| s.trim().to_string())
                    .unwrap_or_else(|| "unknown".to_string());
                
                let autoneg = fs::read_to_string(dev_path.join("autoneg"))
                    .ok()
                    .map(|s| s.trim().to_string())
                    .unwrap_or_else(|| "unknown".to_string());
                
                if speed > 0 || duplex != "unknown" {
                    duplex_map.insert(name, DuplexInfo {
                        duplex,
                        speed,
                        autoneg,
                    });
                }
            }
        }
        
        duplex_map
    }

    #[cfg(not(target_os = "linux"))]
    fn collect_interface_duplex(&self) -> HashMap<String, DuplexInfo> {
        HashMap::new()
    }

    fn collect_wireless_info(&self) -> Vec<WirelessInfo> {
        Vec::new()
    }

    fn collect_cellular_info(&self) -> Vec<CellularInfo> {
        Vec::new()
    }

    fn collect_dns_stats(&self) -> Option<DnsStats> {
        None
    }

    #[cfg(target_os = "linux")]
    fn collect_routing_table(&self) -> Vec<RouteEntry> {
        use std::fs;
        
        let mut routes = Vec::new();
        
        if let Ok(content) = fs::read_to_string("/proc/net/route") {
            for line in content.lines().skip(1) {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 10 {
                    routes.push(RouteEntry {
                        destination: parts[0].to_string(),
                        gateway: parts[1].to_string(),
                        genmask: parts[2].to_string(),
                        flags: parts[3].to_string(),
                        metric: parts[6].parse().unwrap_or(0),
                        interface: parts[9].to_string(),
                    });
                }
            }
        }
        
        routes
    }

    #[cfg(not(target_os = "linux"))]
    fn collect_routing_table(&self) -> Vec<RouteEntry> {
        Vec::new()
    }

    #[cfg(target_os = "linux")]
    fn collect_arp_table(&self) -> Vec<ArpEntry> {
        use std::fs;
        
        let mut arp_entries = Vec::new();
        
        if let Ok(content) = fs::read_to_string("/proc/net/arp") {
            for line in content.lines().skip(1) {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 6 {
                    arp_entries.push(ArpEntry {
                        ip_address: parts[0].to_string(),
                        hw_address: parts[3].to_string(),
                        flags: parts[2].to_string(),
                        device: parts[5].to_string(),
                    });
                }
            }
        }
        
        arp_entries
    }

    #[cfg(not(target_os = "linux"))]
    fn collect_arp_table(&self) -> Vec<ArpEntry> {
        Vec::new()
    }

    #[cfg(target_os = "linux")]
    fn collect_network_namespaces(&self) -> Vec<NetworkNamespace> {
        use std::fs;
        
        let mut namespaces = Vec::new();
        
        if let Ok(paths) = fs::read_dir("/var/run/netns") {
            for path in paths.flatten() {
                let name = path.file_name().to_string_lossy().to_string();
                namespaces.push(NetworkNamespace {
                    name,
                    interfaces: Vec::new(),
                });
            }
        }
        
        namespaces
    }

    #[cfg(not(target_os = "linux"))]
    fn collect_network_namespaces(&self) -> Vec<NetworkNamespace> {
        Vec::new()
    }

    #[cfg(target_os = "linux")]
    fn collect_socket_stats(&self) -> SocketStats {
        use std::fs;
        
        let mut stats = SocketStats::default();
        
        if let Ok(content) = fs::read_to_string("/proc/net/sockstat") {
            for line in content.lines() {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if line.starts_with("sockets:") && parts.len() >= 3 {
                    stats.used = parts[2].parse().unwrap_or(0);
                }
                if line.starts_with("TCP:") && parts.len() >= 7 {
                    stats.tcp_alloc = parts[2].parse().unwrap_or(0);
                    stats.tcp_orphan = parts[4].parse().unwrap_or(0);
                    stats.tcp_tw = parts[6].parse().unwrap_or(0);
                }
            }
        }
        
        stats
    }

    #[cfg(not(target_os = "linux"))]
    fn collect_socket_stats(&self) -> SocketStats {
        SocketStats::default()
    }

    #[cfg(target_os = "linux")]
    fn collect_connection_limits(&self) -> ConnectionLimits {
        use std::fs;
        
        let mut limits = ConnectionLimits::default();
        
        limits.max_files = fs::read_to_string("/proc/sys/fs/file-max")
            .ok()
            .and_then(|s| s.trim().parse().ok())
            .unwrap_or(0);
        
        limits.used_files = fs::read_to_string("/proc/sys/fs/file-nr")
            .ok()
            .map(|s| {
                let parts: Vec<&str> = s.split_whitespace().collect();
                parts.first().and_then(|p| p.parse().ok()).unwrap_or(0)
            })
            .unwrap_or(0);
        
        limits.max_sockets = fs::read_to_string("/proc/sys/net/core/somaxconn")
            .ok()
            .and_then(|s| s.trim().parse().ok())
            .unwrap_or(0);
        
        limits
    }

    #[cfg(not(target_os = "linux"))]
    fn collect_connection_limits(&self) -> ConnectionLimits {
        ConnectionLimits::default()
    }

    #[cfg(target_os = "linux")]
    pub fn collect_cpu_governor(&self) -> Vec<CpuGovernor> {
        use std::fs;
        
        let mut governors = Vec::new();
        let cpus = self.system.cpus();
        
        for (i, _) in cpus.iter().enumerate() {
            let governor_path = format!("/sys/devices/system/cpu/cpu{}/cpufreq/scaling_governor", i);
            let governor = fs::read_to_string(&governor_path)
                .map(|s| s.trim().to_string())
                .unwrap_or_else(|_| "unknown".to_string());
            governors.push(CpuGovernor {
                cpu_index: i,
                governor,
            });
        }
        governors
    }

    #[cfg(not(target_os = "linux"))]
    pub fn collect_cpu_governor(&self) -> Vec<CpuGovernor> {
        Vec::new()
    }

    #[cfg(target_os = "linux")]
    pub fn collect_context_switches(&self) -> ContextSwitches {
        use std::fs;
        
        let mut voluntary: u64 = 0;
        let mut involuntary: u64 = 0;
        
        if let Ok(content) = fs::read_to_string("/proc/stat") {
            for line in content.lines() {
                if line.starts_with("ctxt ") {
                    if let Some(val) = line.split_whitespace().nth(1) {
                        voluntary = val.parse().unwrap_or(0);
                    }
                } else if line.starts_with("intr ") {
                    if let Some(val) = line.split_whitespace().nth(1) {
                        involuntary = val.parse().unwrap_or(0);
                    }
                }
            }
        }
        
        ContextSwitches {
            voluntary,
            involuntary,
            total: voluntary + involuntary,
        }
    }

    #[cfg(not(target_os = "linux"))]
    pub fn collect_context_switches(&self) -> ContextSwitches {
        ContextSwitches {
            voluntary: 0,
            involuntary: 0,
            total: 0,
        }
    }

    #[cfg(target_os = "linux")]
    pub fn collect_interrupts(&self) -> Interrupts {
        use std::fs;
        
        let mut total: u64 = 0;
        let mut per_cpu: Vec<u64> = Vec::new();
        let num_cpus = self.system.cpus().len();
        
        if let Ok(content) = fs::read_to_string("/proc/interrupts") {
            for line in content.lines().skip(1) {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() > num_cpus {
                    let mut cpu_sum: u64 = 0;
                    for i in 1..=num_cpus.min(parts.len() - 1) {
                        if let Ok(val) = parts[i].parse::<u64>() {
                            cpu_sum += val;
                        }
                    }
                    total += cpu_sum;
                }
            }
        }
        
        if let Ok(content) = fs::read_to_string("/proc/interrupts") {
            for cpu_idx in 0..num_cpus {
                let mut cpu_total: u64 = 0;
                for line in content.lines().skip(1) {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() > cpu_idx + 1 {
                        if let Ok(val) = parts[cpu_idx + 1].parse::<u64>() {
                            cpu_total += val;
                        }
                    }
                }
                per_cpu.push(cpu_total);
            }
        }
        
        Interrupts { total, per_cpu }
    }

    #[cfg(not(target_os = "linux"))]
    pub fn collect_interrupts(&self) -> Interrupts {
        Interrupts {
            total: 0,
            per_cpu: Vec::new(),
        }
    }

    #[cfg(target_os = "linux")]
    pub fn collect_softirqs(&self) -> Softirqs {
        use std::fs;
        
        let softirq_names = [
            "HI", "TIMER", "NET_TX", "NET_RX", "BLOCK", "IRQ_POLL",
            "TASKLET", "SCHED", "HRTIMER", "RCU"
        ];
        
        let mut total: u64 = 0;
        let mut per_softirq: Vec<SoftirqInfo> = Vec::new();
        
        if let Ok(content) = fs::read_to_string("/proc/softirqs") {
            for line in content.lines().skip(1) {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if !parts.is_empty() {
                    let idx = per_softirq.len();
                    if idx < softirq_names.len() {
                        let mut count: u64 = 0;
                        for p in parts.iter().skip(1) {
                            if let Ok(v) = p.parse::<u64>() {
                                count += v;
                            }
                        }
                        total += count;
                        per_softirq.push(SoftirqInfo {
                            index: idx as u32,
                            name: softirq_names[idx].to_string(),
                            count,
                        });
                    }
                }
            }
        }
        
        Softirqs { total, per_softirq }
    }

    #[cfg(not(target_os = "linux"))]
    pub fn collect_softirqs(&self) -> Softirqs {
        Softirqs {
            total: 0,
            per_softirq: Vec::new(),
        }
    }

    #[cfg(target_os = "linux")]
    pub fn collect_memory_pressure(&self) -> MemoryPressure {
        use std::fs;
        
        if let Ok(content) = fs::read_to_string("/proc/pressure/memory") {
            let parts: Vec<&str> = content.split_whitespace().collect();
            if parts.len() >= 3 {
                let avg10 = parts[1].parse::<f64>().unwrap_or(0.0);
                let level = if avg10 > 60.0 {
                    "critical".to_string()
                } else if avg10 > 30.0 {
                    "medium".to_string()
                } else {
                    "low".to_string()
                };
                return MemoryPressure {
                    level,
                    numeric_value: (avg10 * 100.0) as u32,
                };
            }
        }
        
        MemoryPressure {
            level: "unknown".to_string(),
            numeric_value: 0,
        }
    }

    #[cfg(not(target_os = "linux"))]
    pub fn collect_memory_pressure(&self) -> MemoryPressure {
        MemoryPressure {
            level: "unknown".to_string(),
            numeric_value: 0,
        }
    }

    #[cfg(target_os = "linux")]
    pub fn collect_swap_rate(&mut self) -> SwapRate {
        use std::fs;
        
        let mut swap_in: u64 = 0;
        let mut swap_out: u64 = 0;
        
        if let Ok(content) = fs::read_to_string("/proc/vmstat") {
            for line in content.lines() {
                if line.starts_with("pswpin ") {
                    if let Some(val) = line.split_whitespace().nth(1) {
                        swap_in = val.parse().unwrap_or(0);
                    }
                } else if line.starts_with("pswpout ") {
                    if let Some(val) = line.split_whitespace().nth(1) {
                        swap_out = val.parse().unwrap_or(0);
                    }
                }
            }
        }
        
        SwapRate {
            swap_in_rate: swap_in,
            swap_out_rate: swap_out,
        }
    }

    #[cfg(not(target_os = "linux"))]
    pub fn collect_swap_rate(&mut self) -> SwapRate {
        SwapRate {
            swap_in_rate: 0,
            swap_out_rate: 0,
        }
    }

    pub fn collect_cpu_steal_time(&self) -> CpuStealTime {
        let cpus = self.system.cpus();
        let mut per_core_steal: Vec<f32> = Vec::new();
        let mut total_steal: f32 = 0.0;
        
        for cpu in cpus {
            let steal = cpu.cpu_usage();
            per_core_steal.push(steal);
            total_steal += steal;
        }
        
        total_steal /= cpus.len().max(1) as f32;
        
        CpuStealTime {
            total_steal,
            per_core_steal,
        }
    }

    #[cfg(target_os = "linux")]
    pub fn collect_io_operations(&self) -> IoOperations {
        use std::fs;
        
        let mut reads: u64 = 0;
        let mut writes: u64 = 0;
        let mut read_bytes: u64 = 0;
        let mut write_bytes: u64 = 0;
        let mut per_disk: Vec<DiskIoStats> = Vec::new();
        
        if let Ok(content) = fs::read_to_string("/proc/diskstats") {
            for line in content.lines() {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 14 {
                    let device = parts[2].to_string();
                    if device.starts_with("loop") || device.starts_with("ram") {
                        continue;
                    }
                    
                    if let (Ok(r), Ok(w), Ok(rb), Ok(wb)) = (
                        parts[5].parse::<u64>(),
                        parts[9].parse::<u64>(),
                        parts[6].parse::<u64>(),
                        parts[10].parse::<u64>(),
                    ) {
                        reads += r;
                        writes += w;
                        read_bytes += rb * 512;
                        write_bytes += wb * 512;
                        
                        per_disk.push(DiskIoStats {
                            device,
                            reads: r,
                            writes: w,
                        });
                    }
                }
            }
        }
        
        IoOperations {
            reads,
            writes,
            read_bytes,
            write_bytes,
            per_disk,
        }
    }

    #[cfg(not(target_os = "linux"))]
    pub fn collect_io_operations(&self) -> IoOperations {
        IoOperations {
            reads: 0,
            writes: 0,
            read_bytes: 0,
            write_bytes: 0,
            per_disk: Vec::new(),
        }
    }

    #[cfg(target_os = "linux")]
    pub fn collect_disk_queue_depth(&self) -> Vec<DiskQueueDepth> {
        use std::fs;
        
        let mut queues: Vec<DiskQueueDepth> = Vec::new();
        
        if let Ok(entries) = fs::read_dir("/sys/block") {
            for entry in entries.flatten() {
                let path = entry.path();
                let device_name = path.file_name().unwrap_or_default().to_string_lossy().to_string();
                
                if device_name.starts_with("loop") || device_name.starts_with("ram") {
                    continue;
                }
                
                let queue_path = path.join("queue");
                
                let queue_depth = fs::read_to_string(queue_path.join("nr_requests"))
                    .ok()
                    .and_then(|s| s.trim().parse::<u32>().ok())
                    .unwrap_or(0);
                
                let avg_queue = fs::read_to_string(queue_path.join("avg_queue_size"))
                    .ok()
                    .and_then(|s| s.trim().parse::<f64>().ok())
                    .unwrap_or(0.0);
                
                queues.push(DiskQueueDepth {
                    device: device_name,
                    queue_depth,
                    avg_queue_size: avg_queue / 100.0,
                });
            }
        }
        
        queues
    }

    #[cfg(not(target_os = "linux"))]
    pub fn collect_disk_queue_depth(&self) -> Vec<DiskQueueDepth> {
        Vec::new()
    }

    pub fn collect_filesystem_stats(&self) -> Vec<FilesystemStats> {
        #[cfg(target_os = "linux")]
        {
            use std::process::Command;
            
            let mut stats: Vec<FilesystemStats> = Vec::new();
            
            if let Ok(output) = Command::new("df")
                .args(["-T", "-B1", "-i"])
                .output()
            {
                if let Ok(content) = String::from_utf8(output.stdout) {
                    for line in content.lines().skip(1) {
                        let parts: Vec<&str> = line.split_whitespace().collect();
                        if parts.len() >= 7 {
                            let fs_type = parts[1].to_string();
                            let mount_point = parts[6].to_string();
                            
                            if mount_point.starts_with("/snap") || mount_point == "tmpfs" || mount_point == "devtmpfs" {
                                continue;
                            }
                            
                            let total: u64 = parts[2].parse().unwrap_or(0);
                            let used: u64 = parts[3].parse().unwrap_or(0);
                            let available: u64 = parts[4].parse().unwrap_or(0);
                            
                            let inode_total: u64 = parts[5].parse().unwrap_or(0);
                            let inode_used: u64 = parts[6].parse().unwrap_or(0);
                            let inode_free = inode_total.saturating_sub(inode_used);
                            
                            let usage_percent = if total > 0 {
                                (used as f32 / total as f32) * 100.0
                            } else {
                                0.0
                            };
                            
                            let inode_usage_percent = if inode_total > 0 {
                                (inode_used as f32 / inode_total as f32) * 100.0
                            } else {
                                0.0
                            };
                            
                            stats.push(FilesystemStats {
                                filesystem: fs_type,
                                mount_point,
                                total,
                                used,
                                available,
                                usage_percent,
                                inode_total,
                                inode_used,
                                inode_free,
                                inode_usage_percent,
                            });
                        }
                    }
                }
            }
            
            return stats;
        }
        
        #[cfg(not(target_os = "linux"))]
        {
            Vec::new()
        }
    }

    pub fn collect_inode_usage(&self) -> Vec<InodeUsage> {
        #[cfg(target_os = "linux")]
        {
            use std::process::Command;
            
            let mut usage: Vec<InodeUsage> = Vec::new();
            
            if let Ok(output) = Command::new("df")
                .args(["-i"])
                .output()
            {
                if let Ok(content) = String::from_utf8(output.stdout) {
                    for line in content.lines().skip(1) {
                        let parts: Vec<&str> = line.split_whitespace().collect();
                        if parts.len() >= 6 {
                            let mount_point = parts[5].to_string();
                            
                            if mount_point.starts_with("/snap") || mount_point == "tmpfs" {
                                continue;
                            }
                            
                            let inode_total: u64 = parts[1].parse().unwrap_or(0);
                            let inode_used: u64 = parts[2].parse().unwrap_or(0);
                            let usage_percent = if inode_total > 0 {
                                (inode_used as f32 / inode_total as f32) * 100.0
                            } else {
                                0.0
                            };
                            
                            usage.push(InodeUsage {
                                filesystem: parts[0].to_string(),
                                mount_point,
                                total: inode_total,
                                used: inode_used,
                                usage_percent,
                            });
                        }
                    }
                }
            }
            
            return usage;
        }
        
        #[cfg(not(target_os = "linux"))]
        {
            Vec::new()
        }
    }

    #[cfg(target_os = "linux")]
    pub fn collect_open_files_count(&self) -> OpenFilesCount {
        use std::fs;
        
        let mut fds: u64 = 0;
        let mut sockets: u64 = 0;
        let mut pipes: u64 = 0;
        
        for (pid, _) in self.system.processes() {
            let fd_path = format!("/proc/{}/fd", pid);
            if let Ok(entries) = fs::read_dir(&fd_path) {
                for entry in entries.flatten() {
                    if let Ok(link) = fs::read_link(entry.path()) {
                        let path_str = link.to_string_lossy();
                        if path_str.starts_with("socket:[") {
                            sockets += 1;
                        } else if path_str.starts_with("pipe:[") {
                            pipes += 1;
                        } else {
                            fds += 1;
                        }
                    }
                }
            }
        }
        
        OpenFilesCount {
            total: fds + sockets + pipes,
            file_descriptors: fds,
            sockets,
            pipes,
        }
    }

    #[cfg(not(target_os = "linux"))]
    pub fn collect_open_files_count(&self) -> OpenFilesCount {
        OpenFilesCount {
            total: 0,
            file_descriptors: 0,
            sockets: 0,
            pipes: 0,
        }
    }

    #[cfg(target_os = "linux")]
    pub fn collect_uptime_detailed(&self) -> UptimeDetailed {
        use std::fs;
        
        let mut seconds: u64 = 0;
        let mut idle_seconds: u64 = 0;
        
        if let Ok(content) = fs::read_to_string("/proc/uptime") {
            let parts: Vec<&str> = content.split_whitespace().collect();
            if parts.len() >= 2 {
                seconds = parts[0].parse().unwrap_or(0);
                idle_seconds = parts[1].parse().unwrap_or(0);
            }
        }
        
        let days = seconds / 86400;
        let hours = (seconds % 86400) / 3600;
        let minutes = (seconds % 3600) / 60;
        let secs = seconds % 60;
        
        let formatted = format!("{} days, {}:{:02}:{:02}", days, hours, minutes, secs);
        
        UptimeDetailed {
            seconds,
            days,
            hours,
            minutes,
            formatted,
            idle_seconds,
        }
    }

    #[cfg(not(target_os = "linux"))]
    pub fn collect_uptime_detailed(&self) -> UptimeDetailed {
        UptimeDetailed {
            seconds: 0,
            days: 0,
            hours: 0,
            minutes: 0,
            formatted: "0 days, 0:00:00".to_string(),
            idle_seconds: 0,
        }
    }

    #[cfg(target_os = "linux")]
    pub fn collect_load_normalized(&self) -> LoadNormalized {
        use std::fs;
        
        let mut one_min: f64 = 0.0;
        let mut five_min: f64 = 0.0;
        let mut fifteen_min: f64 = 0.0;
        
        if let Ok(content) = fs::read_to_string("/proc/loadavg") {
            let parts: Vec<&str> = content.split_whitespace().collect();
            if parts.len() >= 3 {
                one_min = parts[0].parse().unwrap_or(0.0);
                five_min = parts[1].parse().unwrap_or(0.0);
                fifteen_min = parts[2].parse().unwrap_or(0.0);
            }
        }
        
        let num_cpus = self.system.cpus().len().max(1) as f64;
        let normalized = vec![
            one_min / num_cpus,
            five_min / num_cpus,
            fifteen_min / num_cpus,
        ];
        
        LoadNormalized {
            one_minute: one_min,
            five_minutes: five_min,
            fifteen_minutes: fifteen_min,
            normalized_to_cores: normalized,
        }
    }

    #[cfg(not(target_os = "linux"))]
    pub fn collect_load_normalized(&self) -> LoadNormalized {
        LoadNormalized {
            one_minute: 0.0,
            five_minutes: 0.0,
            fifteen_minutes: 0.0,
            normalized_to_cores: vec![0.0, 0.0, 0.0],
        }
    }

    #[cfg(target_os = "linux")]
    pub fn collect_per_process_io(&self) -> Vec<PerProcessIo> {
        use std::fs;
        
        let mut io_stats: Vec<PerProcessIo> = Vec::new();
        
        for (pid, process) in self.system.processes() {
            let io_path = format!("/proc/{}/io", pid);
            if let Ok(content) = fs::read_to_string(&io_path) {
                let mut read_bytes: u64 = 0;
                let mut write_bytes: u64 = 0;
                let mut syscr: u64 = 0;
                let mut syscw: u64 = 0;
                
                for line in content.lines() {
                    if line.starts_with("rchar:") {
                        syscr = line.split_whitespace().nth(1).and_then(|s| s.parse().ok()).unwrap_or(0);
                    } else if line.starts_with("wchar:") {
                        syscw = line.split_whitespace().nth(1).and_then(|s| s.parse().ok()).unwrap_or(0);
                    } else if line.starts_with("read_bytes:") {
                        read_bytes = line.split_whitespace().nth(1).and_then(|s| s.parse().ok()).unwrap_or(0);
                    } else if line.starts_with("write_bytes:") {
                        write_bytes = line.split_whitespace().nth(1).and_then(|s| s.parse().ok()).unwrap_or(0);
                    }
                }
                
                if read_bytes > 0 || write_bytes > 0 || syscr > 0 || syscw > 0 {
                    io_stats.push(PerProcessIo {
                        pid: pid.as_u32(),
                        name: process.name().to_string(),
                        read_bytes,
                        write_bytes,
                        syscr,
                        syscw,
                    });
                }
            }
        }
        
        io_stats.sort_by(|a, b| {
            let a_total = a.read_bytes + a.write_bytes;
            let b_total = b.read_bytes + b.write_bytes;
            b_total.cmp(&a_total)
        });
        
        io_stats.truncate(20);
        io_stats
    }

    #[cfg(not(target_os = "linux"))]
    pub fn collect_per_process_io(&self) -> Vec<PerProcessIo> {
        Vec::new()
    }

    #[cfg(target_os = "linux")]
    pub fn collect_memory_zones(&self) -> MemoryZones {
        use std::fs;
        
        let mut zones: Vec<MemoryZoneInfo> = Vec::new();
        
        let zone_names = ["DMA", "DMA32", "Normal", "HighMem", "Movable"];
        
        for zone_name in &zone_names {
            let zone_path = "/proc/zoneinfo";
            if let Ok(content) = fs::read_to_string(zone_path) {
                let mut in_zone = false;
                let mut current_zone = String::new();
                
                for line in content.lines() {
                    if line.starts_with(&format!("Node {}, zone {}", 0, zone_name)) {
                        in_zone = true;
                        current_zone = zone_name.to_string();
                    } else if line.starts_with("Node") && line.contains("zone") {
                        in_zone = false;
                    }
                    
                    if in_zone {
                        if line.starts_with("  pages free") {
                            let free: u64 = line.split_whitespace().nth(3)
                                .and_then(|s| s.parse().ok())
                                .unwrap_or(0) * 4096;
                            
                            zones.push(MemoryZoneInfo {
                                name: current_zone.clone(),
                                total: 0,
                                used: 0,
                                free,
                                present: 0,
                            });
                            in_zone = false;
                        }
                    }
                }
            }
        }
        
        MemoryZones { zones }
    }

    #[cfg(not(target_os = "linux"))]
    pub fn collect_memory_zones(&self) -> MemoryZones {
        MemoryZones { zones: Vec::new() }
    }

    #[cfg(target_os = "linux")]
    pub fn collect_huge_pages(&self) -> HugePages {
        use std::fs;
        
        let mut total: u64 = 0;
        let mut free: u64 = 0;
        let mut surplus: u64 = 0;
        let mut size_kb: u64 = 0;
        
        if let Ok(content) = fs::read_to_string("/proc/meminfo") {
            for line in content.lines() {
                if line.starts_with("HugePages_Total:") {
                    total = line.split_whitespace().nth(1).and_then(|s| s.parse().ok()).unwrap_or(0);
                } else if line.starts_with("HugePages_Free:") {
                    free = line.split_whitespace().nth(1).and_then(|s| s.parse().ok()).unwrap_or(0);
                } else if line.starts_with("Hugepagesurplus:") {
                    surplus = line.split_whitespace().nth(1).and_then(|s| s.parse().ok()).unwrap_or(0);
                } else if line.starts_with("Hugepagesize:") {
                    size_kb = line.split_whitespace().nth(1).and_then(|s| s.parse().ok()).unwrap_or(2048);
                }
            }
        }
        
        HugePages {
            total,
            free,
            surplus,
            default_size: total * size_kb * 1024,
            size_kb,
        }
    }

    #[cfg(not(target_os = "linux"))]
    pub fn collect_huge_pages(&self) -> HugePages {
        HugePages {
            total: 0,
            free: 0,
            surplus: 0,
            default_size: 0,
            size_kb: 0,
        }
    }

    pub fn collect_kernel_threads(&self) -> KernelThreads {
        let mut threads: Vec<KernelThreadInfo> = Vec::new();
        
        for (pid, process) in self.system.processes() {
            let name = process.name().to_string();
            
            #[cfg(target_os = "linux")]
            if name.starts_with('[') {
                threads.push(KernelThreadInfo {
                    pid: pid.as_u32(),
                    name,
                    state: "kernel".to_string(),
                });
            }
            
            #[cfg(not(target_os = "linux"))]
            let _ = (name, pid);
        }
        
        let count = threads.len() as u64;
        KernelThreads { count, threads }
    }

    pub fn collect_user_threads(&self) -> UserThreads {
        #[cfg(target_os = "linux")]
        {
            let count = self.system.processes().values()
                .filter(|p| !p.name().starts_with('['))
                .count() as u64;
            return UserThreads { count };
        }
        
        #[cfg(not(target_os = "linux"))]
        UserThreads { count: 0 }
    }

    pub fn collect_zombie_processes(&self) -> ZombieProcesses {
        let mut zombies: Vec<ZombieProcessInfo> = Vec::new();
        
        for (pid, process) in self.system.processes() {
            if process.status() == sysinfo::ProcessStatus::Zombie {
                let ppid = process.parent().map(|p| p.as_u32()).unwrap_or(0);
                zombies.push(ZombieProcessInfo {
                    pid: pid.as_u32(),
                    name: process.name().to_string(),
                    ppid,
                });
            }
        }
        
        let count = zombies.len() as u64;
        ZombieProcesses { count, zombies }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuGovernor {
    pub cpu_index: usize,
    pub governor: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextSwitches {
    pub voluntary: u64,
    pub involuntary: u64,
    pub total: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Interrupts {
    pub total: u64,
    pub per_cpu: Vec<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoftirqInfo {
    pub index: u32,
    pub name: String,
    pub count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Softirqs {
    pub total: u64,
    pub per_softirq: Vec<SoftirqInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryPressure {
    pub level: String,
    pub numeric_value: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwapRate {
    pub swap_in_rate: u64,
    pub swap_out_rate: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuStealTime {
    pub total_steal: f32,
    pub per_core_steal: Vec<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskIoStats {
    pub device: String,
    pub reads: u64,
    pub writes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IoOperations {
    pub reads: u64,
    pub writes: u64,
    pub read_bytes: u64,
    pub write_bytes: u64,
    pub per_disk: Vec<DiskIoStats>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskQueueDepth {
    pub device: String,
    pub queue_depth: u32,
    pub avg_queue_size: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilesystemStats {
    pub filesystem: String,
    pub mount_point: String,
    pub total: u64,
    pub used: u64,
    pub available: u64,
    pub usage_percent: f32,
    pub inode_total: u64,
    pub inode_used: u64,
    pub inode_free: u64,
    pub inode_usage_percent: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InodeUsage {
    pub filesystem: String,
    pub mount_point: String,
    pub total: u64,
    pub used: u64,
    pub usage_percent: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenFilesCount {
    pub total: u64,
    pub file_descriptors: u64,
    pub sockets: u64,
    pub pipes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UptimeDetailed {
    pub seconds: u64,
    pub days: u64,
    pub hours: u64,
    pub minutes: u64,
    pub formatted: String,
    pub idle_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadNormalized {
    pub one_minute: f64,
    pub five_minutes: f64,
    pub fifteen_minutes: f64,
    pub normalized_to_cores: Vec<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerProcessIo {
    pub pid: u32,
    pub name: String,
    pub read_bytes: u64,
    pub write_bytes: u64,
    pub syscr: u64,
    pub syscw: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryZoneInfo {
    pub name: String,
    pub total: u64,
    pub used: u64,
    pub free: u64,
    pub present: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryZones {
    pub zones: Vec<MemoryZoneInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HugePages {
    pub total: u64,
    pub free: u64,
    pub surplus: u64,
    pub default_size: u64,
    pub size_kb: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KernelThreadInfo {
    pub pid: u32,
    pub name: String,
    pub state: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KernelThreads {
    pub count: u64,
    pub threads: Vec<KernelThreadInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserThreads {
    pub count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZombieProcessInfo {
    pub pid: u32,
    pub name: String,
    pub ppid: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZombieProcesses {
    pub count: u64,
    pub zombies: Vec<ZombieProcessInfo>,
}
