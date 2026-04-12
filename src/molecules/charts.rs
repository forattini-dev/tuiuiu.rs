//! Chart Components
//!
//! Data visualization components for terminal.

use crate::core::component::{BoxNode, BoxStyle, Color, NamedColor, TextStyle, VNode};

// =============================================================================
// Sparkline
// =============================================================================

/// Sparkline - mini line chart.
#[derive(Debug, Clone)]
pub struct Sparkline {
    data: Vec<f64>,
    width: Option<u16>,
    height: u16,
    color: Color,
    show_min_max: bool,
    label: Option<String>,
}

impl Default for Sparkline {
    fn default() -> Self {
        Self {
            data: Vec::new(),
            width: None,
            height: 1,
            color: Color::Named(NamedColor::Green),
            show_min_max: false,
            label: None,
        }
    }
}

impl Sparkline {
    /// Create a new sparkline.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set data points.
    pub fn data<I>(mut self, data: I) -> Self
    where
        I: IntoIterator<Item = f64>,
    {
        self.data = data.into_iter().collect();
        self
    }

    /// Set data from integers.
    pub fn data_i32<I>(mut self, data: I) -> Self
    where
        I: IntoIterator<Item = i32>,
    {
        self.data = data.into_iter().map(|x| x as f64).collect();
        self
    }

    /// Set width.
    pub fn width(mut self, width: u16) -> Self {
        self.width = Some(width);
        self
    }

    /// Set height (number of rows).
    pub fn height(mut self, height: u16) -> Self {
        self.height = height;
        self
    }

    /// Set color.
    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    /// Show min/max values.
    pub fn show_min_max(mut self) -> Self {
        self.show_min_max = true;
        self
    }

    /// Set label.
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Build the VNode.
    pub fn build(self) -> VNode {
        if self.data.is_empty() {
            return VNode::styled_text("No data", TextStyle::color(Color::Named(NamedColor::Gray)));
        }

        // Sparkline characters (from low to high)
        let chars = ['▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];

        let min = self.data.iter().copied().fold(f64::INFINITY, f64::min);
        let max = self.data.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        let range = max - min;

        let width = self.width.unwrap_or(self.data.len() as u16);
        let data_len = self.data.len();

        // Resample data if needed
        let samples: Vec<f64> = if data_len == width as usize {
            self.data.clone()
        } else {
            (0..width as usize)
                .map(|i| {
                    let idx = (i * data_len) / width as usize;
                    self.data.get(idx).copied().unwrap_or(0.0)
                })
                .collect()
        };

        // Convert to sparkline characters
        let sparkline: String = samples
            .iter()
            .map(|&v| {
                if range == 0.0 {
                    chars[4]
                } else {
                    let normalized = ((v - min) / range * 7.0) as usize;
                    chars[normalized.min(7)]
                }
            })
            .collect();

        let mut children = Vec::new();

        if let Some(label) = &self.label {
            children.push(VNode::styled_text(label.clone(), TextStyle::bold()));
        }

        children.push(VNode::styled_text(sparkline, TextStyle::color(self.color)));

        if self.show_min_max {
            children.push(VNode::styled_text(
                format!("min: {:.1} max: {:.1}", min, max),
                TextStyle {
                    color: Some(Color::Named(NamedColor::Gray)),
                    dim: true,
                    ..Default::default()
                },
            ));
        }

        VNode::Box(BoxNode {
            children,
            style: BoxStyle::default(),
            ..Default::default()
        })
    }
}

// =============================================================================
// BarChart
// =============================================================================

/// Bar orientation.
#[derive(Debug, Clone, Copy, Default)]
pub enum BarOrientation {
    #[default]
    Horizontal,
    Vertical,
}

/// A single bar item.
#[derive(Debug, Clone)]
pub struct BarItem {
    /// Label
    pub label: String,
    /// Value
    pub value: f64,
    /// Color override
    pub color: Option<Color>,
}

impl BarItem {
    /// Create a new bar item.
    pub fn new(label: impl Into<String>, value: f64) -> Self {
        Self {
            label: label.into(),
            value,
            color: None,
        }
    }

    /// Set color.
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }
}

/// Bar chart component.
#[derive(Debug, Clone)]
pub struct BarChart {
    items: Vec<BarItem>,
    width: u16,
    bar_width: u16,
    orientation: BarOrientation,
    color: Color,
    show_values: bool,
    max_value: Option<f64>,
}

impl Default for BarChart {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            width: 40,
            bar_width: 1,
            orientation: BarOrientation::Horizontal,
            color: Color::Named(NamedColor::Cyan),
            show_values: true,
            max_value: None,
        }
    }
}

impl BarChart {
    /// Create a new bar chart.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set bar items.
    pub fn items<I>(mut self, items: I) -> Self
    where
        I: IntoIterator<Item = BarItem>,
    {
        self.items = items.into_iter().collect();
        self
    }

    /// Set data from label-value pairs.
    pub fn data<I, S>(mut self, data: I) -> Self
    where
        I: IntoIterator<Item = (S, f64)>,
        S: Into<String>,
    {
        self.items = data.into_iter().map(|(l, v)| BarItem::new(l, v)).collect();
        self
    }

    /// Set chart width.
    pub fn width(mut self, width: u16) -> Self {
        self.width = width;
        self
    }

    /// Set bar width (height in horizontal mode).
    pub fn bar_width(mut self, width: u16) -> Self {
        self.bar_width = width;
        self
    }

    /// Set orientation.
    pub fn orientation(mut self, orientation: BarOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    /// Use horizontal bars.
    pub fn horizontal(self) -> Self {
        self.orientation(BarOrientation::Horizontal)
    }

    /// Use vertical bars.
    pub fn vertical(self) -> Self {
        self.orientation(BarOrientation::Vertical)
    }

    /// Set default color.
    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    /// Show values on bars.
    pub fn show_values(mut self, show: bool) -> Self {
        self.show_values = show;
        self
    }

    /// Set maximum value (for scaling).
    pub fn max(mut self, max: f64) -> Self {
        self.max_value = Some(max);
        self
    }

    /// Build the VNode.
    pub fn build(self) -> VNode {
        if self.items.is_empty() {
            return VNode::styled_text("No data", TextStyle::color(Color::Named(NamedColor::Gray)));
        }

        let max_value = self
            .max_value
            .unwrap_or_else(|| self.items.iter().map(|i| i.value).fold(0.0, f64::max));

        let max_label_len = self.items.iter().map(|i| i.label.len()).max().unwrap_or(0);

        let mut children = Vec::new();

        match self.orientation {
            BarOrientation::Horizontal => {
                for item in &self.items {
                    let bar_len = if max_value > 0.0 {
                        ((item.value / max_value)
                            * (self.width as f64 - max_label_len as f64 - 4.0))
                            as usize
                    } else {
                        0
                    };

                    let bar = "█".repeat(bar_len);
                    let color = item.color.unwrap_or(self.color);

                    let value_str = if self.show_values {
                        format!(" {:.1}", item.value)
                    } else {
                        String::new()
                    };

                    let line = format!(
                        "{:>width$} │{}{}",
                        item.label,
                        bar,
                        value_str,
                        width = max_label_len
                    );

                    children.push(VNode::styled_text(line, TextStyle::color(color)));
                }
            }
            BarOrientation::Vertical => {
                // Vertical bars - more complex, using block characters
                let height = 8;
                let bar_chars = ['▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];

                for row in (0..height).rev() {
                    let threshold = (row as f64 + 1.0) / height as f64;

                    let mut row_text = String::new();
                    for item in &self.items {
                        let normalized = if max_value > 0.0 {
                            item.value / max_value
                        } else {
                            0.0
                        };

                        let char = if normalized >= threshold {
                            '█'
                        } else if normalized > threshold - (1.0 / height as f64) {
                            let partial = ((normalized - (threshold - 1.0 / height as f64))
                                * height as f64
                                * 8.0) as usize;
                            bar_chars[partial.min(7)]
                        } else {
                            ' '
                        };

                        row_text.push(char);
                        row_text.push(' ');
                    }

                    children.push(VNode::styled_text(row_text, TextStyle::color(self.color)));
                }

                // Labels
                let labels: String = self
                    .items
                    .iter()
                    .map(|i| format!("{:.1}", i.label.chars().next().unwrap_or(' ')))
                    .collect::<Vec<_>>()
                    .join(" ");

                children.push(VNode::styled_text(
                    labels,
                    TextStyle::color(Color::Named(NamedColor::Gray)),
                ));
            }
        }

        VNode::Box(BoxNode {
            children,
            style: BoxStyle::default(),
            ..Default::default()
        })
    }
}

// =============================================================================
// Gauge
// =============================================================================

/// Gauge style.
#[derive(Debug, Clone, Copy, Default)]
pub enum GaugeStyle {
    #[default]
    Bar,
    Arc,
    Circle,
}

/// Gauge component - shows a value as a percentage.
#[derive(Debug, Clone)]
pub struct Gauge {
    value: f64,
    max: f64,
    min: f64,
    width: u16,
    style: GaugeStyle,
    label: Option<String>,
    show_percentage: bool,
    color: Color,
    #[allow(dead_code)]
    background_color: Color,
    thresholds: Vec<(f64, Color)>,
}

impl Default for Gauge {
    fn default() -> Self {
        Self {
            value: 0.0,
            max: 100.0,
            min: 0.0,
            width: 20,
            style: GaugeStyle::Bar,
            label: None,
            show_percentage: true,
            color: Color::Named(NamedColor::Green),
            background_color: Color::Named(NamedColor::BrightBlack),
            thresholds: Vec::new(),
        }
    }
}

impl Gauge {
    /// Create a new gauge.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set value.
    pub fn value(mut self, value: f64) -> Self {
        self.value = value;
        self
    }

    /// Set maximum.
    pub fn max(mut self, max: f64) -> Self {
        self.max = max;
        self
    }

    /// Set minimum.
    pub fn min(mut self, min: f64) -> Self {
        self.min = min;
        self
    }

    /// Set width.
    pub fn width(mut self, width: u16) -> Self {
        self.width = width;
        self
    }

    /// Set style.
    pub fn style(mut self, style: GaugeStyle) -> Self {
        self.style = style;
        self
    }

    /// Set label.
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Show percentage.
    pub fn show_percentage(mut self, show: bool) -> Self {
        self.show_percentage = show;
        self
    }

    /// Set color.
    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    /// Add color threshold.
    pub fn threshold(mut self, value: f64, color: Color) -> Self {
        self.thresholds.push((value, color));
        self.thresholds
            .sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        self
    }

    /// Add common thresholds (green/yellow/red).
    pub fn traffic_light(self) -> Self {
        self.threshold(50.0, Color::Named(NamedColor::Green))
            .threshold(75.0, Color::Named(NamedColor::Yellow))
            .threshold(90.0, Color::Named(NamedColor::Red))
    }

    /// Get color based on value and thresholds.
    fn get_color(&self) -> Color {
        for (threshold, color) in self.thresholds.iter().rev() {
            if self.value >= *threshold {
                return *color;
            }
        }
        self.color
    }

    /// Build the VNode.
    pub fn build(self) -> VNode {
        let range = self.max - self.min;
        let percentage = if range > 0.0 {
            ((self.value - self.min) / range * 100.0).clamp(0.0, 100.0)
        } else {
            0.0
        };

        let filled = ((percentage / 100.0) * self.width as f64) as usize;
        let empty = self.width as usize - filled;

        let color = self.get_color();

        let bar = match self.style {
            GaugeStyle::Bar => {
                let filled_str = "█".repeat(filled);
                let empty_str = "░".repeat(empty);
                format!("{}{}", filled_str, empty_str)
            }
            GaugeStyle::Arc => {
                // Simple arc representation
                let chars: Vec<char> = (0..self.width)
                    .map(|i| {
                        if (i as f64) < (self.width as f64 * percentage / 100.0) {
                            '◼'
                        } else {
                            '◻'
                        }
                    })
                    .collect();
                chars.into_iter().collect()
            }
            GaugeStyle::Circle => {
                // Pie-like representation using Unicode
                let segments = 8;
                let filled_segments = (percentage / 100.0 * segments as f64) as usize;
                let pie_chars = ['○', '◔', '◑', '◕', '●'];
                let idx = (filled_segments * pie_chars.len() / segments).min(pie_chars.len() - 1);
                pie_chars[idx].to_string()
            }
        };

        let mut content = String::new();

        if let Some(label) = &self.label {
            content.push_str(label);
            content.push_str(": ");
        }

        content.push_str(&bar);

        if self.show_percentage {
            content.push_str(&format!(" {:.0}%", percentage));
        }

        VNode::styled_text(content, TextStyle::color(color))
    }
}

// =============================================================================
// LineChart
// =============================================================================

/// Line chart component.
#[derive(Debug, Clone)]
pub struct LineChart {
    data: Vec<Vec<f64>>,
    labels: Vec<String>,
    width: u16,
    height: u16,
    #[allow(dead_code)]
    colors: Vec<Color>,
    show_legend: bool,
}

impl Default for LineChart {
    fn default() -> Self {
        Self {
            data: Vec::new(),
            labels: Vec::new(),
            width: 40,
            height: 10,
            colors: vec![
                Color::Named(NamedColor::Cyan),
                Color::Named(NamedColor::Green),
                Color::Named(NamedColor::Yellow),
                Color::Named(NamedColor::Magenta),
            ],
            show_legend: true,
        }
    }
}

impl LineChart {
    /// Create a new line chart.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a data series.
    pub fn series<I>(mut self, label: impl Into<String>, data: I) -> Self
    where
        I: IntoIterator<Item = f64>,
    {
        self.labels.push(label.into());
        self.data.push(data.into_iter().collect());
        self
    }

    /// Set dimensions.
    pub fn size(mut self, width: u16, height: u16) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    /// Show legend.
    pub fn legend(mut self, show: bool) -> Self {
        self.show_legend = show;
        self
    }

    /// Build the VNode.
    pub fn build(self) -> VNode {
        if self.data.is_empty() {
            return VNode::styled_text("No data", TextStyle::color(Color::Named(NamedColor::Gray)));
        }

        // Find global min/max
        let all_values: Vec<f64> = self.data.iter().flatten().copied().collect();
        let min = all_values.iter().copied().fold(f64::INFINITY, f64::min);
        let max = all_values.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        let range = max - min;

        // Create a simple ASCII line chart
        let mut grid = vec![vec![' '; self.width as usize]; self.height as usize];

        // Plot each series
        for (series_idx, series) in self.data.iter().enumerate() {
            let char = ['●', '◆', '■', '▲'][series_idx % 4];

            for (i, &value) in series.iter().enumerate() {
                let x = (i * self.width as usize) / series.len().max(1);
                let y = if range > 0.0 {
                    ((max - value) / range * (self.height - 1) as f64) as usize
                } else {
                    self.height as usize / 2
                };

                if x < self.width as usize && y < self.height as usize {
                    grid[y][x] = char;
                }
            }
        }

        let mut children: Vec<VNode> = grid
            .into_iter()
            .map(|row| {
                VNode::styled_text(
                    row.into_iter().collect::<String>(),
                    TextStyle::color(Color::Named(NamedColor::White)),
                )
            })
            .collect();

        // Legend
        if self.show_legend && !self.labels.is_empty() {
            let legend_parts: Vec<String> = self
                .labels
                .iter()
                .enumerate()
                .map(|(i, label)| {
                    let char = ['●', '◆', '■', '▲'][i % 4];
                    format!("{} {}", char, label)
                })
                .collect();

            children.push(VNode::styled_text(
                legend_parts.join("  "),
                TextStyle {
                    color: Some(Color::Named(NamedColor::Gray)),
                    dim: true,
                    ..Default::default()
                },
            ));
        }

        VNode::Box(BoxNode {
            children,
            style: BoxStyle::default(),
            ..Default::default()
        })
    }
}

// =============================================================================
// Heatmap
// =============================================================================

/// Heatmap component.
#[derive(Debug, Clone)]
pub struct Heatmap {
    data: Vec<Vec<f64>>,
    row_labels: Vec<String>,
    col_labels: Vec<String>,
    #[allow(dead_code)]
    colors: Vec<Color>,
}

impl Default for Heatmap {
    fn default() -> Self {
        Self {
            data: Vec::new(),
            row_labels: Vec::new(),
            col_labels: Vec::new(),
            colors: vec![
                Color::Named(NamedColor::Blue),
                Color::Named(NamedColor::Cyan),
                Color::Named(NamedColor::Green),
                Color::Named(NamedColor::Yellow),
                Color::Named(NamedColor::Red),
            ],
        }
    }
}

impl Heatmap {
    /// Create a new heatmap.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set data.
    pub fn data(mut self, data: Vec<Vec<f64>>) -> Self {
        self.data = data;
        self
    }

    /// Set row labels.
    pub fn rows<I, S>(mut self, labels: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.row_labels = labels.into_iter().map(Into::into).collect();
        self
    }

    /// Set column labels.
    pub fn cols<I, S>(mut self, labels: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.col_labels = labels.into_iter().map(Into::into).collect();
        self
    }

    /// Build the VNode.
    pub fn build(self) -> VNode {
        if self.data.is_empty() {
            return VNode::styled_text("No data", TextStyle::color(Color::Named(NamedColor::Gray)));
        }

        let heat_chars = ['░', '▒', '▓', '█'];

        let all_values: Vec<f64> = self.data.iter().flatten().copied().collect();
        let min = all_values.iter().copied().fold(f64::INFINITY, f64::min);
        let max = all_values.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        let range = max - min;

        let mut children = Vec::new();

        for (row_idx, row) in self.data.iter().enumerate() {
            let row_label = self
                .row_labels
                .get(row_idx)
                .map(|s| format!("{:>8} ", s))
                .unwrap_or_default();

            let cells: String = row
                .iter()
                .map(|&v| {
                    let normalized = if range > 0.0 { (v - min) / range } else { 0.5 };
                    let idx = (normalized * (heat_chars.len() - 1) as f64) as usize;
                    heat_chars[idx.min(heat_chars.len() - 1)]
                })
                .collect();

            children.push(VNode::styled_text(
                format!("{}{}", row_label, cells),
                TextStyle::color(Color::Named(NamedColor::Yellow)),
            ));
        }

        VNode::Box(BoxNode {
            children,
            style: BoxStyle::default(),
            ..Default::default()
        })
    }
}

// =============================================================================
// RadarChart
// =============================================================================

/// A single axis on the radar chart.
#[derive(Debug, Clone)]
pub struct RadarAxis {
    /// Axis label
    pub label: String,
    /// Maximum value for this axis
    pub max: f64,
}

impl RadarAxis {
    /// Create a new axis.
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            max: 100.0,
        }
    }

    /// Set maximum value.
    pub fn max(mut self, max: f64) -> Self {
        self.max = max;
        self
    }
}

/// A data series for the radar chart.
#[derive(Debug, Clone)]
pub struct RadarSeries {
    /// Series label
    pub label: String,
    /// Values for each axis
    pub values: Vec<f64>,
    /// Series color
    pub color: Option<Color>,
}

impl RadarSeries {
    /// Create a new series.
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            values: Vec::new(),
            color: None,
        }
    }

    /// Set values.
    pub fn values<I>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = f64>,
    {
        self.values = values.into_iter().collect();
        self
    }

    /// Set color.
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }
}

/// Radar (spider) chart component.
#[derive(Debug, Clone)]
pub struct RadarChart {
    axes: Vec<RadarAxis>,
    series: Vec<RadarSeries>,
    size: u16,
    show_legend: bool,
    show_labels: bool,
    show_values: bool,
    fill: bool,
}

impl Default for RadarChart {
    fn default() -> Self {
        Self {
            axes: Vec::new(),
            series: Vec::new(),
            size: 15,
            show_legend: true,
            show_labels: true,
            show_values: false,
            fill: false,
        }
    }
}

impl RadarChart {
    /// Create a new radar chart.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set axes from labels.
    pub fn axes<I, S>(mut self, labels: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.axes = labels.into_iter().map(|s| RadarAxis::new(s)).collect();
        self
    }

    /// Set axes with custom config.
    pub fn with_axes<I>(mut self, axes: I) -> Self
    where
        I: IntoIterator<Item = RadarAxis>,
    {
        self.axes = axes.into_iter().collect();
        self
    }

    /// Add a data series.
    pub fn series(mut self, series: RadarSeries) -> Self {
        self.series.push(series);
        self
    }

    /// Add data with label.
    pub fn data<I>(mut self, label: impl Into<String>, values: I) -> Self
    where
        I: IntoIterator<Item = f64>,
    {
        self.series.push(RadarSeries::new(label).values(values));
        self
    }

    /// Set size (radius in characters).
    pub fn size(mut self, size: u16) -> Self {
        self.size = size;
        self
    }

    /// Show legend.
    pub fn legend(mut self, show: bool) -> Self {
        self.show_legend = show;
        self
    }

    /// Show axis labels.
    pub fn labels(mut self, show: bool) -> Self {
        self.show_labels = show;
        self
    }

    /// Show values on chart.
    pub fn values(mut self, show: bool) -> Self {
        self.show_values = show;
        self
    }

    /// Fill the shape.
    pub fn fill(mut self, fill: bool) -> Self {
        self.fill = fill;
        self
    }

    /// Build the VNode.
    pub fn build(self) -> VNode {
        if self.axes.is_empty() || self.series.is_empty() {
            return VNode::styled_text("No data", TextStyle::color(Color::Named(NamedColor::Gray)));
        }

        let n = self.axes.len();
        let radius = self.size as f64;
        let center = self.size;
        let grid_size = (self.size * 2 + 1) as usize;

        // Create a 2D grid
        let mut grid = vec![vec![' '; grid_size]; grid_size];

        // Draw axes
        for (i, axis) in self.axes.iter().enumerate() {
            let angle =
                2.0 * std::f64::consts::PI * i as f64 / n as f64 - std::f64::consts::PI / 2.0;

            // Draw axis line
            for r in 0..=self.size {
                let x = (center as f64 + r as f64 * angle.cos()).round() as usize;
                let y = (center as f64 + r as f64 * angle.sin()).round() as usize;

                if x < grid_size && y < grid_size {
                    grid[y][x] = if r == 0 { '·' } else { '·' };
                }
            }

            // Draw label at end of axis
            if self.show_labels {
                let label_x = (center as f64 + (radius + 1.0) * angle.cos()).round() as usize;
                let label_y = (center as f64 + (radius + 1.0) * angle.sin()).round() as usize;

                if label_x < grid_size && label_y < grid_size {
                    // Put first char of label
                    if let Some(c) = axis.label.chars().next() {
                        grid[label_y][label_x] = c;
                    }
                }
            }
        }

        // Draw data points and connect them
        let _colors = [
            Color::Named(NamedColor::Cyan),
            Color::Named(NamedColor::Green),
            Color::Named(NamedColor::Magenta),
            Color::Named(NamedColor::Yellow),
        ];

        for (series_idx, series) in self.series.iter().enumerate() {
            let char = ['●', '◆', '■', '▲'][series_idx % 4];
            let mut points = Vec::new();

            for (i, &value) in series.values.iter().enumerate() {
                if i >= n {
                    break;
                }

                let max_val = self.axes.get(i).map(|a| a.max).unwrap_or(100.0);
                let normalized = (value / max_val).min(1.0);
                let r = normalized * radius;

                let angle =
                    2.0 * std::f64::consts::PI * i as f64 / n as f64 - std::f64::consts::PI / 2.0;
                let x = (center as f64 + r * angle.cos()).round() as usize;
                let y = (center as f64 + r * angle.sin()).round() as usize;

                points.push((x, y));

                if x < grid_size && y < grid_size {
                    grid[y][x] = char;
                }
            }

            // Connect points with lines
            if points.len() > 1 {
                for i in 0..points.len() {
                    let (x1, y1) = points[i];
                    let (x2, y2) = points[(i + 1) % points.len()];

                    // Simple Bresenham line
                    let dx = (x2 as i32 - x1 as i32).abs();
                    let dy = (y2 as i32 - y1 as i32).abs();
                    let steps = dx.max(dy);

                    if steps > 0 {
                        for step in 1..steps {
                            let t = step as f64 / steps as f64;
                            let x = (x1 as f64 + t * (x2 as f64 - x1 as f64)).round() as usize;
                            let y = (y1 as f64 + t * (y2 as f64 - y1 as f64)).round() as usize;

                            if x < grid_size && y < grid_size && grid[y][x] == ' ' {
                                grid[y][x] = if self.fill { '░' } else { '·' };
                            }
                        }
                    }
                }
            }
        }

        // Draw center
        grid[center as usize][center as usize] = '┼';

        // Convert grid to VNodes
        let mut children: Vec<VNode> = grid
            .into_iter()
            .map(|row| {
                VNode::styled_text(
                    row.into_iter().collect::<String>(),
                    TextStyle::color(Color::Named(NamedColor::White)),
                )
            })
            .collect();

        // Legend
        if self.show_legend {
            let legend_parts: Vec<String> = self
                .series
                .iter()
                .enumerate()
                .map(|(i, s)| {
                    let char = ['●', '◆', '■', '▲'][i % 4];
                    format!("{} {}", char, s.label)
                })
                .collect();

            children.push(VNode::styled_text(
                legend_parts.join("  "),
                TextStyle {
                    color: Some(Color::Named(NamedColor::Gray)),
                    dim: true,
                    ..Default::default()
                },
            ));
        }

        VNode::Box(BoxNode {
            children,
            style: BoxStyle::default(),
            ..Default::default()
        })
    }
}

// =============================================================================
// ScatterPlot
// =============================================================================

/// A data point on the scatter plot.
#[derive(Debug, Clone, Copy)]
pub struct ScatterPoint {
    /// X coordinate
    pub x: f64,
    /// Y coordinate
    pub y: f64,
}

impl ScatterPoint {
    /// Create a new point.
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

impl From<(f64, f64)> for ScatterPoint {
    fn from((x, y): (f64, f64)) -> Self {
        Self { x, y }
    }
}

/// A data series for the scatter plot.
#[derive(Debug, Clone)]
pub struct ScatterSeries {
    /// Series label
    pub label: String,
    /// Data points
    pub points: Vec<ScatterPoint>,
    /// Series color
    pub color: Option<Color>,
    /// Point character
    pub marker: Option<char>,
}

impl ScatterSeries {
    /// Create a new series.
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            points: Vec::new(),
            color: None,
            marker: None,
        }
    }

    /// Set points from tuples.
    pub fn points<I>(mut self, points: I) -> Self
    where
        I: IntoIterator<Item = (f64, f64)>,
    {
        self.points = points.into_iter().map(ScatterPoint::from).collect();
        self
    }

    /// Set points directly.
    pub fn with_points<I>(mut self, points: I) -> Self
    where
        I: IntoIterator<Item = ScatterPoint>,
    {
        self.points = points.into_iter().collect();
        self
    }

    /// Set color.
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    /// Set marker character.
    pub fn marker(mut self, marker: char) -> Self {
        self.marker = Some(marker);
        self
    }
}

/// Scatter plot component.
#[derive(Debug, Clone)]
pub struct ScatterPlot {
    series: Vec<ScatterSeries>,
    width: u16,
    height: u16,
    x_range: Option<(f64, f64)>,
    y_range: Option<(f64, f64)>,
    show_axes: bool,
    show_legend: bool,
    show_grid: bool,
    x_label: Option<String>,
    y_label: Option<String>,
}

impl Default for ScatterPlot {
    fn default() -> Self {
        Self {
            series: Vec::new(),
            width: 40,
            height: 15,
            x_range: None,
            y_range: None,
            show_axes: true,
            show_legend: true,
            show_grid: false,
            x_label: None,
            y_label: None,
        }
    }
}

impl ScatterPlot {
    /// Create a new scatter plot.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a data series.
    pub fn series(mut self, series: ScatterSeries) -> Self {
        self.series.push(series);
        self
    }

    /// Add data from tuples.
    pub fn data<I>(mut self, label: impl Into<String>, points: I) -> Self
    where
        I: IntoIterator<Item = (f64, f64)>,
    {
        self.series.push(ScatterSeries::new(label).points(points));
        self
    }

    /// Set dimensions.
    pub fn size(mut self, width: u16, height: u16) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    /// Set X axis range.
    pub fn x_range(mut self, min: f64, max: f64) -> Self {
        self.x_range = Some((min, max));
        self
    }

    /// Set Y axis range.
    pub fn y_range(mut self, min: f64, max: f64) -> Self {
        self.y_range = Some((min, max));
        self
    }

    /// Show axes.
    pub fn axes(mut self, show: bool) -> Self {
        self.show_axes = show;
        self
    }

    /// Show legend.
    pub fn legend(mut self, show: bool) -> Self {
        self.show_legend = show;
        self
    }

    /// Show grid.
    pub fn grid(mut self, show: bool) -> Self {
        self.show_grid = show;
        self
    }

    /// Set X axis label.
    pub fn x_label(mut self, label: impl Into<String>) -> Self {
        self.x_label = Some(label.into());
        self
    }

    /// Set Y axis label.
    pub fn y_label(mut self, label: impl Into<String>) -> Self {
        self.y_label = Some(label.into());
        self
    }

    /// Build the VNode.
    pub fn build(self) -> VNode {
        if self.series.is_empty() {
            return VNode::styled_text("No data", TextStyle::color(Color::Named(NamedColor::Gray)));
        }

        // Collect all points to determine range
        let all_points: Vec<&ScatterPoint> = self.series.iter().flat_map(|s| &s.points).collect();

        if all_points.is_empty() {
            return VNode::styled_text("No data", TextStyle::color(Color::Named(NamedColor::Gray)));
        }

        // Determine ranges
        let (x_min, x_max) = self.x_range.unwrap_or_else(|| {
            let min = all_points.iter().map(|p| p.x).fold(f64::INFINITY, f64::min);
            let max = all_points
                .iter()
                .map(|p| p.x)
                .fold(f64::NEG_INFINITY, f64::max);
            let padding = (max - min) * 0.05;
            (min - padding, max + padding)
        });

        let (y_min, y_max) = self.y_range.unwrap_or_else(|| {
            let min = all_points.iter().map(|p| p.y).fold(f64::INFINITY, f64::min);
            let max = all_points
                .iter()
                .map(|p| p.y)
                .fold(f64::NEG_INFINITY, f64::max);
            let padding = (max - min) * 0.05;
            (min - padding, max + padding)
        });

        let x_range = x_max - x_min;
        let y_range = y_max - y_min;

        // Create grid
        let mut grid = vec![vec![' '; self.width as usize]; self.height as usize];

        // Draw grid if enabled
        if self.show_grid {
            for y in 0..self.height as usize {
                for x in 0..self.width as usize {
                    if y == self.height as usize / 2 || x == self.width as usize / 2 {
                        grid[y][x] = '·';
                    }
                }
            }
        }

        // Draw axes
        if self.show_axes {
            // Y axis
            for y in 0..self.height as usize {
                grid[y][0] = '│';
            }

            // X axis
            for x in 0..self.width as usize {
                grid[self.height as usize - 1][x] = '─';
            }

            // Origin
            grid[self.height as usize - 1][0] = '└';
        }

        // Plot points
        let markers = ['●', '◆', '■', '▲', '○', '◇', '□', '△'];

        for (series_idx, series) in self.series.iter().enumerate() {
            let marker = series.marker.unwrap_or(markers[series_idx % markers.len()]);

            for point in &series.points {
                // Normalize to grid coordinates
                let x = if x_range > 0.0 {
                    ((point.x - x_min) / x_range * (self.width - 1) as f64) as usize
                } else {
                    self.width as usize / 2
                };

                let y = if y_range > 0.0 {
                    ((y_max - point.y) / y_range * (self.height - 1) as f64) as usize
                } else {
                    self.height as usize / 2
                };

                if x < self.width as usize && y < self.height as usize {
                    grid[y][x] = marker;
                }
            }
        }

        // Convert to VNodes
        let mut children: Vec<VNode> = Vec::new();

        // Add Y axis label
        if let Some(label) = &self.y_label {
            children.push(VNode::styled_text(
                label.clone(),
                TextStyle {
                    color: Some(Color::Named(NamedColor::Gray)),
                    dim: true,
                    ..Default::default()
                },
            ));
        }

        // Add grid rows
        for row in grid {
            children.push(VNode::styled_text(
                row.into_iter().collect::<String>(),
                TextStyle::color(Color::Named(NamedColor::White)),
            ));
        }

        // Add X axis label
        if let Some(label) = &self.x_label {
            children.push(VNode::styled_text(
                format!("{:>width$}", label, width = self.width as usize),
                TextStyle {
                    color: Some(Color::Named(NamedColor::Gray)),
                    dim: true,
                    ..Default::default()
                },
            ));
        }

        // Add legend
        if self.show_legend {
            let legend_parts: Vec<String> = self
                .series
                .iter()
                .enumerate()
                .map(|(i, s)| {
                    let marker = s.marker.unwrap_or(markers[i % markers.len()]);
                    format!("{} {}", marker, s.label)
                })
                .collect();

            children.push(VNode::styled_text(
                legend_parts.join("  "),
                TextStyle {
                    color: Some(Color::Named(NamedColor::Gray)),
                    dim: true,
                    ..Default::default()
                },
            ));
        }

        VNode::Box(BoxNode {
            children,
            style: BoxStyle::default(),
            ..Default::default()
        })
    }
}
