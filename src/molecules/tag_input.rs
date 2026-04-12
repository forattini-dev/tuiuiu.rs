//! TagInput Molecule
//!
//! An input component for adding and managing multiple tags.
//!
//! ```text
//! ┌─────────────────────────────────────────────────┐
//! │ (rust) (typescript) (python) [Enter new tag...] │
//! └─────────────────────────────────────────────────┘
//! ```

use crate::core::component::{
    BorderStyle, BoxNode, BoxStyle, Color, NamedColor, TextNode, TextStyle, VNode,
};
use crate::core::layout::{AlignItems, FlexDirection, FlexWrap};
use crate::core::signals::create_signal;

// =============================================================================
// TagInput Component
// =============================================================================

/// A single tag in the TagInput
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TagItem {
    /// Tag value/text
    pub value: String,
    /// Optional label (displayed instead of value)
    pub label: Option<String>,
    /// Whether tag can be removed
    pub removable: bool,
    /// Tag color
    pub color: Option<Color>,
}

impl TagItem {
    /// Create a new tag item.
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: None,
            removable: true,
            color: None,
        }
    }

    /// Set a display label.
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Set whether tag is removable.
    pub fn removable(mut self, removable: bool) -> Self {
        self.removable = removable;
        self
    }

    /// Set tag color.
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    /// Get display text.
    pub fn display(&self) -> &str {
        self.label.as_deref().unwrap_or(&self.value)
    }
}

impl From<&str> for TagItem {
    fn from(s: &str) -> Self {
        TagItem::new(s)
    }
}

impl From<String> for TagItem {
    fn from(s: String) -> Self {
        TagItem::new(s)
    }
}

/// TagInput state for reactive updates
#[derive(Debug, Clone, Default)]
pub struct TagInputState {
    /// Current tags
    pub tags: Vec<TagItem>,
    /// Current input text
    pub input: String,
    /// Selected tag index (for keyboard navigation)
    pub selected: Option<usize>,
}

impl TagInputState {
    /// Create with initial tags.
    pub fn with_tags(tags: Vec<TagItem>) -> Self {
        Self {
            tags,
            input: String::new(),
            selected: None,
        }
    }

    /// Add a tag.
    pub fn add(&mut self, tag: TagItem) {
        if !self.tags.iter().any(|t| t.value == tag.value) {
            self.tags.push(tag);
        }
    }

    /// Add tag from current input.
    pub fn add_from_input(&mut self) {
        let value = self.input.trim().to_string();
        if !value.is_empty() {
            self.add(TagItem::new(value));
            self.input.clear();
        }
    }

    /// Remove tag by index.
    pub fn remove(&mut self, index: usize) {
        if index < self.tags.len() && self.tags[index].removable {
            self.tags.remove(index);
            // Adjust selection if needed
            if let Some(sel) = self.selected {
                if sel >= self.tags.len() && !self.tags.is_empty() {
                    self.selected = Some(self.tags.len() - 1);
                } else if self.tags.is_empty() {
                    self.selected = None;
                }
            }
        }
    }

    /// Remove tag by value.
    pub fn remove_by_value(&mut self, value: &str) {
        if let Some(idx) = self.tags.iter().position(|t| t.value == value) {
            self.remove(idx);
        }
    }

    /// Clear all tags.
    pub fn clear(&mut self) {
        self.tags.retain(|t| !t.removable);
        self.selected = None;
    }

    /// Get all tag values.
    pub fn values(&self) -> Vec<&str> {
        self.tags.iter().map(|t| t.value.as_str()).collect()
    }

    /// Select previous tag.
    pub fn select_prev(&mut self) {
        if self.tags.is_empty() {
            self.selected = None;
            return;
        }
        self.selected = Some(match self.selected {
            None => self.tags.len() - 1,
            Some(0) => 0,
            Some(i) => i - 1,
        });
    }

    /// Select next tag.
    pub fn select_next(&mut self) {
        if self.tags.is_empty() {
            self.selected = None;
            return;
        }
        self.selected = Some(match self.selected {
            None => 0,
            Some(i) if i >= self.tags.len() - 1 => self.tags.len() - 1,
            Some(i) => i + 1,
        });
    }

    /// Delete selected tag (backspace behavior).
    pub fn delete_selected(&mut self) {
        if let Some(idx) = self.selected {
            self.remove(idx);
        } else if self.input.is_empty() && !self.tags.is_empty() {
            // Delete last tag if input is empty
            let last = self.tags.len() - 1;
            self.remove(last);
        }
    }
}

/// TagInput component - input for managing multiple tags.
///
/// # Example
/// ```ignore
/// let tag_input = TagInput::new()
///     .placeholder("Add tag...")
///     .add_tag("rust")
///     .add_tag("typescript")
///     .build();
/// ```
#[derive(Debug, Clone)]
pub struct TagInput {
    /// Current tags
    tags: Vec<TagItem>,
    /// Placeholder text for input
    placeholder: String,
    /// Input value
    input_value: String,
    /// Maximum number of tags
    max_tags: Option<usize>,
    /// Allow duplicates
    allow_duplicates: bool,
    /// Separator characters that trigger new tag
    separators: Vec<char>,
    /// Tag background color
    tag_bg: Color,
    /// Tag text color
    tag_fg: Color,
    /// Close button character
    close_char: char,
    /// Border style
    bordered: bool,
    /// Border color
    border_color: Color,
    /// Disabled state
    disabled: bool,
    /// Selected tag index
    selected: Option<usize>,
}

impl Default for TagInput {
    fn default() -> Self {
        Self {
            tags: Vec::new(),
            placeholder: "Add tag...".to_string(),
            input_value: String::new(),
            max_tags: None,
            allow_duplicates: false,
            separators: vec![',', ';', ' '],
            tag_bg: Color::Named(NamedColor::BrightBlack),
            tag_fg: Color::Named(NamedColor::White),
            close_char: '×',
            bordered: true,
            border_color: Color::Named(NamedColor::Gray),
            disabled: false,
            selected: None,
        }
    }
}

impl TagInput {
    /// Create a new tag input.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a tag.
    pub fn add_tag(mut self, tag: impl Into<TagItem>) -> Self {
        let tag = tag.into();
        if self.allow_duplicates || !self.tags.iter().any(|t| t.value == tag.value) {
            if self.max_tags.map_or(true, |max| self.tags.len() < max) {
                self.tags.push(tag);
            }
        }
        self
    }

    /// Set tags.
    pub fn tags(mut self, tags: Vec<TagItem>) -> Self {
        self.tags = tags;
        self
    }

    /// Set placeholder text.
    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    /// Set input value.
    pub fn input_value(mut self, value: impl Into<String>) -> Self {
        self.input_value = value.into();
        self
    }

    /// Set maximum number of tags.
    pub fn max_tags(mut self, max: usize) -> Self {
        self.max_tags = Some(max);
        self
    }

    /// Allow duplicate tags.
    pub fn allow_duplicates(mut self, allow: bool) -> Self {
        self.allow_duplicates = allow;
        self
    }

    /// Set separator characters.
    pub fn separators(mut self, seps: Vec<char>) -> Self {
        self.separators = seps;
        self
    }

    /// Set tag colors.
    pub fn tag_colors(mut self, bg: Color, fg: Color) -> Self {
        self.tag_bg = bg;
        self.tag_fg = fg;
        self
    }

    /// Set close button character.
    pub fn close_char(mut self, c: char) -> Self {
        self.close_char = c;
        self
    }

    /// Set bordered style.
    pub fn bordered(mut self, bordered: bool) -> Self {
        self.bordered = bordered;
        self
    }

    /// Set border color.
    pub fn border_color(mut self, color: Color) -> Self {
        self.border_color = color;
        self
    }

    /// Set disabled state.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Set selected tag index.
    pub fn selected(mut self, index: Option<usize>) -> Self {
        self.selected = index;
        self
    }

    /// Apply state.
    pub fn state(mut self, state: &TagInputState) -> Self {
        self.tags = state.tags.clone();
        self.input_value = state.input.clone();
        self.selected = state.selected;
        self
    }

    /// Build a single tag element.
    fn build_tag(&self, tag: &TagItem, index: usize) -> VNode {
        let is_selected = self.selected == Some(index);
        let bg = tag.color.clone().unwrap_or(self.tag_bg.clone());
        let fg = self.tag_fg.clone();

        let mut content = format!("({})", tag.display());
        if tag.removable {
            content = format!("({} {})", tag.display(), self.close_char);
        }

        VNode::Text(TextNode {
            content,
            style: TextStyle {
                color: Some(fg),
                background: Some(if is_selected {
                    Color::Named(NamedColor::Cyan)
                } else {
                    bg
                }),
                bold: is_selected,
                ..Default::default()
            },
        })
    }

    /// Build the input field.
    fn build_input(&self) -> VNode {
        let text = if self.input_value.is_empty() {
            &self.placeholder
        } else {
            &self.input_value
        };

        let color = if self.input_value.is_empty() {
            Color::Named(NamedColor::Gray)
        } else {
            Color::Named(NamedColor::White)
        };

        VNode::Text(TextNode {
            content: format!("[{}]", text),
            style: TextStyle {
                color: Some(color),
                dim: self.disabled,
                ..Default::default()
            },
        })
    }

    /// Build the tag input VNode.
    pub fn build(self) -> VNode {
        let mut children: Vec<VNode> = self
            .tags
            .iter()
            .enumerate()
            .map(|(i, tag)| self.build_tag(tag, i))
            .collect();

        // Add input field if not at max
        let at_max = self.max_tags.map_or(false, |max| self.tags.len() >= max);
        if !at_max && !self.disabled {
            children.push(self.build_input());
        }

        let mut style = BoxStyle {
            flex_direction: Some(FlexDirection::Row),
            flex_wrap: Some(FlexWrap::Wrap),
            align_items: Some(AlignItems::Center),
            gap: Some(1),
            padding: Some(1),
            ..Default::default()
        };

        if self.bordered {
            style.border_style = Some(BorderStyle::Single);
            style.border_color = Some(self.border_color.clone());
        }

        VNode::Box(BoxNode {
            id: None,
            style,
            children,
            handlers: Default::default(),
        })
    }
}

impl From<TagInput> for VNode {
    fn from(input: TagInput) -> VNode {
        input.build()
    }
}

/// Create reactive tag input state.
pub fn create_tag_input_state(
    initial_tags: Vec<TagItem>,
) -> (
    crate::core::signals::ReadSignal<TagInputState>,
    crate::core::signals::WriteSignal<TagInputState>,
) {
    create_signal(TagInputState::with_tags(initial_tags))
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tag_input_empty() {
        let input = TagInput::new().build();
        matches!(input, VNode::Box(_));
    }

    #[test]
    fn test_tag_input_with_tags() {
        let input = TagInput::new()
            .add_tag("rust")
            .add_tag("typescript")
            .add_tag("python")
            .build();

        if let VNode::Box(node) = input {
            // 3 tags + 1 input field
            assert_eq!(node.children.len(), 4);
        } else {
            panic!("Expected Box");
        }
    }

    #[test]
    fn test_tag_input_max_tags() {
        let input = TagInput::new()
            .max_tags(2)
            .add_tag("one")
            .add_tag("two")
            .add_tag("three") // Should be ignored
            .build();

        if let VNode::Box(node) = input {
            // 2 tags (at max, no input field)
            assert_eq!(node.children.len(), 2);
        } else {
            panic!("Expected Box");
        }
    }

    #[test]
    fn test_tag_input_no_duplicates() {
        let input = TagInput::new()
            .add_tag("rust")
            .add_tag("rust") // Should be ignored
            .add_tag("go")
            .build();

        if let VNode::Box(node) = input {
            // 2 tags + 1 input
            assert_eq!(node.children.len(), 3);
        } else {
            panic!("Expected Box");
        }
    }

    #[test]
    fn test_tag_item() {
        let tag = TagItem::new("value")
            .label("Display Label")
            .removable(false)
            .color(Color::Named(NamedColor::Red));

        assert_eq!(tag.value, "value");
        assert_eq!(tag.display(), "Display Label");
        assert!(!tag.removable);
    }

    #[test]
    fn test_tag_input_state() {
        let mut state = TagInputState::default();
        state.add(TagItem::new("rust"));
        state.add(TagItem::new("go"));

        assert_eq!(state.values(), vec!["rust", "go"]);

        state.remove(0);
        assert_eq!(state.values(), vec!["go"]);
    }

    #[test]
    fn test_tag_input_state_from_input() {
        let mut state = TagInputState::default();
        state.input = "  typescript  ".to_string();
        state.add_from_input();

        assert_eq!(state.values(), vec!["typescript"]);
        assert!(state.input.is_empty());
    }

    #[test]
    fn test_tag_input_state_navigation() {
        let mut state = TagInputState::with_tags(vec![
            TagItem::new("a"),
            TagItem::new("b"),
            TagItem::new("c"),
        ]);

        assert_eq!(state.selected, None);

        state.select_next();
        assert_eq!(state.selected, Some(0));

        state.select_next();
        assert_eq!(state.selected, Some(1));

        state.select_prev();
        assert_eq!(state.selected, Some(0));
    }

    #[test]
    fn test_tag_input_disabled() {
        let input = TagInput::new().add_tag("rust").disabled(true).build();

        if let VNode::Box(node) = input {
            // 1 tag only (no input when disabled)
            assert_eq!(node.children.len(), 1);
        } else {
            panic!("Expected Box");
        }
    }
}
