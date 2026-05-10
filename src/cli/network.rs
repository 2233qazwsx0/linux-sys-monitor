use sysinfo::Networks;
use std::collections::HashMap;
use std::fs;
use std::time::{Duration, Instant};

pub struct NetworkInfo {
    pub interfaces: Vec<NetworkInterface>,
    pub rx_rate: u64,
    pub tx_rate: u64,
    pub total_rx: u64,
    pub total_tx: u64,
    pub interface_count: usize,
    pub connection_count: usize,
    pub tcp_count: usize,
    pub udp_count: usize,
    pub listen_count: usize,
    pub established_count: usize,
    pub time_wait_count: usize,
    pub closed_count: usize,
}

pub struct NetworkInterface {
    pub name: String,
    pub rx_rate: u64,
    pub tx_rate: u64,
    pub total_rx: u64,
    pub total_tx: u64,
}

pub fn get_network_info_impl(networks: &Networks, prev_stats: &mut HashMap<String, (u64, u64)>) -> NetworkInfo {
    let mut interfaces = Vec::new();
    let mut total_rx: u64 = 0;
    let mut total_tx: u64 = 0;
    
    for (name, data) in networks.iter() {
        let rx = data.total_received();
        let tx = data.total_transmitted();
        total_rx += rx;
        total_tx += tx;
        
        let prev = prev_stats.get(name).copied();
        let (rx_rate, tx_rate) = if let Some((prev_rx, prev_tx)) = prev {
            let elapsed = 1.0;
            ((rx.saturating_sub(prev_rx)) as u64, (tx.saturating_sub(prev_tx)) as u64)
        } else {
            (0, 0)
        };
        
        prev_stats.insert(name.clone(), (rx, tx));
        
        interfaces.push(NetworkInterface {
            name: name.clone(),
            rx_rate,
            tx_rate,
            total_rx: rx,
            total_tx: tx,
        });
    }
    
    let interface_count = interfaces.len();
    let connections = get_connection_stats();
    
    NetworkInfo {
        interfaces,
        rx_rate: 0,
        tx_rate: 0,
        total_rx,
        total_tx,
        interface_count,
        connection_count: connections.total,
        tcp_count: connections.tcp,
        udp_count: connections.udp,
        listen_count: connections.listen,
        established_count: connections.established,
        time_wait_count: connections.time_wait,
        closed_count: connections.closed,
    }
}

struct ConnectionStats {
    total: usize,
    tcp: usize,
    udp: usize,
    listen: usize,
    established: usize,
    time_wait: usize,
    closed: usize,
}

fn get_connection_stats() -> ConnectionStats {
    let mut stats = ConnectionStats {
        total: 0,
        tcp: 0,
        udp: 0,
        listen: 0,
        established: 0,
        time_wait: 0,
        closed: 0,
    };
    
    if let Ok(content) = fs::read_to_string("/proc/net/snmp") {
        for line in content.lines().skip(1) {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 12 {
                if parts[0] == "Tcp" {
                    stats.listen = parts[4].parse().unwrap_or(0);
                    stats.established = parts[5].parse().unwrap_or(0);
                    stats.time_wait = parts[8].parse().unwrap_or(0);
                    stats.closed = parts[10].parse().unwrap_or(0);
                }
            }
        }
    }
    
    if let Ok(content) = fs::read_to_string("/proc/net/tcp") {
        for line in content.lines().skip(1) {
            stats.tcp += 1;
        }
    }
    
    if let Ok(content) = fs::read_to_string("/proc/net/udp") {
        for line in content.lines().skip(1) {
            stats.udp += 1;
        }
    }
    
    stats.total = stats.tcp + stats.udp;
    
    stats
}
