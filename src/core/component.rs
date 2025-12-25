//! Component Trait and Types
//!
//! Components are the building blocks of Tuiuiu applications.
//! They define what to render and how to respond to events.

use crate::core::layout::LayoutNode;

// =============================================================================
// Component Trait
// =============================================================================

/// The core component trait.
///
/// Components are functions or structs that return renderable content.
/// They can have state (via signals) and respond to events.
pub trait Component {
    /// Render this component to a layout node tree.
    fn render(&self) -> LayoutNode;

    /// Optional: Called when the component is mounted.
    fn on_mount(&mut self) {}

    /// Optional: Called when the component is unmounted.
    fn on_unmount(&mut self) {}

    /// Optional: Get the component's unique ID.
    fn id(&self) -> Option<u64> {
        None
    }
}

// Implement Component for closures that return LayoutNode
impl<F> Component for F
where
    F: Fn() -> LayoutNode,
{
    fn render(&self) -> LayoutNode {
        self()
    }
}

// =============================================================================
// VNode - Virtual Node
// =============================================================================

/// Virtual node types for the render tree.
#[derive(Debug, Clone)]
pub enum VNode {
    /// A box container
    Box(BoxNode),
    /// Text content
    Text(TextNode),
    /// Spacer element
    Spacer(SpacerNode),
    /// Fragment (multiple children, no wrapper)
    Fragment(Vec<VNode>),
    /// Empty/null node
    Empty,
}

/// Box node properties.
#[derive(Debug, Clone, Default)]
pub struct BoxNode {
    /// Unique ID
    pub id: Option<u64>,
    /// Style properties
    pub style: BoxStyle,
    /// Child nodes
    pub children: Vec<VNode>,
    /// Event handlers
    pub handlers: EventHandlers,
}

/// Box styling properties.
#[derive(Debug, Clone, Default)]
pub struct BoxStyle {
    // Layout
    /// Flex direction
    pub flex_direction: Option<crate::core::layout::FlexDirection>,
    /// Justify content
    pub justify_content: Option<crate::core::layout::JustifyContent>,
    /// Align items
    pub align_items: Option<crate::core::layout::AlignItems>,
    /// Flex wrap
    pub flex_wrap: Option<crate::core::layout::FlexWrap>,
    /// Gap between children
    pub gap: Option<u16>,
    /// Flex grow
    pub flex_grow: Option<f32>,
    /// Flex shrink
    pub flex_shrink: Option<f32>,

    // Sizing
    /// Width
    pub width: Option<crate::core::layout::Size>,
    /// Height
    pub height: Option<crate::core::layout::Size>,
    /// Min width
    pub min_width: Option<u16>,
    /// Min height
    pub min_height: Option<u16>,
    /// Max width
    pub max_width: Option<u16>,
    /// Max height
    pub max_height: Option<u16>,

    // Spacing
    /// Padding (all sides)
    pub padding: Option<u16>,
    /// Padding top
    pub padding_top: Option<u16>,
    /// Padding right
    pub padding_right: Option<u16>,
    /// Padding bottom
    pub padding_bottom: Option<u16>,
    /// Padding left
    pub padding_left: Option<u16>,
    /// Margin (all sides)
    pub margin: Option<u16>,

    // Border
    /// Border style
    pub border_style: Option<BorderStyle>,
    /// Border color
    pub border_color: Option<Color>,

    // Colors
    /// Background color
    pub background: Option<Color>,
}

/// Text node properties.
#[derive(Debug, Clone, Default)]
pub struct TextNode {
    /// Text content
    pub content: String,
    /// Style properties
    pub style: TextStyle,
}

/// Text styling properties.
#[derive(Debug, Clone, Default)]
pub struct TextStyle {
    /// Text color
    pub color: Option<Color>,
    /// Background color
    pub background: Option<Color>,
    /// Bold
    pub bold: bool,
    /// Italic
    pub italic: bool,
    /// Underline
    pub underline: bool,
    /// Strikethrough
    pub strikethrough: bool,
    /// Dim
    pub dim: bool,
    /// Inverse colors
    pub inverse: bool,
    /// Wrap mode
    pub wrap: Option<WrapMode>,
}

/// Spacer node properties.
#[derive(Debug, Clone, Default)]
pub struct SpacerNode {
    /// Horizontal space (characters)
    pub x: u16,
    /// Vertical space (lines)
    pub y: u16,
}

/// Event handlers for a node.
#[derive(Debug, Clone, Default)]
pub struct EventHandlers {
    /// Has click handler
    pub on_click: bool,
    /// Has hover handlers
    pub on_hover: bool,
    /// Has focus handlers
    pub on_focus: bool,
    /// Is focusable
    pub focusable: bool,
}

// =============================================================================
// Styling Types
// =============================================================================

/// Color value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    /// Default terminal color
    Default,
    /// Named color
    Named(NamedColor),
    /// ANSI 256 color
    Ansi256(u8),
    /// True color RGB
    Rgb(u8, u8, u8),
}

impl Default for Color {
    fn default() -> Self {
        Self::Default
    }
}

/// Named terminal colors.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NamedColor {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
    Gray,
}

/// Border style.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BorderStyle {
    #[default]
    None,
    Single,
    Double,
    Round,
    Bold,
    Dashed,
    Dotted,
    Hidden,
    Classic,
}

/// Text wrap mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum WrapMode {
    #[default]
    None,
    Word,
    Char,
    Truncate,
}

// =============================================================================
// Child Types
// =============================================================================

/// A child element that can be a component, node, or string.
pub enum Child {
    /// A VNode
    Node(VNode),
    /// A string (converted to Text)
    Text(String),
    /// A vector of children
    Many(Vec<Child>),
    /// Empty child
    Empty,
}

impl From<VNode> for Child {
    fn from(node: VNode) -> Self {
        Child::Node(node)
    }
}

impl From<String> for Child {
    fn from(s: String) -> Self {
        Child::Text(s)
    }
}

impl From<&str> for Child {
    fn from(s: &str) -> Self {
        Child::Text(s.to_string())
    }
}

impl<T: Into<Child>> From<Vec<T>> for Child {
    fn from(v: Vec<T>) -> Self {
        Child::Many(v.into_iter().map(Into::into).collect())
    }
}

impl From<()> for Child {
    fn from(_: ()) -> Self {
        Child::Empty
    }
}

/// Convert children to VNodes.
pub fn children_to_vnodes(children: impl IntoIterator<Item = impl Into<Child>>) -> Vec<VNode> {
    children
        .into_iter()
        .flat_map(|c| child_to_vnodes(c.into()))
        .collect()
}

fn child_to_vnodes(child: Child) -> Vec<VNode> {
    match child {
        Child::Node(node) => vec![node],
        Child::Text(s) => vec![VNode::Text(TextNode {
            content: s,
            style: TextStyle::default(),
        })],
        Child::Many(children) => children.into_iter().flat_map(child_to_vnodes).collect(),
        Child::Empty => vec![],
    }
}

// =============================================================================
// VNode Helpers
// =============================================================================

impl VNode {
    /// Create a text node with content only.
    pub fn text(content: impl Into<String>) -> Self {
        VNode::Text(TextNode {
            content: content.into(),
            style: TextStyle::default(),
        })
    }

    /// Create a styled text node.
    pub fn styled_text(content: impl Into<String>, style: TextStyle) -> Self {
        VNode::Text(TextNode {
            content: content.into(),
            style,
        })
    }

    /// Create a box with children.
    pub fn container(children: Vec<VNode>) -> Self {
        VNode::Box(BoxNode {
            children,
            ..Default::default()
        })
    }

    /// Create a column box.
    pub fn column(children: Vec<VNode>) -> Self {
        VNode::Box(BoxNode {
            children,
            style: BoxStyle {
                flex_direction: Some(crate::core::layout::FlexDirection::Column),
                ..Default::default()
            },
            ..Default::default()
        })
    }

    /// Create a row box.
    pub fn row(children: Vec<VNode>) -> Self {
        VNode::Box(BoxNode {
            children,
            style: BoxStyle {
                flex_direction: Some(crate::core::layout::FlexDirection::Row),
                ..Default::default()
            },
            ..Default::default()
        })
    }
}

impl TextNode {
    /// Create a new text node.
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            style: TextStyle::default(),
        }
    }

    /// Create with color.
    pub fn with_color(content: impl Into<String>, color: Color) -> Self {
        Self {
            content: content.into(),
            style: TextStyle {
                color: Some(color),
                ..Default::default()
            },
        }
    }

    /// Create bold text.
    pub fn bold(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            style: TextStyle {
                bold: true,
                ..Default::default()
            },
        }
    }

    /// Create colored bold text.
    pub fn colored_bold(content: impl Into<String>, color: Color) -> Self {
        Self {
            content: content.into(),
            style: TextStyle {
                color: Some(color),
                bold: true,
                ..Default::default()
            },
        }
    }
}

impl TextStyle {
    /// Create with just a color.
    pub fn color(color: Color) -> Self {
        Self {
            color: Some(color),
            ..Default::default()
        }
    }

    /// Create bold style.
    pub fn bold() -> Self {
        Self {
            bold: true,
            ..Default::default()
        }
    }

    /// Create with color and bold.
    pub fn color_bold(color: Color) -> Self {
        Self {
            color: Some(color),
            bold: true,
            ..Default::default()
        }
    }

    /// Create dim style.
    pub fn dim() -> Self {
        Self {
            dim: true,
            ..Default::default()
        }
    }
}

impl BoxStyle {
    /// Create column layout style.
    pub fn column() -> Self {
        Self {
            flex_direction: Some(crate::core::layout::FlexDirection::Column),
            ..Default::default()
        }
    }

    /// Create row layout style.
    pub fn row() -> Self {
        Self {
            flex_direction: Some(crate::core::layout::FlexDirection::Row),
            ..Default::default()
        }
    }

    /// Set border style.
    pub fn with_border(mut self, border: BorderStyle) -> Self {
        self.border_style = Some(border);
        self
    }

    /// Set padding all sides.
    pub fn with_padding(mut self, padding: u16) -> Self {
        self.padding = Some(padding);
        self
    }

    /// Set gap.
    pub fn with_gap(mut self, gap: u16) -> Self {
        self.gap = Some(gap);
        self
    }
}
