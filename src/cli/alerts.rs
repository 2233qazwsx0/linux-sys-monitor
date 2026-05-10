use sysinfo::{System, Disks};

#[derive(Debug, Clone)]
pub struct Alert {
    pub level: String,
    pub category: String,
    pub message: String,
    pub value: f32,
    pub threshold: f32,
}

pub struct AlertThresholds {
    pub cpu: f32,
    pub memory: f32,
    pub disk: f32,
    pub temperature: f32,
}

impl Default for AlertThresholds {
    fn default() -> Self {
        Self {
            cpu: 90.0,
            memory: 90.0,
            disk: 90.0,
            temperature: 80.0,
        }
    }
}

pub fn check_alerts_impl(system: &System, disks: &Disks, thresholds: &AlertThresholds) -> Vec<Alert> {
    let mut alerts = Vec::new();
    
    let cpus = system.cpus();
    if !cpus.is_empty() {
        let avg_usage: f32 = cpus.iter().map(|c| c.cpu_usage()).sum::<f32>() / cpus.len() as f32;
        
        if avg_usage >= thresholds.cpu {
            alerts.push(Alert {
                level: "CRITICAL".to_string(),
                category: "CPU".to_string(),
                message: format!("CPU usage critically high at {:.1}%", avg_usage),
                value: avg_usage,
                threshold: thresholds.cpu,
            });
        } else if avg_usage >= thresholds.cpu * 0.8 {
            alerts.push(Alert {
                level: "WARNING".to_string(),
                category: "CPU".to_string(),
                message: format!("CPU usage elevated at {:.1}%", avg_usage),
                value: avg_usage,
                threshold: thresholds.cpu,
            });
        }
    }
    
    let total_mem = system.total_memory();
    let used_mem = system.used_memory();
    if total_mem > 0 {
        let mem_percent = used_mem as f32 / total_mem as f32 * 100.0;
        
        if mem_percent >= thresholds.memory {
            alerts.push(Alert {
                level: "CRITICAL".to_string(),
                category: "Memory".to_string(),
                message: format!("Memory usage critically high at {:.1}%", mem_percent),
                value: mem_percent,
                threshold: thresholds.memory,
            });
        } else if mem_percent >= thresholds.memory * 0.8 {
            alerts.push(Alert {
                level: "WARNING".to_string(),
                category: "Memory".to_string(),
                message: format!("Memory usage elevated at {:.1}%", mem_percent),
                value: mem_percent,
                threshold: thresholds.memory,
            });
        }
    }
    
    for disk in disks.iter() {
        let total = disk.total_space();
        let available = disk.available_space();
        if total > 0 {
            let used = total - available;
            let disk_percent = used as f32 / total as f32 * 100.0;
            let mount = disk.mount_point().to_string_lossy();
            
            if disk_percent >= thresholds.disk {
                alerts.push(Alert {
                    level: "CRITICAL".to_string(),
                    category: format!("Disk({})", mount),
                    message: format!("Disk {} usage critically high at {:.1}%", mount, disk_percent),
                    value: disk_percent,
                    threshold: thresholds.disk,
                });
            } else if disk_percent >= thresholds.disk * 0.8 {
                alerts.push(Alert {
                    level: "WARNING".to_string(),
                    category: format!("Disk({})", mount),
                    message: format!("Disk {} usage elevated at {:.1}%", mount, disk_percent),
                    value: disk_percent,
                    threshold: thresholds.disk,
                });
            }
        }
    }
    
    let temp_paths = [
        "/sys/class/thermal/thermal_zone0/temp",
        "/sys/devices/virtual/thermal/thermal_zone0/temp",
        "/sys/class/hwmon/hwmon0/temp1_input",
    ];
    
    for path in temp_paths {
        if let Ok(content) = std::fs::read_to_string(path) {
            if let Ok(temp_milli) = content.trim().parse::<i64>() {
                let temp = temp_milli as f64 / 1000.0;
                
                if temp >= thresholds.temperature as f64 {
                    alerts.push(Alert {
                        level: "CRITICAL".to_string(),
                        category: "Temperature".to_string(),
                        message: format!("Temperature critically high at {:.1}°C", temp),
                        value: temp as f32,
                        threshold: thresholds.temperature,
                    });
                } else if temp >= thresholds.temperature as f64 * 0.9 {
                    alerts.push(Alert {
                        level: "WARNING".to_string(),
                        category: "Temperature".to_string(),
                        message: format!("Temperature elevated at {:.1}°C", temp),
                        value: temp as f32,
                        threshold: thresholds.temperature,
                    });
                }
                break;
            }
        }
    }
    
    alerts.sort_by(|a, b| {
        let a_order = match a.level.as_str() {
            "CRITICAL" => 0,
            "WARNING" => 1,
            _ => 2,
        };
        let b_order = match b.level.as_str() {
            "CRITICAL" => 0,
            "WARNING" => 1,
            _ => 2,
        };
        a_order.cmp(&b_order)
    });
    
    alerts
}
