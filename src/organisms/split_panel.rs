//! SplitPanel Component
//!
//! A two-pane layout with a resizable divider.
//! Supports horizontal and vertical orientations.

use crate::core::component::{
    BoxNode, BoxStyle, Color, EventHandlers, NamedColor, TextNode, TextStyle, VNode,
};
use crate::core::layout::{AlignItems, FlexDirection, JustifyContent, Size};
use crate::core::signals::{create_signal, ReadSignal, WriteSignal};

/// Split orientation.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum SplitOrientation {
    /// Left/right split
    #[default]
    Horizontal,
    /// Top/bottom split
    Vertical,
}

/// Divider style.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum DividerStyle {
    /// Simple line
    #[default]
    Line,
    /// Double line
    Double,
    /// Dashed line
    Dashed,
    /// No visible divider
    None,
}

impl DividerStyle {
    /// Get the horizontal divider character.
    pub fn h_char(&self) -> char {
        match self {
            DividerStyle::Line => '│',
            DividerStyle::Double => '║',
            DividerStyle::Dashed => '┊',
            DividerStyle::None => ' ',
        }
    }

    /// Get the vertical divider character.
    pub fn v_char(&self) -> char {
        match self {
            DividerStyle::Line => '─',
            DividerStyle::Double => '═',
            DividerStyle::Dashed => '┄',
            DividerStyle::None => ' ',
        }
    }
}

/// SplitPanel component.
///
/// A layout with two resizable panes.
///
/// # Example
/// ```ignore
/// let split = SplitPanel::new()
///     .orientation(SplitOrientation::Horizontal)
///     .first(sidebar)
///     .second(content)
///     .position(25) // First pane is 25%
///     .build();
/// ```
#[derive(Debug, Clone, Default)]
pub struct SplitPanel {
    /// First pane content (left or top)
    first: Option<VNode>,
    /// Second pane content (right or bottom)
    second: Option<VNode>,
    /// Split orientation
    orientation: SplitOrientation,
    /// Split position (percentage 0-100)
    position: u8,
    /// Minimum size for first pane (percentage)
    min_first: u8,
    /// Minimum size for second pane (percentage)
    min_second: u8,
    /// Divider style
    divider_style: DividerStyle,
    /// Divider color
    divider_color: Option<Color>,
    /// Show resize handle indicator
    show_handle: bool,
    /// Total width (if set)
    width: Option<u16>,
    /// Total height (if set)
    height: Option<u16>,
}

impl SplitPanel {
    /// Create a new split panel.
    pub fn new() -> Self {
        Self {
            first: None,
            second: None,
            orientation: SplitOrientation::default(),
            position: 50, // 50% by default
            min_first: 10,
            min_second: 10,
            divider_style: DividerStyle::default(),
            divider_color: None,
            show_handle: true,
            width: None,
            height: None,
        }
    }

    /// Set the first pane content.
    pub fn first(mut self, content: VNode) -> Self {
        self.first = Some(content);
        self
    }

    /// Set the second pane content.
    pub fn second(mut self, content: VNode) -> Self {
        self.second = Some(content);
        self
    }

    /// Set the split orientation.
    pub fn orientation(mut self, orientation: SplitOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    /// Set horizontal orientation.
    pub fn horizontal(self) -> Self {
        self.orientation(SplitOrientation::Horizontal)
    }

    /// Set vertical orientation.
    pub fn vertical(self) -> Self {
        self.orientation(SplitOrientation::Vertical)
    }

    /// Set split position (percentage 0-100).
    pub fn position(mut self, position: u8) -> Self {
        self.position = position.min(100);
        self
    }

    /// Set minimum size for first pane.
    pub fn min_first(mut self, min: u8) -> Self {
        self.min_first = min.min(100);
        self
    }

    /// Set minimum size for second pane.
    pub fn min_second(mut self, min: u8) -> Self {
        self.min_second = min.min(100);
        self
    }

    /// Set divider style.
    pub fn divider(mut self, style: DividerStyle) -> Self {
        self.divider_style = style;
        self
    }

    /// Set divider color.
    pub fn divider_color(mut self, color: Color) -> Self {
        self.divider_color = Some(color);
        self
    }

    /// Set whether to show resize handle.
    pub fn show_handle(mut self, show: bool) -> Self {
        self.show_handle = show;
        self
    }

    /// Set total width.
    pub fn width(mut self, width: u16) -> Self {
        self.width = Some(width);
        self
    }

    /// Set total height.
    pub fn height(mut self, height: u16) -> Self {
        self.height = Some(height);
        self
    }

    /// Build the split panel VNode.
    pub fn build(self) -> VNode {
        let divider_color = self.divider_color.unwrap_or(Color::Named(NamedColor::Gray));

        // Calculate flex grow values based on position
        let first_flex = self.position as f32 / 100.0;
        let second_flex = (100 - self.position) as f32 / 100.0;

        // Extract values before consuming self.first/second
        let orientation = self.orientation;
        let min_first = self.min_first;
        let min_second = self.min_second;
        let width = self.width;
        let height = self.height;

        // Build divider first (before consuming first/second)
        let divider = self.build_divider(divider_color);

        // First pane
        let first_pane = VNode::Box(BoxNode {
            id: None,
            style: BoxStyle {
                flex_direction: Some(FlexDirection::Column),
                flex_grow: Some(first_flex),
                flex_shrink: Some(1.0),
                min_width: if orientation == SplitOrientation::Horizontal {
                    Some(min_first as u16)
                } else {
                    None
                },
                min_height: if orientation == SplitOrientation::Vertical {
                    Some(min_first as u16)
                } else {
                    None
                },
                ..Default::default()
            },
            children: self.first.map(|n| vec![n]).unwrap_or_default(),
            handlers: Default::default(),
        });

        // Second pane
        let second_pane = VNode::Box(BoxNode {
            id: None,
            style: BoxStyle {
                flex_direction: Some(FlexDirection::Column),
                flex_grow: Some(second_flex),
                flex_shrink: Some(1.0),
                min_width: if orientation == SplitOrientation::Horizontal {
                    Some(min_second as u16)
                } else {
                    None
                },
                min_height: if orientation == SplitOrientation::Vertical {
                    Some(min_second as u16)
                } else {
                    None
                },
                ..Default::default()
            },
            children: self.second.map(|n| vec![n]).unwrap_or_default(),
            handlers: Default::default(),
        });

        // Container
        let direction = match orientation {
            SplitOrientation::Horizontal => FlexDirection::Row,
            SplitOrientation::Vertical => FlexDirection::Column,
        };

        VNode::Box(BoxNode {
            id: None,
            style: BoxStyle {
                flex_direction: Some(direction),
                flex_grow: Some(1.0),
                width: width.map(Size::Fixed),
                height: height.map(Size::Fixed),
                ..Default::default()
            },
            children: vec![first_pane, divider, second_pane],
            handlers: Default::default(),
        })
    }

    /// Build the divider.
    fn build_divider(&self, color: Color) -> VNode {
        let divider_char = match self.orientation {
            SplitOrientation::Horizontal => self.divider_style.h_char(),
            SplitOrientation::Vertical => self.divider_style.v_char(),
        };

        // Handle indicator in the middle
        let handle = if self.show_handle {
            match self.orientation {
                SplitOrientation::Horizontal => "┃",
                SplitOrientation::Vertical => "━",
            }
        } else {
            ""
        };

        let content = if self.show_handle {
            format!("{}", handle)
        } else {
            divider_char.to_string()
        };

        let (width, height, direction, align) = match self.orientation {
            SplitOrientation::Horizontal => (
                Some(Size::Fixed(1)),
                None,
                FlexDirection::Column,
                AlignItems::Center,
            ),
            SplitOrientation::Vertical => (
                None,
                Some(Size::Fixed(1)),
                FlexDirection::Row,
                AlignItems::Center,
            ),
        };

        VNode::Box(BoxNode {
            id: None,
            style: BoxStyle {
                flex_direction: Some(direction),
                justify_content: Some(JustifyContent::Center),
                align_items: Some(align),
                width,
                height,
                ..Default::default()
            },
            children: vec![VNode::Text(TextNode {
                content,
                style: TextStyle {
                    color: Some(color),
                    ..Default::default()
                },
            })],
            handlers: EventHandlers {
                focusable: true, // For drag resize
                ..Default::default()
            },
        })
    }
}

impl From<SplitPanel> for VNode {
    fn from(panel: SplitPanel) -> VNode {
        panel.build()
    }
}

// =============================================================================
// SplitPanel State
// =============================================================================

/// State manager for SplitPanel.
///
/// Provides reactive position control.
pub struct SplitPanelState {
    /// Split position (percentage 0-100)
    position_read: ReadSignal<u8>,
    position_write: WriteSignal<u8>,
    /// Minimum for first pane
    min_first: u8,
    /// Minimum for second pane
    min_second: u8,
    /// Is dragging
    dragging_read: ReadSignal<bool>,
    dragging_write: WriteSignal<bool>,
}

impl SplitPanelState {
    /// Create a new split panel state.
    pub fn new(initial_position: u8) -> Self {
        let (position_read, position_write) = create_signal(initial_position.min(100));
        let (dragging_read, dragging_write) = create_signal(false);
        Self {
            position_read,
            position_write,
            min_first: 10,
            min_second: 10,
            dragging_read,
            dragging_write,
        }
    }

    /// Create with 50/50 split.
    pub fn half() -> Self {
        Self::new(50)
    }

    /// Get current position.
    pub fn position(&self) -> u8 {
        self.position_read.get()
    }

    /// Set position.
    pub fn set_position(&self, position: u8) {
        let clamped = position
            .max(self.min_first)
            .min(100 - self.min_second);
        self.position_write.set(clamped);
    }

    /// Set minimum constraints.
    pub fn set_constraints(&mut self, min_first: u8, min_second: u8) {
        self.min_first = min_first;
        self.min_second = min_second;
        // Re-clamp position
        let current = self.position_read.get();
        self.set_position(current);
    }

    /// Increase first pane size.
    pub fn grow_first(&self, amount: u8) {
        let current = self.position_read.get();
        self.set_position(current.saturating_add(amount));
    }

    /// Decrease first pane size.
    pub fn shrink_first(&self, amount: u8) {
        let current = self.position_read.get();
        self.set_position(current.saturating_sub(amount));
    }

    /// Reset to initial position.
    pub fn reset(&self, position: u8) {
        self.set_position(position);
    }

    /// Start dragging.
    pub fn start_drag(&self) {
        self.dragging_write.set(true);
    }

    /// Stop dragging.
    pub fn stop_drag(&self) {
        self.dragging_write.set(false);
    }

    /// Check if dragging.
    pub fn is_dragging(&self) -> bool {
        self.dragging_read.get()
    }

    /// Get the position signal.
    pub fn signal(&self) -> ReadSignal<u8> {
        self.position_read.clone()
    }
}

impl Default for SplitPanelState {
    fn default() -> Self {
        Self::half()
    }
}

// =============================================================================
// ThreePanel (convenience)
// =============================================================================

/// Three-panel layout (sidebar + main + right).
///
/// A convenience wrapper around nested SplitPanels.
#[derive(Debug, Clone, Default)]
pub struct ThreePanel {
    /// Left sidebar
    left: Option<VNode>,
    /// Main content (center)
    center: Option<VNode>,
    /// Right panel
    right: Option<VNode>,
    /// Left panel width (percentage)
    left_width: u8,
    /// Right panel width (percentage)
    right_width: u8,
}

impl ThreePanel {
    /// Create a new three-panel layout.
    pub fn new() -> Self {
        Self {
            left: None,
            center: None,
            right: None,
            left_width: 20,
            right_width: 20,
        }
    }

    /// Set left panel content.
    pub fn left(mut self, content: VNode) -> Self {
        self.left = Some(content);
        self
    }

    /// Set center content.
    pub fn center(mut self, content: VNode) -> Self {
        self.center = Some(content);
        self
    }

    /// Set right panel content.
    pub fn right(mut self, content: VNode) -> Self {
        self.right = Some(content);
        self
    }

    /// Set left panel width.
    pub fn left_width(mut self, width: u8) -> Self {
        self.left_width = width;
        self
    }

    /// Set right panel width.
    pub fn right_width(mut self, width: u8) -> Self {
        self.right_width = width;
        self
    }

    /// Build the three-panel layout.
    pub fn build(self) -> VNode {
        // First, split left | (center + right)
        let left_pane = self.left.unwrap_or(VNode::Empty);

        // Inner split: center | right
        let inner = if self.right.is_some() {
            let right_pos = 100 - self.right_width;
            SplitPanel::new()
                .horizontal()
                .first(self.center.unwrap_or(VNode::Empty))
                .second(self.right.unwrap())
                .position(right_pos)
                .build()
        } else {
            self.center.unwrap_or(VNode::Empty)
        };

        // Outer split
        SplitPanel::new()
            .horizontal()
            .first(left_pane)
            .second(inner)
            .position(self.left_width)
            .build()
    }
}

impl From<ThreePanel> for VNode {
    fn from(panel: ThreePanel) -> VNode {
        panel.build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_panel_horizontal() {
        let panel = SplitPanel::new()
            .horizontal()
            .first(VNode::text("Left"))
            .second(VNode::text("Right"))
            .position(30)
            .build();

        matches!(panel, VNode::Box(_));
    }

    #[test]
    fn test_split_panel_vertical() {
        let panel = SplitPanel::new()
            .vertical()
            .first(VNode::text("Top"))
            .second(VNode::text("Bottom"))
            .position(50)
            .build();

        matches!(panel, VNode::Box(_));
    }

    #[test]
    fn test_split_state() {
        let state = SplitPanelState::half();
        assert_eq!(state.position(), 50);

        state.grow_first(10);
        assert_eq!(state.position(), 60);

        state.shrink_first(20);
        assert_eq!(state.position(), 40);

        // Test clamping
        state.set_position(5); // Below min_first (10)
        assert_eq!(state.position(), 10);
    }

    #[test]
    fn test_three_panel() {
        let panel = ThreePanel::new()
            .left(VNode::text("Sidebar"))
            .center(VNode::text("Content"))
            .right(VNode::text("Details"))
            .left_width(20)
            .right_width(25)
            .build();

        matches!(panel, VNode::Box(_));
    }

    #[test]
    fn test_divider_styles() {
        assert_eq!(DividerStyle::Line.h_char(), '│');
        assert_eq!(DividerStyle::Double.h_char(), '║');
        assert_eq!(DividerStyle::Dashed.h_char(), '┊');
    }
}
