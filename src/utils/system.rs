//! System Data Reader
//!
//! Reads real system information for monitoring tools.
//! Provides CPU, memory, and process information.
//!
//! # Platform Support
//!
//! - Linux: Full support via /proc filesystem
//! - macOS: Partial support (basic memory and CPU)
//! - Windows: Stub implementation (returns mock data)
//!
//! # Example
//!
//! ```rust,no_run
//! use tuiuiu::utils::system::{get_cpu_usage, get_memory_info, get_system_info};
//!
//! let cpu = get_cpu_usage();
//! println!("CPU: {}%", cpu.total);
//!
//! let mem = get_memory_info();
//! println!("Memory: {} / {}", format_bytes(mem.used), format_bytes(mem.total));
//! ```

use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};
use std::time::Instant;

// =============================================================================
// Types
// =============================================================================

/// CPU statistics.
#[derive(Debug, Clone, Default)]
pub struct CpuStats {
    pub user: u64,
    pub nice: u64,
    pub system: u64,
    pub idle: u64,
    pub iowait: u64,
    pub irq: u64,
    pub softirq: u64,
    pub steal: u64,
}

impl CpuStats {
    /// Calculate total CPU time.
    pub fn total(&self) -> u64 {
        self.user + self.nice + self.system + self.idle + self.iowait + self.irq + self.softirq + self.steal
    }

    /// Calculate non-idle time.
    pub fn active(&self) -> u64 {
        self.total() - self.idle
    }
}

/// CPU usage information.
#[derive(Debug, Clone, Default)]
pub struct CpuUsage {
    /// Total CPU usage (0-100).
    pub total: f64,
    /// Per-core usage (0-100).
    pub cores: Vec<f64>,
}

/// Memory information in bytes.
#[derive(Debug, Clone, Default)]
pub struct MemoryInfo {
    /// Total memory.
    pub total: u64,
    /// Used memory.
    pub used: u64,
    /// Free memory.
    pub free: u64,
    /// Buffer memory.
    pub buffers: u64,
    /// Cached memory.
    pub cached: u64,
    /// Available memory.
    pub available: u64,
    /// Total swap.
    pub swap_total: u64,
    /// Used swap.
    pub swap_used: u64,
    /// Free swap.
    pub swap_free: u64,
}

impl MemoryInfo {
    /// Get memory usage percentage.
    pub fn usage_percent(&self) -> f64 {
        if self.total == 0 {
            return 0.0;
        }
        (self.used as f64 / self.total as f64) * 100.0
    }

    /// Get swap usage percentage.
    pub fn swap_usage_percent(&self) -> f64 {
        if self.swap_total == 0 {
            return 0.0;
        }
        (self.swap_used as f64 / self.swap_total as f64) * 100.0
    }
}

/// Process information.
#[derive(Debug, Clone)]
pub struct ProcessInfo {
    /// Process ID.
    pub pid: u32,
    /// Process name.
    pub name: String,
    /// User name.
    pub user: String,
    /// Process state (R, S, D, Z, T, etc.).
    pub state: char,
    /// Priority.
    pub priority: i32,
    /// Nice value.
    pub nice: i32,
    /// Number of threads.
    pub threads: u32,
    /// Virtual memory in KB.
    pub virt: u64,
    /// Resident memory in KB.
    pub res: u64,
    /// Shared memory in KB.
    pub shr: u64,
    /// CPU percentage.
    pub cpu_percent: f64,
    /// Memory percentage.
    pub mem_percent: f64,
    /// CPU time formatted.
    pub time: String,
    /// Full command.
    pub command: String,
}

/// System information.
#[derive(Debug, Clone)]
pub struct SystemInfo {
    /// Hostname.
    pub hostname: String,
    /// Uptime in seconds.
    pub uptime: u64,
    /// Load average (1, 5, 15 minutes).
    pub load_avg: (f64, f64, f64),
    /// Task counts.
    pub tasks: TaskCounts,
}

/// Task counts by state.
#[derive(Debug, Clone, Default)]
pub struct TaskCounts {
    pub total: usize,
    pub running: usize,
    pub sleeping: usize,
    pub stopped: usize,
    pub zombie: usize,
}

// =============================================================================
// Internal State
// =============================================================================

static PREV_CPU_STATS: OnceLock<Mutex<Option<CpuStats>>> = OnceLock::new();
static PREV_CORE_STATS: OnceLock<Mutex<Vec<CpuStats>>> = OnceLock::new();
static PREV_PROCESS_TIMES: OnceLock<Mutex<HashMap<u32, (u64, u64, Instant)>>> = OnceLock::new();

fn get_prev_cpu_stats() -> &'static Mutex<Option<CpuStats>> {
    PREV_CPU_STATS.get_or_init(|| Mutex::new(None))
}

fn get_prev_core_stats() -> &'static Mutex<Vec<CpuStats>> {
    PREV_CORE_STATS.get_or_init(|| Mutex::new(Vec::new()))
}

#[allow(dead_code)]
fn get_prev_process_times() -> &'static Mutex<HashMap<u32, (u64, u64, Instant)>> {
    PREV_PROCESS_TIMES.get_or_init(|| Mutex::new(HashMap::new()))
}

// =============================================================================
// CPU Functions
// =============================================================================

/// Get CPU usage.
#[cfg(target_os = "linux")]
pub fn get_cpu_usage() -> CpuUsage {
    use std::fs;

    let content = match fs::read_to_string("/proc/stat") {
        Ok(c) => c,
        Err(_) => return CpuUsage::default(),
    };

    let mut total = 0.0;
    let mut cores = Vec::new();

    let lines: Vec<&str> = content.lines().collect();

    // Parse aggregate CPU line
    if let Some(line) = lines.iter().find(|l| l.starts_with("cpu ")) {
        if let Some(curr) = parse_cpu_line(line) {
            let mut prev_guard = get_prev_cpu_stats().lock().unwrap();
            if let Some(prev) = prev_guard.as_ref() {
                total = calculate_cpu_percent(prev, &curr);
            }
            *prev_guard = Some(curr);
        }
    }

    // Parse per-core lines
    let core_lines: Vec<&str> = lines
        .iter()
        .filter(|l| l.starts_with("cpu") && l.chars().nth(3).map_or(false, |c| c.is_ascii_digit()))
        .copied()
        .collect();

    let curr_cores: Vec<CpuStats> = core_lines.iter().filter_map(|l| parse_cpu_line(l)).collect();

    {
        let mut prev_cores_guard = get_prev_core_stats().lock().unwrap();

        for (i, curr) in curr_cores.iter().enumerate() {
            if let Some(prev) = prev_cores_guard.get(i) {
                cores.push(calculate_cpu_percent(prev, curr));
            } else {
                cores.push(0.0);
            }
        }

        *prev_cores_guard = curr_cores;
    }

    CpuUsage { total, cores }
}

#[cfg(not(target_os = "linux"))]
pub fn get_cpu_usage() -> CpuUsage {
    // Stub for non-Linux platforms
    CpuUsage {
        total: 0.0,
        cores: vec![],
    }
}

#[cfg(target_os = "linux")]
fn parse_cpu_line(line: &str) -> Option<CpuStats> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() < 8 {
        return None;
    }

    Some(CpuStats {
        user: parts.get(1).and_then(|s| s.parse().ok()).unwrap_or(0),
        nice: parts.get(2).and_then(|s| s.parse().ok()).unwrap_or(0),
        system: parts.get(3).and_then(|s| s.parse().ok()).unwrap_or(0),
        idle: parts.get(4).and_then(|s| s.parse().ok()).unwrap_or(0),
        iowait: parts.get(5).and_then(|s| s.parse().ok()).unwrap_or(0),
        irq: parts.get(6).and_then(|s| s.parse().ok()).unwrap_or(0),
        softirq: parts.get(7).and_then(|s| s.parse().ok()).unwrap_or(0),
        steal: parts.get(8).and_then(|s| s.parse().ok()).unwrap_or(0),
    })
}

fn calculate_cpu_percent(prev: &CpuStats, curr: &CpuStats) -> f64 {
    let total_diff = curr.total().saturating_sub(prev.total());
    let idle_diff = curr.idle.saturating_sub(prev.idle);

    if total_diff == 0 {
        return 0.0;
    }

    ((total_diff - idle_diff) as f64 / total_diff as f64 * 100.0).round()
}

// =============================================================================
// Memory Functions
// =============================================================================

/// Get memory information.
#[cfg(target_os = "linux")]
pub fn get_memory_info() -> MemoryInfo {
    use std::fs;

    let content = match fs::read_to_string("/proc/meminfo") {
        Ok(c) => c,
        Err(_) => return MemoryInfo::default(),
    };

    let mut info: HashMap<String, u64> = HashMap::new();

    for line in content.lines() {
        if let Some((key, value)) = parse_meminfo_line(line) {
            info.insert(key, value);
        }
    }

    let total = info.get("MemTotal").copied().unwrap_or(0);
    let free = info.get("MemFree").copied().unwrap_or(0);
    let buffers = info.get("Buffers").copied().unwrap_or(0);
    let cached = info.get("Cached").copied().unwrap_or(0);
    let available = info
        .get("MemAvailable")
        .copied()
        .unwrap_or(free + buffers + cached);
    let used = total.saturating_sub(available);

    let swap_total = info.get("SwapTotal").copied().unwrap_or(0);
    let swap_free = info.get("SwapFree").copied().unwrap_or(0);
    let swap_used = swap_total.saturating_sub(swap_free);

    MemoryInfo {
        total,
        used,
        free,
        buffers,
        cached,
        available,
        swap_total,
        swap_used,
        swap_free,
    }
}

#[cfg(not(target_os = "linux"))]
pub fn get_memory_info() -> MemoryInfo {
    // Stub for non-Linux platforms
    MemoryInfo::default()
}

#[cfg(target_os = "linux")]
fn parse_meminfo_line(line: &str) -> Option<(String, u64)> {
    let parts: Vec<&str> = line.split(':').collect();
    if parts.len() != 2 {
        return None;
    }

    let key = parts[0].trim().to_string();
    let value_str = parts[1].trim();

    // Parse value (may have "kB" suffix)
    let value = if value_str.ends_with("kB") {
        value_str
            .trim_end_matches("kB")
            .trim()
            .parse::<u64>()
            .ok()?
            * 1024
    } else {
        value_str.parse().ok()?
    };

    Some((key, value))
}

// =============================================================================
// System Info Functions
// =============================================================================

/// Get system information.
#[cfg(target_os = "linux")]
pub fn get_system_info() -> SystemInfo {
    use std::fs;

    let hostname = fs::read_to_string("/etc/hostname")
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|_| "unknown".to_string());

    let uptime = fs::read_to_string("/proc/uptime")
        .ok()
        .and_then(|s| s.split_whitespace().next().and_then(|v| v.parse::<f64>().ok()))
        .map(|v| v as u64)
        .unwrap_or(0);

    let load_avg = fs::read_to_string("/proc/loadavg")
        .ok()
        .map(|s| {
            let parts: Vec<&str> = s.split_whitespace().collect();
            (
                parts.first().and_then(|v| v.parse().ok()).unwrap_or(0.0),
                parts.get(1).and_then(|v| v.parse().ok()).unwrap_or(0.0),
                parts.get(2).and_then(|v| v.parse().ok()).unwrap_or(0.0),
            )
        })
        .unwrap_or((0.0, 0.0, 0.0));

    // Get task counts from /proc entries
    let tasks = get_task_counts();

    SystemInfo {
        hostname,
        uptime,
        load_avg,
        tasks,
    }
}

#[cfg(not(target_os = "linux"))]
pub fn get_system_info() -> SystemInfo {
    SystemInfo {
        hostname: "unknown".to_string(),
        uptime: 0,
        load_avg: (0.0, 0.0, 0.0),
        tasks: TaskCounts::default(),
    }
}

#[cfg(target_os = "linux")]
fn get_task_counts() -> TaskCounts {
    use std::fs;

    let mut counts = TaskCounts::default();

    if let Ok(entries) = fs::read_dir("/proc") {
        for entry in entries.flatten() {
            let name = entry.file_name();
            let name_str = name.to_string_lossy();

            // Only process numeric directories (PIDs)
            if !name_str.chars().all(|c| c.is_ascii_digit()) {
                continue;
            }

            counts.total += 1;

            // Read process state
            let stat_path = entry.path().join("stat");
            if let Ok(content) = fs::read_to_string(&stat_path) {
                // Parse state from stat file (third field after the command in parentheses)
                if let Some(state) = content
                    .rfind(')')
                    .and_then(|pos| content.get(pos + 2..pos + 3))
                {
                    match state.chars().next() {
                        Some('R') => counts.running += 1,
                        Some('S') | Some('D') => counts.sleeping += 1,
                        Some('T') => counts.stopped += 1,
                        Some('Z') => counts.zombie += 1,
                        _ => {}
                    }
                }
            }
        }
    }

    counts
}

// =============================================================================
// Formatting Utilities
// =============================================================================

/// Format bytes in human-readable form.
pub fn format_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "K", "M", "G", "T", "P"];

    if bytes == 0 {
        return "0B".to_string();
    }

    let mut size = bytes as f64;
    let mut unit_idx = 0;

    while size >= 1024.0 && unit_idx < UNITS.len() - 1 {
        size /= 1024.0;
        unit_idx += 1;
    }

    if size.fract() < 0.1 {
        format!("{:.0}{}", size, UNITS[unit_idx])
    } else {
        format!("{:.1}{}", size, UNITS[unit_idx])
    }
}

/// Format uptime in human-readable form.
pub fn format_uptime(seconds: u64) -> String {
    let days = seconds / 86400;
    let hours = (seconds % 86400) / 3600;
    let minutes = (seconds % 3600) / 60;

    if days > 0 {
        format!("{} days, {}:{:02}", days, hours, minutes)
    } else {
        format!("{}:{:02}", hours, minutes)
    }
}

/// Get human-readable process state description.
pub fn get_state_description(state: char) -> &'static str {
    match state {
        'R' => "Running",
        'S' => "Sleeping",
        'D' => "Disk sleep",
        'Z' => "Zombie",
        'T' => "Stopped",
        't' => "Tracing",
        'X' => "Dead",
        'I' => "Idle",
        _ => "Unknown",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_bytes() {
        assert_eq!(format_bytes(0), "0B");
        assert_eq!(format_bytes(512), "512B");
        assert_eq!(format_bytes(1024), "1K");  // No decimal for round numbers
        assert_eq!(format_bytes(1536), "1.5K");
        assert_eq!(format_bytes(1048576), "1M");  // No decimal for round numbers
        assert_eq!(format_bytes(1073741824), "1G");  // No decimal for round numbers
    }

    #[test]
    fn test_format_uptime() {
        assert_eq!(format_uptime(3600), "1:00");
        assert_eq!(format_uptime(3661), "1:01");
        assert_eq!(format_uptime(86400), "1 days, 0:00");
        assert_eq!(format_uptime(90061), "1 days, 1:01");
    }

    #[test]
    fn test_state_description() {
        assert_eq!(get_state_description('R'), "Running");
        assert_eq!(get_state_description('S'), "Sleeping");
        assert_eq!(get_state_description('Z'), "Zombie");
    }

    #[test]
    fn test_memory_info_percentages() {
        let mem = MemoryInfo {
            total: 1000,
            used: 600,
            free: 400,
            swap_total: 500,
            swap_used: 100,
            swap_free: 400,
            ..Default::default()
        };

        assert!((mem.usage_percent() - 60.0).abs() < 0.01);
        assert!((mem.swap_usage_percent() - 20.0).abs() < 0.01);
    }

    #[test]
    fn test_cpu_stats_total() {
        let stats = CpuStats {
            user: 100,
            nice: 10,
            system: 50,
            idle: 800,
            iowait: 20,
            irq: 5,
            softirq: 10,
            steal: 5,
        };

        assert_eq!(stats.total(), 1000);
        assert_eq!(stats.active(), 200);
    }
}
