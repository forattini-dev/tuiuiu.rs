//! Time Heatmap Component
//!
//! Calendar-based activity heatmap visualization similar to GitHub's contribution graph.
//!
//! Features:
//! - Calendar grid with date cells
//! - Intensity-based coloring
//! - Multiple time scales (day, week, month)
//! - Legend with intensity levels
//! - Customizable color schemes

use crate::core::component::{VNode, BoxNode, BoxStyle, TextStyle, Color, NamedColor};
use crate::core::layout::FlexDirection;

// =============================================================================
// Types
// =============================================================================

/// Time scale for heatmap grouping.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum TimeScale {
    #[default]
    Day,
    Week,
    Month,
}

/// Heatmap cell data.
#[derive(Debug, Clone)]
pub struct HeatmapCell {
    /// Date/time label (e.g., "2024-01-15" or "Mon")
    pub label: String,
    /// Value for intensity calculation
    pub value: f64,
    /// Optional tooltip text
    pub tooltip: Option<String>,
}

impl HeatmapCell {
    /// Create a new heatmap cell.
    pub fn new(label: impl Into<String>, value: f64) -> Self {
        Self {
            label: label.into(),
            value,
            tooltip: None,
        }
    }

    /// Set tooltip.
    pub fn tooltip(mut self, tooltip: impl Into<String>) -> Self {
        self.tooltip = Some(tooltip.into());
        self
    }
}

/// Heatmap data organized by rows and columns.
#[derive(Debug, Clone, Default)]
pub struct HeatmapData {
    /// Row labels (e.g., days of week: Mon, Tue, ...)
    pub row_labels: Vec<String>,
    /// Column labels (e.g., week numbers or months)
    pub col_labels: Vec<String>,
    /// Data matrix [row][col]
    pub cells: Vec<Vec<Option<HeatmapCell>>>,
}

impl HeatmapData {
    /// Create new heatmap data.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set row labels.
    pub fn row_labels<I, S>(mut self, labels: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.row_labels = labels.into_iter().map(|s| s.into()).collect();
        self
    }

    /// Set column labels.
    pub fn col_labels<I, S>(mut self, labels: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.col_labels = labels.into_iter().map(|s| s.into()).collect();
        self
    }

    /// Set cells from a flat list of values (fills row by row).
    pub fn from_values(mut self, values: Vec<f64>, rows: usize, cols: usize) -> Self {
        self.cells = Vec::with_capacity(rows);
        let mut idx = 0;
        for _row in 0..rows {
            let mut row_cells = Vec::with_capacity(cols);
            for _col in 0..cols {
                if idx < values.len() {
                    let cell = HeatmapCell::new(format!("{}", idx), values[idx]);
                    row_cells.push(Some(cell));
                    idx += 1;
                } else {
                    row_cells.push(None);
                }
            }
            self.cells.push(row_cells);
        }
        self
    }

    /// Set cells matrix directly.
    pub fn cells(mut self, cells: Vec<Vec<Option<HeatmapCell>>>) -> Self {
        self.cells = cells;
        self
    }
}

// =============================================================================
// Component
// =============================================================================

/// Time heatmap component.
#[derive(Debug, Clone)]
pub struct TimeHeatmap {
    /// Heatmap data
    data: HeatmapData,
    /// Time scale
    scale: TimeScale,
    /// Title
    title: Option<String>,
    /// Show legend
    show_legend: bool,
    /// Show row labels
    show_row_labels: bool,
    /// Show column labels
    show_col_labels: bool,
    /// Custom intensity colors (5 levels)
    colors: Option<[Color; 5]>,
    /// Cell character
    cell_char: char,
    /// Empty cell character
    empty_char: char,
    /// Label width
    label_width: usize,
}

impl Default for TimeHeatmap {
    fn default() -> Self {
        Self {
            data: HeatmapData::default(),
            scale: TimeScale::Day,
            title: None,
            show_legend: true,
            show_row_labels: true,
            show_col_labels: true,
            colors: None,
            cell_char: '█',
            empty_char: '░',
            label_width: 4,
        }
    }
}

impl TimeHeatmap {
    /// Create a new time heatmap.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set data.
    pub fn data(mut self, data: HeatmapData) -> Self {
        self.data = data;
        self
    }

    /// Set time scale.
    pub fn scale(mut self, scale: TimeScale) -> Self {
        self.scale = scale;
        self
    }

    /// Set title.
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Show/hide legend.
    pub fn show_legend(mut self, show: bool) -> Self {
        self.show_legend = show;
        self
    }

    /// Show/hide row labels.
    pub fn show_row_labels(mut self, show: bool) -> Self {
        self.show_row_labels = show;
        self
    }

    /// Show/hide column labels.
    pub fn show_col_labels(mut self, show: bool) -> Self {
        self.show_col_labels = show;
        self
    }

    /// Set custom intensity colors (5 levels from low to high).
    pub fn colors(mut self, colors: [Color; 5]) -> Self {
        self.colors = Some(colors);
        self
    }

    /// Set cell character.
    pub fn cell_char(mut self, ch: char) -> Self {
        self.cell_char = ch;
        self
    }

    /// Set empty cell character.
    pub fn empty_char(mut self, ch: char) -> Self {
        self.empty_char = ch;
        self
    }

    /// Set label width.
    pub fn label_width(mut self, width: usize) -> Self {
        self.label_width = width;
        self
    }

    /// Get color for intensity level (0-4).
    fn get_color(&self, level: usize) -> Color {
        if let Some(ref colors) = self.colors {
            colors[level.min(4)].clone()
        } else {
            // Default GitHub-like green gradient
            match level {
                0 => Color::Named(NamedColor::Gray),
                1 => Color::Rgb(0, 68, 0),      // Dark green
                2 => Color::Rgb(0, 102, 0),     // Medium-dark green
                3 => Color::Rgb(0, 153, 0),     // Medium green
                _ => Color::Rgb(0, 204, 0),     // Bright green
            }
        }
    }

    /// Calculate intensity level from value.
    fn calculate_level(&self, value: f64, min: f64, max: f64) -> usize {
        if (max - min).abs() < f64::EPSILON {
            return if value > 0.0 { 4 } else { 0 };
        }
        let normalized = (value - min) / (max - min);
        match normalized {
            n if n <= 0.0 => 0,
            n if n <= 0.25 => 1,
            n if n <= 0.5 => 2,
            n if n <= 0.75 => 3,
            _ => 4,
        }
    }

    /// Build the VNode.
    pub fn build(self) -> VNode {
        if self.data.cells.is_empty() {
            return VNode::styled_text(
                "No data".to_string(),
                TextStyle {
                    color: Some(Color::Named(NamedColor::Gray)),
                    dim: true,
                    ..Default::default()
                },
            );
        }

        // Calculate min/max for normalization
        let mut min_val = f64::INFINITY;
        let mut max_val = f64::NEG_INFINITY;
        for row in &self.data.cells {
            for cell in row.iter().flatten() {
                min_val = min_val.min(cell.value);
                max_val = max_val.max(cell.value);
            }
        }

        let mut children: Vec<VNode> = Vec::new();

        // Title
        if let Some(ref title) = self.title {
            children.push(VNode::styled_text(
                title.clone(),
                TextStyle {
                    bold: true,
                    ..Default::default()
                },
            ));
            children.push(VNode::text(String::new())); // Empty line
        }

        // Column labels (if showing)
        if self.show_col_labels && !self.data.col_labels.is_empty() {
            let mut col_label_parts: Vec<VNode> = Vec::new();

            // Padding for row labels
            if self.show_row_labels {
                col_label_parts.push(VNode::text(" ".repeat(self.label_width + 1)));
            }

            for label in &self.data.col_labels {
                col_label_parts.push(VNode::styled_text(
                    format!("{} ", label),
                    TextStyle {
                        color: Some(Color::Named(NamedColor::Gray)),
                        dim: true,
                        ..Default::default()
                    },
                ));
            }

            children.push(VNode::Box(BoxNode {
                children: col_label_parts,
                style: BoxStyle {
                    flex_direction: Some(FlexDirection::Row),
                    ..Default::default()
                },
                ..Default::default()
            }));
        }

        // Data rows
        for (row_idx, row) in self.data.cells.iter().enumerate() {
            let mut row_parts: Vec<VNode> = Vec::new();

            // Row label
            if self.show_row_labels {
                let label = self.data.row_labels.get(row_idx)
                    .cloned()
                    .unwrap_or_default();
                let padded = format!("{:>width$} ", label, width = self.label_width);
                row_parts.push(VNode::styled_text(
                    padded,
                    TextStyle {
                        color: Some(Color::Named(NamedColor::Gray)),
                        dim: true,
                        ..Default::default()
                    },
                ));
            }

            // Cells
            for cell in row {
                let (ch, color) = if let Some(c) = cell {
                    let level = self.calculate_level(c.value, min_val, max_val);
                    (self.cell_char, self.get_color(level))
                } else {
                    (self.empty_char, Color::Named(NamedColor::Gray))
                };

                row_parts.push(VNode::styled_text(
                    ch.to_string(),
                    TextStyle {
                        color: Some(color),
                        ..Default::default()
                    },
                ));
            }

            children.push(VNode::Box(BoxNode {
                children: row_parts,
                style: BoxStyle {
                    flex_direction: Some(FlexDirection::Row),
                    ..Default::default()
                },
                ..Default::default()
            }));
        }

        // Legend
        if self.show_legend {
            children.push(VNode::text(String::new())); // Empty line
            children.push(self.build_legend());
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

    /// Build legend showing intensity levels.
    fn build_legend(&self) -> VNode {
        let mut parts: Vec<VNode> = Vec::new();

        parts.push(VNode::styled_text(
            "Less ".to_string(),
            TextStyle {
                color: Some(Color::Named(NamedColor::Gray)),
                dim: true,
                ..Default::default()
            },
        ));

        for level in 0..5 {
            parts.push(VNode::styled_text(
                self.cell_char.to_string(),
                TextStyle {
                    color: Some(self.get_color(level)),
                    ..Default::default()
                },
            ));
        }

        parts.push(VNode::styled_text(
            " More".to_string(),
            TextStyle {
                color: Some(Color::Named(NamedColor::Gray)),
                dim: true,
                ..Default::default()
            },
        ));

        VNode::Box(BoxNode {
            children: parts,
            style: BoxStyle {
                flex_direction: Some(FlexDirection::Row),
                ..Default::default()
            },
            ..Default::default()
        })
    }
}

// =============================================================================
// Helper Functions
// =============================================================================

/// Create a weekly heatmap (7 rows x N weeks).
pub fn create_weekly_heatmap(weeks: usize) -> HeatmapData {
    HeatmapData::new()
        .row_labels(["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"])
        .col_labels((1..=weeks).map(|w| format!("W{}", w)))
}

/// Create a monthly heatmap.
pub fn create_monthly_heatmap() -> HeatmapData {
    HeatmapData::new()
        .row_labels(["Jan", "Feb", "Mar", "Apr", "May", "Jun",
                     "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"])
}
