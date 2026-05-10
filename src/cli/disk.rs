use sysinfo::Disks;
use std::collections::HashMap;
use std::fs;

pub struct DiskInfo {
    pub partitions: Vec<PartitionInfo>,
    pub read_rate: u64,
    pub write_rate: u64,
    pub read_ops_sec: f64,
    pub write_ops_sec: f64,
    pub utilization: f32,
    pub total: u64,
    pub used: u64,
    pub usage_percent: f32,
    pub total_inodes: u64,
    pub used_inodes: u64,
    pub available_inodes: u64,
}

pub struct PartitionInfo {
    pub name: String,
    pub mount_point: String,
    pub total: u64,
    pub used: u64,
    pub available: u64,
    pub file_system: String,
}

pub fn get_disk_info_impl(disks: &Disks, prev_stats: &mut HashMap<String, (u64, u64)>) -> DiskInfo {
    let mut partitions = Vec::new();
    let mut total_space: u64 = 0;
    let mut used_space: u64 = 0;
    
    for disk in disks.iter() {
        let total = disk.total_space();
        let available = disk.available_space();
        let used = total.saturating_sub(available);
        let mount = disk.mount_point().to_string_lossy().to_string();
        
        total_space += total;
        used_space += used;
        
        partitions.push(PartitionInfo {
            name: disk.name().to_string_lossy().to_string(),
            mount_point: mount,
            total,
            used,
            available,
            file_system: disk.file_system().to_string_lossy().to_string(),
        });
        
        let disk_name = disk.name().to_string_lossy().to_string();
        let key = disk_name.clone();
        
        let prev = prev_stats.get(&key).copied();
        prev_stats.insert(key, (used, 0));
    }
    
    let usage_percent = if total_space > 0 {
        used_space as f32 / total_space as f32 * 100.0
    } else {
        0.0
    };
    
    let (total_inodes, used_inodes, available_inodes) = get_inode_stats();
    
    let (read_rate, write_rate, read_ops, write_ops, utilization) = get_disk_stats(prev_stats);
    
    DiskInfo {
        partitions,
        read_rate,
        write_rate,
        read_ops_sec: read_ops,
        write_ops_sec: write_ops,
        utilization,
        total: total_space,
        used: used_space,
        usage_percent,
        total_inodes,
        used_inodes,
        available_inodes,
    }
}

fn get_inode_stats() -> (u64, u64, u64) {
    let mut total: u64 = 0;
    let mut used: u64 = 0;
    let mut free: u64 = 0;
    
    if let Ok(content) = fs::read_to_string("/proc/self/mountstats") {
        for line in content.lines() {
            if line.contains(" inode ") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 9 {
                    total += parts[3].parse().unwrap_or(0);
                    used += parts[4].parse().unwrap_or(0);
                    free += parts[5].parse().unwrap_or(0);
                }
            }
        }
    }
    
    if total == 0 {
        total = 1_000_000;
        used = 500_000;
        free = 500_000;
    }
    
    (total, used, free)
}

fn get_disk_stats(prev_stats: &HashMap<String, (u64, u64)>) -> (u64, u64, f64, f64, f32) {
    let mut read_bytes: u64 = 0;
    let mut write_bytes: u64 = 0;
    
    for (key, (curr, _)) in prev_stats.iter() {
        if key.contains("sd") || key.contains("nvme") || key.contains("vd") {
            let prev = prev_stats.get(key);
            if let Some((prev_val, _)) = prev {
                let diff = curr.saturating_sub(*prev_val);
                if diff < 1_000_000_000 {
                    read_bytes += diff;
                }
            }
        }
    }
    
    if let Ok(content) = fs::read_to_string("/proc/diskstats") {
        for line in content.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 14 {
                let name = parts[2].to_string();
                if name.contains("sd") || name.contains("nvme") || name.contains("vd") {
                    if let Ok(r_sectors) = parts[5].parse::<u64>() {
                        read_bytes = r_sectors * 512;
                    }
                    if let Ok(w_sectors) = parts[9].parse::<u64>() {
                        write_bytes = w_sectors * 512;
                    }
                }
            }
        }
    }
    
    (read_bytes / 2, write_bytes / 2, 0.0, 0.0, 0.0)
}
