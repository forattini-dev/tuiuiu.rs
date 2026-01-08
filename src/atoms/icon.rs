//! Icon Atom
//!
//! Icon component using Unicode characters and emoji.
//!
//! ```text
//! ✓ ✗ ⚙ ⚡ ★ ♥ ✎ 🔒 📁 💾
//! ```

use crate::core::component::{Color, NamedColor, TextNode, TextStyle, VNode};

// =============================================================================
// Predefined Icons
// =============================================================================

/// Common icon set using Unicode characters
pub mod icons {
    // Status icons
    pub const CHECK: &str = "✓";
    pub const CROSS: &str = "✗";
    pub const WARNING: &str = "⚠";
    pub const INFO: &str = "ℹ";
    pub const QUESTION: &str = "?";

    // Action icons
    pub const PLUS: &str = "+";
    pub const MINUS: &str = "-";
    pub const EDIT: &str = "✎";
    pub const DELETE: &str = "✕";
    pub const SAVE: &str = "💾";
    pub const COPY: &str = "📋";

    // Navigation icons
    pub const ARROW_UP: &str = "↑";
    pub const ARROW_DOWN: &str = "↓";
    pub const ARROW_LEFT: &str = "←";
    pub const ARROW_RIGHT: &str = "→";
    pub const CHEVRON_UP: &str = "▲";
    pub const CHEVRON_DOWN: &str = "▼";
    pub const CHEVRON_LEFT: &str = "◀";
    pub const CHEVRON_RIGHT: &str = "▶";

    // UI icons
    pub const MENU: &str = "☰";
    pub const CLOSE: &str = "×";
    pub const SEARCH: &str = "🔍";
    pub const SETTINGS: &str = "⚙";
    pub const HOME: &str = "🏠";
    pub const USER: &str = "👤";
    pub const USERS: &str = "👥";

    // File icons
    pub const FILE: &str = "📄";
    pub const FOLDER: &str = "📁";
    pub const FOLDER_OPEN: &str = "📂";

    // Communication
    pub const MAIL: &str = "✉";
    pub const BELL: &str = "🔔";
    pub const CHAT: &str = "💬";

    // Media
    pub const PLAY: &str = "▶";
    pub const PAUSE: &str = "⏸";
    pub const STOP: &str = "⏹";
    pub const RECORD: &str = "⏺";

    // Misc
    pub const STAR: &str = "★";
    pub const STAR_EMPTY: &str = "☆";
    pub const HEART: &str = "♥";
    pub const HEART_EMPTY: &str = "♡";
    pub const LOCK: &str = "🔒";
    pub const UNLOCK: &str = "🔓";
    pub const LIGHTNING: &str = "⚡";
    pub const FIRE: &str = "🔥";
    pub const CLOCK: &str = "🕐";
    pub const CALENDAR: &str = "📅";

    // Loading/Progress
    pub const SPINNER: &str = "◌";
    pub const LOADING: &str = "⟳";
    pub const REFRESH: &str = "↻";

    // Boxes
    pub const BOX_EMPTY: &str = "☐";
    pub const BOX_CHECKED: &str = "☑";
    pub const BOX_CROSSED: &str = "☒";
    pub const RADIO_EMPTY: &str = "○";
    pub const RADIO_FILLED: &str = "●";
}

/// Icon size
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum IconSize {
    /// Small (no extra spacing)
    Small,
    /// Medium (default)
    #[default]
    Medium,
    /// Large (extra spacing)
    Large,
}

// =============================================================================
// Icon Component
// =============================================================================

/// Icon component for displaying symbols.
///
/// # Example
/// ```ignore
/// let icon = Icon::new(icons::CHECK)
///     .color(Color::Named(NamedColor::Green))
///     .build();
/// ```
#[derive(Debug, Clone)]
pub struct Icon {
    /// Icon character/string
    icon: String,
    /// Icon size
    size: IconSize,
    /// Icon color
    color: Option<Color>,
    /// Bold style
    bold: bool,
    /// Dim style
    dim: bool,
    /// Label text (shown after icon)
    label: Option<String>,
    /// Spinning animation hint
    spinning: bool,
}

impl Icon {
    /// Create a new icon.
    pub fn new(icon: impl Into<String>) -> Self {
        Self {
            icon: icon.into(),
            size: IconSize::Medium,
            color: None,
            bold: false,
            dim: false,
            label: None,
            spinning: false,
        }
    }

    // Convenience constructors for common icons

    /// Check icon
    pub fn check() -> Self {
        Self::new(icons::CHECK).color(Color::Named(NamedColor::Green))
    }

    /// Cross/error icon
    pub fn cross() -> Self {
        Self::new(icons::CROSS).color(Color::Named(NamedColor::Red))
    }

    /// Warning icon
    pub fn warning() -> Self {
        Self::new(icons::WARNING).color(Color::Named(NamedColor::Yellow))
    }

    /// Info icon
    pub fn info() -> Self {
        Self::new(icons::INFO).color(Color::Named(NamedColor::Cyan))
    }

    /// Loading spinner
    pub fn loading() -> Self {
        Self::new(icons::LOADING).spinning(true)
    }

    /// Set size.
    pub fn size(mut self, size: IconSize) -> Self {
        self.size = size;
        self
    }

    /// Set color.
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    /// Set bold.
    pub fn bold(mut self, bold: bool) -> Self {
        self.bold = bold;
        self
    }

    /// Set dim.
    pub fn dim(mut self, dim: bool) -> Self {
        self.dim = dim;
        self
    }

    /// Set label.
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Set spinning hint.
    pub fn spinning(mut self, spinning: bool) -> Self {
        self.spinning = spinning;
        self
    }

    /// Small size.
    pub fn small(self) -> Self {
        self.size(IconSize::Small)
    }

    /// Large size.
    pub fn large(self) -> Self {
        self.size(IconSize::Large)
    }

    /// Build the icon VNode.
    pub fn build(self) -> VNode {
        let mut content = String::new();

        // Size padding
        let pad = match self.size {
            IconSize::Small => "",
            IconSize::Medium => "",
            IconSize::Large => " ",
        };

        content.push_str(pad);
        content.push_str(&self.icon);
        content.push_str(pad);

        // Add label
        if let Some(ref label) = self.label {
            content.push(' ');
            content.push_str(label);
        }

        VNode::Text(TextNode {
            content,
            style: TextStyle {
                color: self.color,
                bold: self.bold,
                dim: self.dim,
                ..Default::default()
            },
        })
    }
}

impl From<Icon> for VNode {
    fn from(icon: Icon) -> VNode {
        icon.build()
    }
}

impl From<&str> for Icon {
    fn from(s: &str) -> Self {
        Icon::new(s)
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_icon_basic() {
        let icon = Icon::new(icons::CHECK).build();
        if let VNode::Text(node) = icon {
            assert_eq!(node.content, icons::CHECK);
        } else {
            panic!("Expected Text node");
        }
    }

    #[test]
    fn test_icon_with_color() {
        let icon = Icon::new(icons::STAR)
            .color(Color::Named(NamedColor::Yellow))
            .build();
        if let VNode::Text(node) = icon {
            assert!(node.style.color.is_some());
        } else {
            panic!("Expected Text node");
        }
    }

    #[test]
    fn test_icon_with_label() {
        let icon = Icon::new(icons::HOME)
            .label("Home")
            .build();
        if let VNode::Text(node) = icon {
            assert!(node.content.contains("Home"));
            assert!(node.content.contains(icons::HOME));
        } else {
            panic!("Expected Text node");
        }
    }

    #[test]
    fn test_predefined_icons() {
        let check = Icon::check().build();
        let cross = Icon::cross().build();
        let warning = Icon::warning().build();
        let info = Icon::info().build();

        matches!(check, VNode::Text(_));
        matches!(cross, VNode::Text(_));
        matches!(warning, VNode::Text(_));
        matches!(info, VNode::Text(_));
    }

    #[test]
    fn test_icon_sizes() {
        let small = Icon::new(icons::STAR).small().build();
        let large = Icon::new(icons::STAR).large().build();

        if let (VNode::Text(s), VNode::Text(l)) = (small, large) {
            // Large has extra padding
            assert!(l.content.len() >= s.content.len());
        }
    }

    #[test]
    fn test_icon_from_str() {
        let icon: Icon = "⭐".into();
        assert_eq!(icon.icon, "⭐");
    }

    #[test]
    fn test_icons_module() {
        // Ensure icons are valid strings
        assert!(!icons::CHECK.is_empty());
        assert!(!icons::MENU.is_empty());
        assert!(!icons::FOLDER.is_empty());
    }
}
