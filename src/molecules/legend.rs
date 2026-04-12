//! Legend Component
//!
//! Legend for data visualization charts.

use crate::core::component::{BoxNode, BoxStyle, Color, NamedColor, TextStyle, VNode};
use crate::core::layout::FlexDirection;

/// Legend item.
#[derive(Debug, Clone)]
pub struct LegendItem {
    /// Item label
    pub label: String,
    /// Item color
    pub color: Color,
    /// Item value (optional)
    pub value: Option<String>,
    /// Whether item is active/visible
    pub active: bool,
}

impl LegendItem {
    /// Create a new legend item.
    pub fn new(label: impl Into<String>, color: Color) -> Self {
        Self {
            label: label.into(),
            color,
            value: None,
            active: true,
        }
    }

    /// Set value.
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Set active state.
    pub fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }
}

/// Legend layout direction.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum LegendLayout {
    #[default]
    Horizontal,
    Vertical,
}

/// Legend marker style.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum LegendMarker {
    #[default]
    Square,
    Circle,
    Line,
    Dash,
    Triangle,
}

/// Legend component.
#[derive(Debug, Clone)]
pub struct Legend {
    items: Vec<LegendItem>,
    layout: LegendLayout,
    marker: LegendMarker,
    show_values: bool,
    title: Option<String>,
    max_columns: Option<usize>,
}

impl Default for Legend {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            layout: LegendLayout::Horizontal,
            marker: LegendMarker::Square,
            show_values: false,
            title: None,
            max_columns: None,
        }
    }
}

impl Legend {
    /// Create a new legend.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create legend with items.
    pub fn with_items<I>(items: I) -> Self
    where
        I: IntoIterator<Item = LegendItem>,
    {
        Self {
            items: items.into_iter().collect(),
            ..Default::default()
        }
    }

    /// Add an item.
    pub fn item(mut self, item: LegendItem) -> Self {
        self.items.push(item);
        self
    }

    /// Add items.
    pub fn items<I>(mut self, items: I) -> Self
    where
        I: IntoIterator<Item = LegendItem>,
    {
        self.items.extend(items);
        self
    }

    /// Add a simple item with label and color.
    pub fn add(mut self, label: impl Into<String>, color: Color) -> Self {
        self.items.push(LegendItem::new(label, color));
        self
    }

    /// Set layout direction.
    pub fn layout(mut self, layout: LegendLayout) -> Self {
        self.layout = layout;
        self
    }

    /// Set horizontal layout.
    pub fn horizontal(self) -> Self {
        self.layout(LegendLayout::Horizontal)
    }

    /// Set vertical layout.
    pub fn vertical(self) -> Self {
        self.layout(LegendLayout::Vertical)
    }

    /// Set marker style.
    pub fn marker(mut self, marker: LegendMarker) -> Self {
        self.marker = marker;
        self
    }

    /// Show values.
    pub fn show_values(mut self, show: bool) -> Self {
        self.show_values = show;
        self
    }

    /// Set title.
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set max columns for horizontal layout.
    pub fn max_columns(mut self, columns: usize) -> Self {
        self.max_columns = Some(columns);
        self
    }

    /// Get marker character.
    fn get_marker(&self) -> &str {
        match self.marker {
            LegendMarker::Square => "■",
            LegendMarker::Circle => "●",
            LegendMarker::Line => "━",
            LegendMarker::Dash => "─",
            LegendMarker::Triangle => "▲",
        }
    }

    /// Build a single legend item.
    fn build_item(&self, item: &LegendItem) -> VNode {
        let marker = self.get_marker();

        let marker_style = if item.active {
            TextStyle {
                color: Some(item.color.clone()),
                bold: true,
                ..Default::default()
            }
        } else {
            TextStyle {
                color: Some(Color::Named(NamedColor::Gray)),
                dim: true,
                ..Default::default()
            }
        };

        let label_style = if item.active {
            TextStyle::default()
        } else {
            TextStyle {
                color: Some(Color::Named(NamedColor::Gray)),
                dim: true,
                strikethrough: true,
                ..Default::default()
            }
        };

        let _text = if self.show_values {
            if let Some(value) = &item.value {
                format!("{} {} ({})", marker, item.label, value)
            } else {
                format!("{} {}", marker, item.label)
            }
        } else {
            format!("{} {}", marker, item.label)
        };

        // Build with marker colored and label default
        let parts: Vec<VNode> = vec![
            VNode::styled_text(marker.to_string(), marker_style),
            VNode::styled_text(
                format!(
                    " {}",
                    if self.show_values {
                        if let Some(value) = &item.value {
                            format!("{} ({})", item.label, value)
                        } else {
                            item.label.clone()
                        }
                    } else {
                        item.label.clone()
                    }
                ),
                label_style,
            ),
        ];

        VNode::Box(BoxNode {
            children: parts,
            style: BoxStyle {
                flex_direction: Some(FlexDirection::Row),
                ..Default::default()
            },
            ..Default::default()
        })
    }

    /// Build the VNode.
    pub fn build(self) -> VNode {
        let mut children: Vec<VNode> = Vec::new();

        // Add title if present
        if let Some(title) = &self.title {
            children.push(VNode::styled_text(
                title.clone(),
                TextStyle {
                    bold: true,
                    ..Default::default()
                },
            ));
        }

        // Build items
        let items: Vec<VNode> = self
            .items
            .iter()
            .map(|item| self.build_item(item))
            .collect();

        // Create items container
        let items_container = VNode::Box(BoxNode {
            children: items,
            style: BoxStyle {
                flex_direction: Some(match self.layout {
                    LegendLayout::Horizontal => FlexDirection::Row,
                    LegendLayout::Vertical => FlexDirection::Column,
                }),
                gap: Some(if self.layout == LegendLayout::Horizontal {
                    2
                } else {
                    0
                }),
                ..Default::default()
            },
            ..Default::default()
        });

        children.push(items_container);

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
