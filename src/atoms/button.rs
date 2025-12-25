//! Button Component

use crate::core::component::{VNode, BoxNode, BoxStyle, TextNode, Color, NamedColor, BorderStyle};

/// Button component.
#[derive(Debug, Clone, Default)]
pub struct Button {
    label: String,
    disabled: bool,
    variant: ButtonVariant,
}

#[derive(Debug, Clone, Copy, Default)]
pub enum ButtonVariant {
    #[default]
    Primary,
    Secondary,
    Outline,
    Ghost,
}

impl Button {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            ..Default::default()
        }
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn primary(self) -> Self { self.variant(ButtonVariant::Primary) }
    pub fn secondary(self) -> Self { self.variant(ButtonVariant::Secondary) }
    pub fn outline(self) -> Self { self.variant(ButtonVariant::Outline) }
    pub fn ghost(self) -> Self { self.variant(ButtonVariant::Ghost) }

    pub fn build(self) -> VNode {
        let bg = match self.variant {
            ButtonVariant::Primary => Some(Color::Named(NamedColor::Blue)),
            ButtonVariant::Secondary => Some(Color::Named(NamedColor::Gray)),
            ButtonVariant::Outline | ButtonVariant::Ghost => None,
        };

        VNode::Box(BoxNode {
            id: None,
            style: BoxStyle {
                padding: Some(1),
                border_style: Some(BorderStyle::Round),
                background: bg,
                ..Default::default()
            },
            children: vec![VNode::Text(TextNode {
                content: self.label,
                style: Default::default(),
            })],
            handlers: Default::default(),
        })
    }
}

impl From<Button> for VNode {
    fn from(b: Button) -> VNode { b.build() }
}
