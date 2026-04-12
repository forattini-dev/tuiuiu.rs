//! Scroll Panel Component
//! 
//! Panel wrapper that combines an optional title with a [`ScrollArea`].

use crate::core::component::{BoxNode, BoxStyle, TextStyle, VNode};
use crate::core::layout::{AlignItems, FlexDirection};
use crate::organisms::scroll_area::{ScrollArea, ScrollbarVisibility};

/// Height options for [`ScrollPanel`].
#[derive(Debug, Clone, Copy)]
pub enum ScrollPanelHeight {
    /// Auto height.
    Auto,
    /// Fill available space.
    Fill,
    /// Fixed height.
    Fixed(u16),
}

/// Properties for creating a [`ScrollPanel`].
#[derive(Debug, Clone)]
pub struct ScrollPanelProps {
    /// Optional panel title.
    pub title: Option<String>,
    /// Scrollable content.
    pub content: Vec<VNode>,
    /// Fixed panel width.
    pub width: Option<crate::core::layout::Size>,
    /// Panel height.
    pub height: Option<ScrollPanelHeight>,
    /// Minimum height.
    pub min_height: Option<u16>,
    /// Maximum height.
    pub max_height: Option<u16>,
    /// Flex grow.
    pub flex_grow: Option<f32>,
    /// Border style.
    pub border_style: Option<crate::core::component::BorderStyle>,
    /// Border color.
    pub border_color: Option<crate::core::component::Color>,
    /// Title color.
    pub title_color: Option<crate::core::component::Color>,
    /// Show scrollbar.
    pub show_scrollbar: bool,
    /// Scrollbar color.
    pub scrollbar_color: Option<crate::core::component::Color>,
    /// Scrollbar track color.
    pub track_color: Option<crate::core::component::Color>,
    /// Is panel active.
    pub is_active: bool,
}

impl Default for ScrollPanelProps {
    fn default() -> Self {
        Self {
            title: None,
            content: Vec::new(),
            width: None,
            height: Some(ScrollPanelHeight::Fill),
            min_height: None,
            max_height: None,
            flex_grow: None,
            border_style: Some(crate::core::component::BorderStyle::Round),
            border_color: Some(crate::core::component::Color::Named(
                crate::core::component::NamedColor::Gray,
            )),
            title_color: Some(crate::core::component::Color::Named(
                crate::core::component::NamedColor::Gray,
            )),
            show_scrollbar: true,
            scrollbar_color: None,
            track_color: None,
            is_active: true,
        }
    }
}

/// Render a scrollable panel with an optional title.
pub fn ScrollPanel(props: ScrollPanelProps) -> VNode {
    let mut children = Vec::new();

    if let Some(title) = props.title {
        children.push(VNode::styled_text(
            title,
            TextStyle {
                color: props.title_color,
                dim: true,
                ..Default::default()
            },
        ));
    }

    let height = match props.height.unwrap_or(ScrollPanelHeight::Fill) {
        ScrollPanelHeight::Auto | ScrollPanelHeight::Fill => 10,
        ScrollPanelHeight::Fixed(lines) => lines,
    };

    let mut area = ScrollArea::new()
        .content(props.content)
        .height(height)
        .scrollbar(if props.show_scrollbar {
            ScrollbarVisibility::Auto
        } else {
            ScrollbarVisibility::Never
        });

    if props.scrollbar_color.is_some() {
        area = area.scrollbar_color(props.scrollbar_color);
    }
    if props.track_color.is_some() {
        area = area.track_color(props.track_color);
    }

    children.push(area.build());

    let mut style = BoxStyle {
        flex_direction: Some(FlexDirection::Column),
        align_items: Some(AlignItems::Stretch),
        width: props.width,
        min_height: props.min_height,
        max_height: props.max_height,
        border_style: props.border_style,
        border_color: props.border_color,
        padding: Some(1),
        ..Default::default()
    };
    if let Some(flex_grow) = props.flex_grow {
        style.flex_grow = Some(flex_grow);
    }

    VNode::Box(BoxNode {
        style,
        children,
        ..Default::default()
    })
}

/// ScrollPanel factory.
pub fn createScrollPanel(props: ScrollPanelProps) -> VNode {
    ScrollPanel(props)
}

/// Snake_case factory for consistency with other Rust APIs.
pub fn create_scroll_panel(props: ScrollPanelProps) -> VNode {
    ScrollPanel(props)
}
