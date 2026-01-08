//! Metric Display Component
//!
//! Dashboard metric component with auto-tracking, sparkline, delta, and threshold-based colors.
//!
//! Features:
//! - Value display with formatting
//! - Trend sparkline visualization
//! - Delta percentage calculation
//! - Threshold-based color coding
//! - Horizontal/vertical layouts

use crate::core::component::{VNode, BoxNode, BoxStyle, TextStyle, Color, NamedColor};
use crate::core::layout::FlexDirection;

// =============================================================================
// Types
// =============================================================================

/// Layout direction for metric display.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum MetricLayout {
    #[default]
    Horizontal,
    Vertical,
}

/// Size variant for metric display.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum MetricSize {
    Compact,
    #[default]
    Normal,
    Large,
}

/// Threshold range as (min, max) inclusive.
#[derive(Debug, Clone, Copy)]
pub struct ThresholdRange(pub f64, pub f64);

impl ThresholdRange {
    /// Check if value is within range.
    pub fn contains(&self, value: f64) -> bool {
        value >= self.0 && value <= self.1
    }
}

/// Threshold configuration for color mapping.
#[derive(Debug, Clone, Default)]
pub struct ThresholdConfig {
    /// Range for success/green coloring
    pub success: Option<ThresholdRange>,
    /// Range for warning/yellow coloring
    pub warning: Option<ThresholdRange>,
    /// Range for error/red coloring
    pub error: Option<ThresholdRange>,
    /// Range for info/blue coloring
    pub info: Option<ThresholdRange>,
}

impl ThresholdConfig {
    /// Create a new threshold config.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set success range.
    pub fn success(mut self, min: f64, max: f64) -> Self {
        self.success = Some(ThresholdRange(min, max));
        self
    }

    /// Set warning range.
    pub fn warning(mut self, min: f64, max: f64) -> Self {
        self.warning = Some(ThresholdRange(min, max));
        self
    }

    /// Set error range.
    pub fn error(mut self, min: f64, max: f64) -> Self {
        self.error = Some(ThresholdRange(min, max));
        self
    }

    /// Set info range.
    pub fn info(mut self, min: f64, max: f64) -> Self {
        self.info = Some(ThresholdRange(min, max));
        self
    }

    /// Get color for value based on thresholds.
    pub fn get_color(&self, value: f64) -> Option<Color> {
        if let Some(ref range) = self.success {
            if range.contains(value) {
                return Some(Color::Named(NamedColor::Green));
            }
        }
        if let Some(ref range) = self.warning {
            if range.contains(value) {
                return Some(Color::Named(NamedColor::Yellow));
            }
        }
        if let Some(ref range) = self.error {
            if range.contains(value) {
                return Some(Color::Named(NamedColor::Red));
            }
        }
        if let Some(ref range) = self.info {
            if range.contains(value) {
                return Some(Color::Named(NamedColor::Blue));
            }
        }
        None
    }
}

/// Get threshold color for a value.
pub fn get_threshold_color(value: f64, config: &ThresholdConfig) -> Option<Color> {
    config.get_color(value)
}

// =============================================================================
// Metric State
// =============================================================================

/// Metric state with auto-tracking.
#[derive(Debug, Clone)]
pub struct MetricState {
    /// Current value
    pub value: f64,
    /// Value history for sparkline
    pub history: Vec<f64>,
    /// Maximum history size
    pub history_size: usize,
    /// Metric label
    pub label: String,
    /// Unit suffix
    pub unit: String,
    /// Threshold configuration
    pub thresholds: Option<ThresholdConfig>,
}

impl Default for MetricState {
    fn default() -> Self {
        Self {
            value: 0.0,
            history: Vec::new(),
            history_size: 10,
            label: String::new(),
            unit: String::new(),
            thresholds: None,
        }
    }
}

impl MetricState {
    /// Create a new metric state.
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            ..Default::default()
        }
    }

    /// Set unit suffix.
    pub fn unit(mut self, unit: impl Into<String>) -> Self {
        self.unit = unit.into();
        self
    }

    /// Set initial value.
    pub fn initial(mut self, value: f64) -> Self {
        self.value = value;
        self.history.push(value);
        self
    }

    /// Set history size.
    pub fn history_size(mut self, size: usize) -> Self {
        self.history_size = size;
        self
    }

    /// Set threshold configuration.
    pub fn thresholds(mut self, config: ThresholdConfig) -> Self {
        self.thresholds = Some(config);
        self
    }

    /// Update value and track history.
    pub fn set(&mut self, value: f64) {
        self.value = value;
        self.history.push(value);
        if self.history.len() > self.history_size {
            self.history.remove(0);
        }
    }

    /// Get delta percentage from previous value.
    pub fn delta(&self) -> f64 {
        if self.history.len() < 2 {
            return 0.0;
        }
        let prev = self.history[self.history.len() - 2];
        let curr = self.history[self.history.len() - 1];
        if prev == 0.0 {
            if curr == 0.0 { 0.0 } else { 100.0 }
        } else {
            ((curr - prev) / prev.abs()) * 100.0
        }
    }

    /// Get color based on thresholds.
    pub fn color(&self) -> Option<Color> {
        self.thresholds.as_ref().and_then(|t| t.get_color(self.value))
    }
}

/// Create a new metric state.
pub fn create_metric(label: impl Into<String>) -> MetricState {
    MetricState::new(label)
}

// =============================================================================
// Sparkline Helper
// =============================================================================

/// Build a simple sparkline from data.
fn build_sparkline(data: &[f64], width: usize) -> String {
    if data.is_empty() {
        return " ".repeat(width);
    }

    let min = data.iter().cloned().fold(f64::INFINITY, f64::min);
    let max = data.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let range = if (max - min).abs() < f64::EPSILON { 1.0 } else { max - min };

    let blocks = ['▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];

    // Sample data to fit width
    let step = if data.len() > width {
        data.len() as f64 / width as f64
    } else {
        1.0
    };

    let mut result = String::new();
    let mut i = 0.0;
    while result.len() < width && (i as usize) < data.len() {
        let idx = i as usize;
        if idx < data.len() {
            let normalized = (data[idx] - min) / range;
            let block_idx = ((normalized * 7.0) as usize).min(7);
            result.push(blocks[block_idx]);
        }
        i += step;
    }

    // Pad if needed
    while result.len() < width {
        result.push(' ');
    }

    result
}

/// Format delta with arrow.
fn format_delta(delta: f64) -> String {
    if delta > 0.0 {
        format!("▲{:.1}%", delta)
    } else if delta < 0.0 {
        format!("▼{:.1}%", delta.abs())
    } else {
        "—".to_string()
    }
}

// =============================================================================
// Component
// =============================================================================

/// Metric display component.
#[derive(Debug, Clone)]
pub struct MetricDisplay {
    /// Metric state
    state: Option<MetricState>,
    /// Label (if not using state)
    label: Option<String>,
    /// Value (if not using state)
    value: Option<f64>,
    /// Unit (if not using state)
    unit: Option<String>,
    /// Trend data (if not using state)
    trend: Option<Vec<f64>>,
    /// Delta (if not using state)
    delta: Option<f64>,
    /// Thresholds (if not using state)
    thresholds: Option<ThresholdConfig>,
    /// Layout direction
    layout: MetricLayout,
    /// Size variant
    size: MetricSize,
    /// Width of sparkline
    trend_width: usize,
    /// Show trend sparkline
    show_trend: bool,
    /// Show delta indicator
    show_delta: bool,
}

impl Default for MetricDisplay {
    fn default() -> Self {
        Self {
            state: None,
            label: None,
            value: None,
            unit: None,
            trend: None,
            delta: None,
            thresholds: None,
            layout: MetricLayout::Horizontal,
            size: MetricSize::Normal,
            trend_width: 10,
            show_trend: true,
            show_delta: true,
        }
    }
}

impl MetricDisplay {
    /// Create a new metric display.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create with metric state.
    pub fn with_state(state: MetricState) -> Self {
        Self {
            state: Some(state),
            ..Default::default()
        }
    }

    /// Set metric state.
    pub fn state(mut self, state: MetricState) -> Self {
        self.state = Some(state);
        self
    }

    /// Set label.
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Set value.
    pub fn value(mut self, value: f64) -> Self {
        self.value = Some(value);
        self
    }

    /// Set unit.
    pub fn unit(mut self, unit: impl Into<String>) -> Self {
        self.unit = Some(unit.into());
        self
    }

    /// Set trend data.
    pub fn trend(mut self, data: Vec<f64>) -> Self {
        self.trend = Some(data);
        self
    }

    /// Set delta.
    pub fn delta(mut self, delta: f64) -> Self {
        self.delta = Some(delta);
        self
    }

    /// Set thresholds.
    pub fn thresholds(mut self, config: ThresholdConfig) -> Self {
        self.thresholds = Some(config);
        self
    }

    /// Set layout.
    pub fn layout(mut self, layout: MetricLayout) -> Self {
        self.layout = layout;
        self
    }

    /// Set size.
    pub fn size(mut self, size: MetricSize) -> Self {
        self.size = size;
        self
    }

    /// Set trend width.
    pub fn trend_width(mut self, width: usize) -> Self {
        self.trend_width = width;
        self
    }

    /// Show/hide trend.
    pub fn show_trend(mut self, show: bool) -> Self {
        self.show_trend = show;
        self
    }

    /// Show/hide delta.
    pub fn show_delta(mut self, show: bool) -> Self {
        self.show_delta = show;
        self
    }

    /// Build the VNode.
    pub fn build(self) -> VNode {
        // Resolve values from state or props
        let label = self.state.as_ref().map(|s| s.label.clone())
            .or(self.label)
            .unwrap_or_default();

        let value = self.state.as_ref().map(|s| s.value)
            .or(self.value)
            .unwrap_or(0.0);

        let unit = self.state.as_ref().map(|s| s.unit.clone())
            .or(self.unit)
            .unwrap_or_default();

        let trend = self.state.as_ref().map(|s| s.history.clone())
            .or(self.trend);

        let delta = self.delta.or_else(|| self.state.as_ref().map(|s| s.delta()));

        let thresholds = self.thresholds.or_else(|| self.state.as_ref().and_then(|s| s.thresholds.clone()));

        // Resolve color
        let value_color = thresholds.as_ref()
            .and_then(|t| t.get_color(value));

        // Delta color
        let delta_color = delta.map(|d| {
            if d > 0.0 {
                Color::Named(NamedColor::Green)
            } else if d < 0.0 {
                Color::Named(NamedColor::Red)
            } else {
                Color::Named(NamedColor::Gray)
            }
        });

        // Format value
        let formatted_value = if value.fract() == 0.0 {
            format!("{}{}", value as i64, unit)
        } else {
            format!("{:.1}{}", value, unit)
        };

        // Size-based styling
        let (label_style, value_style) = match self.size {
            MetricSize::Compact => (
                TextStyle { color: Some(Color::Named(NamedColor::Gray)), dim: true, ..Default::default() },
                TextStyle::default(),
            ),
            MetricSize::Normal => (
                TextStyle::default(),
                TextStyle::default(),
            ),
            MetricSize::Large => (
                TextStyle::default(),
                TextStyle { bold: true, ..Default::default() },
            ),
        };

        // Apply value color
        let value_text_style = TextStyle {
            color: value_color.or(value_style.color),
            bold: value_style.bold,
            ..Default::default()
        };

        match self.layout {
            MetricLayout::Vertical => {
                let mut children: Vec<VNode> = Vec::new();

                // Label
                children.push(VNode::styled_text(label, label_style));

                // Value row
                let mut value_row: Vec<VNode> = vec![
                    VNode::styled_text(formatted_value, value_text_style),
                ];

                if self.show_delta {
                    if let Some(d) = delta {
                        let delta_style = TextStyle {
                            color: delta_color,
                            ..Default::default()
                        };
                        value_row.push(VNode::styled_text(format!(" {}", format_delta(d)), delta_style));
                    }
                }

                children.push(VNode::Box(BoxNode {
                    children: value_row,
                    style: BoxStyle {
                        flex_direction: Some(FlexDirection::Row),
                        ..Default::default()
                    },
                    ..Default::default()
                }));

                // Trend sparkline
                if self.show_trend {
                    if let Some(ref t) = trend {
                        if t.len() > 1 {
                            let sparkline = build_sparkline(t, self.trend_width);
                            children.push(VNode::styled_text(
                                sparkline,
                                TextStyle {
                                    color: Some(Color::Named(NamedColor::Cyan)),
                                    ..Default::default()
                                },
                            ));
                        }
                    }
                }

                VNode::Box(BoxNode {
                    children,
                    style: BoxStyle {
                        flex_direction: Some(FlexDirection::Column),
                        ..Default::default()
                    },
                    ..Default::default()
                })
            }
            MetricLayout::Horizontal => {
                let mut children: Vec<VNode> = Vec::new();

                // Label
                children.push(VNode::styled_text(format!("{}:", label), label_style));
                children.push(VNode::text(" ".to_string()));

                // Value
                children.push(VNode::styled_text(formatted_value, value_text_style));

                // Delta
                if self.show_delta {
                    if let Some(d) = delta {
                        let delta_style = TextStyle {
                            color: delta_color,
                            ..Default::default()
                        };
                        children.push(VNode::text(" ".to_string()));
                        children.push(VNode::styled_text(format_delta(d), delta_style));
                    }
                }

                // Trend sparkline
                if self.show_trend {
                    if let Some(ref t) = trend {
                        if t.len() > 1 {
                            let sparkline = build_sparkline(t, self.trend_width);
                            children.push(VNode::text(" ".to_string()));
                            children.push(VNode::styled_text(
                                sparkline,
                                TextStyle {
                                    color: Some(Color::Named(NamedColor::Cyan)),
                                    ..Default::default()
                                },
                            ));
                        }
                    }
                }

                VNode::Box(BoxNode {
                    children,
                    style: BoxStyle {
                        flex_direction: Some(FlexDirection::Row),
                        ..Default::default()
                    },
                    ..Default::default()
                })
            }
        }
    }
}
