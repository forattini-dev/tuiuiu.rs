//! Gantt Chart Component
//!
//! Project timeline visualization with tasks, progress, and milestones.
//!
//! Features:
//! - Task timeline with start/end dates
//! - Progress percentage display
//! - Status color coding
//! - Milestone markers
//! - Legend

use crate::core::component::{VNode, BoxNode, BoxStyle, TextStyle, Color, NamedColor};
use crate::core::layout::FlexDirection;

// =============================================================================
// Types
// =============================================================================

/// Task status.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum TaskStatus {
    #[default]
    Pending,
    InProgress,
    Complete,
    Blocked,
}

impl TaskStatus {
    /// Get color for status.
    pub fn color(&self) -> Color {
        match self {
            TaskStatus::Pending => Color::Named(NamedColor::Yellow),
            TaskStatus::InProgress => Color::Named(NamedColor::Blue),
            TaskStatus::Complete => Color::Named(NamedColor::Green),
            TaskStatus::Blocked => Color::Named(NamedColor::Red),
        }
    }

    /// Get label for status.
    pub fn label(&self) -> &'static str {
        match self {
            TaskStatus::Pending => "pending",
            TaskStatus::InProgress => "in progress",
            TaskStatus::Complete => "complete",
            TaskStatus::Blocked => "blocked",
        }
    }
}

/// Gantt task.
#[derive(Debug, Clone)]
pub struct GanttTask {
    /// Task ID
    pub id: String,
    /// Task name
    pub name: String,
    /// Start date (days from epoch or similar)
    pub start_day: i64,
    /// End date (days from epoch or similar)
    pub end_day: i64,
    /// Completion percentage (0-100)
    pub progress: u8,
    /// Task status
    pub status: TaskStatus,
    /// Is milestone (single point in time)
    pub is_milestone: bool,
    /// Optional dependency on other task ID
    pub depends_on: Option<String>,
}

impl GanttTask {
    /// Create a new task.
    pub fn new(id: impl Into<String>, name: impl Into<String>, start: i64, end: i64) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            start_day: start,
            end_day: end,
            progress: 0,
            status: TaskStatus::Pending,
            is_milestone: false,
            depends_on: None,
        }
    }

    /// Set progress.
    pub fn progress(mut self, progress: u8) -> Self {
        self.progress = progress.min(100);
        self
    }

    /// Set status.
    pub fn status(mut self, status: TaskStatus) -> Self {
        self.status = status;
        self
    }

    /// Mark as milestone.
    pub fn milestone(mut self) -> Self {
        self.is_milestone = true;
        self
    }

    /// Set dependency.
    pub fn depends_on(mut self, task_id: impl Into<String>) -> Self {
        self.depends_on = Some(task_id.into());
        self
    }
}

// =============================================================================
// Component
// =============================================================================

/// Gantt chart component.
#[derive(Debug, Clone)]
pub struct GanttChart {
    /// Tasks to display
    tasks: Vec<GanttTask>,
    /// Chart width
    width: usize,
    /// Title
    title: Option<String>,
    /// Show legend
    show_legend: bool,
    /// Label width (chars for task names)
    label_width: usize,
}

impl Default for GanttChart {
    fn default() -> Self {
        Self {
            tasks: Vec::new(),
            width: 80,
            title: None,
            show_legend: true,
            label_width: 20,
        }
    }
}

impl GanttChart {
    /// Create a new gantt chart.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add tasks.
    pub fn tasks<I>(mut self, tasks: I) -> Self
    where
        I: IntoIterator<Item = GanttTask>,
    {
        self.tasks = tasks.into_iter().collect();
        self
    }

    /// Add a single task.
    pub fn task(mut self, task: GanttTask) -> Self {
        self.tasks.push(task);
        self
    }

    /// Set width.
    pub fn width(mut self, width: usize) -> Self {
        self.width = width;
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

    /// Set label width.
    pub fn label_width(mut self, width: usize) -> Self {
        self.label_width = width;
        self
    }

    /// Build legend.
    fn build_legend(&self) -> VNode {
        let statuses = [
            TaskStatus::Pending,
            TaskStatus::InProgress,
            TaskStatus::Complete,
            TaskStatus::Blocked,
        ];

        let items: Vec<VNode> = statuses.iter().map(|status| {
            VNode::Box(BoxNode {
                children: vec![
                    VNode::styled_text(
                        "■".to_string(),
                        TextStyle {
                            color: Some(status.color()),
                            ..Default::default()
                        },
                    ),
                    VNode::text(" ".to_string()),
                    VNode::styled_text(
                        status.label().to_string(),
                        TextStyle {
                            color: Some(Color::Named(NamedColor::Gray)),
                            ..Default::default()
                        },
                    ),
                    VNode::text("  ".to_string()),
                ],
                style: BoxStyle {
                    flex_direction: Some(FlexDirection::Row),
                    ..Default::default()
                },
                ..Default::default()
            })
        }).collect();

        VNode::Box(BoxNode {
            children: items,
            style: BoxStyle {
                flex_direction: Some(FlexDirection::Row),
                ..Default::default()
            },
            ..Default::default()
        })
    }

    /// Build the VNode.
    pub fn build(self) -> VNode {
        if self.tasks.is_empty() {
            return VNode::styled_text(
                "No tasks".to_string(),
                TextStyle {
                    color: Some(Color::Named(NamedColor::Gray)),
                    dim: true,
                    ..Default::default()
                },
            );
        }

        // Calculate date range
        let min_day = self.tasks.iter().map(|t| t.start_day).min().unwrap_or(0);
        let max_day = self.tasks.iter().map(|t| t.end_day).max().unwrap_or(0);
        let day_range = (max_day - min_day).max(1);
        let bar_width = self.width.saturating_sub(self.label_width + 2);

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

        // Task rows
        for task in &self.tasks {
            let duration = task.end_day - task.start_day;
            let offset = ((task.start_day - min_day) as f64 / day_range as f64 * bar_width as f64) as usize;
            let bar_len = ((duration as f64 / day_range as f64) * bar_width as f64).ceil() as usize;
            let bar_len = bar_len.max(1).min(bar_width - offset);

            let fill_len = (task.progress as usize * bar_len) / 100;

            // Task name (truncated/padded)
            let name = if task.name.len() > self.label_width {
                format!("{}…", &task.name[..self.label_width - 1])
            } else {
                format!("{:width$}", task.name, width = self.label_width)
            };

            // Build bar
            let bar = if task.is_milestone {
                format!("{}◆", " ".repeat(offset))
            } else {
                let filled = "█".repeat(fill_len);
                let empty = "░".repeat(bar_len.saturating_sub(fill_len));
                format!("{}{}{}", " ".repeat(offset), filled, empty)
            };

            // Task row
            let row = VNode::Box(BoxNode {
                children: vec![
                    VNode::styled_text(
                        name,
                        TextStyle {
                            color: Some(Color::Named(NamedColor::Gray)),
                            ..Default::default()
                        },
                    ),
                    VNode::text(" ".to_string()),
                    VNode::styled_text(
                        bar,
                        TextStyle {
                            color: Some(task.status.color()),
                            ..Default::default()
                        },
                    ),
                ],
                style: BoxStyle {
                    flex_direction: Some(FlexDirection::Row),
                    ..Default::default()
                },
                ..Default::default()
            });

            children.push(row);
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
}
