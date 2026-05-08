use crate::metrics::{SystemMetrics, TimeRange};
use std::collections::HashMap;

pub fn export_json(data: &[SystemMetrics]) -> String {
    serde_json::to_string_pretty(data).unwrap_or_default()
}

pub fn export_json_compact(data: &[SystemMetrics]) -> String {
    serde_json::to_string(data).unwrap_or_default()
}

pub fn export_csv(data: &[SystemMetrics]) -> String {
    let mut wtr = csv::Writer::from_writer(vec![]);
    
    wtr.write_record(&[
        "timestamp", "uptime", "hostname", "cpu_usage", "cpu_cores", 
        "memory_total", "memory_used", "memory_available", "memory_percent",
        "swap_total", "swap_used", "swap_percent",
        "disk_read_rate", "disk_write_rate",
        "network_rx_rate", "network_tx_rate",
    ]).ok();
    
    for m in data {
        wtr.write_record(&[
            m.timestamp.to_string(),
            m.uptime.to_string(),
            m.hostname.clone(),
            m.cpu.usage.to_string(),
            m.cpu.core_count.to_string(),
            m.memory.total.to_string(),
            m.memory.used.to_string(),
            m.memory.available.to_string(),
            m.memory.usage_percent.to_string(),
            m.swap.total.to_string(),
            m.swap.used.to_string(),
            m.swap.usage_percent.to_string(),
            m.disk.read_rate.to_string(),
            m.disk.write_rate.to_string(),
            m.network.rx_rate.to_string(),
            m.network.tx_rate.to_string(),
        ]).ok();
    }
    
    String::from_utf8(wtr.into_inner().unwrap_or_default()).unwrap_or_default()
}

pub fn export_prometheus(data: &[SystemMetrics]) -> String {
    let mut output = String::new();
    let hostname = data.first().map(|m| m.hostname.as_str()).unwrap_or("unknown");
    
    for m in data {
        let ts = m.timestamp * 1000;
        
        output.push_str(&format!("# TYPE system_cpu_usage gauge\n"));
        output.push_str(&format!("system_cpu_usage{{hostname=\"{}\"}} {} {}\n", 
            sanitize_label(hostname), m.cpu.usage, ts));
        
        output.push_str(&format!("# TYPE system_cpu_cores gauge\n"));
        output.push_str(&format!("system_cpu_cores{{hostname=\"{}\"}} {} {}\n", 
            sanitize_label(hostname), m.cpu.core_count, ts));
        
        for (i, &usage) in m.cpu.per_core.iter().enumerate() {
            output.push_str(&format!("system_cpu_core_usage{{hostname=\"{}\",core=\"{}\"}} {} {}\n", 
                sanitize_label(hostname), i, usage, ts));
        }
        
        output.push_str(&format!("# TYPE system_memory_total gauge\n"));
        output.push_str(&format!("system_memory_total{{hostname=\"{}\"}} {} {}\n", 
            sanitize_label(hostname), m.memory.total, ts));
        output.push_str(&format!("# TYPE system_memory_used gauge\n"));
        output.push_str(&format!("system_memory_used{{hostname=\"{}\"}} {} {}\n", 
            sanitize_label(hostname), m.memory.used, ts));
        output.push_str(&format!("# TYPE system_memory_available gauge\n"));
        output.push_str(&format!("system_memory_available{{hostname=\"{}\"}} {} {}\n", 
            sanitize_label(hostname), m.memory.available, ts));
        output.push_str(&format!("# TYPE system_memory_usage_percent gauge\n"));
        output.push_str(&format!("system_memory_usage_percent{{hostname=\"{}\"}} {} {}\n", 
            sanitize_label(hostname), m.memory.usage_percent, ts));
        
        output.push_str(&format!("# TYPE system_swap_total gauge\n"));
        output.push_str(&format!("system_swap_total{{hostname=\"{}\"}} {} {}\n", 
            sanitize_label(hostname), m.swap.total, ts));
        output.push_str(&format!("# TYPE system_swap_used gauge\n"));
        output.push_str(&format!("system_swap_used{{hostname=\"{}\"}} {} {}\n", 
            sanitize_label(hostname), m.swap.used, ts));
        output.push_str(&format!("# TYPE system_swap_usage_percent gauge\n"));
        output.push_str(&format!("system_swap_usage_percent{{hostname=\"{}\"}} {} {}\n", 
            sanitize_label(hostname), m.swap.usage_percent, ts));
        
        output.push_str(&format!("# TYPE system_disk_read_rate gauge\n"));
        output.push_str(&format!("system_disk_read_rate{{hostname=\"{}\"}} {} {}\n", 
            sanitize_label(hostname), m.disk.read_rate, ts));
        output.push_str(&format!("# TYPE system_disk_write_rate gauge\n"));
        output.push_str(&format!("system_disk_write_rate{{hostname=\"{}\"}} {} {}\n", 
            sanitize_label(hostname), m.disk.write_rate, ts));
        
        output.push_str(&format!("# TYPE system_network_rx_rate gauge\n"));
        output.push_str(&format!("system_network_rx_rate{{hostname=\"{}\"}} {} {}\n", 
            sanitize_label(hostname), m.network.rx_rate, ts));
        output.push_str(&format!("# TYPE system_network_tx_rate gauge\n"));
        output.push_str(&format!("system_network_tx_rate{{hostname=\"{}\"}} {} {}\n", 
            sanitize_label(hostname), m.network.tx_rate, ts));
    }
    
    output
}

pub fn export_influxdb(data: &[SystemMetrics], measurement: &str) -> String {
    let mut output = String::new();
    
    for m in data {
        let ts = m.timestamp * 1_000_000_000;
        
        let fields = format!(
            "cpu_usage={},cpu_cores={},memory_total={}u,memory_used={}u,memory_available={}u,memory_percent={},swap_total={}u,swap_used={}u,swap_percent={},disk_read_rate={}u,disk_write_rate={}u,network_rx_rate={}u,network_tx_rate={}u",
            m.cpu.usage,
            m.cpu.core_count,
            m.memory.total,
            m.memory.used,
            m.memory.available,
            m.memory.usage_percent,
            m.swap.total,
            m.swap.used,
            m.swap.usage_percent,
            m.disk.read_rate,
            m.disk.write_rate,
            m.network.rx_rate,
            m.network.tx_rate,
        );
        
        output.push_str(&format!(
            "{} hostname=\"{}\",uptime={} {} {}\n",
            measurement,
            escape_influx_string(&m.hostname),
            m.uptime,
            fields,
            ts
        ));
    }
    
    output
}

pub fn export_graphite(data: &[SystemMetrics], prefix: &str) -> String {
    let mut output = String::new();
    
    for m in data {
        let timestamp = m.timestamp;
        
        output.push_str(&format!("{}.cpu.usage {} {}\n", prefix, m.cpu.usage, timestamp));
        output.push_str(&format!("{}.cpu.cores {} {}\n", prefix, m.cpu.core_count, timestamp));
        
        for (i, &usage) in m.cpu.per_core.iter().enumerate() {
            output.push_str(&format!("{}.cpu.core.{}.usage {} {}\n", prefix, i, usage, timestamp));
        }
        
        output.push_str(&format!("{}.memory.total {} {}\n", prefix, m.memory.total, timestamp));
        output.push_str(&format!("{}.memory.used {} {}\n", prefix, m.memory.used, timestamp));
        output.push_str(&format!("{}.memory.available {} {}\n", prefix, m.memory.available, timestamp));
        output.push_str(&format!("{}.memory.percent {} {}\n", prefix, m.memory.usage_percent, timestamp));
        
        output.push_str(&format!("{}.swap.total {} {}\n", prefix, m.swap.total, timestamp));
        output.push_str(&format!("{}.swap.used {} {}\n", prefix, m.swap.used, timestamp));
        output.push_str(&format!("{}.swap.percent {} {}\n", prefix, m.swap.usage_percent, timestamp));
        
        output.push_str(&format!("{}.disk.read_rate {} {}\n", prefix, m.disk.read_rate, timestamp));
        output.push_str(&format!("{}.disk.write_rate {} {}\n", prefix, m.disk.write_rate, timestamp));
        
        output.push_str(&format!("{}.network.rx_rate {} {}\n", prefix, m.network.rx_rate, timestamp));
        output.push_str(&format!("{}.network.tx_rate {} {}\n", prefix, m.network.tx_rate, timestamp));
    }
    
    output
}

pub fn filter_by_time_range(data: &[SystemMetrics], range: &TimeRange) -> Vec<SystemMetrics> {
    data.iter()
        .filter(|m| m.timestamp >= range.start && m.timestamp <= range.end)
        .cloned()
        .collect()
}

pub fn get_metric_values(data: &[SystemMetrics], metric_path: &str) -> Vec<f64> {
    data.iter().filter_map(|m| {
        let parts: Vec<&str> = metric_path.split('.').collect();
        match parts.as_slice() {
            ["cpu", "usage"] => Some(m.cpu.usage as f64),
            ["cpu", "core_count"] => Some(m.cpu.core_count as f64),
            ["memory", "usage_percent"] => Some(m.memory.usage_percent as f64),
            ["memory", "total"] => Some(m.memory.total as f64),
            ["memory", "used"] => Some(m.memory.used as f64),
            ["swap", "usage_percent"] => Some(m.swap.usage_percent as f64),
            ["swap", "total"] => Some(m.swap.total as f64),
            ["swap", "used"] => Some(m.swap.used as f64),
            ["disk", "read_rate"] => Some(m.disk.read_rate as f64),
            ["disk", "write_rate"] => Some(m.disk.write_rate as f64),
            ["network", "rx_rate"] => Some(m.network.rx_rate as f64),
            ["network", "tx_rate"] => Some(m.network.tx_rate as f64),
            _ => None,
        }
    }).collect()
}

fn sanitize_label(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
}

fn escape_influx_string(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace(',', "\\,")
        .replace(' ', "\\ ")
}

pub fn compare_metrics(data: &[SystemMetrics], metrics: &[String]) -> HashMap<String, Vec<f64>> {
    let mut result = HashMap::new();
    for metric in metrics {
        let values = get_metric_values(data, metric);
        if !values.is_empty() {
            result.insert(metric.clone(), values);
        }
    }
    result
}
