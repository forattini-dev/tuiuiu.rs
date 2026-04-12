//! Skeleton Component
//!
//! A loading placeholder component that shows animated placeholders
//! while content is loading.
//!
//! # Example
//!
//! ```rust,ignore
//! use tuiuiu::atoms::skeleton::{Skeleton, SkeletonText, SkeletonCard};
//!
//! // Simple skeleton
//! let skeleton = Skeleton::new(20, 1);
//!
//! // Text skeleton with multiple lines
//! let text_skeleton = SkeletonText::new()
//!     .lines(3)
//!     .width(40);
//!
//! // Card skeleton
//! let card_skeleton = SkeletonCard::new(50, 10);
//! ```

use crate::core::component::VNode;
use crate::core::tick::oscillate;

// =============================================================================
// Skeleton Characters
// =============================================================================

/// Characters used for skeleton animation.
#[derive(Clone, Copy)]
pub struct SkeletonChars {
    /// Character for the skeleton fill.
    pub fill: char,
    /// Animation characters (cycle through these).
    pub animation: &'static [char],
}

impl Default for SkeletonChars {
    fn default() -> Self {
        Self {
            fill: '░',
            animation: &['░', '▒', '▓', '▒'],
        }
    }
}

/// ASCII-friendly skeleton characters.
pub const SKELETON_ASCII: SkeletonChars = SkeletonChars {
    fill: '#',
    animation: &['#', '=', '-', '='],
};

/// Block skeleton characters.
pub const SKELETON_BLOCKS: SkeletonChars = SkeletonChars {
    fill: '▒',
    animation: &['░', '▒', '▓', '█'],
};

/// Dot skeleton characters.
pub const SKELETON_DOTS: SkeletonChars = SkeletonChars {
    fill: '·',
    animation: &['·', '•', '●', '•'],
};

// =============================================================================
// Skeleton Component
// =============================================================================

/// Basic skeleton loading placeholder.
#[derive(Clone)]
pub struct Skeleton {
    width: u16,
    height: u16,
    animated: bool,
    chars: SkeletonChars,
    label: Option<String>,
}

impl Skeleton {
    /// Create a new skeleton with the given dimensions.
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            width,
            height,
            animated: true,
            chars: SkeletonChars::default(),
            label: None,
        }
    }

    /// Create a line skeleton (single row).
    pub fn line(width: u16) -> Self {
        Self::new(width, 1)
    }

    /// Create a block skeleton.
    pub fn block(width: u16, height: u16) -> Self {
        Self::new(width, height)
    }

    /// Enable or disable animation.
    pub fn animated(mut self, animated: bool) -> Self {
        self.animated = animated;
        self
    }

    /// Set custom skeleton characters.
    pub fn chars(mut self, chars: SkeletonChars) -> Self {
        self.chars = chars;
        self
    }

    /// Use ASCII-friendly characters.
    pub fn ascii(mut self) -> Self {
        self.chars = SKELETON_ASCII;
        self
    }

    /// Add a label to display.
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Render the skeleton to a string.
    pub fn render(&self) -> String {
        let char_idx = if self.animated {
            // oscillate returns 0.0-1.0, scale to animation index
            let t = oscillate(0.5); // Speed of animation
            (t * (self.chars.animation.len() - 1) as f64).round() as usize
        } else {
            0
        };

        let fill_char = self
            .chars
            .animation
            .get(char_idx)
            .unwrap_or(&self.chars.fill);

        let mut lines = Vec::new();
        for _ in 0..self.height {
            let line: String = std::iter::repeat(*fill_char)
                .take(self.width as usize)
                .collect();
            lines.push(line);
        }

        if let Some(ref label) = self.label {
            // Center the label on the first line if there's room
            if self.width as usize > label.len() + 4 {
                let padding = (self.width as usize - label.len() - 2) / 2;
                let label_line = format!(
                    "{} {} {}",
                    std::iter::repeat(*fill_char)
                        .take(padding)
                        .collect::<String>(),
                    label,
                    std::iter::repeat(*fill_char)
                        .take(self.width as usize - padding - label.len() - 2)
                        .collect::<String>()
                );
                if !lines.is_empty() {
                    lines[0] = label_line;
                }
            }
        }

        lines.join("\n")
    }
}

impl From<Skeleton> for VNode {
    fn from(skeleton: Skeleton) -> Self {
        VNode::text(skeleton.render())
    }
}

// =============================================================================
// SkeletonText Component
// =============================================================================

/// Text skeleton with multiple lines of varying widths.
#[derive(Clone)]
pub struct SkeletonText {
    lines: u16,
    width: u16,
    animated: bool,
    vary_width: bool,
    chars: SkeletonChars,
}

impl Default for SkeletonText {
    fn default() -> Self {
        Self::new()
    }
}

impl SkeletonText {
    /// Create a new text skeleton.
    pub fn new() -> Self {
        Self {
            lines: 3,
            width: 40,
            animated: true,
            vary_width: true,
            chars: SkeletonChars::default(),
        }
    }

    /// Set the number of lines.
    pub fn lines(mut self, lines: u16) -> Self {
        self.lines = lines;
        self
    }

    /// Set the maximum width.
    pub fn width(mut self, width: u16) -> Self {
        self.width = width;
        self
    }

    /// Enable or disable animation.
    pub fn animated(mut self, animated: bool) -> Self {
        self.animated = animated;
        self
    }

    /// Enable or disable varying line widths.
    pub fn vary_width(mut self, vary: bool) -> Self {
        self.vary_width = vary;
        self
    }

    /// Set custom skeleton characters.
    pub fn chars(mut self, chars: SkeletonChars) -> Self {
        self.chars = chars;
        self
    }

    /// Render the text skeleton.
    pub fn render(&self) -> String {
        let char_idx = if self.animated {
            let t = oscillate(0.5);
            (t * (self.chars.animation.len() - 1) as f64).round() as usize
        } else {
            0
        };

        let fill_char = self
            .chars
            .animation
            .get(char_idx)
            .unwrap_or(&self.chars.fill);

        let mut lines = Vec::new();
        for i in 0..self.lines {
            let line_width = if self.vary_width {
                // Vary width: full, 80%, 60%, repeat
                match i % 3 {
                    0 => self.width,
                    1 => (self.width as f32 * 0.8) as u16,
                    _ => (self.width as f32 * 0.6) as u16,
                }
            } else {
                self.width
            };

            let line: String = std::iter::repeat(*fill_char)
                .take(line_width as usize)
                .collect();
            lines.push(line);
        }

        lines.join("\n")
    }
}

impl From<SkeletonText> for VNode {
    fn from(skeleton: SkeletonText) -> Self {
        VNode::text(skeleton.render())
    }
}

// =============================================================================
// SkeletonCard Component
// =============================================================================

/// Card-style skeleton with border and content area.
#[derive(Clone)]
pub struct SkeletonCard {
    width: u16,
    height: u16,
    animated: bool,
    show_avatar: bool,
    show_title: bool,
    show_lines: u16,
}

impl SkeletonCard {
    /// Create a new card skeleton.
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            width,
            height,
            animated: true,
            show_avatar: true,
            show_title: true,
            show_lines: 2,
        }
    }

    /// Enable or disable animation.
    pub fn animated(mut self, animated: bool) -> Self {
        self.animated = animated;
        self
    }

    /// Show or hide the avatar placeholder.
    pub fn avatar(mut self, show: bool) -> Self {
        self.show_avatar = show;
        self
    }

    /// Show or hide the title placeholder.
    pub fn title(mut self, show: bool) -> Self {
        self.show_title = show;
        self
    }

    /// Set the number of text lines.
    pub fn lines(mut self, lines: u16) -> Self {
        self.show_lines = lines;
        self
    }

    /// Render the card skeleton.
    pub fn render(&self) -> String {
        let animation_chars = ['░', '▒', '▓', '▒'];
        let char_idx = if self.animated {
            let t = oscillate(0.5);
            (t * (animation_chars.len() - 1) as f64).round() as usize
        } else {
            0
        };

        let fill_char = animation_chars.get(char_idx).unwrap_or(&'░');

        let inner_width = self.width.saturating_sub(4) as usize;
        let mut lines = Vec::new();

        // Top border
        lines.push(format!("┌{}┐", "─".repeat(inner_width + 2)));

        // Avatar and title row
        if self.show_avatar || self.show_title {
            let avatar = if self.show_avatar {
                format!("[{}]", fill_char)
            } else {
                String::new()
            };

            let title_width = if self.show_avatar {
                inner_width.saturating_sub(5)
            } else {
                inner_width
            };

            let title = if self.show_title {
                std::iter::repeat(*fill_char)
                    .take(title_width)
                    .collect::<String>()
            } else {
                " ".repeat(title_width)
            };

            let gap = if self.show_avatar && self.show_title {
                " "
            } else {
                ""
            };
            let content = format!("{}{}{}", avatar, gap, title);
            let padding = inner_width.saturating_sub(content.chars().count());
            lines.push(format!("│ {}{} │", content, " ".repeat(padding)));
            lines.push(format!("│ {} │", " ".repeat(inner_width)));
        }

        // Text lines
        for i in 0..self.show_lines {
            let line_width = match i % 3 {
                0 => inner_width,
                1 => (inner_width as f32 * 0.75) as usize,
                _ => (inner_width as f32 * 0.5) as usize,
            };
            let text: String = std::iter::repeat(*fill_char).take(line_width).collect();
            let padding = inner_width.saturating_sub(line_width);
            lines.push(format!("│ {}{} │", text, " ".repeat(padding)));
        }

        // Fill remaining height
        let current_lines = lines.len();
        let target_lines = self.height.saturating_sub(1) as usize;
        for _ in current_lines..target_lines {
            lines.push(format!("│ {} │", " ".repeat(inner_width)));
        }

        // Bottom border
        lines.push(format!("└{}┘", "─".repeat(inner_width + 2)));

        lines.join("\n")
    }
}

impl From<SkeletonCard> for VNode {
    fn from(skeleton: SkeletonCard) -> Self {
        VNode::text(skeleton.render())
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skeleton_basic() {
        let skeleton = Skeleton::new(10, 1).animated(false);
        let rendered = skeleton.render();
        // Use chars().count() because skeleton chars are multi-byte Unicode
        assert_eq!(rendered.chars().count(), 10);
    }

    #[test]
    fn test_skeleton_line() {
        let skeleton = Skeleton::line(20).animated(false);
        let rendered = skeleton.render();
        assert!(!rendered.contains('\n'));
        assert_eq!(rendered.chars().count(), 20);
    }

    #[test]
    fn test_skeleton_block() {
        let skeleton = Skeleton::block(10, 3).animated(false);
        let rendered = skeleton.render();
        let lines: Vec<&str> = rendered.lines().collect();
        assert_eq!(lines.len(), 3);
    }

    #[test]
    fn test_skeleton_text() {
        let skeleton = SkeletonText::new().lines(3).width(20).animated(false);
        let rendered = skeleton.render();
        let lines: Vec<&str> = rendered.lines().collect();
        assert_eq!(lines.len(), 3);
    }

    #[test]
    fn test_skeleton_text_vary_width() {
        let skeleton = SkeletonText::new()
            .lines(3)
            .width(30)
            .vary_width(true)
            .animated(false);
        let rendered = skeleton.render();
        let lines: Vec<&str> = rendered.lines().collect();
        // Lines should have different widths
        assert!(lines[0].chars().count() > lines[1].chars().count());
    }

    #[test]
    fn test_skeleton_card() {
        let skeleton = SkeletonCard::new(40, 8).animated(false);
        let rendered = skeleton.render();
        assert!(rendered.contains('┌'));
        assert!(rendered.contains('└'));
    }

    #[test]
    fn test_skeleton_ascii() {
        let skeleton = Skeleton::line(10).ascii().animated(false);
        let rendered = skeleton.render();
        assert!(rendered.contains('#'));
    }
}
