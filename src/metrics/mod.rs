pub mod collector;
pub mod ring_buffer;
pub mod export;
pub mod analysis;
pub mod alerts;
pub mod scheduling;
pub mod storage;

pub use collector::{
    SystemMetrics, MetricsCollector, NetworkDetails, NetworkInterface, TcpStates,
    UdpEndpoints, ListeningPort, ConnectionInfo, BandwidthTotal, PacketCounts,
    ErrorCounts, DuplexInfo, WirelessInfo, CellularInfo, DnsStats, RouteEntry,
    ArpEntry, NetworkNamespace, SocketStats, ConnectionLimits, UdpEndpoint,
    CpuGovernor, ContextSwitches, Interrupts, Softirqs, SoftirqInfo,
    MemoryPressure, SwapRate, CpuStealTime, IoOperations, DiskIoStats,
    DiskQueueDepth, FilesystemStats, InodeUsage, OpenFilesCount, UptimeDetailed,
    LoadNormalized, PerProcessIo, MemoryZones, MemoryZoneInfo, HugePages,
    KernelThreads, KernelThreadInfo, UserThreads, ZombieProcesses, ZombieProcessInfo,
};
pub use ring_buffer::RingBuffer;

use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tokio::sync::broadcast;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertConfig {
    pub cpu_threshold: f32,
    pub memory_threshold: f32,
    pub disk_threshold: f32,
}

impl Default for AlertConfig {
    fn default() -> Self {
        Self {
            cpu_threshold: 80.0,
            memory_threshold: 85.0,
            disk_threshold: 90.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alert {
    pub alert_type: String,
    pub value: f32,
    pub threshold: f32,
    pub timestamp: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertResponse {
    pub alerts: Vec<Alert>,
    pub config: AlertConfig,
}

#[derive(Clone)]
pub struct AppState {
    pub tx: Arc<broadcast::Sender<SystemMetrics>>,
    pub alert_config: Arc<Mutex<AlertConfig>>,
    pub history: Arc<Mutex<RingBuffer<SystemMetrics>>>,
}

impl AppState {
    pub fn new() -> Self {
        let (tx, _) = broadcast::channel::<SystemMetrics>(100);
        Self {
            tx: Arc::new(tx),
            alert_config: Arc::new(Mutex::new(AlertConfig::default())),
            history: Arc::new(Mutex::new(RingBuffer::new(3600))),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeRange {
    pub start: i64,
    pub end: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregationResult {
    pub metric_name: String,
    pub count: usize,
    pub min: f64,
    pub max: f64,
    pub mean: f64,
    pub std_dev: f64,
    pub sum: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnomalyResult {
    pub timestamp: i64,
    pub metric_name: String,
    pub value: f64,
    pub expected: f64,
    pub deviation: f64,
    pub severity: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrendResult {
    pub metric_name: String,
    pub direction: String,
    pub slope: f64,
    pub correlation: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeakResult {
    pub timestamp: i64,
    pub metric_name: String,
    pub value: f64,
    pub peak_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduledExport {
    pub id: String,
    pub name: String,
    pub format: String,
    pub cron_expression: String,
    pub enabled: bool,
    pub destination: String,
    pub last_run: Option<i64>,
    pub next_run: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationConfig {
    pub email: Option<EmailConfig>,
    pub webhook: Option<WebhookConfig>,
    pub slack: Option<SlackConfig>,
    pub telegram: Option<TelegramConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailConfig {
    pub smtp_host: String,
    pub smtp_port: u16,
    pub username: String,
    pub password: String,
    pub from_addr: String,
    pub to_addrs: Vec<String>,
    pub use_tls: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookConfig {
    pub url: String,
    pub secret: Option<String>,
    pub headers: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlackConfig {
    pub webhook_url: String,
    pub channel: Option<String>,
    pub username: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelegramConfig {
    pub bot_token: String,
    pub chat_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataRetentionConfig {
    pub max_age_days: u32,
    pub max_records: usize,
    pub compression_enabled: bool,
    pub cleanup_interval_hours: u32,
}

impl Default for DataRetentionConfig {
    fn default() -> Self {
        Self {
            max_age_days: 30,
            max_records: 86400,
            compression_enabled: true,
            cleanup_interval_hours: 6,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComparisonResult {
    pub metrics: Vec<String>,
    pub correlations: std::collections::HashMap<String, f64>,
    pub insights: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportData {
    pub title: String,
    pub generated_at: i64,
    pub time_range: TimeRange,
    pub summary: std::collections::HashMap<String, AggregationResult>,
    pub anomalies: Vec<AnomalyResult>,
    pub trends: Vec<TrendResult>,
    pub peaks: Vec<PeakResult>,
    pub recommendations: Vec<String>,
}

// ============================================
// Advanced Analysis Features (50)
// ============================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictiveAnalysis {
    pub metric_name: String,
    pub current_value: f64,
    pub predicted_value: f64,
    pub prediction_horizon: String,
    pub confidence_interval: (f64, f64),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnomalyDetectionResult {
    pub timestamp: i64,
    pub metric_name: String,
    pub value: f64,
    pub expected_range: (f64, f64),
    pub anomaly_score: f64,
    pub algorithm: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaselineProfile {
    pub metric_name: String,
    pub baseline_mean: f64,
    pub baseline_std_dev: f64,
    pub baseline_min: f64,
    pub baseline_max: f64,
    pub samples_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrendForecast {
    pub metric_name: String,
    pub direction: String,
    pub slope: f64,
    pub intercept: f64,
    pub r_squared: f64,
    pub forecast_values: Vec<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapacityPlan {
    pub resource_type: String,
    pub current_usage: f64,
    pub projected_usage_30d: f64,
    pub projected_usage_90d: f64,
    pub capacity_limit: f64,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BottleneckAnalysis {
    pub resource_type: String,
    pub utilization_percent: f32,
    pub wait_time_ms: f64,
    pub impact_score: f64,
    pub suggestions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RootCauseAnalysis {
    pub event_time: i64,
    pub symptom: String,
    pub probable_causes: Vec<CauseCandidate>,
    pub correlation_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CauseCandidate {
    pub cause_type: String,
    pub description: String,
    pub probability: f64,
    pub evidence: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceScore {
    pub overall_score: f32,
    pub cpu_score: f32,
    pub memory_score: f32,
    pub disk_score: f32,
    pub network_score: f32,
    pub grade: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoricalComparison {
    pub metric_name: String,
    pub current_value: f64,
    pub previous_value: f64,
    pub change_percent: f64,
    pub period: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeakDetectionResult {
    pub timestamp: i64,
    pub metric_name: String,
    pub peak_value: f64,
    pub peak_type: String,
    pub duration_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeriodicityAnalysis {
    pub metric_name: String,
    pub period_seconds: u64,
    pub amplitude: f64,
    pub phase: f64,
    pub confidence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorrelationAnalysis {
    pub metric_a: String,
    pub metric_b: String,
    pub correlation_coefficient: f64,
    pub relationship_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImpactAssessment {
    pub affected_component: String,
    pub severity: String,
    pub user_impact: String,
    pub business_impact: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationRecommendation {
    pub category: String,
    pub description: String,
    pub expected_improvement: String,
    pub effort_level: String,
    pub priority: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoOptimizationAction {
    pub action_type: String,
    pub target: String,
    pub before_value: String,
    pub after_value: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceReservation {
    pub resource_type: String,
    pub reserved_amount: u64,
    pub used_amount: u64,
    pub utilization_percent: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostEstimate {
    pub resource_type: String,
    pub usage_amount: f64,
    pub unit_price: f64,
    pub total_cost: f64,
    pub currency: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SloTracker {
    pub slo_name: String,
    pub target_percent: f64,
    pub current_percent: f64,
    pub status: String,
    pub violations_24h: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlaReport {
    pub report_name: String,
    pub period: String,
    pub availability_percent: f64,
    pub incidents_count: u32,
    pub mttr_minutes: f64,
    pub mtbf_hours: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthScore {
    pub overall_health: u32,
    pub components: std::collections::HashMap<String, u32>,
    pub last_check: i64,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskAssessment {
    pub risk_id: String,
    pub risk_type: String,
    pub probability: f64,
    pub impact: String,
    pub mitigation: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VulnerabilityScanResult {
    pub vuln_id: String,
    pub severity: String,
    pub description: String,
    pub affected_component: String,
    pub remediation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigAuditResult {
    pub config_item: String,
    pub current_value: String,
    pub expected_value: String,
    pub compliant: bool,
    pub remediation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceCheck {
    pub standard: String,
    pub requirement: String,
    pub status: String,
    pub last_verified: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityAlert {
    pub alert_id: String,
    pub severity: String,
    pub source: String,
    pub description: String,
    pub timestamp: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntrusionDetectionEvent {
    pub event_id: String,
    pub event_type: String,
    pub source_ip: String,
    pub target: String,
    pub severity: String,
    pub timestamp: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrafficAnalysis {
    pub protocol: String,
    pub bytes_in: u64,
    pub bytes_out: u64,
    pub packets_in: u64,
    pub packets_out: u64,
    pub connections_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtocolAnalysis {
    pub protocol_name: String,
    pub request_count: u64,
    pub error_count: u64,
    pub avg_latency_ms: f64,
    pub throughput_mbps: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PacketCapture {
    pub capture_id: String,
    pub timestamp: i64,
    pub source_ip: String,
    pub dest_ip: String,
    pub protocol: String,
    pub payload_size: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionReplay {
    pub session_id: String,
    pub user_id: String,
    pub start_time: i64,
    pub end_time: i64,
    pub actions: Vec<SessionAction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionAction {
    pub timestamp: i64,
    pub action_type: String,
    pub details: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogAggregation {
    pub log_source: String,
    pub log_level: String,
    pub count: u64,
    pub messages: Vec<String>,
    pub time_range: TimeRange,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricCorrelation {
    pub metric_pairs: Vec<(String, String)>,
    pub correlation_matrix: Vec<Vec<f64>>,
    pub significant_correlations: Vec<(String, String, f64)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventCorrelation {
    pub event_group: Vec<EventInfo>,
    pub correlation_rule: String,
    pub score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventInfo {
    pub event_id: String,
    pub event_type: String,
    pub timestamp: i64,
    pub source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertAggregation {
    pub alert_group: Vec<AggregatedAlert>,
    pub aggregation_key: String,
    pub count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregatedAlert {
    pub alert_type: String,
    pub severity: String,
    pub first_occurrence: i64,
    pub last_occurrence: i64,
    pub affected_hosts: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertSuppression {
    pub suppressed_alert_id: String,
    pub suppression_rule: String,
    pub suppressed_until: i64,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertEscalation {
    pub original_alert_id: String,
    pub escalation_level: u32,
    pub escalated_to: String,
    pub escalation_time: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoRemediation {
    pub action_id: String,
    pub trigger_condition: String,
    pub action_taken: String,
    pub result: String,
    pub timestamp: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaybookExecution {
    pub playbook_id: String,
    pub playbook_name: String,
    pub status: String,
    pub started_at: i64,
    pub completed_at: Option<i64>,
    pub steps: Vec<PlaybookStep>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaybookStep {
    pub step_name: String,
    pub status: String,
    pub output: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeTracking {
    pub change_id: String,
    pub change_type: String,
    pub target_resource: String,
    pub change_details: String,
    pub changed_by: String,
    pub timestamp: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupStatus {
    pub backup_id: String,
    pub backup_type: String,
    pub status: String,
    pub start_time: i64,
    pub completion_time: Option<i64>,
    pub size_bytes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisasterRecoveryStatus {
    pub site: String,
    pub replication_status: String,
    pub rpo_minutes: u32,
    pub rto_minutes: u32,
    pub last_test: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailoverStatus {
    pub primary_site: String,
    pub secondary_site: String,
    pub failover_state: String,
    pub last_failover: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadBalancerStats {
    pub pool_name: String,
    pub active_connections: u32,
    pub total_connections: u64,
    pub servers_healthy: u32,
    pub servers_total: u32,
    pub throughput_mbps: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckResult {
    pub check_name: String,
    pub status: String,
    pub response_time_ms: f64,
    pub last_check: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanaryDeployment {
    pub deployment_id: String,
    pub version: String,
    pub traffic_percent: u32,
    pub error_rate: f64,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RollbackHistory {
    pub rollback_id: String,
    pub from_version: String,
    pub to_version: String,
    pub reason: String,
    pub timestamp: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigManagement {
    pub config_id: String,
    pub config_name: String,
    pub current_value: String,
    pub source: String,
    pub last_modified: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecretManagement {
    pub secret_name: String,
    pub secret_type: String,
    pub rotation_status: String,
    pub last_rotated: i64,
    pub expires_at: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificateManagement {
    pub cert_id: String,
    pub domain: String,
    pub issuer: String,
    pub valid_from: i64,
    pub valid_until: i64,
    pub status: String,
}

// ============================================
// Advanced UI Features (50) - UI metadata
// ============================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Visualization3DConfig {
    pub enabled: bool,
    pub chart_type: String,
    pub perspective: String,
    pub rotation_speed: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeatmapConfig {
    pub enabled: bool,
    pub color_scheme: String,
    pub cell_size: u32,
    pub interpolation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipGraphConfig {
    pub enabled: bool,
    pub layout: String,
    pub show_labels: bool,
    pub edge_style: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopologyMapConfig {
    pub enabled: bool,
    pub zoom_level: f32,
    pub center_coordinates: (f64, f64),
    pub show_legend: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardLayout {
    pub layout_id: String,
    pub name: String,
    pub widgets: Vec<WidgetConfig>,
    pub is_default: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WidgetConfig {
    pub widget_id: String,
    pub widget_type: String,
    pub position: WidgetPosition,
    pub size: WidgetSize,
    pub data_source: String,
    pub refresh_interval: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WidgetPosition {
    pub row: u32,
    pub column: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WidgetSize {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GanttChartConfig {
    pub enabled: bool,
    pub show_milestones: bool,
    pub show_progress: bool,
    pub date_format: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineViewConfig {
    pub enabled: bool,
    pub zoom_level: String,
    pub show_markers: bool,
    pub filter_categories: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalendarViewConfig {
    pub enabled: bool,
    pub view_type: String,
    pub show_events: bool,
    pub timezone: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapViewConfig {
    pub enabled: bool,
    pub map_provider: String,
    pub default_zoom: u32,
    pub show_markers: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollaborationSettings {
    pub real_time_collab_enabled: bool,
    pub show_cursors: bool,
    pub show_presence: bool,
    pub max_concurrent_users: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentSettings {
    pub comments_enabled: bool,
    pub allow_replies: bool,
    pub require_approval: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShareSettings {
    pub share_enabled: bool,
    pub default_permission: String,
    pub allow_public_links: bool,
    pub expiration_days: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportConfig {
    pub formats: Vec<String>,
    pub include_metadata: bool,
    pub compression_enabled: bool,
    pub custom_template: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdfExportConfig {
    pub page_size: String,
    pub orientation: String,
    pub include_charts: bool,
    pub header_text: Option<String>,
    pub footer_text: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrintViewConfig {
    pub optimized_for_print: bool,
    pub hide_navigation: bool,
    pub monochrome: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeSettings {
    pub current_theme: String,
    pub custom_theme: Option<String>,
    pub accent_color: String,
    pub font_family: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeEditorConfig {
    pub primary_color: String,
    pub secondary_color: String,
    pub background_color: String,
    pub text_color: String,
    pub border_radius: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationSettings {
    pub animations_enabled: bool,
    pub transition_duration_ms: u32,
    pub easing_function: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadingAnimationConfig {
    pub style: String,
    pub show_progress: bool,
    pub spinner_color: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmptyStateConfig {
    pub title: String,
    pub description: String,
    pub icon: String,
    pub show_action_button: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TutorialStep {
    pub step_number: u32,
    pub target_element: String,
    pub title: String,
    pub content: String,
    pub placement: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TooltipConfig {
    pub enabled: bool,
    pub delay_ms: u32,
    pub position: String,
    pub show_on_hover: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextMenuConfig {
    pub enabled: bool,
    pub items: Vec<ContextMenuItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextMenuItem {
    pub id: String,
    pub label: String,
    pub shortcut: Option<String>,
    pub action: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyboardShortcuts {
    pub shortcuts: std::collections::HashMap<String, String>,
    pub custom_shortcuts: Vec<CustomShortcut>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomShortcut {
    pub action: String,
    pub key_combination: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandPaletteConfig {
    pub enabled: bool,
    pub shortcut: String,
    pub max_recent_items: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchConfig {
    pub search_enabled: bool,
    pub search_sources: Vec<String>,
    pub fuzzy_search: bool,
    pub highlight_results: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterConfig {
    pub filters_enabled: bool,
    pub saved_filters: Vec<SavedFilter>,
    pub default_filter: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedFilter {
    pub filter_id: String,
    pub filter_name: String,
    pub filter_conditions: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SortConfig {
    pub default_sort_field: String,
    pub default_sort_direction: String,
    pub multi_sort_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationConfig {
    pub items_per_page: u32,
    pub show_page_numbers: bool,
    pub show_total_count: bool,
    pub infinite_scroll: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualScrollConfig {
    pub enabled: bool,
    pub item_height: u32,
    pub buffer_size: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DragDropConfig {
    pub drag_enabled: bool,
    pub drop_enabled: bool,
    pub ghost_preview: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResizeConfig {
    pub resizable_columns: bool,
    pub resizable_panels: bool,
    pub min_width: u32,
    pub min_height: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextMenuItemConfig {
    pub show_context_menu: bool,
    pub custom_items: Vec<ContextMenuItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FullscreenConfig {
    pub fullscreen_enabled: bool,
    pub remember_state: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PictureInPictureConfig {
    pub pip_enabled: bool,
    pub default_size: String,
    pub default_position: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponsiveLayoutConfig {
    pub breakpoints: Vec<BreakpointConfig>,
    pub mobile_first: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BreakpointConfig {
    pub name: String,
    pub min_width: u32,
    pub max_width: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MobileConfig {
    pub touch_optimized: bool,
    pub swipe_navigation: bool,
    pub pull_to_refresh: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct I18nConfig {
    pub default_locale: String,
    pub supported_locales: Vec<String>,
    pub fallback_locale: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceControlConfig {
    pub enabled: bool,
    pub language: String,
    pub continuous_mode: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GestureConfig {
    pub swipe_enabled: bool,
    pub pinch_zoom: bool,
    pub long_press: bool,
    pub double_tap: bool,
}

// ============================================
// Data Storage Features (50)
// ============================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SqliteStorageConfig {
    pub db_path: String,
    pub auto_vacuum: String,
    pub journal_mode: String,
    pub cache_size: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostgresStorageConfig {
    pub host: String,
    pub port: u16,
    pub database: String,
    pub username: String,
    pub connection_pool_size: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MysqlStorageConfig {
    pub host: String,
    pub port: u16,
    pub database: String,
    pub username: String,
    pub connection_pool_size: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimescaleDbConfig {
    pub host: String,
    pub port: u16,
    pub database: String,
    pub retention_policy: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClickHouseConfig {
    pub host: String,
    pub port: u16,
    pub database: String,
    pub cluster_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfluxDbConfig {
    pub url: String,
    pub org: String,
    pub bucket: String,
    pub token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrometheusConfig {
    pub url: String,
    pub remote_write_url: Option<String>,
    pub scrape_interval: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphiteConfig {
    pub host: String,
    pub port: u16,
    pub prefix: String,
    pub protocol: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElasticsearchConfig {
    pub url: String,
    pub index_prefix: String,
    pub username: Option<String>,
    pub shards: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LokiConfig {
    pub url: String,
    pub tenant_id: Option<String>,
    pub auth_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct S3Config {
    pub bucket: String,
    pub region: String,
    pub access_key: Option<String>,
    pub secret_key: Option<String>,
    pub endpoint: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GcsConfig {
    pub bucket: String,
    pub project_id: String,
    pub service_account_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AzureBlobConfig {
    pub connection_string: Option<String>,
    pub container_name: String,
    pub account_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AliyunOssConfig {
    pub bucket: String,
    pub endpoint: String,
    pub access_key_id: Option<String>,
    pub access_key_secret: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TencentCosConfig {
    pub bucket: String,
    pub region: String,
    pub secret_id: Option<String>,
    pub secret_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinioConfig {
    pub endpoint: String,
    pub access_key: Option<String>,
    pub secret_key: Option<String>,
    pub bucket: String,
    pub use_ssl: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataCompressionConfig {
    pub enabled: bool,
    pub algorithm: String,
    pub level: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataEncryptionConfig {
    pub enabled: bool,
    pub algorithm: String,
    pub key_management: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataBackupConfig {
    pub enabled: bool,
    pub schedule: String,
    pub retention_days: u32,
    pub destination: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataRecoveryConfig {
    pub recovery_point_objective_minutes: u32,
    pub recovery_time_objective_minutes: u32,
    pub last_recovery_point: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataMigrationConfig {
    pub source_type: String,
    pub target_type: String,
    pub batch_size: u32,
    pub parallel_workers: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataSyncConfig {
    pub enabled: bool,
    pub sync_interval_seconds: u32,
    pub bidirectional: bool,
    pub conflict_resolution: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataReplicationConfig {
    pub replication_factor: u32,
    pub sync_mode: String,
    pub consistency_level: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataShardingConfig {
    pub shard_key: String,
    pub num_shards: u32,
    pub shard_algorithm: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataArchiveConfig {
    pub enabled: bool,
    pub archive_after_days: u32,
    pub compression_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataLifecycleConfig {
    pub hot_storage_days: u32,
    pub warm_storage_days: u32,
    pub cold_storage_days: u32,
    pub delete_after_days: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataRetentionPolicy {
    pub metric_name: String,
    pub retention_days: u32,
    pub aggregation_level: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataCleanupConfig {
    pub auto_cleanup_enabled: bool,
    pub cleanup_schedule: String,
    pub max_age_hours: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataValidationConfig {
    pub validate_on_write: bool,
    pub validate_on_read: bool,
    pub strict_schema: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataAuditConfig {
    pub audit_enabled: bool,
    pub log_reads: bool,
    pub log_writes: bool,
    pub log_deletes: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataLineageRecord {
    pub record_id: String,
    pub source_system: String,
    pub transformation: String,
    pub timestamp: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataQualityCheck {
    pub check_name: String,
    pub check_type: String,
    pub status: String,
    pub last_run: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataCatalogEntry {
    pub table_name: String,
    pub description: String,
    pub columns: Vec<ColumnInfo>,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColumnInfo {
    pub name: String,
    pub data_type: String,
    pub nullable: bool,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataDictionaryEntry {
    pub field_name: String,
    pub field_type: String,
    pub allowed_values: Vec<String>,
    pub default_value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetadataConfig {
    pub collect_metadata: bool,
    pub metadata_sources: Vec<String>,
    pub update_interval_hours: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagManagementConfig {
    pub tag_taxonomy: String,
    pub auto_tagging_enabled: bool,
    pub tag_permissions: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionConfig {
    pub role_based_access: bool,
    pub permissions: Vec<PermissionEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionEntry {
    pub role: String,
    pub resource: String,
    pub actions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessControlConfig {
    pub authentication_method: String,
    pub authorization_model: String,
    pub session_timeout_minutes: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLogConfig {
    pub retention_days: u32,
    pub log_all_actions: bool,
    pub alert_on_suspicious: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationLogEntry {
    pub operation_id: String,
    pub operation_type: String,
    pub user: String,
    pub resource: String,
    pub timestamp: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionControlConfig {
    pub enabled: bool,
    pub max_versions: u32,
    pub auto_versioning: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotConfig {
    pub snapshot_enabled: bool,
    pub snapshot_schedule: String,
    pub retention_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RollbackConfig {
    pub allow_rollback: bool,
    pub rollback_timeout_hours: u32,
    pub require_approval: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiTenancyConfig {
    pub enabled: bool,
    pub isolation_level: String,
    pub share_resources: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataIsolationConfig {
    pub tenant_isolation_enabled: bool,
    pub shared_schema: bool,
    pub row_level_security: bool,
}
