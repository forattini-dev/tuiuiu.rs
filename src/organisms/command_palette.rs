//! CommandPalette Component
//!
//! A keyboard-driven command palette (Cmd+K style) for quick navigation
//! and action execution. Supports fuzzy search, command grouping, and
//! keyboard navigation.

use crate::core::component::{
    BoxNode, BoxStyle, BorderStyle, Color, EventHandlers, NamedColor, TextNode, TextStyle, VNode,
};
use crate::core::layout::{AlignItems, FlexDirection, JustifyContent, Size};
use crate::core::signals::{create_signal, ReadSignal, WriteSignal};

// =============================================================================
// Command Definition
// =============================================================================

/// A single command that can be executed from the palette.
#[derive(Debug, Clone)]
pub struct Command {
    /// Unique identifier for the command.
    pub id: String,
    /// Display label for the command.
    pub label: String,
    /// Optional description/subtitle.
    pub description: Option<String>,
    /// Optional keyboard shortcut hint (e.g., "Ctrl+S").
    pub shortcut: Option<String>,
    /// Group/category for organizing commands.
    pub group: Option<String>,
    /// Icon (single character or emoji).
    pub icon: Option<String>,
    /// Whether the command is currently disabled.
    pub disabled: bool,
    /// Search keywords (in addition to label).
    pub keywords: Vec<String>,
}

impl Command {
    /// Create a new command with the given id and label.
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            description: None,
            shortcut: None,
            group: None,
            icon: None,
            disabled: false,
            keywords: Vec::new(),
        }
    }

    /// Set the description.
    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    /// Set the keyboard shortcut hint.
    pub fn shortcut(mut self, shortcut: impl Into<String>) -> Self {
        self.shortcut = Some(shortcut.into());
        self
    }

    /// Set the group/category.
    pub fn group(mut self, group: impl Into<String>) -> Self {
        self.group = Some(group.into());
        self
    }

    /// Set the icon.
    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Set disabled state.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Add search keywords.
    pub fn keywords(mut self, keywords: Vec<&str>) -> Self {
        self.keywords = keywords.into_iter().map(String::from).collect();
        self
    }

    /// Check if this command matches the search query.
    pub fn matches(&self, query: &str) -> bool {
        if query.is_empty() {
            return true;
        }

        let query_lower = query.to_lowercase();

        // Check label
        if self.label.to_lowercase().contains(&query_lower) {
            return true;
        }

        // Check description
        if let Some(ref desc) = self.description {
            if desc.to_lowercase().contains(&query_lower) {
                return true;
            }
        }

        // Check keywords
        for keyword in &self.keywords {
            if keyword.to_lowercase().contains(&query_lower) {
                return true;
            }
        }

        // Check group
        if let Some(ref group) = self.group {
            if group.to_lowercase().contains(&query_lower) {
                return true;
            }
        }

        false
    }

    /// Calculate match score (higher = better match).
    /// Used for sorting results.
    pub fn match_score(&self, query: &str) -> i32 {
        if query.is_empty() {
            return 0;
        }

        let query_lower = query.to_lowercase();
        let label_lower = self.label.to_lowercase();

        // Exact match in label is best
        if label_lower == query_lower {
            return 100;
        }

        // Starts with query is very good
        if label_lower.starts_with(&query_lower) {
            return 80;
        }

        // Contains in label is good
        if label_lower.contains(&query_lower) {
            return 60;
        }

        // Keyword match
        for keyword in &self.keywords {
            if keyword.to_lowercase().contains(&query_lower) {
                return 40;
            }
        }

        // Description match
        if let Some(ref desc) = self.description {
            if desc.to_lowercase().contains(&query_lower) {
                return 20;
            }
        }

        0
    }
}

// =============================================================================
// Command Group
// =============================================================================

/// A group of related commands.
#[derive(Debug, Clone)]
pub struct CommandGroup {
    /// Group name.
    pub name: String,
    /// Commands in this group.
    pub commands: Vec<Command>,
    /// Icon for the group.
    pub icon: Option<String>,
}

impl CommandGroup {
    /// Create a new command group.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            commands: Vec::new(),
            icon: None,
        }
    }

    /// Add a command to the group.
    pub fn add(mut self, command: Command) -> Self {
        // Set the group name on the command
        let mut cmd = command;
        cmd.group = Some(self.name.clone());
        self.commands.push(cmd);
        self
    }

    /// Set the group icon.
    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }
}

// =============================================================================
// Command Palette Component
// =============================================================================

/// Command palette component.
///
/// A modal-style overlay with a search input and list of commands.
///
/// # Example
/// ```ignore
/// let commands = vec![
///     Command::new("file.save", "Save File").shortcut("Ctrl+S"),
///     Command::new("file.open", "Open File").shortcut("Ctrl+O"),
///     Command::new("edit.undo", "Undo").shortcut("Ctrl+Z").group("Edit"),
/// ];
///
/// let palette = CommandPalette::new()
///     .commands(commands)
///     .placeholder("Type a command...")
///     .open(true)
///     .build();
/// ```
#[derive(Debug, Clone, Default)]
pub struct CommandPalette {
    /// Whether the palette is open/visible.
    open: bool,
    /// All available commands.
    commands: Vec<Command>,
    /// Current search query.
    query: String,
    /// Selected command index in filtered list.
    selected_index: usize,
    /// Placeholder text for empty input.
    placeholder: String,
    /// Maximum number of visible results.
    max_visible: usize,
    /// Width of the palette.
    width: u16,
    /// Show command groups/categories.
    show_groups: bool,
    /// Show keyboard shortcuts.
    show_shortcuts: bool,
    /// Show command descriptions.
    show_descriptions: bool,
    /// Header title.
    title: Option<String>,
    /// Accent color for highlights.
    accent_color: Color,
    /// Use ASCII characters instead of Unicode.
    ascii: bool,
}

impl CommandPalette {
    /// Create a new command palette.
    pub fn new() -> Self {
        Self {
            open: false,
            commands: Vec::new(),
            query: String::new(),
            selected_index: 0,
            placeholder: "Search commands...".to_string(),
            max_visible: 10,
            width: 60,
            show_groups: true,
            show_shortcuts: true,
            show_descriptions: true,
            title: None,
            accent_color: Color::Named(NamedColor::Blue),
            ascii: false,
        }
    }

    /// Set whether the palette is open.
    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    /// Set the commands list.
    pub fn commands(mut self, commands: Vec<Command>) -> Self {
        self.commands = commands;
        self
    }

    /// Add a single command.
    pub fn add_command(mut self, command: Command) -> Self {
        self.commands.push(command);
        self
    }

    /// Add a command group.
    pub fn add_group(mut self, group: CommandGroup) -> Self {
        for cmd in group.commands {
            self.commands.push(cmd);
        }
        self
    }

    /// Set the search query.
    pub fn query(mut self, query: impl Into<String>) -> Self {
        self.query = query.into();
        self
    }

    /// Set the selected index.
    pub fn selected(mut self, index: usize) -> Self {
        self.selected_index = index;
        self
    }

    /// Set the placeholder text.
    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    /// Set maximum visible results.
    pub fn max_visible(mut self, max: usize) -> Self {
        self.max_visible = max;
        self
    }

    /// Set the palette width.
    pub fn width(mut self, width: u16) -> Self {
        self.width = width;
        self
    }

    /// Set whether to show groups.
    pub fn show_groups(mut self, show: bool) -> Self {
        self.show_groups = show;
        self
    }

    /// Set whether to show shortcuts.
    pub fn show_shortcuts(mut self, show: bool) -> Self {
        self.show_shortcuts = show;
        self
    }

    /// Set whether to show descriptions.
    pub fn show_descriptions(mut self, show: bool) -> Self {
        self.show_descriptions = show;
        self
    }

    /// Set the title.
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set the accent color.
    pub fn accent_color(mut self, color: Color) -> Self {
        self.accent_color = color;
        self
    }

    /// Use ASCII characters.
    pub fn ascii(mut self, ascii: bool) -> Self {
        self.ascii = ascii;
        self
    }

    /// Get filtered and sorted commands based on query.
    pub fn filtered_commands(&self) -> Vec<&Command> {
        let mut results: Vec<&Command> = self
            .commands
            .iter()
            .filter(|cmd| !cmd.disabled && cmd.matches(&self.query))
            .collect();

        // Sort by match score (descending)
        results.sort_by(|a, b| {
            b.match_score(&self.query).cmp(&a.match_score(&self.query))
        });

        results
    }

    /// Build the palette VNode.
    pub fn build(self) -> VNode {
        if !self.open {
            return VNode::Empty;
        }

        let filtered = self.filtered_commands();
        let accent = self.accent_color;

        // Build header with title and close button
        let header = self.build_header();

        // Build search input
        let search_input = self.build_search_input();

        // Build results list
        let results_list = self.build_results(&filtered);

        // Build footer with help
        let footer = self.build_footer(filtered.len());

        // Combine all parts
        let mut palette_children = Vec::new();

        if let Some(header) = header {
            palette_children.push(header);
        }

        palette_children.push(search_input);
        palette_children.push(results_list);
        palette_children.push(footer);

        // Palette box
        let palette_box = VNode::Box(BoxNode {
            id: None,
            style: BoxStyle {
                flex_direction: Some(FlexDirection::Column),
                width: Some(Size::Fixed(self.width)),
                border_style: Some(BorderStyle::Round),
                border_color: Some(accent),
                background: Some(Color::Named(NamedColor::Black)),
                padding: Some(1),
                gap: Some(1),
                ..Default::default()
            },
            children: palette_children,
            handlers: EventHandlers {
                focusable: true,
                ..Default::default()
            },
        });

        // Center container
        let center_container = VNode::Box(BoxNode {
            id: None,
            style: BoxStyle {
                flex_direction: Some(FlexDirection::Column),
                justify_content: Some(JustifyContent::FlexStart),
                align_items: Some(AlignItems::Center),
                flex_grow: Some(1.0),
                padding_top: Some(3), // Some space from top
                ..Default::default()
            },
            children: vec![palette_box],
            handlers: Default::default(),
        });

        // Backdrop overlay
        VNode::Box(BoxNode {
            id: None,
            style: BoxStyle {
                flex_direction: Some(FlexDirection::Column),
                justify_content: Some(JustifyContent::FlexStart),
                align_items: Some(AlignItems::Center),
                flex_grow: Some(1.0),
                background: Some(Color::Rgb(0, 0, 0)),
                ..Default::default()
            },
            children: vec![center_container],
            handlers: Default::default(),
        })
    }

    /// Build the header row.
    fn build_header(&self) -> Option<VNode> {
        self.title.as_ref().map(|title| {
            VNode::Box(BoxNode {
                id: None,
                style: BoxStyle {
                    flex_direction: Some(FlexDirection::Row),
                    justify_content: Some(JustifyContent::SpaceBetween),
                    align_items: Some(AlignItems::Center),
                    ..Default::default()
                },
                children: vec![
                    VNode::Text(TextNode {
                        content: title.clone(),
                        style: TextStyle {
                            bold: true,
                            color: Some(self.accent_color),
                            ..Default::default()
                        },
                    }),
                    VNode::Text(TextNode {
                        content: "[ESC]".to_string(),
                        style: TextStyle {
                            color: Some(Color::Named(NamedColor::Gray)),
                            ..Default::default()
                        },
                    }),
                ],
                handlers: Default::default(),
            })
        })
    }

    /// Build the search input area.
    fn build_search_input(&self) -> VNode {
        let search_icon = if self.ascii { ">" } else { "⌕" };

        let display_text = if self.query.is_empty() {
            self.placeholder.clone()
        } else {
            self.query.clone()
        };

        let text_color = if self.query.is_empty() {
            Color::Named(NamedColor::Gray)
        } else {
            Color::Named(NamedColor::White)
        };

        VNode::Box(BoxNode {
            id: None,
            style: BoxStyle {
                flex_direction: Some(FlexDirection::Row),
                align_items: Some(AlignItems::Center),
                gap: Some(1),
                border_style: Some(BorderStyle::Single),
                border_color: Some(self.accent_color),
                padding: Some(1),
                ..Default::default()
            },
            children: vec![
                VNode::Text(TextNode {
                    content: search_icon.to_string(),
                    style: TextStyle {
                        color: Some(self.accent_color),
                        ..Default::default()
                    },
                }),
                VNode::Text(TextNode {
                    content: display_text,
                    style: TextStyle {
                        color: Some(text_color),
                        ..Default::default()
                    },
                }),
            ],
            handlers: EventHandlers {
                focusable: true,
                ..Default::default()
            },
        })
    }

    /// Build the results list.
    fn build_results(&self, filtered: &[&Command]) -> VNode {
        if filtered.is_empty() {
            return VNode::Box(BoxNode {
                id: None,
                style: BoxStyle {
                    flex_direction: Some(FlexDirection::Column),
                    align_items: Some(AlignItems::Center),
                    padding: Some(2),
                    ..Default::default()
                },
                children: vec![VNode::Text(TextNode {
                    content: "No commands found".to_string(),
                    style: TextStyle {
                        color: Some(Color::Named(NamedColor::Gray)),
                        italic: true,
                        ..Default::default()
                    },
                })],
                handlers: Default::default(),
            });
        }

        let mut result_items = Vec::new();
        let mut current_group: Option<String> = None;

        for (idx, cmd) in filtered.iter().take(self.max_visible).enumerate() {
            // Add group header if needed
            if self.show_groups {
                if let Some(ref group) = cmd.group {
                    if current_group.as_ref() != Some(group) {
                        current_group = Some(group.clone());
                        result_items.push(self.build_group_header(group));
                    }
                }
            }

            // Add command item
            let is_selected = idx == self.selected_index;
            result_items.push(self.build_command_item(cmd, is_selected));
        }

        // Show "more results" indicator if there are hidden results
        if filtered.len() > self.max_visible {
            let remaining = filtered.len() - self.max_visible;
            result_items.push(VNode::Text(TextNode {
                content: format!("  ... and {} more", remaining),
                style: TextStyle {
                    color: Some(Color::Named(NamedColor::Gray)),
                    italic: true,
                    ..Default::default()
                },
            }));
        }

        VNode::Box(BoxNode {
            id: None,
            style: BoxStyle {
                flex_direction: Some(FlexDirection::Column),
                gap: Some(0),
                ..Default::default()
            },
            children: result_items,
            handlers: Default::default(),
        })
    }

    /// Build a group header.
    fn build_group_header(&self, name: &str) -> VNode {
        VNode::Box(BoxNode {
            id: None,
            style: BoxStyle {
                flex_direction: Some(FlexDirection::Row),
                padding_top: Some(1),
                padding_bottom: Some(0),
                ..Default::default()
            },
            children: vec![VNode::Text(TextNode {
                content: name.to_uppercase(),
                style: TextStyle {
                    color: Some(Color::Named(NamedColor::Gray)),
                    bold: true,
                    ..Default::default()
                },
            })],
            handlers: Default::default(),
        })
    }

    /// Build a single command item.
    fn build_command_item(&self, cmd: &Command, selected: bool) -> VNode {
        let bg_color = if selected {
            Some(self.accent_color)
        } else {
            None
        };

        let text_color = if selected {
            Color::Named(NamedColor::Black)
        } else {
            Color::Named(NamedColor::White)
        };

        let mut row_children = Vec::new();

        // Selection indicator
        let indicator = if selected {
            if self.ascii { ">" } else { "▸" }
        } else {
            " "
        };

        row_children.push(VNode::Text(TextNode {
            content: indicator.to_string(),
            style: TextStyle {
                color: Some(if selected {
                    Color::Named(NamedColor::Black)
                } else {
                    self.accent_color
                }),
                bold: selected,
                ..Default::default()
            },
        }));

        // Icon if present
        if let Some(ref icon) = cmd.icon {
            row_children.push(VNode::Text(TextNode {
                content: format!("{} ", icon),
                style: TextStyle {
                    color: Some(text_color),
                    ..Default::default()
                },
            }));
        }

        // Label and description column
        let mut label_col_children = Vec::new();

        label_col_children.push(VNode::Text(TextNode {
            content: cmd.label.clone(),
            style: TextStyle {
                color: Some(text_color),
                bold: true,
                ..Default::default()
            },
        }));

        if self.show_descriptions {
            if let Some(ref desc) = cmd.description {
                label_col_children.push(VNode::Text(TextNode {
                    content: desc.clone(),
                    style: TextStyle {
                        color: Some(if selected {
                            Color::Named(NamedColor::BrightBlack)
                        } else {
                            Color::Named(NamedColor::Gray)
                        }),
                        italic: true,
                        ..Default::default()
                    },
                }));
            }
        }

        row_children.push(VNode::Box(BoxNode {
            id: None,
            style: BoxStyle {
                flex_direction: Some(FlexDirection::Column),
                flex_grow: Some(1.0),
                gap: Some(0),
                ..Default::default()
            },
            children: label_col_children,
            handlers: Default::default(),
        }));

        // Shortcut if present
        if self.show_shortcuts {
            if let Some(ref shortcut) = cmd.shortcut {
                row_children.push(VNode::Text(TextNode {
                    content: shortcut.clone(),
                    style: TextStyle {
                        color: Some(if selected {
                            Color::Named(NamedColor::BrightBlack)
                        } else {
                            Color::Named(NamedColor::Gray)
                        }),
                        ..Default::default()
                    },
                }));
            }
        }

        VNode::Box(BoxNode {
            id: None,
            style: BoxStyle {
                flex_direction: Some(FlexDirection::Row),
                align_items: Some(AlignItems::Center),
                gap: Some(1),
                background: bg_color,
                padding_left: Some(1),
                padding_right: Some(1),
                ..Default::default()
            },
            children: row_children,
            handlers: Default::default(),
        })
    }

    /// Build the footer with keyboard hints.
    fn build_footer(&self, result_count: usize) -> VNode {
        let arrow_keys = if self.ascii { "[UP/DN]" } else { "[↑↓]" };

        // Separator line
        let separator = VNode::Text(TextNode {
            content: "─".repeat(self.width.saturating_sub(4) as usize),
            style: TextStyle {
                color: Some(Color::Named(NamedColor::BrightBlack)),
                ..Default::default()
            },
        });

        // Footer content row
        let footer_row = VNode::Box(BoxNode {
            id: None,
            style: BoxStyle {
                flex_direction: Some(FlexDirection::Row),
                justify_content: Some(JustifyContent::SpaceBetween),
                align_items: Some(AlignItems::Center),
                ..Default::default()
            },
            children: vec![
                VNode::Text(TextNode {
                    content: format!("{} results", result_count),
                    style: TextStyle {
                        color: Some(Color::Named(NamedColor::Gray)),
                        ..Default::default()
                    },
                }),
                VNode::Text(TextNode {
                    content: format!("{} Navigate  [Enter] Select  [Esc] Close", arrow_keys),
                    style: TextStyle {
                        color: Some(Color::Named(NamedColor::Gray)),
                        ..Default::default()
                    },
                }),
            ],
            handlers: Default::default(),
        });

        VNode::Box(BoxNode {
            id: None,
            style: BoxStyle {
                flex_direction: Some(FlexDirection::Column),
                gap: Some(1),
                ..Default::default()
            },
            children: vec![separator, footer_row],
            handlers: Default::default(),
        })
    }
}

impl From<CommandPalette> for VNode {
    fn from(palette: CommandPalette) -> VNode {
        palette.build()
    }
}

// =============================================================================
// Command Palette State
// =============================================================================

/// State manager for CommandPalette.
///
/// Provides reactive state management for the palette with signals.
pub struct CommandPaletteState {
    /// Whether palette is open.
    open_read: ReadSignal<bool>,
    open_write: WriteSignal<bool>,
    /// Current search query.
    query_read: ReadSignal<String>,
    query_write: WriteSignal<String>,
    /// Selected index in filtered results.
    selected_read: ReadSignal<usize>,
    selected_write: WriteSignal<usize>,
    /// All registered commands.
    commands_read: ReadSignal<Vec<Command>>,
    commands_write: WriteSignal<Vec<Command>>,
}

impl CommandPaletteState {
    /// Create a new command palette state.
    pub fn new() -> Self {
        let (open_read, open_write) = create_signal(false);
        let (query_read, query_write) = create_signal(String::new());
        let (selected_read, selected_write) = create_signal(0usize);
        let (commands_read, commands_write) = create_signal(Vec::new());

        Self {
            open_read,
            open_write,
            query_read,
            query_write,
            selected_read,
            selected_write,
            commands_read,
            commands_write,
        }
    }

    /// Check if palette is open.
    pub fn is_open(&self) -> bool {
        self.open_read.get()
    }

    /// Open the palette.
    pub fn show(&self) {
        self.open_write.set(true);
        self.query_write.set(String::new());
        self.selected_write.set(0);
    }

    /// Close the palette.
    pub fn hide(&self) {
        self.open_write.set(false);
    }

    /// Toggle the palette.
    pub fn toggle(&self) {
        if self.is_open() {
            self.hide();
        } else {
            self.show();
        }
    }

    /// Get the current query.
    pub fn query(&self) -> String {
        self.query_read.get()
    }

    /// Set the search query.
    pub fn set_query(&self, query: impl Into<String>) {
        self.query_write.set(query.into());
        self.selected_write.set(0); // Reset selection on query change
    }

    /// Append a character to the query.
    pub fn append_char(&self, c: char) {
        let mut q = self.query_read.get();
        q.push(c);
        self.set_query(q);
    }

    /// Remove the last character from the query.
    pub fn backspace(&self) {
        let mut q = self.query_read.get();
        q.pop();
        self.set_query(q);
    }

    /// Clear the query.
    pub fn clear_query(&self) {
        self.set_query(String::new());
    }

    /// Get the selected index.
    pub fn selected(&self) -> usize {
        self.selected_read.get()
    }

    /// Move selection up.
    pub fn select_prev(&self) {
        let current = self.selected_read.get();
        if current > 0 {
            self.selected_write.set(current - 1);
        }
    }

    /// Move selection down.
    pub fn select_next(&self, max_items: usize) {
        let current = self.selected_read.get();
        if current + 1 < max_items {
            self.selected_write.set(current + 1);
        }
    }

    /// Move selection to first item.
    pub fn select_first(&self) {
        self.selected_write.set(0);
    }

    /// Move selection to last item.
    pub fn select_last(&self, max_items: usize) {
        if max_items > 0 {
            self.selected_write.set(max_items - 1);
        }
    }

    /// Get all commands.
    pub fn commands(&self) -> Vec<Command> {
        self.commands_read.get()
    }

    /// Register commands.
    pub fn register(&self, commands: Vec<Command>) {
        self.commands_write.set(commands);
    }

    /// Add a single command.
    pub fn add_command(&self, command: Command) {
        let mut cmds = self.commands_read.get();
        cmds.push(command);
        self.commands_write.set(cmds);
    }

    /// Remove a command by ID.
    pub fn remove_command(&self, id: &str) {
        let mut cmds = self.commands_read.get();
        cmds.retain(|c| c.id != id);
        self.commands_write.set(cmds);
    }

    /// Get filtered commands based on current query.
    pub fn filtered(&self) -> Vec<Command> {
        let query = self.query_read.get();
        let commands = self.commands_read.get();

        let mut results: Vec<Command> = commands
            .into_iter()
            .filter(|cmd| !cmd.disabled && cmd.matches(&query))
            .collect();

        // Sort by match score
        results.sort_by(|a, b| b.match_score(&query).cmp(&a.match_score(&query)));

        results
    }

    /// Get the currently selected command (if any).
    pub fn selected_command(&self) -> Option<Command> {
        let filtered = self.filtered();
        let idx = self.selected_read.get();
        filtered.into_iter().nth(idx)
    }

    /// Execute the selected command and close the palette.
    /// Returns the selected command ID if any.
    pub fn execute_selected(&self) -> Option<String> {
        let cmd = self.selected_command();
        self.hide();
        cmd.map(|c| c.id)
    }

    /// Get the open signal for reactive binding.
    pub fn open_signal(&self) -> ReadSignal<bool> {
        self.open_read.clone()
    }

    /// Get the query signal for reactive binding.
    pub fn query_signal(&self) -> ReadSignal<String> {
        self.query_read.clone()
    }

    /// Get the selected signal for reactive binding.
    pub fn selected_signal(&self) -> ReadSignal<usize> {
        self.selected_read.clone()
    }
}

impl Default for CommandPaletteState {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_creation() {
        let cmd = Command::new("test.command", "Test Command")
            .description("A test command")
            .shortcut("Ctrl+T")
            .group("Testing")
            .icon("🧪");

        assert_eq!(cmd.id, "test.command");
        assert_eq!(cmd.label, "Test Command");
        assert_eq!(cmd.description, Some("A test command".to_string()));
        assert_eq!(cmd.shortcut, Some("Ctrl+T".to_string()));
        assert_eq!(cmd.group, Some("Testing".to_string()));
    }

    #[test]
    fn test_command_matching() {
        let cmd = Command::new("file.save", "Save File")
            .description("Save the current file")
            .keywords(vec!["write", "store"]);

        // Matches label
        assert!(cmd.matches("save"));
        assert!(cmd.matches("file"));
        assert!(cmd.matches("SAVE")); // Case insensitive

        // Matches description
        assert!(cmd.matches("current"));

        // Matches keywords
        assert!(cmd.matches("write"));
        assert!(cmd.matches("store"));

        // Doesn't match
        assert!(!cmd.matches("delete"));
        assert!(!cmd.matches("open"));
    }

    #[test]
    fn test_command_match_score() {
        let cmd = Command::new("file.save", "Save File")
            .description("Save the current file")
            .keywords(vec!["write"]);

        // Exact match is best
        assert_eq!(cmd.match_score("save file"), 100);

        // Starts with is very good
        assert_eq!(cmd.match_score("save"), 80);

        // Contains in label
        assert_eq!(cmd.match_score("file"), 60);

        // Keyword match
        assert_eq!(cmd.match_score("write"), 40);

        // Description match
        assert_eq!(cmd.match_score("current"), 20);

        // No match
        assert_eq!(cmd.match_score("delete"), 0);
    }

    #[test]
    fn test_command_group() {
        let group = CommandGroup::new("File")
            .add(Command::new("file.save", "Save"))
            .add(Command::new("file.open", "Open"));

        assert_eq!(group.name, "File");
        assert_eq!(group.commands.len(), 2);
        assert_eq!(group.commands[0].group, Some("File".to_string()));
    }

    #[test]
    fn test_palette_closed_returns_empty() {
        let palette = CommandPalette::new().open(false).build();
        matches!(palette, VNode::Empty);
    }

    #[test]
    fn test_palette_open_returns_box() {
        let palette = CommandPalette::new()
            .open(true)
            .add_command(Command::new("test", "Test"))
            .build();
        matches!(palette, VNode::Box(_));
    }

    #[test]
    fn test_palette_filtering() {
        let palette = CommandPalette::new()
            .commands(vec![
                Command::new("file.save", "Save File"),
                Command::new("file.open", "Open File"),
                Command::new("edit.undo", "Undo"),
            ])
            .query("file");

        let filtered = palette.filtered_commands();
        assert_eq!(filtered.len(), 2);
    }

    #[test]
    fn test_palette_state() {
        let state = CommandPaletteState::new();

        assert!(!state.is_open());
        assert_eq!(state.query(), "");

        state.show();
        assert!(state.is_open());

        state.set_query("test");
        assert_eq!(state.query(), "test");

        state.append_char('!');
        assert_eq!(state.query(), "test!");

        state.backspace();
        assert_eq!(state.query(), "test");

        state.hide();
        assert!(!state.is_open());
    }

    #[test]
    fn test_state_navigation() {
        let state = CommandPaletteState::new();
        state.register(vec![
            Command::new("a", "A"),
            Command::new("b", "B"),
            Command::new("c", "C"),
        ]);

        assert_eq!(state.selected(), 0);

        state.select_next(3);
        assert_eq!(state.selected(), 1);

        state.select_next(3);
        assert_eq!(state.selected(), 2);

        state.select_next(3); // At end, should stay
        assert_eq!(state.selected(), 2);

        state.select_prev();
        assert_eq!(state.selected(), 1);

        state.select_first();
        assert_eq!(state.selected(), 0);

        state.select_last(3);
        assert_eq!(state.selected(), 2);
    }

    #[test]
    fn test_state_execute() {
        let state = CommandPaletteState::new();
        state.register(vec![
            Command::new("cmd1", "Command 1"),
            Command::new("cmd2", "Command 2"),
        ]);

        state.show();
        state.select_next(2); // Select second command

        let executed = state.execute_selected();
        assert_eq!(executed, Some("cmd2".to_string()));
        assert!(!state.is_open()); // Should close after execution
    }
}
