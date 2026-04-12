//! Tag Atom
//!
//! A label component with optional close/remove button.
//!
//! ```text
//! [ JavaScript ]  [ TypeScript ✕ ]  [ Rust ✕ ]
//! ```

use crate::core::component::{Color, NamedColor, TextNode, TextStyle, VNode};

// =============================================================================
// Tag Component
// =============================================================================

/// Tag variant for styling
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum TagVariant {
    /// Default gray tag
    #[default]
    Default,
    /// Primary colored tag
    Primary,
    /// Success/green tag
    Success,
    /// Warning/yellow tag
    Warning,
    /// Danger/red tag
    Danger,
    /// Info/blue tag
    Info,
}

/// Tag size
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum TagSize {
    /// Small tag
    Small,
    /// Medium/default tag
    #[default]
    Medium,
    /// Large tag
    Large,
}

/// Tag component - a labeled element with optional close button.
///
/// # Example
/// ```ignore
/// let tag = Tag::new("JavaScript")
///     .variant(TagVariant::Primary)
///     .closable(true)
///     .build();
/// ```
#[derive(Debug, Clone)]
pub struct Tag {
    /// Tag text content
    text: String,
    /// Tag variant
    variant: TagVariant,
    /// Tag size
    size: TagSize,
    /// Whether the tag is closable
    closable: bool,
    /// Close button character
    close_char: char,
    /// Icon/emoji (optional)
    icon: Option<String>,
    /// Outline style (no background fill)
    outline: bool,
    /// Rounded corners
    rounded: bool,
    /// Custom background color
    bg_color: Option<Color>,
    /// Custom text color
    fg_color: Option<Color>,
    /// Disabled state
    disabled: bool,
}

impl Tag {
    /// Create a new tag with the given text.
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            variant: TagVariant::Default,
            size: TagSize::Medium,
            closable: false,
            close_char: '×',
            icon: None,
            outline: false,
            rounded: true,
            bg_color: None,
            fg_color: None,
            disabled: false,
        }
    }

    /// Set the tag variant.
    pub fn variant(mut self, variant: TagVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Set the tag size.
    pub fn size(mut self, size: TagSize) -> Self {
        self.size = size;
        self
    }

    /// Set whether the tag is closable.
    pub fn closable(mut self, closable: bool) -> Self {
        self.closable = closable;
        self
    }

    /// Set the close button character.
    pub fn close_char(mut self, c: char) -> Self {
        self.close_char = c;
        self
    }

    /// Set an icon or emoji.
    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Set outline style (no fill).
    pub fn outline(mut self, outline: bool) -> Self {
        self.outline = outline;
        self
    }

    /// Set rounded corners.
    pub fn rounded(mut self, rounded: bool) -> Self {
        self.rounded = rounded;
        self
    }

    /// Set custom background color.
    pub fn bg_color(mut self, color: Color) -> Self {
        self.bg_color = Some(color);
        self
    }

    /// Set custom text color.
    pub fn fg_color(mut self, color: Color) -> Self {
        self.fg_color = Some(color);
        self
    }

    /// Set disabled state.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Convenience: create a primary variant.
    pub fn primary(text: impl Into<String>) -> Self {
        Self::new(text).variant(TagVariant::Primary)
    }

    /// Convenience: create a success variant.
    pub fn success(text: impl Into<String>) -> Self {
        Self::new(text).variant(TagVariant::Success)
    }

    /// Convenience: create a warning variant.
    pub fn warning(text: impl Into<String>) -> Self {
        Self::new(text).variant(TagVariant::Warning)
    }

    /// Convenience: create a danger variant.
    pub fn danger(text: impl Into<String>) -> Self {
        Self::new(text).variant(TagVariant::Danger)
    }

    /// Convenience: create an info variant.
    pub fn info(text: impl Into<String>) -> Self {
        Self::new(text).variant(TagVariant::Info)
    }

    /// Get colors based on variant.
    fn get_colors(&self) -> (Color, Color) {
        if let Some(bg) = &self.bg_color {
            let fg = self
                .fg_color
                .clone()
                .unwrap_or(Color::Named(NamedColor::White));
            return (bg.clone(), fg);
        }

        let (bg, fg) = match self.variant {
            TagVariant::Default => (
                Color::Named(NamedColor::BrightBlack),
                Color::Named(NamedColor::White),
            ),
            TagVariant::Primary => (
                Color::Named(NamedColor::Blue),
                Color::Named(NamedColor::White),
            ),
            TagVariant::Success => (
                Color::Named(NamedColor::Green),
                Color::Named(NamedColor::Black),
            ),
            TagVariant::Warning => (
                Color::Named(NamedColor::Yellow),
                Color::Named(NamedColor::Black),
            ),
            TagVariant::Danger => (
                Color::Named(NamedColor::Red),
                Color::Named(NamedColor::White),
            ),
            TagVariant::Info => (
                Color::Named(NamedColor::Cyan),
                Color::Named(NamedColor::Black),
            ),
        };

        (bg, self.fg_color.clone().unwrap_or(fg))
    }

    /// Build the tag VNode.
    pub fn build(self) -> VNode {
        let (bg_color, fg_color) = self.get_colors();
        let mut children = Vec::new();

        // Opening bracket
        let bracket_char = if self.rounded { '(' } else { '[' };
        let close_bracket = if self.rounded { ')' } else { ']' };

        // Icon
        if let Some(ref icon) = self.icon {
            children.push(VNode::Text(TextNode {
                content: format!("{} ", icon),
                style: TextStyle {
                    color: Some(fg_color.clone()),
                    ..Default::default()
                },
            }));
        }

        // Main text
        children.push(VNode::Text(TextNode {
            content: self.text.clone(),
            style: TextStyle {
                color: Some(fg_color.clone()),
                dim: self.disabled,
                ..Default::default()
            },
        }));

        // Close button
        if self.closable {
            children.push(VNode::Text(TextNode {
                content: format!(" {}", self.close_char),
                style: TextStyle {
                    color: Some(if self.disabled {
                        Color::Named(NamedColor::Gray)
                    } else {
                        fg_color.clone()
                    }),
                    dim: self.disabled,
                    ..Default::default()
                },
            }));
        }

        // Build as inline text with brackets
        let padding = match self.size {
            TagSize::Small => "",
            TagSize::Medium => " ",
            TagSize::Large => "  ",
        };

        let content_parts: Vec<String> = children
            .iter()
            .filter_map(|n| match n {
                VNode::Text(t) => Some(t.content.clone()),
                _ => None,
            })
            .collect();
        let content = content_parts.join("");

        let display = format!(
            "{}{}{}{}{}",
            bracket_char, padding, content, padding, close_bracket
        );

        VNode::Text(TextNode {
            content: display,
            style: TextStyle {
                color: Some(fg_color),
                background: if self.outline { None } else { Some(bg_color) },
                dim: self.disabled,
                ..Default::default()
            },
        })
    }
}

impl From<Tag> for VNode {
    fn from(tag: Tag) -> VNode {
        tag.build()
    }
}

impl From<&str> for Tag {
    fn from(s: &str) -> Self {
        Tag::new(s)
    }
}

impl From<String> for Tag {
    fn from(s: String) -> Self {
        Tag::new(s)
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tag_basic() {
        let tag = Tag::new("JavaScript").build();
        if let VNode::Text(node) = tag {
            assert!(node.content.contains("JavaScript"));
        } else {
            panic!("Expected Text node");
        }
    }

    #[test]
    fn test_tag_closable() {
        let tag = Tag::new("Rust").closable(true).build();
        if let VNode::Text(node) = tag {
            assert!(node.content.contains("×"));
            assert!(node.content.contains("Rust"));
        } else {
            panic!("Expected Text node");
        }
    }

    #[test]
    fn test_tag_with_icon() {
        let tag = Tag::new("Star").icon("⭐").build();
        if let VNode::Text(node) = tag {
            assert!(node.content.contains("⭐"));
            assert!(node.content.contains("Star"));
        } else {
            panic!("Expected Text node");
        }
    }

    #[test]
    fn test_tag_variants() {
        let primary = Tag::primary("Test").build();
        let success = Tag::success("Test").build();
        let warning = Tag::warning("Test").build();
        let danger = Tag::danger("Test").build();
        let info = Tag::info("Test").build();

        // All should create Text nodes
        matches!(primary, VNode::Text(_));
        matches!(success, VNode::Text(_));
        matches!(warning, VNode::Text(_));
        matches!(danger, VNode::Text(_));
        matches!(info, VNode::Text(_));
    }

    #[test]
    fn test_tag_from_str() {
        let tag: Tag = "TypeScript".into();
        assert_eq!(tag.text, "TypeScript");
    }

    #[test]
    fn test_tag_custom_close_char() {
        let tag = Tag::new("Remove").closable(true).close_char('✕').build();
        if let VNode::Text(node) = tag {
            assert!(node.content.contains("✕"));
        } else {
            panic!("Expected Text node");
        }
    }

    #[test]
    fn test_tag_square_brackets() {
        let tag = Tag::new("Test").rounded(false).build();
        if let VNode::Text(node) = tag {
            assert!(node.content.starts_with('['));
            assert!(node.content.ends_with(']'));
        } else {
            panic!("Expected Text node");
        }
    }
}
