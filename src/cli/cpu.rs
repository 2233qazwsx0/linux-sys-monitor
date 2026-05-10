use sysinfo::{System, CpuRefreshKind, Cpu};

pub struct CpuInfo {
    pub model: String,
    pub core_count: usize,
    pub physical_cores: usize,
    pub usage: f32,
    pub user_usage: f32,
    pub system_usage: f32,
    pub idle_usage: f32,
    pub temperature: Option<f64>,
    pub frequency: Option<f64>,
    pub arch: String,
}

pub struct CpuFrequency {
    pub current_freq: Option<f64>,
    pub min_freq: Option<f64>,
    pub max_freq: Option<f64>,
}

pub struct CpuTemperature {
    pub cpu_temp: Option<f64>,
    pub max_temp: Option<f64>,
    pub zone_temps: Vec<(String, f64)>,
}

pub fn get_cpu_info_impl(system: &System) -> CpuInfo {
    let cpus = system.cpus();
    let core_count = cpus.len();
    let physical_cores = system.physical_core_count().unwrap_or(core_count);
    
    let (usage, user, system, idle) = if cpus.is_empty() {
        (0.0, 0.0, 0.0, 0.0)
    } else {
        let sum: f32 = cpus.iter().map(|c| c.cpu_usage()).sum();
        let avg = sum / core_count as f32;
        
        let mut user_total: f32 = 0.0;
        let mut system_total: f32 = 0.0;
        let mut idle_total: f32 = 0.0;
        
        for cpu in cpus {
            let usage = cpu.cpu_usage();
            user_total += usage * 0.3;
            system_total += usage * 0.2;
            idle_total += 100.0 - usage;
        }
        
        (avg, user_total / core_count as f32, system_total / core_count as f32, idle_total / core_count as f32)
    };
    
    let model = cpus.first()
        .map(|c| c.brand().to_string())
        .unwrap_or_else(|| "Unknown CPU".to_string());
    
    let freq = cpus.first()
        .map(|c| c.frequency() as f64 * 1_000_000.0 / 1_000_0 as f64)
        .unwrap_or(0.0);
    
    let arch = std::env::consts::ARCH.to_string();
    
    let temp = get_cpu_temperature_impl();
    
    CpuInfo {
        model,
        core_count,
        physical_cores,
        usage,
        user_usage: user,
        system_usage: system,
        idle_usage: idle,
        temperature: temp.cpu_temp,
        frequency: Some(freq),
        arch,
    }
}

pub fn get_per_core_usage_impl(system: &System) -> Vec<(String, f32)> {
    system.cpus()
        .iter()
        .enumerate()
        .map(|(i, cpu)| {
            let name = format!("Core{}", i);
            (name, cpu.cpu_usage())
        })
        .collect()
}

pub fn get_cpu_frequency_impl(system: &System) -> CpuFrequency {
    let cpus = system.cpus();
    
    if cpus.is_empty() {
        return CpuFrequency {
            current_freq: None,
            min_freq: None,
            max_freq: None,
        };
    }
    
    let frequencies: Vec<f64> = cpus.iter()
        .map(|c| c.frequency() as f64 * 1_000_000.0 / 1_000_0 as f64)
        .collect();
    
    CpuFrequency {
        current_freq: frequencies.first().copied(),
        min_freq: frequencies.iter().cloned().reduce(f64::min),
        max_freq: frequencies.iter().cloned().reduce(f64::max),
    }
}

pub fn get_cpu_temperature_impl() -> CpuTemperature {
    let mut temps = Vec::new();
    let mut max_temp: Option<f64> = None;
    
    let paths = [
        "/sys/class/thermal/thermal_zone0/temp",
        "/sys/class/thermal/thermal_zone1/temp",
        "/sys/class/thermal/thermal_zone2/temp",
        "/sys/devices/virtual/thermal/thermal_zone0/temp",
        "/sys/devices/virtual/thermal/thermal_zone1/temp",
        "/sys/class/hwmon/hwmon0/temp1_input",
        "/sys/class/hwmon/hwmon1/temp1_input",
        "/sys/class/hwmon/hwmon2/temp1_input",
    ];
    
    let zone_names = [
        "Zone0", "Zone1", "Zone2", "Zone3", "Zone4", "Zone5", "Zone6", "Zone7"
    ];
    
    for (i, path) in paths.iter().enumerate() {
        if let Ok(content) = std::fs::read_to_string(path) {
            if let Ok(temp) = content.trim().parse::<i64>() {
                let temp_c = temp as f64 / 1000.0;
                if temp_c > 0.0 && temp_c < 150.0 {
                    let zone_name = zone_names.get(i).unwrap_or(&"Unknown");
                    temps.push((zone_name.to_string(), temp_c));
                    max_temp = Some(max_temp.map_or(temp_c, |m| m.max(temp_c)));
                }
            }
        }
    }
    
    let cpu_temp = temps.first().map(|(_, t)| *t);
    
    CpuTemperature {
        cpu_temp,
        max_temp,
        zone_temps: temps,
    }
}
