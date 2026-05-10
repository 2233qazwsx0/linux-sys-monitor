use std::process::Command;

pub struct GpuInfo {
    pub name: String,
    pub memory_total: Option<u64>,
    pub vram_used: Option<u64>,
    pub vram_free: Option<u64>,
    pub utilization: Option<f32>,
    pub memory_usage: Option<f32>,
    pub temperature: Option<f64>,
    pub power_draw: Option<f32>,
    pub power_limit: Option<f32>,
    pub clock_speed: Option<f32>,
    pub max_clock_speed: Option<f32>,
    pub fan_speeds: Vec<u32>,
    pub pcie_utilization: Option<f32>,
    pub pcie_gen: Option<u32>,
    pub pcie_width: Option<u32>,
}

pub fn get_gpu_info_impl() -> GpuInfo {
    let mut info = GpuInfo {
        name: "N/A".to_string(),
        memory_total: None,
        vram_used: None,
        vram_free: None,
        utilization: None,
        memory_usage: None,
        temperature: None,
        power_draw: None,
        power_limit: None,
        clock_speed: None,
        max_clock_speed: None,
        fan_speeds: Vec::new(),
        pcie_utilization: None,
        pcie_gen: None,
        pcie_width: None,
    };
    
    if let Ok(output) = Command::new("nvidia-smi")
        .args(["--query-gpu=name,memory.total,memory.used,memory.free,utilization.gpu,utilization.memory,temperature.gpu,power.draw,power.limit,clocks.current.graphics,clocks.max.graphics,fan.speed,pcie.utilization.gpu,pcie.gen,pcie.width", "--format=csv,noheader,nounits"])
        .output()
    {
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            if let Some(line) = stdout.lines().next() {
                let parts: Vec<&str> = line.split(',').map(|s| s.trim()).collect();
                if parts.len() >= 15 {
                    info.name = parts[0].to_string();
                    info.memory_total = parts[1].parse::<u64>().ok().map(|m| m * 1024 * 1024);
                    info.vram_used = parts[2].parse::<u64>().ok().map(|m| m * 1024 * 1024);
                    info.vram_free = parts[3].parse::<u64>().ok().map(|m| m * 1024 * 1024);
                    info.utilization = parts[4].parse::<f32>().ok();
                    info.memory_usage = parts[5].parse::<f32>().ok();
                    info.temperature = parts[6].parse::<f64>().ok();
                    info.power_draw = parts[7].parse::<f32>().ok();
                    info.power_limit = parts[8].parse::<f32>().ok();
                    info.clock_speed = parts[9].parse::<f32>().ok();
                    info.max_clock_speed = parts[10].parse::<f32>().ok();
                    info.fan_speeds = parts[11].parse::<u32>().ok().map(|f| vec![f]).unwrap_or_default();
                    info.pcie_utilization = parts[12].parse::<f32>().ok();
                    info.pcie_gen = parts[13].parse::<u32>().ok();
                    info.pcie_width = parts[14].parse::<u32>().ok();
                }
            }
            return info;
        }
    }
    
    if let Ok(output) = Command::new("rocm-smi")
        .args(["--showid", "--showmeminfo", "--showtemp", "--showutilization"])
        .output()
    {
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            info.name = "AMD GPU".to_string();
            
            for line in stdout.lines() {
                let parts: Vec<&str> = line.split(':').collect();
                if parts.len() >= 2 {
                    let key = parts[0].trim();
                    let value = parts[1].trim();
                    
                    if key.contains("Temperature") {
                        info.temperature = value.trim_end_matches('C').trim().parse::<f64>().ok();
                    } else if key.contains("GPU Utilization") {
                        info.utilization = value.trim_end_matches('%').trim().parse::<f32>().ok();
                    } else if key.contains("Memory Used") {
                        let used: u64 = value.split_whitespace().next().unwrap_or("0").parse().unwrap_or(0);
                        info.vram_used = Some(used * 1024 * 1024);
                    }
                }
            }
            return info;
        }
    }
    
    if let Ok(content) = std::fs::read_to_string("/proc/driver/nvidia/gpus/0/info") {
        for line in content.lines() {
            if line.contains("Model") {
                if let Some(name) = line.split(':').nth(1) {
                    info.name = name.trim().to_string();
                }
            } else if line.contains("Bus Bandwidth") {
                if let Some(bw) = line.split(':').nth(1) {
                    info.pcie_width = Some(bw.trim().parse().unwrap_or(16));
                }
            }
        }
    }
    
    if let Ok(content) = std::fs::read_to_string("/sys/class/drm/card0/device/gpu_busy_percent") {
        info.utilization = content.trim().parse::<f32>().ok();
    }
    
    if let Ok(content) = std::fs::read_to_string("/sys/class/drm/card0/device/hwmon/hwmon0/temp1_input") {
        info.temperature = content.trim().parse::<i64>().ok().map(|t| t as f64 / 1000.0);
    }
    
    if let Ok(content) = std::fs::read_to_string("/sys/class/drm/card0/device/mem_info_vid_used") {
        if let Ok(bytes) = content.trim().parse::<u64>() {
            info.vram_used = Some(bytes);
        }
    }
    
    if let Ok(content) = std::fs::read_to_string("/sys/class/drm/card0/device/mem_info_vid_total") {
        if let Ok(bytes) = content.trim().parse::<u64>() {
            info.memory_total = Some(bytes);
        }
    }
    
    if info.name == "N/A" && info.memory_total.is_some() {
        info.name = "Linux GPU".to_string();
    }
    
    info
}
