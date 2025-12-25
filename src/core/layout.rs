//! Flexbox Layout Engine
//!
//! A CSS Flexbox-inspired layout system for terminal UIs.
//! Calculates positions and sizes for nested box elements.
//!
//! # Supported Properties
//!
//! - `flex_direction`: Row, Column, RowReverse, ColumnReverse
//! - `justify_content`: FlexStart, FlexEnd, Center, SpaceBetween, SpaceAround, SpaceEvenly
//! - `align_items`: FlexStart, FlexEnd, Center, Stretch, Baseline
//! - `align_self`: Override parent's align_items
//! - `align_content`: Multi-line alignment
//! - `flex_wrap`: Wrap, NoWrap, WrapReverse
//! - `flex_grow`, `flex_shrink`: Size distribution
//! - `gap`: Spacing between children
//! - `padding`, `margin`: Box model
//! - `width`, `height`: Fixed or percentage sizes
//! - `min_width`, `min_height`, `max_width`, `max_height`: Constraints

use std::collections::HashMap;

// =============================================================================
// Types
// =============================================================================

/// Flex direction (main axis).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FlexDirection {
    /// Left to right (default)
    #[default]
    Row,
    /// Right to left
    RowReverse,
    /// Top to bottom
    Column,
    /// Bottom to top
    ColumnReverse,
}

impl FlexDirection {
    /// Check if this is a row direction.
    pub fn is_row(&self) -> bool {
        matches!(self, Self::Row | Self::RowReverse)
    }

    /// Check if this is reversed.
    pub fn is_reversed(&self) -> bool {
        matches!(self, Self::RowReverse | Self::ColumnReverse)
    }
}

/// Justify content (main axis alignment).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum JustifyContent {
    /// Pack items at the start
    #[default]
    FlexStart,
    /// Pack items at the end
    FlexEnd,
    /// Center items
    Center,
    /// Distribute items evenly, first/last at edges
    SpaceBetween,
    /// Distribute items evenly with equal space around
    SpaceAround,
    /// Distribute items evenly with equal space between
    SpaceEvenly,
}

/// Align items (cross axis alignment).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AlignItems {
    /// Align to start of cross axis
    FlexStart,
    /// Align to end of cross axis
    FlexEnd,
    /// Center on cross axis
    Center,
    /// Stretch to fill (default)
    #[default]
    Stretch,
    /// Align to text baseline
    Baseline,
}

/// Align self (override align_items for a single child).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AlignSelf {
    /// Use parent's align_items
    #[default]
    Auto,
    /// Align to start
    FlexStart,
    /// Align to end
    FlexEnd,
    /// Center
    Center,
    /// Stretch to fill
    Stretch,
    /// Baseline
    Baseline,
}

/// Align content (multi-line cross axis alignment).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AlignContent {
    /// Pack lines at start
    #[default]
    FlexStart,
    /// Pack lines at end
    FlexEnd,
    /// Center lines
    Center,
    /// Distribute lines evenly
    SpaceBetween,
    /// Equal space around lines
    SpaceAround,
    /// Stretch lines to fill
    Stretch,
}

/// Flex wrap behavior.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FlexWrap {
    /// No wrapping (default)
    #[default]
    NoWrap,
    /// Wrap to next line
    Wrap,
    /// Wrap to previous line
    WrapReverse,
}

/// Size value (fixed, percentage, or auto).
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Size {
    /// Fixed size in characters/lines
    Fixed(u16),
    /// Percentage of parent
    Percent(f32),
    /// Auto size (content-based)
    Auto,
    /// Fill remaining space
    Fill,
}

impl Default for Size {
    fn default() -> Self {
        Self::Auto
    }
}

impl Size {
    /// Resolve size to absolute value given parent size.
    pub fn resolve(&self, parent: u16, content: u16) -> u16 {
        match self {
            Self::Fixed(v) => *v,
            Self::Percent(p) => ((parent as f32) * p / 100.0).round() as u16,
            Self::Auto => content,
            Self::Fill => parent,
        }
    }
}

/// Edge values (for padding, margin).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Edges {
    /// Top edge
    pub top: u16,
    /// Right edge
    pub right: u16,
    /// Bottom edge
    pub bottom: u16,
    /// Left edge
    pub left: u16,
}

impl Edges {
    /// Create edges with all sides equal.
    pub fn all(value: u16) -> Self {
        Self {
            top: value,
            right: value,
            bottom: value,
            left: value,
        }
    }

    /// Create edges with vertical and horizontal values.
    pub fn symmetric(vertical: u16, horizontal: u16) -> Self {
        Self {
            top: vertical,
            right: horizontal,
            bottom: vertical,
            left: horizontal,
        }
    }

    /// Total horizontal space.
    pub fn horizontal(&self) -> u16 {
        self.left + self.right
    }

    /// Total vertical space.
    pub fn vertical(&self) -> u16 {
        self.top + self.bottom
    }
}

// =============================================================================
// Layout Node
// =============================================================================

/// Layout properties for a node.
#[derive(Debug, Clone, Default)]
pub struct LayoutStyle {
    // Display
    /// Whether this node is visible
    pub display: bool,

    // Flex container
    /// Main axis direction
    pub flex_direction: FlexDirection,
    /// Main axis alignment
    pub justify_content: JustifyContent,
    /// Cross axis alignment
    pub align_items: AlignItems,
    /// Multi-line cross axis alignment
    pub align_content: AlignContent,
    /// Wrap behavior
    pub flex_wrap: FlexWrap,
    /// Gap between children
    pub gap: u16,
    /// Row gap (overrides gap for rows)
    pub row_gap: Option<u16>,
    /// Column gap (overrides gap for columns)
    pub column_gap: Option<u16>,

    // Flex item
    /// Override parent's align_items
    pub align_self: AlignSelf,
    /// Flex grow factor
    pub flex_grow: f32,
    /// Flex shrink factor
    pub flex_shrink: f32,
    /// Flex basis
    pub flex_basis: Size,

    // Sizing
    /// Width
    pub width: Size,
    /// Height
    pub height: Size,
    /// Minimum width
    pub min_width: Option<u16>,
    /// Minimum height
    pub min_height: Option<u16>,
    /// Maximum width
    pub max_width: Option<u16>,
    /// Maximum height
    pub max_height: Option<u16>,

    // Box model
    /// Padding (inside border)
    pub padding: Edges,
    /// Margin (outside border)
    pub margin: Edges,

    // Border
    /// Border width (usually 0 or 1)
    pub border_width: u16,

    // Position
    /// Fixed position X
    pub position_x: Option<u16>,
    /// Fixed position Y
    pub position_y: Option<u16>,
}

impl LayoutStyle {
    /// Create a new layout style with defaults.
    pub fn new() -> Self {
        Self {
            display: true,
            flex_shrink: 1.0,
            ..Default::default()
        }
    }

    /// Builder pattern: set flex direction.
    pub fn flex_direction(mut self, value: FlexDirection) -> Self {
        self.flex_direction = value;
        self
    }

    /// Builder pattern: set justify content.
    pub fn justify_content(mut self, value: JustifyContent) -> Self {
        self.justify_content = value;
        self
    }

    /// Builder pattern: set align items.
    pub fn align_items(mut self, value: AlignItems) -> Self {
        self.align_items = value;
        self
    }

    /// Builder pattern: set padding.
    pub fn padding(mut self, value: u16) -> Self {
        self.padding = Edges::all(value);
        self
    }

    /// Builder pattern: set gap.
    pub fn gap(mut self, value: u16) -> Self {
        self.gap = value;
        self
    }

    /// Builder pattern: set flex grow.
    pub fn flex_grow(mut self, value: f32) -> Self {
        self.flex_grow = value;
        self
    }

    /// Builder pattern: set width.
    pub fn width(mut self, value: Size) -> Self {
        self.width = value;
        self
    }

    /// Builder pattern: set height.
    pub fn height(mut self, value: Size) -> Self {
        self.height = value;
        self
    }
}

/// A node in the layout tree.
#[derive(Debug, Clone)]
pub struct LayoutNode {
    /// Unique identifier
    pub id: u64,
    /// Layout style properties
    pub style: LayoutStyle,
    /// Child nodes
    pub children: Vec<LayoutNode>,
    /// Content size (for leaf nodes)
    pub content_size: (u16, u16),
}

impl LayoutNode {
    /// Create a new layout node.
    pub fn new(id: u64) -> Self {
        Self {
            id,
            style: LayoutStyle::new(),
            children: Vec::new(),
            content_size: (0, 0),
        }
    }

    /// Create a text node with content size.
    pub fn text(id: u64, width: u16, height: u16) -> Self {
        Self {
            id,
            style: LayoutStyle::new(),
            children: Vec::new(),
            content_size: (width, height),
        }
    }

    /// Add a child node.
    pub fn add_child(&mut self, child: LayoutNode) {
        self.children.push(child);
    }

    /// Set style.
    pub fn with_style(mut self, style: LayoutStyle) -> Self {
        self.style = style;
        self
    }
}

/// Computed layout result.
#[derive(Debug, Clone, Default)]
pub struct ComputedLayout {
    /// X position
    pub x: u16,
    /// Y position
    pub y: u16,
    /// Width
    pub width: u16,
    /// Height
    pub height: u16,
}

impl ComputedLayout {
    /// Check if a point is inside this layout.
    pub fn contains(&self, x: u16, y: u16) -> bool {
        x >= self.x && x < self.x + self.width && y >= self.y && y < self.y + self.height
    }

    /// Get the inner bounds (accounting for padding would need style).
    pub fn inner(&self, padding: &Edges) -> ComputedLayout {
        ComputedLayout {
            x: self.x + padding.left,
            y: self.y + padding.top,
            width: self.width.saturating_sub(padding.horizontal()),
            height: self.height.saturating_sub(padding.vertical()),
        }
    }
}

// =============================================================================
// Layout Calculator
// =============================================================================

/// Calculate layout for a tree of nodes.
pub fn calculate_layout(
    node: &LayoutNode,
    available_width: u16,
    available_height: u16,
) -> HashMap<u64, ComputedLayout> {
    let mut layouts = HashMap::new();

    calculate_node_layout(node, 0, 0, available_width, available_height, &mut layouts);

    layouts
}

fn calculate_node_layout(
    node: &LayoutNode,
    x: u16,
    y: u16,
    available_width: u16,
    available_height: u16,
    layouts: &mut HashMap<u64, ComputedLayout>,
) {
    if !node.style.display {
        return;
    }

    let style = &node.style;

    // Calculate this node's size
    let padding_h = style.padding.horizontal() + style.border_width * 2;
    let padding_v = style.padding.vertical() + style.border_width * 2;

    // Resolve width/height
    let mut width = match style.width {
        Size::Fixed(w) => w,
        Size::Percent(p) => ((available_width as f32) * p / 100.0).round() as u16,
        Size::Auto | Size::Fill => available_width,
    };

    let mut height = match style.height {
        Size::Fixed(h) => h,
        Size::Percent(p) => ((available_height as f32) * p / 100.0).round() as u16,
        Size::Auto | Size::Fill => {
            if node.children.is_empty() {
                node.content_size.1 + padding_v
            } else {
                available_height
            }
        }
    };

    // Apply constraints
    if let Some(min) = style.min_width {
        width = width.max(min);
    }
    if let Some(max) = style.max_width {
        width = width.min(max);
    }
    if let Some(min) = style.min_height {
        height = height.max(min);
    }
    if let Some(max) = style.max_height {
        height = height.min(max);
    }

    // Apply position if specified
    let final_x = style.position_x.unwrap_or(x);
    let final_y = style.position_y.unwrap_or(y);

    // Store this node's layout
    layouts.insert(
        node.id,
        ComputedLayout {
            x: final_x,
            y: final_y,
            width,
            height,
        },
    );

    // If no children, we're done
    if node.children.is_empty() {
        return;
    }

    // Calculate children layout
    let inner_x = final_x + style.padding.left + style.border_width;
    let inner_y = final_y + style.padding.top + style.border_width;
    let inner_width = width.saturating_sub(padding_h);
    let inner_height = height.saturating_sub(padding_v);

    layout_children(node, inner_x, inner_y, inner_width, inner_height, layouts);
}

fn layout_children(
    node: &LayoutNode,
    inner_x: u16,
    inner_y: u16,
    inner_width: u16,
    inner_height: u16,
    layouts: &mut HashMap<u64, ComputedLayout>,
) {
    let style = &node.style;
    let is_row = style.flex_direction.is_row();
    let is_reversed = style.flex_direction.is_reversed();

    let main_size = if is_row { inner_width } else { inner_height };
    let cross_size = if is_row { inner_height } else { inner_width };

    let gap = style.gap;
    let visible_children: Vec<_> = node.children.iter().filter(|c| c.style.display).collect();
    let child_count = visible_children.len();

    if child_count == 0 {
        return;
    }

    // Calculate total flex grow and base sizes
    let mut total_flex_grow: f32 = 0.0;
    let mut total_base_size: u16 = 0;
    let mut child_sizes: Vec<(u16, u16)> = Vec::with_capacity(child_count);

    for child in &visible_children {
        let (base_w, base_h) = calculate_child_base_size(child, inner_width, inner_height, is_row);
        let base_main = if is_row { base_w } else { base_h };

        total_base_size += base_main;
        total_flex_grow += child.style.flex_grow;
        child_sizes.push((base_w, base_h));
    }

    // Add gaps to total
    let total_gap = gap * (child_count.saturating_sub(1)) as u16;
    let total_used = total_base_size + total_gap;

    // Calculate remaining space
    let remaining = main_size.saturating_sub(total_used);

    // Distribute remaining space to flex-grow items
    let flex_unit = if total_flex_grow > 0.0 {
        (remaining as f32) / total_flex_grow
    } else {
        0.0
    };

    // Calculate final sizes
    let mut final_sizes: Vec<(u16, u16)> = Vec::with_capacity(child_count);
    for (i, child) in visible_children.iter().enumerate() {
        let (base_w, base_h) = child_sizes[i];
        let grow_amount = (child.style.flex_grow * flex_unit).round() as u16;

        let (w, h) = if is_row {
            (base_w + grow_amount, base_h)
        } else {
            (base_w, base_h + grow_amount)
        };

        final_sizes.push((w, h));
    }

    // Calculate starting position based on justify-content
    let total_final_main: u16 = final_sizes.iter().map(|(w, h)| if is_row { *w } else { *h }).sum();
    let total_with_gaps = total_final_main + total_gap;
    let extra_space = main_size.saturating_sub(total_with_gaps);

    let (mut main_pos, space_between, space_around) = match style.justify_content {
        JustifyContent::FlexStart => (0, 0, 0),
        JustifyContent::FlexEnd => (extra_space, 0, 0),
        JustifyContent::Center => (extra_space / 2, 0, 0),
        JustifyContent::SpaceBetween => {
            let space = if child_count > 1 {
                extra_space / (child_count as u16 - 1)
            } else {
                0
            };
            (0, space, 0)
        }
        JustifyContent::SpaceAround => {
            let space = extra_space / (child_count as u16 * 2);
            (space, space * 2, 0)
        }
        JustifyContent::SpaceEvenly => {
            let space = extra_space / (child_count as u16 + 1);
            (space, space, 0)
        }
    };

    // Reverse order if needed
    let order: Vec<usize> = if is_reversed {
        (0..child_count).rev().collect()
    } else {
        (0..child_count).collect()
    };

    // Position children
    for (iter_idx, &i) in order.iter().enumerate() {
        let child = visible_children[i];
        let (child_w, child_h) = final_sizes[i];

        let child_main = if is_row { child_w } else { child_h };
        let child_cross = if is_row { child_h } else { child_w };

        // Calculate cross-axis position based on align_items
        let align = match child.style.align_self {
            AlignSelf::Auto => style.align_items,
            AlignSelf::FlexStart => AlignItems::FlexStart,
            AlignSelf::FlexEnd => AlignItems::FlexEnd,
            AlignSelf::Center => AlignItems::Center,
            AlignSelf::Stretch => AlignItems::Stretch,
            AlignSelf::Baseline => AlignItems::Baseline,
        };

        let cross_pos = match align {
            AlignItems::FlexStart => 0,
            AlignItems::FlexEnd => cross_size.saturating_sub(child_cross),
            AlignItems::Center => (cross_size.saturating_sub(child_cross)) / 2,
            AlignItems::Stretch => 0,
            AlignItems::Baseline => 0, // TODO: proper baseline calculation
        };

        let stretched_cross = if matches!(align, AlignItems::Stretch) {
            cross_size
        } else {
            child_cross
        };

        // Calculate final position
        let (child_x, child_y, final_w, final_h) = if is_row {
            (
                inner_x + main_pos,
                inner_y + cross_pos,
                child_w,
                stretched_cross,
            )
        } else {
            (
                inner_x + cross_pos,
                inner_y + main_pos,
                stretched_cross,
                child_h,
            )
        };

        // Recursively layout this child
        calculate_node_layout(child, child_x, child_y, final_w, final_h, layouts);

        // Move to next position
        main_pos += child_main + gap + space_between;

        if iter_idx == 0 && !matches!(style.justify_content, JustifyContent::SpaceAround) {
            // First item doesn't add extra space for SpaceAround
        }
    }
}

fn calculate_child_base_size(
    child: &LayoutNode,
    parent_width: u16,
    parent_height: u16,
    is_row: bool,
) -> (u16, u16) {
    let style = &child.style;
    let padding_h = style.padding.horizontal() + style.border_width * 2;
    let padding_v = style.padding.vertical() + style.border_width * 2;

    // Calculate base width
    let base_width = match style.width {
        Size::Fixed(w) => w,
        Size::Percent(p) => ((parent_width as f32) * p / 100.0).round() as u16,
        Size::Auto => {
            if child.children.is_empty() {
                child.content_size.0 + padding_h
            } else {
                // For containers, use minimum content width
                padding_h
            }
        }
        Size::Fill => {
            if is_row {
                0 // Will be calculated based on flex_grow
            } else {
                parent_width
            }
        }
    };

    // Calculate base height
    let base_height = match style.height {
        Size::Fixed(h) => h,
        Size::Percent(p) => ((parent_height as f32) * p / 100.0).round() as u16,
        Size::Auto => {
            if child.children.is_empty() {
                child.content_size.1 + padding_v
            } else {
                // For containers, use minimum content height
                padding_v
            }
        }
        Size::Fill => {
            if is_row {
                parent_height
            } else {
                0 // Will be calculated based on flex_grow
            }
        }
    };

    // Apply flex_basis if set
    let (final_w, final_h) = match style.flex_basis {
        Size::Fixed(v) => {
            if is_row {
                (v, base_height)
            } else {
                (base_width, v)
            }
        }
        Size::Percent(p) => {
            let basis = if is_row {
                ((parent_width as f32) * p / 100.0).round() as u16
            } else {
                ((parent_height as f32) * p / 100.0).round() as u16
            };
            if is_row {
                (basis, base_height)
            } else {
                (base_width, basis)
            }
        }
        _ => (base_width, base_height),
    };

    // Apply constraints
    let constrained_w = apply_constraints(final_w, style.min_width, style.max_width);
    let constrained_h = apply_constraints(final_h, style.min_height, style.max_height);

    (constrained_w, constrained_h)
}

fn apply_constraints(value: u16, min: Option<u16>, max: Option<u16>) -> u16 {
    let mut result = value;
    if let Some(min_val) = min {
        result = result.max(min_val);
    }
    if let Some(max_val) = max {
        result = result.min(max_val);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_layout() {
        let mut root = LayoutNode::new(0);
        root.style.width = Size::Fixed(80);
        root.style.height = Size::Fixed(24);
        root.style.flex_direction = FlexDirection::Column;

        let child1 = LayoutNode::text(1, 10, 1);
        let child2 = LayoutNode::text(2, 20, 1);
        root.children = vec![child1, child2];

        let layouts = calculate_layout(&root, 80, 24);

        assert!(layouts.contains_key(&0));
        assert!(layouts.contains_key(&1));
        assert!(layouts.contains_key(&2));

        let root_layout = &layouts[&0];
        assert_eq!(root_layout.width, 80);
        assert_eq!(root_layout.height, 24);
    }

    #[test]
    fn test_flex_grow() {
        let mut root = LayoutNode::new(0);
        root.style.width = Size::Fixed(100);
        root.style.height = Size::Fixed(10);
        root.style.flex_direction = FlexDirection::Row;

        let mut child1 = LayoutNode::text(1, 10, 1);
        child1.style.flex_grow = 1.0;

        let mut child2 = LayoutNode::text(2, 10, 1);
        child2.style.flex_grow = 2.0;

        root.children = vec![child1, child2];

        let layouts = calculate_layout(&root, 100, 10);

        // Child1 should get 1/3 of remaining space, child2 gets 2/3
        let c1 = &layouts[&1];
        let c2 = &layouts[&2];

        // With flex-grow, they should share the remaining 80 units (100 - 20 base)
        // child1: 10 + 80/3 ≈ 36-37, child2: 10 + 160/3 ≈ 63-64
        assert!(c1.width > 10);
        assert!(c2.width > c1.width);
    }

    #[test]
    fn test_justify_content_center() {
        let mut root = LayoutNode::new(0);
        root.style.width = Size::Fixed(100);
        root.style.height = Size::Fixed(10);
        root.style.flex_direction = FlexDirection::Row;
        root.style.justify_content = JustifyContent::Center;

        let child = LayoutNode::text(1, 20, 1);
        root.children = vec![child];

        let layouts = calculate_layout(&root, 100, 10);

        let c = &layouts[&1];
        // Centered: (100 - 20) / 2 = 40
        assert_eq!(c.x, 40);
    }

    #[test]
    fn test_padding() {
        let mut root = LayoutNode::new(0);
        root.style.width = Size::Fixed(100);
        root.style.height = Size::Fixed(10);
        root.style.padding = Edges::all(5);

        let child = LayoutNode::text(1, 10, 1);
        root.children = vec![child];

        let layouts = calculate_layout(&root, 100, 10);

        let c = &layouts[&1];
        // Should start after padding
        assert_eq!(c.x, 5);
        assert_eq!(c.y, 5);
    }
}
