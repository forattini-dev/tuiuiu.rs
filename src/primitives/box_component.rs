//! Box Component
//!
//! A container component with flexbox layout capabilities.

use crate::core::component::{VNode, BoxNode, BoxStyle, Color, BorderStyle, Child, children_to_vnodes};
use crate::core::layout::{FlexDirection, JustifyContent, AlignItems, FlexWrap, Size};

/// Box component builder.
#[derive(Debug, Clone, Default)]
pub struct BoxComponent {
    style: BoxStyle,
    children: Vec<VNode>,
    id: Option<u64>,
}

impl BoxComponent {
    /// Create a new box.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set an ID for this box.
    pub fn id(mut self, id: u64) -> Self {
        self.id = Some(id);
        self
    }

    // === Layout Properties ===

    /// Set flex direction.
    pub fn flex_direction(mut self, direction: FlexDirection) -> Self {
        self.style.flex_direction = Some(direction);
        self
    }

    /// Shorthand for column direction.
    pub fn column(self) -> Self {
        self.flex_direction(FlexDirection::Column)
    }

    /// Shorthand for row direction.
    pub fn row(self) -> Self {
        self.flex_direction(FlexDirection::Row)
    }

    /// Set justify content (main axis alignment).
    pub fn justify_content(mut self, value: JustifyContent) -> Self {
        self.style.justify_content = Some(value);
        self
    }

    /// Set align items (cross axis alignment).
    pub fn align_items(mut self, value: AlignItems) -> Self {
        self.style.align_items = Some(value);
        self
    }

    /// Set flex wrap.
    pub fn flex_wrap(mut self, value: FlexWrap) -> Self {
        self.style.flex_wrap = Some(value);
        self
    }

    /// Set gap between children.
    pub fn gap(mut self, value: u16) -> Self {
        self.style.gap = Some(value);
        self
    }

    /// Set flex grow.
    pub fn flex_grow(mut self, value: f32) -> Self {
        self.style.flex_grow = Some(value);
        self
    }

    /// Set flex shrink.
    pub fn flex_shrink(mut self, value: f32) -> Self {
        self.style.flex_shrink = Some(value);
        self
    }

    // === Sizing ===

    /// Set width.
    pub fn width(mut self, value: u16) -> Self {
        self.style.width = Some(Size::Fixed(value));
        self
    }

    /// Set width as percentage.
    pub fn width_percent(mut self, value: f32) -> Self {
        self.style.width = Some(Size::Percent(value));
        self
    }

    /// Set width to fill available space.
    pub fn width_fill(mut self) -> Self {
        self.style.width = Some(Size::Fill);
        self
    }

    /// Set height.
    pub fn height(mut self, value: u16) -> Self {
        self.style.height = Some(Size::Fixed(value));
        self
    }

    /// Set height as percentage.
    pub fn height_percent(mut self, value: f32) -> Self {
        self.style.height = Some(Size::Percent(value));
        self
    }

    /// Set height to fill available space.
    pub fn height_fill(mut self) -> Self {
        self.style.height = Some(Size::Fill);
        self
    }

    /// Set minimum width.
    pub fn min_width(mut self, value: u16) -> Self {
        self.style.min_width = Some(value);
        self
    }

    /// Set minimum height.
    pub fn min_height(mut self, value: u16) -> Self {
        self.style.min_height = Some(value);
        self
    }

    /// Set maximum width.
    pub fn max_width(mut self, value: u16) -> Self {
        self.style.max_width = Some(value);
        self
    }

    /// Set maximum height.
    pub fn max_height(mut self, value: u16) -> Self {
        self.style.max_height = Some(value);
        self
    }

    // === Padding ===

    /// Set padding on all sides.
    pub fn padding(mut self, value: u16) -> Self {
        self.style.padding = Some(value);
        self
    }

    /// Set horizontal padding.
    pub fn padding_x(mut self, value: u16) -> Self {
        self.style.padding_left = Some(value);
        self.style.padding_right = Some(value);
        self
    }

    /// Set vertical padding.
    pub fn padding_y(mut self, value: u16) -> Self {
        self.style.padding_top = Some(value);
        self.style.padding_bottom = Some(value);
        self
    }

    /// Set top padding.
    pub fn padding_top(mut self, value: u16) -> Self {
        self.style.padding_top = Some(value);
        self
    }

    /// Set right padding.
    pub fn padding_right(mut self, value: u16) -> Self {
        self.style.padding_right = Some(value);
        self
    }

    /// Set bottom padding.
    pub fn padding_bottom(mut self, value: u16) -> Self {
        self.style.padding_bottom = Some(value);
        self
    }

    /// Set left padding.
    pub fn padding_left(mut self, value: u16) -> Self {
        self.style.padding_left = Some(value);
        self
    }

    // === Margin ===

    /// Set margin on all sides.
    pub fn margin(mut self, value: u16) -> Self {
        self.style.margin = Some(value);
        self
    }

    // === Border ===

    /// Set border style.
    pub fn border(mut self, style: BorderStyle) -> Self {
        self.style.border_style = Some(style);
        self
    }

    /// Shorthand for single border.
    pub fn border_single(self) -> Self {
        self.border(BorderStyle::Single)
    }

    /// Shorthand for double border.
    pub fn border_double(self) -> Self {
        self.border(BorderStyle::Double)
    }

    /// Shorthand for round border.
    pub fn border_round(self) -> Self {
        self.border(BorderStyle::Round)
    }

    /// Shorthand for bold border.
    pub fn border_bold(self) -> Self {
        self.border(BorderStyle::Bold)
    }

    /// Set border color.
    pub fn border_color(mut self, color: Color) -> Self {
        self.style.border_color = Some(color);
        self
    }

    // === Colors ===

    /// Set background color.
    pub fn background(mut self, color: Color) -> Self {
        self.style.background = Some(color);
        self
    }

    /// Set background to a named color.
    pub fn bg(mut self, color: crate::core::component::NamedColor) -> Self {
        self.style.background = Some(Color::Named(color));
        self
    }

    /// Set background to RGB.
    pub fn bg_rgb(mut self, r: u8, g: u8, b: u8) -> Self {
        self.style.background = Some(Color::Rgb(r, g, b));
        self
    }

    // === Children ===

    /// Add children to this box.
    pub fn children<I, C>(mut self, children: I) -> Self
    where
        I: IntoIterator<Item = C>,
        C: Into<Child>,
    {
        self.children = children_to_vnodes(children);
        self
    }

    /// Add a single child.
    pub fn child<C: Into<Child>>(mut self, child: C) -> Self {
        self.children.extend(children_to_vnodes([child]));
        self
    }

    /// Build into a VNode.
    pub fn build(self) -> VNode {
        VNode::Box(BoxNode {
            id: self.id,
            style: self.style,
            children: self.children,
            handlers: Default::default(),
        })
    }
}

impl From<BoxComponent> for VNode {
    fn from(b: BoxComponent) -> VNode {
        b.build()
    }
}

/// Create a box with default settings.
pub fn box_() -> BoxComponent {
    BoxComponent::new()
}

/// Create a column (vertical box).
pub fn column() -> BoxComponent {
    BoxComponent::new().column()
}

/// Create a row (horizontal box).
pub fn row() -> BoxComponent {
    BoxComponent::new().row()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_box_builder() {
        let b = BoxComponent::new()
            .column()
            .padding(1)
            .gap(2)
            .border_round();

        assert!(matches!(b.style.flex_direction, Some(FlexDirection::Column)));
        assert_eq!(b.style.padding, Some(1));
        assert_eq!(b.style.gap, Some(2));
        assert!(matches!(b.style.border_style, Some(BorderStyle::Round)));
    }

    #[test]
    fn test_box_with_children() {
        use crate::primitives::Text;

        let b = BoxComponent::new()
            .children([
                Text::new("Hello").build(),
                Text::new("World").build(),
            ]);

        assert_eq!(b.children.len(), 2);
    }
}
