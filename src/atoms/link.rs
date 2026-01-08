//! Link Atom
//!
//! Hyperlink-style text component for navigation.
//!
//! ```text
//! Click here to learn more
//! ^^^^^^^^^^^^^^
//! ```

use crate::core::component::{Color, NamedColor, TextNode, TextStyle, VNode};

// =============================================================================
// Link Component
// =============================================================================

/// Link variant styles
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum LinkVariant {
    /// Default link style (blue, underlined)
    #[default]
    Default,
    /// Subtle link (no underline until hover)
    Subtle,
    /// Button-like link
    Button,
    /// Navigation link
    Nav,
}

/// Link component for navigation and actions.
///
/// # Example
/// ```ignore
/// let link = Link::new("Learn more")
///     .href("https://example.com")
///     .build();
/// ```
#[derive(Debug, Clone)]
pub struct Link {
    /// Link text
    text: String,
    /// Target URL or identifier
    href: Option<String>,
    /// Link variant
    variant: LinkVariant,
    /// Text color
    color: Color,
    /// Hover color (reserved for future interactive support)
    #[allow(dead_code)]
    hover_color: Color,
    /// Visited color (reserved for future state tracking)
    #[allow(dead_code)]
    visited_color: Color,
    /// Show underline
    underline: bool,
    /// Bold text
    bold: bool,
    /// Disabled state
    disabled: bool,
    /// External link (shows indicator)
    external: bool,
    /// Icon before text
    icon: Option<String>,
}

impl Link {
    /// Create a new link.
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            href: None,
            variant: LinkVariant::Default,
            color: Color::Named(NamedColor::Cyan),
            hover_color: Color::Named(NamedColor::BrightCyan),
            visited_color: Color::Named(NamedColor::Magenta),
            underline: true,
            bold: false,
            disabled: false,
            external: false,
            icon: None,
        }
    }

    /// Set the href/target.
    pub fn href(mut self, href: impl Into<String>) -> Self {
        self.href = Some(href.into());
        self
    }

    /// Set variant.
    pub fn variant(mut self, variant: LinkVariant) -> Self {
        self.variant = variant;
        // Apply variant-specific defaults
        match variant {
            LinkVariant::Default => {
                self.underline = true;
            }
            LinkVariant::Subtle => {
                self.underline = false;
            }
            LinkVariant::Button => {
                self.underline = false;
                self.bold = true;
            }
            LinkVariant::Nav => {
                self.underline = false;
                self.color = Color::Named(NamedColor::White);
            }
        }
        self
    }

    /// Set color.
    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    /// Set underline.
    pub fn underline(mut self, underline: bool) -> Self {
        self.underline = underline;
        self
    }

    /// Set bold.
    pub fn bold(mut self, bold: bool) -> Self {
        self.bold = bold;
        self
    }

    /// Set disabled state.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Mark as external link.
    pub fn external(mut self, external: bool) -> Self {
        self.external = external;
        self
    }

    /// Set icon.
    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Build the link VNode.
    pub fn build(self) -> VNode {
        let mut content = String::new();

        // Icon prefix
        if let Some(ref icon) = self.icon {
            content.push_str(icon);
            content.push(' ');
        }

        // Link text
        content.push_str(&self.text);

        // External indicator
        if self.external {
            content.push_str(" ↗");
        }

        let color = if self.disabled {
            Color::Named(NamedColor::Gray)
        } else {
            self.color
        };

        VNode::Text(TextNode {
            content,
            style: TextStyle {
                color: Some(color),
                underline: self.underline && !self.disabled,
                bold: self.bold,
                dim: self.disabled,
                ..Default::default()
            },
        })
    }
}

impl From<Link> for VNode {
    fn from(link: Link) -> VNode {
        link.build()
    }
}

impl From<&str> for Link {
    fn from(s: &str) -> Self {
        Link::new(s)
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_link_basic() {
        let link = Link::new("Click here").build();
        if let VNode::Text(node) = link {
            assert_eq!(node.content, "Click here");
            assert!(node.style.underline);
        } else {
            panic!("Expected Text node");
        }
    }

    #[test]
    fn test_link_external() {
        let link = Link::new("External")
            .external(true)
            .build();
        if let VNode::Text(node) = link {
            assert!(node.content.contains("↗"));
        } else {
            panic!("Expected Text node");
        }
    }

    #[test]
    fn test_link_with_icon() {
        let link = Link::new("Home")
            .icon("🏠")
            .build();
        if let VNode::Text(node) = link {
            assert!(node.content.starts_with("🏠"));
        } else {
            panic!("Expected Text node");
        }
    }

    #[test]
    fn test_link_disabled() {
        let link = Link::new("Disabled")
            .disabled(true)
            .build();
        if let VNode::Text(node) = link {
            assert!(node.style.dim);
            assert!(!node.style.underline);
        } else {
            panic!("Expected Text node");
        }
    }

    #[test]
    fn test_link_variants() {
        let subtle = Link::new("Subtle").variant(LinkVariant::Subtle).build();
        let button = Link::new("Button").variant(LinkVariant::Button).build();
        let nav = Link::new("Nav").variant(LinkVariant::Nav).build();

        if let VNode::Text(n) = subtle {
            assert!(!n.style.underline);
        }
        if let VNode::Text(n) = button {
            assert!(n.style.bold);
        }
        matches!(nav, VNode::Text(_));
    }

    #[test]
    fn test_link_from_str() {
        let link: Link = "Quick link".into();
        assert_eq!(link.text, "Quick link");
    }
}
