use crate::metrics::{ScheduledExport, SystemMetrics};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::sync::broadcast;
use tokio_cron_scheduler::{Job, JobScheduler};

#[derive(Clone)]
pub struct Scheduler {
    exports: Arc<Mutex<Vec<ScheduledExport>>>,
    scheduler: Arc<Mutex<Option<JobScheduler>>>,
}

impl Scheduler {
    pub fn new() -> Self {
        Self {
            exports: Arc::new(Mutex::new(Vec::new())),
            scheduler: Arc::new(Mutex::new(None)),
        }
    }

    pub async fn start(&self) -> Result<(), String> {
        let sched = JobScheduler::new()
            .await
            .map_err(|e| format!("Failed to create scheduler: {}", e))?;
        
        *self.scheduler.lock().unwrap() = Some(sched.clone());
        
        Ok(())
    }

    pub fn add_export(&self, export: ScheduledExport) {
        self.exports.lock().unwrap().push(export);
    }

    pub fn remove_export(&self, id: &str) -> bool {
        let mut exports = self.exports.lock().unwrap();
        let len_before = exports.len();
        exports.retain(|e| e.id != id);
        exports.len() < len_before
    }

    pub fn list_exports(&self) -> Vec<ScheduledExport> {
        self.exports.lock().unwrap().clone()
    }

    pub fn update_export(&self, export: ScheduledExport) -> bool {
        let mut exports = self.exports.lock().unwrap();
        if let Some(idx) = exports.iter().position(|e| e.id == export.id) {
            exports[idx] = export;
            true
        } else {
            false
        }
    }

    pub fn get_export(&self, id: &str) -> Option<ScheduledExport> {
        self.exports.lock().unwrap()
            .iter()
            .find(|e| e.id == id)
            .cloned()
    }
}

pub async fn execute_scheduled_export(
    export: &ScheduledExport,
    data: &[SystemMetrics],
    tx: &broadcast::Sender<SystemMetrics>,
) -> Result<String, String> {
    let content = match export.format.as_str() {
        "json" => crate::metrics::export::export_json(data),
        "csv" => crate::metrics::export::export_csv(data),
        "prometheus" => crate::metrics::export::export_prometheus(data),
        "influxdb" => crate::metrics::export::export_influxdb(data, "system_metrics"),
        "graphite" => crate::metrics::export::export_graphite(data, "system"),
        _ => return Err(format!("Unknown format: {}", export.format)),
    };

    Ok(content)
}

pub fn validate_cron_expression(expr: &str) -> bool {
    let parts: Vec<&str> = expr.split_whitespace().collect();
    parts.len() == 5
}

pub fn parse_cron_next_run(expr: &str, from_timestamp: i64) -> Option<i64> {
    if !validate_cron_expression(expr) {
        return None;
    }
    
    let parts: Vec<&str> = expr.split_whitespace().collect();
    let minute = parts[0].parse::<u32>().ok()?;
    let hour = parts[1].parse::<u32>().ok()?;
    
    if minute > 59 || hour > 23 {
        return None;
    }
    
    let base = chrono::DateTime::from_timestamp(from_timestamp, 0)?;
    let next = base + chrono::Duration::minutes(1);
    
    Some(next.timestamp())
}

#[derive(Clone)]
pub struct ExportHistory {
    entries: Arc<Mutex<Vec<ExportHistoryEntry>>>,
}

#[derive(Debug, Clone)]
pub struct ExportHistoryEntry {
    pub export_id: String,
    pub timestamp: i64,
    pub status: String,
    pub records_count: usize,
    pub file_size: usize,
    pub error: Option<String>,
}

impl ExportHistory {
    pub fn new() -> Self {
        Self {
            entries: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn add_entry(&self, entry: ExportHistoryEntry) {
        let mut entries = self.entries.lock().unwrap();
        entries.push(entry);
        if entries.len() > 1000 {
            entries.drain(0..100);
        }
    }

    pub fn get_entries(&self, limit: usize) -> Vec<ExportHistoryEntry> {
        let entries = self.entries.lock().unwrap();
        entries.iter()
            .rev()
            .take(limit)
            .cloned()
            .collect()
    }

    pub fn get_entries_by_export(&self, export_id: &str) -> Vec<ExportHistoryEntry> {
        let entries = self.entries.lock().unwrap();
        entries.iter()
            .filter(|e| e.export_id == export_id)
            .cloned()
            .collect()
    }
}

pub fn generate_export_id() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    format!("export_{}_{}", duration.as_secs(), duration.subsec_nanos())
}

pub fn estimate_export_size(data: &[SystemMetrics], format: &str) -> usize {
    let base_size = match format {
        "json" => 200,
        "csv" => 100,
        "prometheus" => 250,
        "influxdb" => 180,
        "graphite" => 150,
        _ => 200,
    };
    
    data.len() * base_size
}

pub fn validate_export_config(export: &ScheduledExport) -> Result<(), String> {
    if export.name.is_empty() {
        return Err("Export name cannot be empty".to_string());
    }
    
    if !validate_cron_expression(&export.cron_expression) {
        return Err("Invalid cron expression".to_string());
    }
    
    let valid_formats = ["json", "csv", "prometheus", "influxdb", "graphite"];
    if !valid_formats.contains(&export.format.as_str()) {
        return Err(format!("Invalid format: {}. Valid formats: {:?}", 
            export.format, valid_formats));
    }
    
    Ok(())
}
