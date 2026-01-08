//! File Browser Component
//!
//! A comprehensive file system navigation component.

use crate::core::component::{VNode, BoxNode, BoxStyle, TextStyle, Color, NamedColor, BorderStyle};
use crate::core::layout::{FlexDirection, Size};

/// File item type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileItemType {
    File,
    Directory,
    Symlink,
    Unknown,
}

impl Default for FileItemType {
    fn default() -> Self {
        Self::File
    }
}

/// File system item.
#[derive(Debug, Clone)]
pub struct FileItem {
    /// Item name
    pub name: String,
    /// Full path
    pub path: String,
    /// Item type
    pub item_type: FileItemType,
    /// File size in bytes
    pub size: Option<u64>,
    /// Last modified timestamp (Unix epoch)
    pub modified: Option<u64>,
    /// File extension
    pub extension: Option<String>,
    /// Is hidden file (starts with .)
    pub hidden: bool,
    /// Is read-only
    pub readonly: bool,
    /// Children (for directories)
    pub children: Option<Vec<FileItem>>,
    /// Is expanded (for tree view)
    pub expanded: bool,
    /// Depth in tree
    pub depth: usize,
}

impl FileItem {
    /// Create a new file item.
    pub fn file(name: impl Into<String>, path: impl Into<String>) -> Self {
        let name = name.into();
        let extension = name.rfind('.').map(|i| name[i + 1..].to_string());
        Self {
            name,
            path: path.into(),
            item_type: FileItemType::File,
            size: None,
            modified: None,
            extension,
            hidden: false,
            readonly: false,
            children: None,
            expanded: false,
            depth: 0,
        }
    }

    /// Create a new directory item.
    pub fn directory(name: impl Into<String>, path: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            path: path.into(),
            item_type: FileItemType::Directory,
            size: None,
            modified: None,
            extension: None,
            hidden: false,
            readonly: false,
            children: Some(Vec::new()),
            expanded: false,
            depth: 0,
        }
    }

    /// Set file size.
    pub fn size(mut self, size: u64) -> Self {
        self.size = Some(size);
        self
    }

    /// Set modified time.
    pub fn modified(mut self, timestamp: u64) -> Self {
        self.modified = Some(timestamp);
        self
    }

    /// Set as hidden.
    pub fn hidden(mut self, hidden: bool) -> Self {
        self.hidden = hidden;
        self
    }

    /// Set as readonly.
    pub fn readonly(mut self, readonly: bool) -> Self {
        self.readonly = readonly;
        self
    }

    /// Add children (for directories).
    pub fn children<I>(mut self, children: I) -> Self
    where
        I: IntoIterator<Item = FileItem>,
    {
        self.children = Some(children.into_iter().collect());
        self
    }

    /// Set expanded state.
    pub fn expanded(mut self, expanded: bool) -> Self {
        self.expanded = expanded;
        self
    }

    /// Set depth.
    pub fn depth(mut self, depth: usize) -> Self {
        self.depth = depth;
        self
    }

    /// Get icon for this item.
    pub fn icon(&self) -> &str {
        match self.item_type {
            FileItemType::Directory => {
                if self.expanded { "📂" } else { "📁" }
            }
            FileItemType::Symlink => "🔗",
            FileItemType::File => {
                match self.extension.as_deref() {
                    Some("rs") => "🦀",
                    Some("js") | Some("ts") | Some("jsx") | Some("tsx") => "📜",
                    Some("py") => "🐍",
                    Some("go") => "🐹",
                    Some("md") | Some("txt") => "📝",
                    Some("json") | Some("yaml") | Some("yml") | Some("toml") => "⚙️",
                    Some("html") | Some("htm") => "🌐",
                    Some("css") | Some("scss") | Some("sass") => "🎨",
                    Some("png") | Some("jpg") | Some("jpeg") | Some("gif") | Some("svg") => "🖼️",
                    Some("zip") | Some("tar") | Some("gz") | Some("rar") => "📦",
                    Some("pdf") => "📕",
                    Some("exe") | Some("bin") => "⚡",
                    _ => "📄",
                }
            }
            FileItemType::Unknown => "❓",
        }
    }

    /// Format file size.
    pub fn format_size(&self) -> String {
        match self.size {
            Some(size) => {
                if size < 1024 {
                    format!("{} B", size)
                } else if size < 1024 * 1024 {
                    format!("{:.1} KB", size as f64 / 1024.0)
                } else if size < 1024 * 1024 * 1024 {
                    format!("{:.1} MB", size as f64 / (1024.0 * 1024.0))
                } else {
                    format!("{:.1} GB", size as f64 / (1024.0 * 1024.0 * 1024.0))
                }
            }
            None => String::from("-"),
        }
    }
}

/// Sort field for file lists.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum FileSortField {
    #[default]
    Name,
    Size,
    Modified,
    Type,
}

/// Sort direction.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum FileSortDirection {
    #[default]
    Ascending,
    Descending,
}

/// File browser state.
#[derive(Debug, Clone)]
pub struct FileBrowserState {
    /// Current directory path
    pub current_path: String,
    /// Selected item index
    pub selected: usize,
    /// Scroll offset
    pub offset: usize,
    /// Visible height
    pub visible_height: usize,
    /// Sort field
    pub sort_field: FileSortField,
    /// Sort direction
    pub sort_direction: FileSortDirection,
    /// Show hidden files
    pub show_hidden: bool,
    /// Current items
    pub items: Vec<FileItem>,
}

impl Default for FileBrowserState {
    fn default() -> Self {
        Self {
            current_path: ".".to_string(),
            selected: 0,
            offset: 0,
            visible_height: 15,
            sort_field: FileSortField::Name,
            sort_direction: FileSortDirection::Ascending,
            show_hidden: false,
            items: Vec::new(),
        }
    }
}

impl FileBrowserState {
    /// Create new state.
    pub fn new(path: impl Into<String>) -> Self {
        Self {
            current_path: path.into(),
            ..Default::default()
        }
    }

    /// Set items.
    pub fn set_items(&mut self, items: Vec<FileItem>) {
        self.items = items;
        self.selected = 0;
        self.offset = 0;
        self.sort();
    }

    /// Sort items.
    pub fn sort(&mut self) {
        let dir = self.sort_direction;
        match self.sort_field {
            FileSortField::Name => {
                self.items.sort_by(|a, b| {
                    // Directories first
                    let type_cmp = (a.item_type != FileItemType::Directory)
                        .cmp(&(b.item_type != FileItemType::Directory));
                    if type_cmp != std::cmp::Ordering::Equal {
                        return type_cmp;
                    }
                    let cmp = a.name.to_lowercase().cmp(&b.name.to_lowercase());
                    if dir == FileSortDirection::Descending {
                        cmp.reverse()
                    } else {
                        cmp
                    }
                });
            }
            FileSortField::Size => {
                self.items.sort_by(|a, b| {
                    let cmp = a.size.cmp(&b.size);
                    if dir == FileSortDirection::Descending {
                        cmp.reverse()
                    } else {
                        cmp
                    }
                });
            }
            FileSortField::Modified => {
                self.items.sort_by(|a, b| {
                    let cmp = a.modified.cmp(&b.modified);
                    if dir == FileSortDirection::Descending {
                        cmp.reverse()
                    } else {
                        cmp
                    }
                });
            }
            FileSortField::Type => {
                self.items.sort_by(|a, b| {
                    let cmp = a.extension.cmp(&b.extension);
                    if dir == FileSortDirection::Descending {
                        cmp.reverse()
                    } else {
                        cmp
                    }
                });
            }
        }

        // Filter hidden if needed
        if !self.show_hidden {
            self.items.retain(|item| !item.hidden);
        }
    }

    /// Move selection up.
    pub fn up(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
            self.adjust_scroll();
        }
    }

    /// Move selection down.
    pub fn down(&mut self) {
        if self.selected < self.items.len().saturating_sub(1) {
            self.selected += 1;
            self.adjust_scroll();
        }
    }

    /// Adjust scroll offset.
    fn adjust_scroll(&mut self) {
        if self.selected < self.offset {
            self.offset = self.selected;
        } else if self.selected >= self.offset + self.visible_height {
            self.offset = self.selected.saturating_sub(self.visible_height) + 1;
        }
    }

    /// Get selected item.
    pub fn selected_item(&self) -> Option<&FileItem> {
        self.items.get(self.selected)
    }

    /// Toggle hidden files.
    pub fn toggle_hidden(&mut self) {
        self.show_hidden = !self.show_hidden;
        self.sort();
    }
}

/// Create a new file browser state.
pub fn create_file_browser_state(path: impl Into<String>) -> FileBrowserState {
    FileBrowserState::new(path)
}

/// File browser component.
#[derive(Debug, Clone)]
pub struct FileBrowser {
    state: FileBrowserState,
    show_path: bool,
    show_details: bool,
    show_status: bool,
    width: Option<u16>,
}

impl Default for FileBrowser {
    fn default() -> Self {
        Self {
            state: FileBrowserState::default(),
            show_path: true,
            show_details: true,
            show_status: true,
            width: None,
        }
    }
}

impl FileBrowser {
    /// Create a new file browser.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create with state.
    pub fn with_state(state: FileBrowserState) -> Self {
        Self {
            state,
            ..Default::default()
        }
    }

    /// Set state.
    pub fn state(mut self, state: FileBrowserState) -> Self {
        self.state = state;
        self
    }

    /// Show/hide path bar.
    pub fn show_path(mut self, show: bool) -> Self {
        self.show_path = show;
        self
    }

    /// Show/hide file details.
    pub fn show_details(mut self, show: bool) -> Self {
        self.show_details = show;
        self
    }

    /// Show/hide status bar.
    pub fn show_status(mut self, show: bool) -> Self {
        self.show_status = show;
        self
    }

    /// Set width.
    pub fn width(mut self, width: u16) -> Self {
        self.width = Some(width);
        self
    }

    /// Build path breadcrumbs.
    fn build_path_bar(&self) -> VNode {
        VNode::Box(BoxNode {
            children: vec![
                VNode::styled_text(
                    format!(" 📁 {} ", self.state.current_path),
                    TextStyle {
                        color: Some(Color::Named(NamedColor::Cyan)),
                        bold: true,
                        ..Default::default()
                    },
                ),
            ],
            style: BoxStyle {
                border_style: Some(BorderStyle::Single),
                ..Default::default()
            },
            ..Default::default()
        })
    }

    /// Build a single file item.
    fn build_item(&self, item: &FileItem, is_selected: bool) -> VNode {
        let icon = item.icon();
        let indent = "  ".repeat(item.depth);

        let name_style = if is_selected {
            TextStyle {
                color: Some(Color::Named(NamedColor::Black)),
                bold: true,
                inverse: true,
                ..Default::default()
            }
        } else {
            match item.item_type {
                FileItemType::Directory => TextStyle {
                    color: Some(Color::Named(NamedColor::Blue)),
                    bold: true,
                    ..Default::default()
                },
                FileItemType::Symlink => TextStyle {
                    color: Some(Color::Named(NamedColor::Cyan)),
                    ..Default::default()
                },
                _ => TextStyle::default(),
            }
        };

        let details = if self.show_details {
            format!("  {:>8}", item.format_size())
        } else {
            String::new()
        };

        VNode::styled_text(
            format!("{}{} {}{}", indent, icon, item.name, details),
            name_style,
        )
    }

    /// Build status bar.
    fn build_status_bar(&self) -> VNode {
        let total = self.state.items.len();
        let dirs = self.state.items.iter().filter(|i| i.item_type == FileItemType::Directory).count();
        let files = total - dirs;

        let status = format!(
            " {} items ({} dirs, {} files) | {}",
            total,
            dirs,
            files,
            if self.state.show_hidden { "Hidden: ON" } else { "Hidden: OFF" }
        );

        VNode::styled_text(
            status,
            TextStyle {
                color: Some(Color::Named(NamedColor::Gray)),
                dim: true,
                ..Default::default()
            },
        )
    }

    /// Build the VNode.
    pub fn build(self) -> VNode {
        let mut children: Vec<VNode> = Vec::new();

        // Path bar
        if self.show_path {
            children.push(self.build_path_bar());
        }

        // File list
        let visible_end = (self.state.offset + self.state.visible_height).min(self.state.items.len());
        for idx in self.state.offset..visible_end {
            if let Some(item) = self.state.items.get(idx) {
                let is_selected = idx == self.state.selected;
                children.push(self.build_item(item, is_selected));
            }
        }

        // Status bar
        if self.show_status {
            children.push(self.build_status_bar());
        }

        let mut container_style = BoxStyle {
            flex_direction: Some(FlexDirection::Column),
            border_style: Some(BorderStyle::Round),
            padding_left: Some(1),
            padding_right: Some(1),
            ..Default::default()
        };

        if let Some(w) = self.width {
            container_style.width = Some(Size::Fixed(w));
        }

        VNode::Box(BoxNode {
            children,
            style: container_style,
            ..Default::default()
        })
    }
}
