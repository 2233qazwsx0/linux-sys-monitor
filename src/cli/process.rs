use sysinfo::System;
use std::collections::HashMap;

pub enum ProcessSort {
    Cpu,
    Memory,
    Pid,
    Name,
}

pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub cpu_usage: f32,
    pub memory: u64,
    pub memory_percent: f32,
    pub status: String,
    pub user: String,
    pub command: String,
}

pub struct ProcessTreeNode {
    pub pid: u32,
    pub ppid: u32,
    pub name: String,
    pub cpu_usage: f32,
    pub depth: usize,
}

pub fn get_processes_impl(system: &System, sort_by: ProcessSort) -> Vec<ProcessInfo> {
    let total_mem = system.total_memory() as f32;
    
    let mut processes: Vec<ProcessInfo> = system.processes()
        .iter()
        .map(|(pid, process)| {
            let memory = process.memory();
            let memory_percent = if total_mem > 0.0 {
                memory as f32 / total_mem * 100.0
            } else {
                0.0
            };
            
            let status = format!("{:?}", process.status());
            
            ProcessInfo {
                pid: pid.as_u32(),
                name: process.name().to_string(),
                cpu_usage: process.cpu_usage(),
                memory,
                memory_percent,
                status,
                user: "unknown".to_string(),
                command: process.cmd().iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>()
                    .join(" "),
            }
        })
        .collect();
    
    match sort_by {
        ProcessSort::Cpu => {
            processes.sort_by(|a, b| b.cpu_usage.partial_cmp(&a.cpu_usage).unwrap_or(std::cmp::Ordering::Equal));
        }
        ProcessSort::Memory => {
            processes.sort_by(|a, b| b.memory.partial_cmp(&a.memory).unwrap_or(std::cmp::Ordering::Equal));
        }
        ProcessSort::Pid => {
            processes.sort_by(|a, b| a.pid.cmp(&b.pid));
        }
        ProcessSort::Name => {
            processes.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
        }
    }
    
    processes
}

pub fn get_process_tree_impl(system: &System) -> Vec<ProcessTreeNode> {
    let mut ppid_map: HashMap<u32, u32> = HashMap::new();
    let mut children: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut process_info: HashMap<u32, (String, f32)> = HashMap::new();
    
    for (pid, process) in system.processes() {
        let pid_u32 = pid.as_u32();
        let name = process.name().to_string();
        let cpu_usage = process.cpu_usage();
        
        process_info.insert(pid_u32, (name, cpu_usage));
        
        if let Ok(content) = std::fs::read_to_string(format!("/proc/{}/status", pid_u32)) {
            for line in content.lines() {
                if line.starts_with("PPid:") {
                    if let Some(ppid) = line.split_whitespace().nth(1) {
                        let ppid_u32: u32 = ppid.parse().unwrap_or(0);
                        ppid_map.insert(pid_u32, ppid_u32);
                        children.entry(ppid_u32).or_insert_with(Vec::new).push(pid_u32);
                    }
                    break;
                }
            }
        }
    }
    
    let mut result = Vec::new();
    
    fn traverse(
        pid: u32,
        depth: usize,
        children: &HashMap<u32, Vec<u32>>,
        process_info: &HashMap<u32, (String, f32)>,
        ppid_map: &HashMap<u32, u32>,
        result: &mut Vec<ProcessTreeNode>,
    ) {
        if let Some((name, cpu_usage)) = process_info.get(&pid) {
            result.push(ProcessTreeNode {
                pid,
                ppid: *ppid_map.get(&pid).unwrap_or(&0),
                name: name.clone(),
                cpu_usage: *cpu_usage,
                depth,
            });
        }
        
        if let Some(child_pids) = children.get(&pid) {
            for &child_pid in child_pids {
                traverse(child_pid, depth + 1, &children, &process_info, &ppid_map, result);
            }
        }
    }
    
    traverse(1, 0, &children, &process_info, &ppid_map, &mut result);
    
    result.sort_by(|a, b| b.cpu_usage.partial_cmp(&a.cpu_usage).unwrap_or(std::cmp::Ordering::Equal));
    
    result.truncate(50);
    
    result
}
