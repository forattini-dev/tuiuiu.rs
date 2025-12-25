//! Text Input Component

use crate::core::component::{VNode, BoxNode, BoxStyle, TextNode, BorderStyle};

/// Text input component.
#[derive(Debug, Clone, Default)]
pub struct TextInput {
    value: String,
    placeholder: String,
    disabled: bool,
}

impl TextInput {
    pub fn new() -> Self { Self::default() }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = value.into();
        self
    }

    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn build(self) -> VNode {
        let display = if self.value.is_empty() {
            self.placeholder.clone()
        } else {
            self.value.clone()
        };

        VNode::Box(BoxNode {
            style: BoxStyle {
                padding: Some(0),
                border_style: Some(BorderStyle::Single),
                ..Default::default()
            },
            children: vec![VNode::Text(TextNode {
                content: display,
                style: Default::default(),
            })],
            ..Default::default()
        })
    }
}

impl From<TextInput> for VNode {
    fn from(t: TextInput) -> VNode { t.build() }
}
