use crate::metrics::{SystemMetrics, DataRetentionConfig};
use flate2::write::GzEncoder;
use flate2::read::GzDecoder;
use flate2::Compression;
use std::collections::VecDeque;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write, BufReader, BufWriter};
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct DataStorage {
    data_dir: PathBuf,
    backup_dir: PathBuf,
    retention_config: DataRetentionConfig,
    current_batch: Vec<SystemMetrics>,
    batch_size: usize,
}

impl DataStorage {
    pub fn new(data_dir: PathBuf, backup_dir: PathBuf) -> Self {
        std::fs::create_dir_all(&data_dir).ok();
        std::fs::create_dir_all(&backup_dir).ok();
        
        Self {
            data_dir,
            backup_dir,
            retention_config: DataRetentionConfig::default(),
            current_batch: Vec::new(),
            batch_size: 100,
        }
    }

    pub fn set_retention_config(&mut self, config: DataRetentionConfig) {
        self.retention_config = config;
    }

    pub fn add_metrics(&mut self, metrics: SystemMetrics) {
        self.current_batch.push(metrics);
        
        if self.current_batch.len() >= self.batch_size {
            self.flush_batch().ok();
        }
    }

    pub fn flush_batch(&mut self) -> Result<(), String> {
        if self.current_batch.is_empty() {
            return Ok(());
        }
        
        let filename = format!(
            "metrics_{}.json.gz",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs()
        );
        
        let filepath = self.data_dir.join(&filename);
        let file = File::create(&filepath)
            .map_err(|e| format!("Failed to create file: {}", e))?;
        
        let mut encoder = GzEncoder::new(BufWriter::new(file), Compression::default());
        
        let json = serde_json::to_vec(&self.current_batch)
            .map_err(|e| format!("Failed to serialize: {}", e))?;
        
        encoder.write_all(&json)
            .map_err(|e| format!("Failed to write compressed data: {}", e))?;
        
        encoder.finish()
            .map_err(|e| format!("Failed to finish compression: {}", e))?;
        
        self.current_batch.clear();
        
        Ok(())
    }

    pub fn load_metrics(&self, count: usize) -> Vec<SystemMetrics> {
        let mut all_metrics = Vec::new();
        
        if let Ok(entries) = std::fs::read_dir(&self.data_dir) {
            let mut files: Vec<_> = entries
                .filter_map(|e| e.ok())
                .filter(|e| e.path().extension().map_or(false, |ext| ext == "gz"))
                .collect();
            
            files.sort_by_key(|e| std::cmp::Reverse(e.path()));
            
            for entry in files.iter().take(10) {
                if all_metrics.len() >= count {
                    break;
                }
                
                if let Ok(file) = File::open(entry.path()) {
                    let reader = BufReader::new(file);
                    let mut decoder = GzDecoder::new(reader);
                    let mut contents = String::new();
                    
                    if decoder.read_to_string(&mut contents).is_ok() {
                        if let Ok(metrics) = serde_json::from_str::<Vec<SystemMetrics>>(&contents) {
                            all_metrics.extend(metrics);
                        }
                    }
                }
            }
        }
        
        all_metrics.truncate(count);
        all_metrics
    }

    pub fn cleanup_old_data(&self) -> Result<usize, String> {
        let max_age = self.retention_config.max_age_days as i64 * 24 * 60 * 60;
        let cutoff = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64 - max_age;
        
        let mut removed_count = 0;
        
        if let Ok(entries) = std::fs::read_dir(&self.data_dir) {
            for entry in entries.filter_map(|e| e.ok()) {
                let path = entry.path();
                if let Ok(metadata) = entry.metadata() {
                    if let Ok(modified) = metadata.modified() {
                        let modified_secs = modified
                            .duration_since(UNIX_EPOCH)
                            .unwrap_or_default()
                            .as_secs() as i64;
                        
                        if modified_secs < cutoff {
                            if std::fs::remove_file(path).is_ok() {
                                removed_count += 1;
                            }
                        }
                    }
                }
            }
        }
        
        Ok(removed_count)
    }

    pub fn compress_file(&self, filepath: &PathBuf) -> Result<PathBuf, String> {
        let input = File::open(filepath)
            .map_err(|e| format!("Failed to open file: {}", e))?;
        
        let output_path = filepath.with_extension("gz");
        let output = File::create(&output_path)
            .map_err(|e| format!("Failed to create output file: {}", e))?;
        
        let mut encoder = GzEncoder::new(BufWriter::new(output), Compression::best());
        let mut reader = BufReader::new(input);
        
        std::io::copy(&mut reader, &mut encoder)
            .map_err(|e| format!("Failed to compress: {}", e))?;
        
        encoder.finish()
            .map_err(|e| format!("Failed to finalize compression: {}", e))?;
        
        Ok(output_path)
    }

    pub fn decompress_file(&self, filepath: &PathBuf) -> Result<String, String> {
        let file = File::open(filepath)
            .map_err(|e| format!("Failed to open file: {}", e))?;
        
        let mut decoder = GzDecoder::new(BufReader::new(file));
        let mut contents = String::new();
        
        decoder.read_to_string(&mut contents)
            .map_err(|e| format!("Failed to decompress: {}", e))?;
        
        Ok(contents)
    }
}

pub struct IncrementalBackup {
    backup_path: PathBuf,
    last_backup_time: Option<i64>,
    last_backup_hash: Option<u64>,
}

impl IncrementalBackup {
    pub fn new(backup_path: PathBuf) -> Self {
        std::fs::create_dir_all(&backup_path).ok();
        
        Self {
            backup_path,
            last_backup_time: None,
            last_backup_hash: None,
        }
    }

    pub fn create_backup(&mut self, data: &[SystemMetrics], compression: bool) -> Result<String, String> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        let backup_name = if compression {
            format!("backup_{}.json.gz", timestamp)
        } else {
            format!("backup_{}.json", timestamp)
        };
        
        let backup_path = self.backup_path.join(&backup_name);
        
        let json = serde_json::to_vec(data)
            .map_err(|e| format!("Failed to serialize: {}", e))?;
        
        let file = File::create(&backup_path)
            .map_err(|e| format!("Failed to create backup file: {}", e))?;
        
        if compression {
            let mut encoder = GzEncoder::new(BufWriter::new(file), Compression::default());
            encoder.write_all(&json)
                .map_err(|e| format!("Failed to write compressed backup: {}", e))?;
            encoder.finish()
                .map_err(|e| format!("Failed to finalize backup: {}", e))?;
        } else {
            let mut writer = BufWriter::new(file);
            writer.write_all(&json)
                .map_err(|e| format!("Failed to write backup: {}", e))?;
        }
        
        let hash = simple_hash(&json);
        self.last_backup_time = Some(timestamp as i64);
        self.last_backup_hash = Some(hash);
        
        Ok(backup_name)
    }

    pub fn get_incremental_changes(&self, old_data: &[SystemMetrics], new_data: &[SystemMetrics]) -> Vec<SystemMetrics> {
        let old_hashes: std::collections::HashSet<u64> = old_data.iter()
            .map(|m| simple_hash(&serde_json::to_vec(m).unwrap_or_default()))
            .collect();
        
        new_data.iter()
            .filter(|m| {
                let hash = simple_hash(&serde_json::to_vec(m).unwrap_or_default());
                !old_hashes.contains(&hash)
            })
            .cloned()
            .collect()
    }

    pub fn restore_backup(&self, backup_name: &str) -> Result<Vec<SystemMetrics>, String> {
        let backup_path = self.backup_path.join(backup_name);
        
        if !backup_path.exists() {
            return Err(format!("Backup file not found: {}", backup_name));
        }
        
        let contents = if backup_name.ends_with(".gz") {
            let file = File::open(&backup_path)
                .map_err(|e| format!("Failed to open backup: {}", e))?;
            let mut decoder = GzDecoder::new(BufReader::new(file));
            let mut contents = String::new();
            decoder.read_to_string(&mut contents)
                .map_err(|e| format!("Failed to decompress: {}", e))?;
            contents
        } else {
            std::fs::read_to_string(&backup_path)
                .map_err(|e| format!("Failed to read backup: {}", e))?
        };
        
        serde_json::from_str(&contents)
            .map_err(|e| format!("Failed to parse backup: {}", e))
    }

    pub fn list_backups(&self) -> Vec<BackupInfo> {
        let mut backups = Vec::new();
        
        if let Ok(entries) = std::fs::read_dir(&self.backup_path) {
            for entry in entries.filter_map(|e| e.ok()) {
                let path = entry.path();
                if let Ok(metadata) = entry.metadata() {
                    let name = path.file_name()
                        .map(|n| n.to_string_lossy().to_string())
                        .unwrap_or_default();
                    
                    let modified = metadata.modified()
                        .ok()
                        .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
                        .map(|d| d.as_secs() as i64);
                    
                    backups.push(BackupInfo {
                        name,
                        size: metadata.len(),
                        modified,
                    });
                }
            }
        }
        
        backups.sort_by_key(|b| std::cmp::Reverse(b.modified));
        backups
    }
}

#[derive(Debug, Clone)]
pub struct BackupInfo {
    pub name: String,
    pub size: u64,
    pub modified: Option<i64>,
}

fn simple_hash(data: &[u8]) -> u64 {
    let mut hash: u64 = 0;
    for (i, &byte) in data.iter().enumerate() {
        hash = hash.wrapping_add((byte as u64).wrapping_mul((i as u64).wrapping_add(1)));
        hash = hash.rotate_left(5);
    }
    hash
}

pub struct DataRetention {
    config: DataRetentionConfig,
    last_cleanup: Option<i64>,
}

impl DataRetention {
    pub fn new(config: DataRetentionConfig) -> Self {
        Self {
            config,
            last_cleanup: None,
        }
    }

    pub fn should_cleanup(&self) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64;
        
        match self.last_cleanup {
            Some(last) => now - last >= self.config.cleanup_interval_hours as i64 * 3600,
            None => true,
        }
    }

    pub fn update_cleanup_time(&mut self) {
        self.last_cleanup = Some(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs() as i64
        );
    }

    pub fn filter_by_retention(&self, data: &[SystemMetrics]) -> Vec<SystemMetrics> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64;
        
        let cutoff = now - (self.config.max_age_days as i64 * 24 * 60 * 60);
        
        let mut filtered: Vec<_> = data.iter()
            .filter(|m| m.timestamp >= cutoff)
            .cloned()
            .collect();
        
        if filtered.len() > self.config.max_records {
            filtered.truncate(self.config.max_records);
        }
        
        filtered
    }

    pub fn get_storage_stats(&self, data_dir: &PathBuf) -> StorageStats {
        let mut total_size: u64 = 0;
        let mut file_count = 0;
        let mut compressed_count = 0;
        
        if let Ok(entries) = std::fs::read_dir(data_dir) {
            for entry in entries.filter_map(|e| e.ok()) {
                if let Ok(metadata) = entry.metadata() {
                    total_size += metadata.len();
                    file_count += 1;
                    
                    if entry.path().extension().map_or(false, |ext| ext == "gz") {
                        compressed_count += 1;
                    }
                }
            }
        }
        
        StorageStats {
            total_size,
            file_count,
            compressed_count,
            compression_ratio: if file_count > 0 {
                compressed_count as f64 / file_count as f64
            } else {
                0.0
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct StorageStats {
    pub total_size: u64,
    pub file_count: usize,
    pub compressed_count: usize,
    pub compression_ratio: f64,
}
