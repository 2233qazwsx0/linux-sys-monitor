use sysinfo::System;
use std::fs;

pub struct MemoryInfo {
    pub total: u64,
    pub used: u64,
    pub free: u64,
    pub available: u64,
    pub usage_percent: f32,
    pub buffers: u64,
    pub cached: u64,
    pub active: u64,
    pub inactive: u64,
    pub s_reclaimable: u64,
    pub shmem: u64,
    pub dirty: u64,
    pub writeback: u64,
    pub swap_cached: u64,
}

pub struct SwapInfo {
    pub total: u64,
    pub used: u64,
    pub free: u64,
}

pub fn get_memory_info_impl(system: &System) -> MemoryInfo {
    let total = system.total_memory();
    let used = system.used_memory();
    let available = system.available_memory();
    let free = system.free_memory();
    
    let usage_percent = if total > 0 {
        used as f32 / total as f32 * 100.0
    } else {
        0.0
    };
    
    let mut buffers: u64 = 0;
    let mut cached: u64 = 0;
    let mut active: u64 = 0;
    let mut inactive: u64 = 0;
    let mut s_reclaimable: u64 = 0;
    let mut shmem: u64 = 0;
    let mut dirty: u64 = 0;
    let mut writeback: u64 = 0;
    let mut swap_cached: u64 = 0;
    
    if let Ok(content) = fs::read_to_string("/proc/meminfo") {
        for line in content.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                let value: u64 = parts[1].parse().unwrap_or(0) * 1024;
                match parts[0].trim_end_matches(':') {
                    "Buffers" => buffers = value,
                    "Cached" => cached = value,
                    "Active" => active = value,
                    "Inactive" => inactive = value,
                    "SReclaimable" => s_reclaimable = value,
                    "Shmem" => shmem = value,
                    "Dirty" => dirty = value,
                    "Writeback" => writeback = value,
                    "SwapCached" => swap_cached = value,
                    _ => {}
                }
            }
        }
    }
    
    MemoryInfo {
        total,
        used,
        free,
        available,
        usage_percent,
        buffers,
        cached,
        active,
        inactive,
        s_reclaimable,
        shmem,
        dirty,
        writeback,
        swap_cached,
    }
}

pub fn get_swap_info_impl() -> SwapInfo {
    let mut total: u64 = 0;
    let mut used: u64 = 0;
    let mut free: u64 = 0;
    
    if let Ok(content) = fs::read_to_string("/proc/meminfo") {
        for line in content.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                let value: u64 = parts[1].parse().unwrap_or(0) * 1024;
                match parts[0].trim_end_matches(':') {
                    "SwapTotal" => total = value,
                    "SwapFree" => free = value,
                    _ => {}
                }
            }
        }
    }
    
    used = total.saturating_sub(free);
    
    SwapInfo { total, used, free }
}
