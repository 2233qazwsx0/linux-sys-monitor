use std::fs;
use sysinfo::System;

pub struct SystemInfoData {
    pub hostname: String,
    pub os_name: String,
    pub kernel_version: String,
    pub arch: String,
    pub uptime: String,
}

pub struct LoadAverage {
    pub one_minute: f64,
    pub five_minute: f64,
    pub fifteen_minute: f64,
    pub running_procs: usize,
    pub total_procs: usize,
}

pub struct UserInfo {
    pub username: String,
    pub tty: String,
    pub host: String,
    pub login_time: String,
}

pub fn get_system_info_impl() -> SystemInfoData {
    let hostname = System::host_name().unwrap_or_else(|| "unknown".to_string());
    
    let os_name = fs::read_to_string("/etc/os-release")
        .ok()
        .and_then(|content| {
            content.lines()
                .find(|l| l.starts_with("PRETTY_NAME="))
                .map(|l| l.trim_start_matches("PRETTY_NAME=").trim_matches('"').to_string())
        })
        .unwrap_or_else(|| "Linux".to_string());
    
    let kernel_version = System::kernel_version().unwrap_or_else(|| "unknown".to_string());
    let arch = std::env::consts::ARCH.to_string();
    let uptime = get_uptime_string();
    
    SystemInfoData {
        hostname,
        os_name,
        kernel_version,
        arch,
        uptime,
    }
}

pub fn get_uptime_string() -> String {
    if let Ok(content) = fs::read_to_string("/proc/uptime") {
        if let Some(uptime) = content.split_whitespace().next() {
            if let Ok(secs) = uptime.parse::<f64>() {
                return format_uptime(secs as i64);
            }
        }
    }
    "unknown".to_string()
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

pub fn get_load_average_impl() -> LoadAverage {
    let mut load_avg = LoadAverage {
        one_minute: 0.0,
        five_minute: 0.0,
        fifteen_minute: 0.0,
        running_procs: 0,
        total_procs: 0,
    };
    
    if let Ok(content) = fs::read_to_string("/proc/loadavg") {
        let parts: Vec<&str> = content.split_whitespace().collect();
        if parts.len() >= 4 {
            load_avg.one_minute = parts[0].parse().unwrap_or(0.0);
            load_avg.five_minute = parts[1].parse().unwrap_or(0.0);
            load_avg.fifteen_minute = parts[2].parse().unwrap_or(0.0);
            
            let proc_parts: Vec<&str> = parts[3].split('/').collect();
            if proc_parts.len() >= 2 {
                load_avg.running_procs = proc_parts[0].parse().unwrap_or(0);
                load_avg.total_procs = proc_parts[1].parse().unwrap_or(0);
            }
        }
    }
    
    load_avg
}

pub fn get_users_impl() -> Vec<UserInfo> {
    let mut users = Vec::new();
    
    if let Ok(content) = fs::read_to_string("/var/run/utmp") {
        let lines: Vec<&str> = content.lines().collect();
        for line in lines.iter().take(10) {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 {
                let username = parts[0].to_string();
                let tty = parts[1].to_string();
                let host = parts[2].to_string();
                
                if !username.is_empty() && username != "~" {
                    users.push(UserInfo {
                        username,
                        tty,
                        host,
                        login_time: "now".to_string(),
                    });
                }
            }
        }
    }
    
    if users.is_empty() {
        if let Ok(content) = fs::read_to_string("/etc/passwd") {
            for line in content.lines().take(5) {
                let parts: Vec<&str> = line.split(':').collect();
                if parts.len() >= 5 {
                    users.push(UserInfo {
                        username: parts[0].to_string(),
                        tty: "tty".to_string(),
                        host: "localhost".to_string(),
                        login_time: "active".to_string(),
                    });
                }
            }
        }
    }
    
    users
}
