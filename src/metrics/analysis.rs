use crate::metrics::{
    SystemMetrics, AggregationResult, AnomalyResult, TrendResult, PeakResult,
    TimeRange, ComparisonResult,
};
use std::collections::HashMap;

pub fn aggregate(data: &[SystemMetrics], metric_path: &str) -> Option<AggregationResult> {
    let values: Vec<f64> = data.iter().filter_map(|m| {
        extract_metric(m, metric_path)
    }).collect();
    
    if values.is_empty() {
        return None;
    }
    
    let count = values.len();
    let min = values.iter().cloned().fold(f64::INFINITY, f64::min);
    let max = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let sum: f64 = values.iter().sum();
    let mean = sum / count as f64;
    
    let variance: f64 = values.iter()
        .map(|&v| (v - mean).powi(2))
        .sum::<f64>() / count as f64;
    let std_dev = variance.sqrt();
    
    Some(AggregationResult {
        metric_name: metric_path.to_string(),
        count,
        min,
        max,
        mean,
        std_dev,
        sum,
    })
}

pub fn aggregate_all(data: &[SystemMetrics]) -> HashMap<String, AggregationResult> {
    let metric_paths = vec![
        "cpu.usage",
        "memory.usage_percent",
        "swap.usage_percent",
        "disk.read_rate",
        "disk.write_rate",
        "network.rx_rate",
        "network.tx_rate",
    ];
    
    let mut results = HashMap::new();
    for path in metric_paths {
        if let Some(agg) = aggregate(data, path) {
            results.insert(path.to_string(), agg);
        }
    }
    results
}

pub fn detect_anomalies(data: &[SystemMetrics], metric_path: &str, threshold_std: f64) -> Vec<AnomalyResult> {
    let values: Vec<(i64, f64)> = data.iter()
        .filter_map(|m| {
            extract_metric(m, metric_path).map(|v| (m.timestamp, v))
        })
        .collect();
    
    if values.len() < 3 {
        return Vec::new();
    }
    
    let mean = values.iter().map(|(_, v)| v).sum::<f64>() / values.len() as f64;
    let variance: f64 = values.iter()
        .map(|(_, v)| (v - mean).powi(2))
        .sum::<f64>() / values.len() as f64;
    let std_dev = variance.sqrt();
    
    let mut anomalies = Vec::new();
    
    for &(ts, value) in &values {
        let deviation = if std_dev > 0.0 {
            (value - mean).abs() / std_dev
        } else {
            0.0
        };
        
        if deviation > threshold_std {
            let severity = if deviation > threshold_std * 3.0 {
                "critical".to_string()
            } else if deviation > threshold_std * 2.0 {
                "high".to_string()
            } else {
                "medium".to_string()
            };
            
            anomalies.push(AnomalyResult {
                timestamp: ts,
                metric_name: metric_path.to_string(),
                value,
                expected: mean,
                deviation,
                severity,
            });
        }
    }
    
    anomalies
}

pub fn detect_all_anomalies(data: &[SystemMetrics], threshold_std: f64) -> Vec<AnomalyResult> {
    let metric_paths = vec![
        "cpu.usage",
        "memory.usage_percent",
        "disk.read_rate",
        "disk.write_rate",
        "network.rx_rate",
        "network.tx_rate",
    ];
    
    let mut all_anomalies = Vec::new();
    for path in metric_paths {
        all_anomalies.extend(detect_anomalies(data, path, threshold_std));
    }
    all_anomalies.sort_by_key(|a| a.timestamp);
    all_anomalies
}

pub fn analyze_trend(data: &[SystemMetrics], metric_path: &str) -> Option<TrendResult> {
    let points: Vec<(f64, f64)> = data.iter()
        .filter_map(|m| {
            extract_metric(m, metric_path).map(|v| (m.timestamp as f64, v))
        })
        .collect();
    
    if points.len() < 2 {
        return None;
    }
    
    let n = points.len() as f64;
    let sum_x: f64 = points.iter().map(|(x, _)| x).sum();
    let sum_y: f64 = points.iter().map(|(_, y)| y).sum();
    let sum_xy: f64 = points.iter().map(|(x, y)| x * y).sum();
    let sum_xx: f64 = points.iter().map(|(x, _)| x * x).sum();
    
    let denominator = n * sum_xx - sum_x * sum_x;
    if denominator.abs() < f64::EPSILON {
        return None;
    }
    
    let slope = (n * sum_xy - sum_x * sum_y) / denominator;
    let intercept = (sum_y - slope * sum_x) / n;
    
    let y_mean = sum_y / n;
    let ss_tot: f64 = points.iter()
        .map(|(_, y)| (y - y_mean).powi(2))
        .sum();
    let ss_res: f64 = points.iter()
        .map(|(x, y)| {
            let predicted = slope * x + intercept;
            (y - predicted).powi(2)
        })
        .sum();
    
    let correlation = if ss_tot > 0.0 {
        1.0 - (ss_res / ss_tot)
    } else {
        0.0
    };
    
    let direction = if slope > 0.1 {
        "increasing"
    } else if slope < -0.1 {
        "decreasing"
    } else {
        "stable"
    }.to_string();
    
    Some(TrendResult {
        metric_name: metric_path.to_string(),
        direction,
        slope,
        correlation,
    })
}

pub fn analyze_all_trends(data: &[SystemMetrics]) -> Vec<TrendResult> {
    let metric_paths = vec![
        "cpu.usage",
        "memory.usage_percent",
        "swap.usage_percent",
        "disk.read_rate",
        "disk.write_rate",
        "network.rx_rate",
        "network.tx_rate",
    ];
    
    metric_paths.iter()
        .filter_map(|path| analyze_trend(data, path))
        .collect()
}

pub fn detect_peaks(data: &[SystemMetrics], metric_path: &str, threshold: f64) -> Vec<PeakResult> {
    let points: Vec<(i64, f64)> = data.iter()
        .filter_map(|m| {
            extract_metric(m, metric_path).map(|v| (m.timestamp, v))
        })
        .collect();
    
    if points.len() < 3 {
        return Vec::new();
    }
    
    let mean: f64 = points.iter().map(|(_, v)| v).sum::<f64>() / points.len() as f64;
    let threshold_value = mean + threshold * mean.abs();
    
    let mut peaks = Vec::new();
    
    for i in 1..points.len() - 1 {
        let (_, prev) = points[i - 1];
        let (ts, curr) = points[i];
        let (_, next) = points[i + 1];
        
        if curr > prev && curr > next && curr > threshold_value {
            let peak_type = if curr > threshold_value * 1.5 {
                "sharp"
            } else {
                "gradual"
            }.to_string();
            
            peaks.push(PeakResult {
                timestamp: ts,
                metric_name: metric_path.to_string(),
                value: curr,
                peak_type,
            });
        }
    }
    
    peaks
}

pub fn detect_all_peaks(data: &[SystemMetrics], threshold: f64) -> Vec<PeakResult> {
    let metric_paths = vec![
        "cpu.usage",
        "memory.usage_percent",
        "disk.read_rate",
        "disk.write_rate",
        "network.rx_rate",
        "network.tx_rate",
    ];
    
    let mut all_peaks = Vec::new();
    for path in metric_paths {
        all_peaks.extend(detect_peaks(data, path, threshold));
    }
    all_peaks.sort_by_key(|p| p.timestamp);
    all_peaks
}

pub fn compare_metrics(data: &[SystemMetrics], metric_pairs: &[(String, String)]) -> ComparisonResult {
    let mut correlations = HashMap::new();
    let mut insights = Vec::new();
    let mut all_metrics = Vec::new();
    
    for (m1, m2) in metric_pairs {
        let values1: Vec<f64> = data.iter().filter_map(|m| extract_metric(m, m1)).collect();
        let values2: Vec<f64> = data.iter().filter_map(|m| extract_metric(m, m2)).collect();
        
        if values1.len() != values2.len() || values1.len() < 2 {
            continue;
        }
        
        all_metrics.push(m1.clone());
        all_metrics.push(m2.clone());
        
        let n = values1.len() as f64;
        let mean1 = values1.iter().sum::<f64>() / n;
        let mean2 = values2.iter().sum::<f64>() / n;
        
        let numerator: f64 = values1.iter()
            .zip(values2.iter())
            .map(|(v1, v2)| (v1 - mean1) * (v2 - mean2))
            .sum();
        
        let denom1: f64 = values1.iter().map(|v| (v - mean1).powi(2)).sum();
        let denom2: f64 = values2.iter().map(|v| (v - mean2).powi(2)).sum();
        let denominator = (denom1 * denom2).sqrt();
        
        let correlation = if denominator > 0.0 {
            numerator / denominator
        } else {
            0.0
        };
        
        let key = format!("{}_vs_{}", m1, m2);
        correlations.insert(key.clone(), correlation);
        
        if correlation.abs() > 0.7 {
            let relation = if correlation > 0.0 { "正相关" } else { "负相关" };
            insights.push(format!("{} 和 {} 呈现强{} (相关系数: {:.3})", m1, m2, relation, correlation));
        }
    }
    
    ComparisonResult {
        metrics: all_metrics,
        correlations,
        insights,
    }
}

fn extract_metric(m: &SystemMetrics, path: &str) -> Option<f64> {
    match path {
        "cpu.usage" => Some(m.cpu.usage as f64),
        "cpu.core_count" => Some(m.cpu.core_count as f64),
        "memory.usage_percent" => Some(m.memory.usage_percent as f64),
        "memory.total" => Some(m.memory.total as f64),
        "memory.used" => Some(m.memory.used as f64),
        "memory.available" => Some(m.memory.available as f64),
        "swap.usage_percent" => Some(m.swap.usage_percent as f64),
        "swap.total" => Some(m.swap.total as f64),
        "swap.used" => Some(m.swap.used as f64),
        "disk.read_rate" => Some(m.disk.read_rate as f64),
        "disk.write_rate" => Some(m.disk.write_rate as f64),
        "network.rx_rate" => Some(m.network.rx_rate as f64),
        "network.tx_rate" => Some(m.network.tx_rate as f64),
        _ => None,
    }
}

pub fn filter_by_time_range(data: &[SystemMetrics], range: &TimeRange) -> Vec<SystemMetrics> {
    data.iter()
        .filter(|m| m.timestamp >= range.start && m.timestamp <= range.end)
        .cloned()
        .collect()
}

pub fn generate_recommendations(data: &[SystemMetrics]) -> Vec<String> {
    let mut recommendations = Vec::new();
    
    if let Some(cpu_agg) = aggregate(data, "cpu.usage") {
        if cpu_agg.mean > 80.0 {
            recommendations.push("CPU 平均使用率较高 (>80%)，建议优化进程或升级硬件".to_string());
        }
        if cpu_agg.max > 95.0 {
            recommendations.push("检测到 CPU 峰值使用率 >95%，可能存在资源瓶颈".to_string());
        }
    }
    
    if let Some(mem_agg) = aggregate(data, "memory.usage_percent") {
        if mem_agg.mean > 85.0 {
            recommendations.push("内存平均使用率较高 (>85%)，建议增加内存或优化内存使用".to_string());
        }
        if mem_agg.max > 95.0 {
            recommendations.push("检测到内存峰值使用率 >95%，存在内存不足风险".to_string());
        }
    }
    
    if let Some(swap_agg) = aggregate(data, "swap.usage_percent") {
        if swap_agg.mean > 30.0 {
            recommendations.push("Swap 使用率较高，可能需要增加物理内存".to_string());
        }
    }
    
    let anomalies = detect_all_anomalies(data, 2.0);
    if anomalies.len() > 5 {
        recommendations.push(format!("检测到 {} 个异常数据点，建议检查系统稳定性", anomalies.len()));
    }
    
    let trends = analyze_all_trends(data);
    for trend in trends {
        if trend.metric_name == "memory.usage_percent" && trend.direction == "increasing" {
            recommendations.push("内存使用呈上升趋势，建议监控内存泄漏".to_string());
        }
    }
    
    if recommendations.is_empty() {
        recommendations.push("系统运行状态良好，未检测到明显问题".to_string());
    }
    
    recommendations
}
