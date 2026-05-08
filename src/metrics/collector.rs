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
    pub disks: Vec<DiskInfo>,
    pub network: NetworkMetrics,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processes: Option<Vec<ProcessInfo>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub battery: Option<BatteryInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<TemperatureMetrics>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_average: Option<LoadAverageMetrics>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_connections: Option<NetworkConnectionsMetrics>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub io_wait: Option<IoWaitMetrics>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_details: Option<MemoryDetailsMetrics>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container: Option<ContainerInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_memory: Option<VirtualMemoryMetrics>,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemperatureMetrics {
    pub cpu_temp: f32,
    pub gpu_temp: Option<f32>,
    pub max_temp: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadAverageMetrics {
    pub load_1: f64,
    pub load_5: f64,
    pub load_15: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConnectionsMetrics {
    pub tcp_count: u32,
    pub udp_count: u32,
    pub tcp_established: u32,
    pub tcp_listening: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IoWaitMetrics {
    pub iowait_percent: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryDetailsMetrics {
    pub buffers: u64,
    pub cached: u64,
    pub swap_cached: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerInfo {
    pub is_container: bool,
    pub container_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualMemoryMetrics {
    pub page_faults: u64,
    pub major_page_faults: u64,
    pub pages_paged_in: u64,
    pub pages_paged_out: u64,
    pub pages_swapped_in: u64,
    pub pages_swapped_out: u64,
}

pub struct MetricsCollector {
    system: System,
    disks: Disks,
    networks: Networks,
    last_disk_stats: (u64, u64),
    last_network_stats: (u64, u64),
    boot_time: i64,
    last_cpu_times: Option<(u64, u64, u64, u64, u64)>,
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
            last_cpu_times: None,
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
        
        let hostname = System::host_name().unwrap_or_else(|| "Unknown".to_string());
        let os_version = System::os_version().unwrap_or_else(|| "Unknown".to_string());
        let kernel = System::kernel_version().unwrap_or_else(|| "Unknown".to_string());
        
        let cpu = self.collect_cpu();
        let memory = self.collect_memory();
        let swap = self.collect_swap();
        let disk = self.collect_disk_io();
        let disks = self.collect_disk_info();
        let network = self.collect_network();
        let processes = self.collect_processes();
        let battery = self.collect_battery();
        let temperature = self.collect_temperature();
        let load_average = self.collect_load_average();
        let network_connections = self.collect_network_connections();
        let io_wait = self.collect_io_wait();
        let memory_details = self.collect_memory_details();
        let container = self.collect_container_info();
        let virtual_memory = self.collect_virtual_memory();
        
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
            processes,
            battery,
            temperature,
            load_average,
            network_connections,
            io_wait,
            memory_details,
            container,
            virtual_memory,
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
    
    pub fn collect_network_security_info(&mut self) -> NetworkSecurityInfo {
        let listening_ports = self.collect_listening_ports();
        let interfaces = self.collect_interface_details();
        let bandwidth_quota = self.collect_bandwidth_quota();
        let ssh_sessions = self.collect_ssh_sessions();
        let dns_servers = self.collect_dns_resolvers();
        let gateway = self.collect_gateway();
        let ssl_certificates = self.collect_ssl_certificates();
        let connection_states = self.collect_connection_states();
        let packet_loss = self.collect_packet_loss();
        let primary_interface = self.detect_primary_interface();
        
        NetworkSecurityInfo {
            listening_ports,
            interfaces,
            bandwidth_quota,
            ssh_sessions,
            dns_servers,
            gateway,
            ssl_certificates,
            connection_states,
            packet_loss,
            primary_interface,
        }
    }
    
    fn collect_listening_ports(&mut self) -> Vec<PortInfo> {
        let mut ports = Vec::new();
        
        if let Ok(content) = fs::read_to_string("/proc/net/tcp") {
            for line in content.lines().skip(1) {
                if let Some(port_info) = self.parse_proc_net_line(line, "tcp") {
                    ports.push(port_info);
                }
            }
        }
        
        if let Ok(content) = fs::read_to_string("/proc/net/tcp6") {
            for line in content.lines().skip(1) {
                if let Some(port_info) = self.parse_proc_net_line(line, "tcp6") {
                    ports.push(port_info);
                }
            }
        }
        
        if let Ok(content) = fs::read_to_string("/proc/net/udp") {
            for line in content.lines().skip(1) {
                if let Some(port_info) = self.parse_proc_net_line(line, "udp") {
                    ports.push(port_info);
                }
            }
        }
        
        if let Ok(content) = fs::read_to_string("/proc/net/udp6") {
            for line in content.lines().skip(1) {
                if let Some(port_info) = self.parse_proc_net_line(line, "udp6") {
                    ports.push(port_info);
                }
            }
        }
        
        ports
    }
    
    fn parse_proc_net_line(&mut self, line: &str, protocol: &str) -> Option<PortInfo> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 10 {
            return None;
        }
        
        let local_addr = parts[1];
        let state_hex = parts[3];
        
        let state = match u8::from_str_radix(state_hex, 16).ok()? {
            0x01 => "ESTABLISHED",
            0x02 => "SYN_SENT",
            0x03 => "SYN_RECV",
            0x04 => "FIN_WAIT1",
            0x05 => "FIN_WAIT2",
            0x06 => "TIME_WAIT",
            0x07 => "CLOSE",
            0x08 => "CLOSE_WAIT",
            0x09 => "LAST_ACK",
            0x0A => "LISTEN",
            0x0B => "CLOSING",
            _ => "UNKNOWN",
        };
        
        if state != "LISTEN" && protocol.starts_with("tcp") {
            return None;
        }
        
        let (ip_addr, port) = self.parse_hex_address(local_addr)?;
        let port_num = u16::from_str_radix(port, 16).ok()?;
        
        let inode = parts[9].parse::<u64>().ok()?;
        let (pid, program) = self.find_process_by_inode(inode);
        
        Some(PortInfo {
            protocol: protocol.replace("6", "").to_uppercase(),
            local_address: ip_addr,
            local_port: port_num,
            state: state.to_string(),
            pid,
            program,
        })
    }
    
    fn parse_hex_address(&self, hex: &str) -> Option<(String, String)> {
        let parts: Vec<&str> = hex.split(':').collect();
        if parts.len() != 2 {
            return None;
        }
        
        let addr_hex = parts[0];
        let port = parts[1].to_string();
        
        let addr_bytes: Vec<u8> = addr_hex
            .chars()
            .collect::<Vec<char>>()
            .chunks(2)
            .map(|c| u8::from_str_radix(&c.iter().collect::<String>(), 16).unwrap_or(0))
            .collect();
        
        let ip_addr = if addr_bytes.len() == 4 {
            format!("{}.{}.{}.{}", addr_bytes[0], addr_bytes[1], addr_bytes[2], addr_bytes[3])
        } else if addr_bytes.len() == 16 {
            let parts: Vec<String> = addr_bytes
                .chunks(2)
                .map(|c| format!("{:02x}{:02x}", c[0], c[1]))
                .collect();
            format!("{}:{}:{}:{}:{}:{}:{}:{}", 
                &parts[0..2].join(""), &parts[2..4].join(""),
                &parts[4..6].join(""), &parts[6..8].join(""),
                &parts[8..10].join(""), &parts[10..12].join(""),
                &parts[12..14].join(""), &parts[14..16].join(""))
        } else {
            return None;
        };
        
        Some((ip_addr, port))
    }
    
    fn find_process_by_inode(&mut self, inode: u64) -> (Option<u32>, Option<String>) {
        let proc_path = "/proc";
        
        if let Ok(entries) = fs::read_dir(proc_path) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    if let Some(name) = path.file_name() {
                        if let Ok(pid) = name.to_string_lossy().parse::<u32>() {
                            let fd_path = path.join("fd");
                            if let Ok(fd_entries) = fs::read_dir(&fd_path) {
                                for fd_entry in fd_entries.flatten() {
                                    if let Ok(link) = fs::read_link(fd_entry.path()) {
                                        let link_str = link.to_string_lossy();
                                        if link_str.contains(&inode.to_string()) {
                                            let program = self.process_cache
                                                .entry(pid)
                                                .or_insert_with(|| {
                                                    fs::read_to_string(path.join("comm"))
                                                        .map(|s| s.trim().to_string())
                                                        .unwrap_or_else(|_| "unknown".to_string())
                                                });
                                            return (Some(pid), Some(program.clone()));
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        
        (None, None)
    }
    
    fn collect_interface_details(&self) -> Vec<InterfaceInfo> {
        let mut interfaces = Vec::new();
        
        if let Ok(content) = fs::read_to_string("/proc/net/dev") {
            for line in content.lines().skip(2) {
                let parts: Vec<&str> = line.split(':').collect();
                if parts.len() == 2 {
                    let name = parts[0].trim().to_string();
                    let stats: Vec<&str> = parts[1].split_whitespace().collect();
                    
                    if stats.len() >= 16 {
                        let rx_bytes = stats[0].parse::<u64>().unwrap_or(0);
                        let rx_packets = stats[1].parse::<u64>().unwrap_or(0);
                        let rx_errors = stats[2].parse::<u64>().unwrap_or(0);
                        let _rx_dropped = stats[3].parse::<u64>().unwrap_or(0);
                        let tx_bytes = stats[8].parse::<u64>().unwrap_or(0);
                        let tx_packets = stats[9].parse::<u64>().unwrap_or(0);
                        let tx_errors = stats[10].parse::<u64>().unwrap_or(0);
                        let _tx_dropped = stats[11].parse::<u64>().unwrap_or(0);
                        
                        let (ip_addr, mac_addr) = self.get_interface_addresses(&name);
                        
                        interfaces.push(InterfaceInfo {
                            name,
                            ip_address: ip_addr,
                            mac_address: mac_addr,
                            rx_bytes,
                            tx_bytes,
                            rx_packets,
                            tx_packets,
                            rx_errors,
                            tx_errors,
                            is_primary: false,
                        });
                    }
                }
            }
        }
        
        interfaces
    }
    
    fn get_interface_addresses(&self, iface: &str) -> (String, String) {
        let mut ip_addr = String::from("N/A");
        let mut mac_addr = String::from("N/A");
        
        if let Ok(content) = fs::read_to_string(format!("/sys/class/net/{}/address", iface)) {
            mac_addr = content.trim().to_string();
        }
        
        if let Ok(output) = std::process::Command::new("ip")
            .args(["-4", "addr", "show", iface])
            .output()
        {
            let output_str = String::from_utf8_lossy(&output.stdout);
            for line in output_str.lines() {
                if line.trim().starts_with("inet ") {
                    let parts: Vec<&str> = line.trim().split_whitespace().collect();
                    if !parts.is_empty() {
                        ip_addr = parts[1].split('/').next().unwrap_or("N/A").to_string();
                    }
                }
            }
        }
        
        (ip_addr, mac_addr)
    }
    
    fn collect_bandwidth_quota(&self) -> BandwidthQuota {
        let mut total_rx: u64 = 0;
        let mut total_tx: u64 = 0;
        
        for (_name, data) in self.networks.iter() {
            total_rx += data.received();
            total_tx += data.transmitted();
        }
        
        BandwidthQuota {
            total_rx,
            total_tx,
            session_start_rx: self.session_start_rx,
            session_start_tx: self.session_start_tx,
        }
    }
    
    fn collect_ssh_sessions(&self) -> Vec<SshSession> {
        let mut sessions = Vec::new();
        
        let proc_path = "/proc";
        if let Ok(entries) = fs::read_dir(proc_path) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    if let Some(name) = path.file_name() {
                        if let Ok(pid) = name.to_string_lossy().parse::<u32>() {
                            if let Ok(cmdline) = fs::read_to_string(path.join("cmdline")) {
                                let cmd = cmdline.replace('\0', " ");
                                if cmd.contains("sshd:") || cmd.contains("sshd") {
                                    if let Ok(status) = fs::read_to_string(path.join("status")) {
                                        let mut user = String::from("unknown");
                                        let mut remote_addr = String::from("N/A");
                                        
                                        for line in status.lines() {
                                            if line.starts_with("Name:") {
                                                let proc_name = line.trim_start_matches("Name:").trim();
                                                if proc_name.contains("sshd") {
                                                    user = format!("sshd-{}", pid);
                                                }
                                            }
                                        }
                                        
                                        let session_type = if cmd.contains("-sftp") || cmd.contains("sftp") {
                                            "SFTP".to_string()
                                        } else {
                                            "SSH".to_string()
                                        };
                                        
                                        sessions.push(SshSession {
                                            pid,
                                            user,
                                            remote_addr,
                                            session_type,
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        
        sessions
    }
    
    fn collect_dns_resolvers(&self) -> DnsResolver {
        let mut nameservers = Vec::new();
        
        if let Ok(content) = fs::read_to_string("/etc/resolv.conf") {
            for line in content.lines() {
                if line.starts_with("nameserver") {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 2 {
                        nameservers.push(parts[1].to_string());
                    }
                }
            }
        }
        
        DnsResolver { nameservers }
    }
    
    fn collect_gateway(&self) -> GatewayInfo {
        let mut gateway = String::from("N/A");
        
        if let Ok(content) = fs::read_to_string("/proc/net/route") {
            for line in content.lines().skip(1) {
                let parts: Vec<&str> = line.split('\t').collect();
                if parts.len() >= 3 {
                    let iface = parts[0];
                    let gateway_hex = parts[2];
                    let destination = parts[1];
                    
                    if destination == "00000000" && !gateway_hex.starts_with("00000000") {
                        let gateway_bytes: Vec<u8> = gateway_hex
                            .chars()
                            .collect::<Vec<char>>()
                            .chunks(2)
                            .map(|c| u8::from_str_radix(&c.iter().collect::<String>(), 16).unwrap_or(0))
                            .collect();
                        
                        if gateway_bytes.len() == 4 {
                            gateway = format!("{}.{}.{}.{}", 
                                gateway_bytes[0], gateway_bytes[1], 
                                gateway_bytes[2], gateway_bytes[3]);
                            break;
                        }
                    }
                }
            }
        }
        
        GatewayInfo { default_gateway: gateway }
    }
    
    fn collect_ssl_certificates(&self) -> Vec<SslCertificate> {
        let mut certs = Vec::new();
        
        let cert_paths = vec![
            "/etc/ssl/certs/ssl-cert-snakeoil.pem",
            "/etc/ssl/certs/ca-certificates.crt",
            "/etc/pki/tls/certs/localhost.crt",
        ];
        
        for cert_path in cert_paths {
            if let Ok(content) = fs::read_to_string(cert_path) {
                if let Some(cert_info) = self.parse_openssl_output(cert_path, &content) {
                    certs.push(cert_info);
                }
            }
        }
        
        if let Ok(output) = std::process::Command::new("find")
            .args(["/etc/ssl", "-name", "*.crt", "-o", "-name", "*.pem"])
            .output()
        {
            let paths = String::from_utf8_lossy(&output.stdout);
            for path in paths.lines().take(5) {
                if let Ok(output) = std::process::Command::new("openssl")
                    .args(["x509", "-in", path, "-noout", "-subject", "-issuer", "-enddate"])
                    .output()
                {
                    let output_str = String::from_utf8_lossy(&output.stdout);
                    let cert = self.parse_ssl_cert(path, &output_str);
                    if cert.is_some() && certs.len() < 5 {
                        certs.push(cert.unwrap());
                    }
                }
            }
        }
        
        certs
    }
    
    fn parse_ssl_cert(&self, path: &str, output: &str) -> Option<SslCertificate> {
        let mut subject = String::new();
        let mut issuer = String::new();
        let mut expiry_date = String::new();
        let mut days_until_expiry: i64 = 0;
        
        for line in output.lines() {
            let line = line.trim();
            if line.starts_with("subject=") {
                subject = line.trim_start_matches("subject=").to_string();
            } else if line.starts_with("issuer=") {
                issuer = line.trim_start_matches("issuer=").to_string();
            } else if line.starts_with("notAfter=") {
                expiry_date = line.trim_start_matches("notAfter=").to_string();
                
                if let Ok(dt) = chrono::NaiveDateTime::parse_from_str(&expiry_date, "%b %d %H:%M:%S %Y %Z") {
                    let now = chrono::Utc::now().naive_utc();
                    days_until_expiry = (dt.timestamp() - now.timestamp()) / 86400;
                }
            }
        }
        
        if subject.is_empty() && issuer.is_empty() {
            return None;
        }
        
        Some(SslCertificate {
            path: path.to_string(),
            subject,
            issuer,
            expiry_date,
            days_until_expiry,
            is_valid: days_until_expiry > 0,
        })
    }
    
    fn parse_openssl_output(&self, path: &str, _content: &str) -> Option<SslCertificate> {
        if let Ok(output) = std::process::Command::new("openssl")
            .args(["x509", "-in", path, "-noout", "-subject", "-issuer", "-enddate"])
            .output()
        {
            let output_str = String::from_utf8_lossy(&output.stdout);
            return self.parse_ssl_cert(path, &output_str);
        }
        None
    }
    
    fn collect_connection_states(&self) -> Vec<ConnectionState> {
        let mut state_counts: HashMap<String, u32> = HashMap::new();
        
        for proto in &["tcp", "tcp6", "udp", "udp6"] {
            if let Ok(content) = fs::read_to_string(format!("/proc/net/{}", proto)) {
                for line in content.lines().skip(1) {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 4 {
                        let state_hex = parts[3];
                        let state = match u8::from_str_radix(state_hex, 16).ok()? {
                            0x01 => "ESTABLISHED",
                            0x02 => "SYN_SENT",
                            0x03 => "SYN_RECV",
                            0x04 => "FIN_WAIT1",
                            0x05 => "FIN_WAIT2",
                            0x06 => "TIME_WAIT",
                            0x07 => "CLOSE",
                            0x08 => "CLOSE_WAIT",
                            0x09 => "LAST_ACK",
                            0x0A => "LISTEN",
                            0x0B => "CLOSING",
                            _ => "UNKNOWN",
                        };
                        *state_counts.entry(state.to_string()).or_insert(0) += 1;
                    }
                }
            }
        }
        
        state_counts
            .into_iter()
            .map(|(state, count)| ConnectionState { state, count })
            .collect()
    }
    
    fn collect_packet_loss(&self) -> Vec<PacketLossInfo> {
        let mut packet_info = Vec::new();
        
        if let Ok(content) = fs::read_to_string("/proc/net/dev") {
            for line in content.lines().skip(2) {
                let parts: Vec<&str> = line.split(':').collect();
                if parts.len() == 2 {
                    let iface = parts[0].trim().to_string();
                    let stats: Vec<&str> = parts[1].split_whitespace().collect();
                    
                    if stats.len() >= 16 {
                        let rx_packets = stats[1].parse::<u64>().unwrap_or(0);
                        let rx_errors = stats[2].parse::<u64>().unwrap_or(0);
                        let rx_dropped = stats[3].parse::<u64>().unwrap_or(0);
                        let tx_packets = stats[9].parse::<u64>().unwrap_or(0);
                        let tx_errors = stats[10].parse::<u64>().unwrap_or(0);
                        let tx_dropped = stats[11].parse::<u64>().unwrap_or(0);
                        
                        let total_packets = rx_packets + tx_packets;
                        let total_drops = rx_dropped + tx_dropped + rx_errors + tx_errors;
                        let drop_rate = if total_packets > 0 {
                            (total_drops as f64 / total_packets as f64) * 100.0
                        } else {
                            0.0
                        };
                        
                        packet_info.push(PacketLossInfo {
                            interface: iface,
                            rx_dropped,
                            tx_dropped,
                            rx_errors,
                            tx_errors,
                            total_packets,
                            drop_rate,
                        });
                    }
                }
            }
        }
        
        packet_info
    }
    
    fn detect_primary_interface(&self) -> String {
        let default_route = self.collect_gateway();
        if default_route.default_gateway != "N/A" {
            if let Ok(output) = std::process::Command::new("ip")
                .args(["route", "get", &default_route.default_gateway])
                .output()
            {
                let output_str = String::from_utf8_lossy(&output.stdout);
                for line in output_str.lines() {
                    if line.contains("dev") {
                        let parts: Vec<&str> = line.split("dev").collect();
                        if parts.len() >= 2 {
                            let iface = parts[1].trim().split_whitespace().next().unwrap_or("unknown");
                            return iface.to_string();
                        }
                    }
                }
            }
        }
        
        "eth0".to_string()
    }

    fn collect_temperature(&self) -> Option<TemperatureMetrics> {
        let mut cpu_temp = 0.0f32;
        let mut max_temp = 0.0f32;
        let mut gpu_temp = None;
        
        if let Ok(thermal_zones) = fs::read_dir("/sys/class/thermal") {
            for entry in thermal_zones.flatten() {
                if let Ok(zone_type) = fs::read_to_string(entry.path().join("type")) {
                    let zone_type = zone_type.trim().to_lowercase();
                    let temp_path = entry.path().join("temp");
                    if let Ok(temp_str) = fs::read_to_string(&temp_path) {
                        if let Ok(temp_millidegrees) = temp_str.trim().parse::<i32>() {
                            let temp_celsius = temp_millidegrees as f32 / 1000.0;
                            
                            if zone_type.contains("x86_pkg") || zone_type.contains("cpu") || zone_type.contains("coretemp") {
                                if cpu_temp == 0.0 || temp_celsius > cpu_temp {
                                    cpu_temp = temp_celsius;
                                }
                            }
                            
                            if zone_type.contains("gpu") || zone_type.contains("radeon") || zone_type.contains("nvidia") {
                                gpu_temp = Some(temp_celsius);
                            }
                            
                            if temp_celsius > max_temp {
                                max_temp = temp_celsius;
                            }
                        }
                    }
                }
            }
        }
        
        if cpu_temp == 0.0 && max_temp == 0.0 {
            if let Ok(gpu_path) = fs::read_to_string("/sys/class/hwmon/hwmon0/name") {
                if gpu_path.trim().contains("nvidia") {
                    if let Ok(temp_str) = fs::read_to_string("/sys/class/hwmon/hwmon0/temp1_input") {
                        if let Ok(temp_millidegrees) = temp_str.trim().parse::<i32>() {
                            let temp_celsius = temp_millidegrees as f32 / 1000.0;
                            gpu_temp = Some(temp_celsius);
                            cpu_temp = temp_celsius;
                            max_temp = temp_celsius;
                        }
                    }
                }
            }
        }
        
        if cpu_temp > 0.0 || max_temp > 0.0 {
            Some(TemperatureMetrics {
                cpu_temp: if cpu_temp > 0.0 { cpu_temp } else { max_temp },
                gpu_temp,
                max_temp,
            })
        } else {
            None
        }
    }

    fn collect_load_average(&self) -> Option<LoadAverageMetrics> {
        if let Ok(content) = fs::read_to_string("/proc/loadavg") {
            let parts: Vec<&str> = content.split_whitespace().collect();
            if parts.len() >= 3 {
                let load_1 = parts[0].parse::<f64>().unwrap_or(0.0);
                let load_5 = parts[1].parse::<f64>().unwrap_or(0.0);
                let load_15 = parts[2].parse::<f64>().unwrap_or(0.0);
                
                return Some(LoadAverageMetrics {
                    load_1,
                    load_5,
                    load_15,
                });
            }
        }
        None
    }

    fn collect_network_connections(&self) -> Option<NetworkConnectionsMetrics> {
        let mut tcp_count = 0u32;
        let mut udp_count = 0u32;
        let mut tcp_established = 0u32;
        let mut tcp_listening = 0u32;
        
        if let Ok(content) = fs::read_to_string("/proc/net/tcp") {
            for line in content.lines().skip(1) {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 4 {
                    tcp_count += 1;
                    let state = u32::from_str_radix(parts[3], 16).unwrap_or(0);
                    if state == 0x01 { tcp_established += 1; }
                    if state == 0x0A { tcp_listening += 1; }
                }
            }
        }
        
        if let Ok(content) = fs::read_to_string("/proc/net/tcp6") {
            for line in content.lines().skip(1) {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 4 {
                    tcp_count += 1;
                    let state = u32::from_str_radix(parts[3], 16).unwrap_or(0);
                    if state == 0x01 { tcp_established += 1; }
                    if state == 0x0A { tcp_listening += 1; }
                }
            }
        }
        
        if let Ok(content) = fs::read_to_string("/proc/net/udp") {
            for line in content.lines().skip(1) {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 4 {
                    udp_count += 1;
                }
            }
        }
        
        if let Ok(content) = fs::read_to_string("/proc/net/udp6") {
            for line in content.lines().skip(1) {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 4 {
                    udp_count += 1;
                }
            }
        }
        
        Some(NetworkConnectionsMetrics {
            tcp_count,
            udp_count,
            tcp_established,
            tcp_listening,
        })
    }

    fn collect_io_wait(&mut self) -> Option<IoWaitMetrics> {
        let mut total_idle = 0u64;
        let mut total_iowait = 0u64;
        let mut total_all = 0u64;
        
        if let Ok(content) = fs::read_to_string("/proc/stat") {
            if let Some(cpu_line) = content.lines().find(|l| l.starts_with("cpu ")) {
                let parts: Vec<&str> = cpu_line.split_whitespace().collect();
                if parts.len() >= 8 {
                    let values: Vec<u64> = parts[1..]
                        .iter()
                        .filter_map(|s| s.parse::<u64>().ok())
                        .collect();
                    
                    if values.len() >= 4 {
                        total_iowait = values.get(3).copied().unwrap_or(0);
                        total_idle = values.get(3).copied().unwrap_or(0) + values.get(4).copied().unwrap_or(0);
                        total_all = values.iter().sum();
                    }
                }
            }
        }
        
        let iowait_percent = if let Some((prev_total, prev_iowait)) = self.last_cpu_times {
            if total_all > prev_total {
                let total_diff = total_all - prev_total;
                let iowait_diff = total_iowait - prev_iowait;
                if total_diff > 0 {
                    (iowait_diff as f32 / total_diff as f32) * 100.0
                } else {
                    0.0
                }
            } else {
                0.0
            }
        } else {
            0.0
        };
        
        self.last_cpu_times = Some((total_all, total_iowait));
        
        Some(IoWaitMetrics {
            iowait_percent: iowait_percent.max(0.0),
        })
    }

    fn collect_memory_details(&self) -> Option<MemoryDetailsMetrics> {
        let mut buffers = 0u64;
        let mut cached = 0u64;
        let mut swap_cached = 0u64;
        
        if let Ok(content) = fs::read_to_string("/proc/meminfo") {
            for line in content.lines() {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    let key = parts[0].trim_end_matches(':');
                    let value: u64 = parts[1].parse().unwrap_or(0) * 1024;
                    
                    match key {
                        "Buffers" => buffers = value,
                        "Cached" => cached = value,
                        "SwapCached" => swap_cached = value,
                        _ => {}
                    }
                }
            }
        }
        
        Some(MemoryDetailsMetrics {
            buffers,
            cached,
            swap_cached,
        })
    }

    fn collect_container_info(&self) -> Option<ContainerInfo> {
        let mut is_container = false;
        let mut container_type: Option<String> = None;
        
        if fs::read_to_string("/.dockerenv").is_ok() {
            is_container = true;
            container_type = Some("docker".to_string());
        }
        
        if let Ok(cgroup) = fs::read_to_string("/proc/1/cgroup") {
            let cgroup_lower = cgroup.to_lowercase();
            if cgroup_lower.contains("docker") || cgroup_lower.contains("containerd") {
                is_container = true;
                if cgroup_lower.contains("kubernetes") || cgroup_lower.contains("k8s") {
                    container_type = Some("kubernetes".to_string());
                } else if cgroup_lower.contains("docker") {
                    container_type = Some("docker".to_string());
                } else if cgroup_lower.contains("containerd") {
                    container_type = Some("containerd".to_string());
                } else {
                    container_type = Some("container".to_string());
                }
            }
        }
        
        if let Ok(env) = fs::read_to_string("/proc/1/environ") {
            if env.contains("KUBERNETES") || env.contains("K8S_") || env.contains("KUBECTL_") {
                is_container = true;
                container_type = Some("kubernetes".to_string());
            }
        }
        
        Some(ContainerInfo {
            is_container,
            container_type,
        })
    }

    fn collect_virtual_memory(&self) -> Option<VirtualMemoryMetrics> {
        let mut page_faults = 0u64;
        let mut major_page_faults = 0u64;
        let mut pages_paged_in = 0u64;
        let mut pages_paged_out = 0u64;
        let mut pages_swapped_in = 0u64;
        let mut pages_swapped_out = 0u64;
        
        if let Ok(content) = fs::read_to_string("/proc/vmstat") {
            for line in content.lines() {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    let key = parts[0];
                    let value: u64 = parts[1].parse().unwrap_or(0);
                    
                    match key {
                        "pgfault" => page_faults = value,
                        "pgmajfault" => major_page_faults = value,
                        "pgpgin" => pages_paged_in = value,
                        "pgpgout" => pages_paged_out = value,
                        "pswpin" => pages_swapped_in = value,
                        "pswpout" => pages_swapped_out = value,
                        _ => {}
                    }
                }
            }
        }
        
        Some(VirtualMemoryMetrics {
            page_faults,
            major_page_faults,
            pages_paged_in,
            pages_paged_out,
            pages_swapped_in,
            pages_swapped_out,
        })
    }
}
