//! File Browser Component
//!
//! A comprehensive file system navigation component.

use crate::core::component::{BorderStyle, BoxNode, BoxStyle, Color, NamedColor, TextStyle, VNode};
use crate::core::layout::{FlexDirection, Size};

/// File item type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileItemType {
    File,
    Directory,
    Symlink,
    Device,
    Socket,
    Fifo,
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
    /// Created timestamp (Unix epoch)
    pub created: Option<u64>,
    /// Accessed timestamp (Unix epoch)
    pub accessed: Option<u64>,
    /// Permissions string
    pub permissions: Option<String>,
    /// Owner user/group names
    pub owner: Option<String>,
    pub group: Option<String>,
    /// MIME type (if known)
    pub mime_type: Option<String>,
    /// Symlink target/path
    pub link_target: Option<String>,
    /// Is executable
    pub is_executable: bool,
    /// Is hidden file (starts with .)
    pub hidden: bool,
    /// Is read-only
    pub readonly: bool,
    /// File extension
    pub extension: Option<String>,
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
            created: None,
            accessed: None,
            permissions: None,
            owner: None,
            group: None,
            mime_type: None,
            link_target: None,
            is_executable: false,
            hidden: false,
            readonly: false,
            extension,
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
            created: None,
            accessed: None,
            permissions: None,
            owner: None,
            group: None,
            mime_type: None,
            link_target: None,
            is_executable: false,
            hidden: false,
            readonly: false,
            extension: None,
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
                if self.expanded {
                    "📂"
                } else {
                    "📁"
                }
            }
            FileItemType::Symlink => "🔗",
            FileItemType::Device => "💾",
            FileItemType::Socket => "🔌",
            FileItemType::Fifo => "📤",
            FileItemType::File => match self.extension.as_deref() {
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
            },
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
    Extension,
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
            FileSortField::Extension => {
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
            children: vec![VNode::styled_text(
                format!(" 📁 {} ", self.state.current_path),
                TextStyle {
                    color: Some(Color::Named(NamedColor::Cyan)),
                    bold: true,
                    ..Default::default()
                },
            )],
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
        let dirs = self
            .state
            .items
            .iter()
            .filter(|i| i.item_type == FileItemType::Directory)
            .count();
        let files = total - dirs;

        let status = format!(
            " {} items ({} dirs, {} files) | {}",
            total,
            dirs,
            files,
            if self.state.show_hidden {
                "Hidden: ON"
            } else {
                "Hidden: OFF"
            }
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
        let visible_end =
            (self.state.offset + self.state.visible_height).min(self.state.items.len());
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

// =============================================================================
// JS-Compatible API
// =============================================================================

use std::collections::HashSet;
use std::time::{SystemTime, UNIX_EPOCH};

/// Predicate to filter entries.
pub type FileFilter = fn(&FileItem) -> bool;

/// Sort rule for file collections.
#[derive(Debug, Clone, Copy)]
pub struct FileSorter {
    pub field: FileSortField,
    pub direction: FileSortDirection,
}

/// File item type options.
pub type FileListColumn = FileColumn;
pub type FileDirectoryTreeOptions = DirectoryTreeOptions;

#[derive(Debug, Clone, Copy)]
pub struct FileColumn {
    pub field: FileItemField,
    pub label: &'static str,
    pub width: Option<usize>,
}

#[derive(Debug, Clone, Copy)]
pub enum FileItemField {
    Name,
    Size,
    Modified,
    Permissions,
    Type,
    Owner,
}

/// Directory tree component options.
pub struct DirectoryTreeOptions {
    pub items: Vec<FileItem>,
    pub selected: Option<String>,
    pub expanded: Option<HashSet<String>>,
    pub on_select: Option<fn(&FileItem)>,
    pub on_toggle: Option<fn(&FileItem, bool)>,
    pub on_open: Option<fn(&FileItem)>,
    pub show_hidden: bool,
    pub icons: Option<FileIcons>,
    pub indent_size: usize,
    pub line_style: &'static str,
    pub max_depth: usize,
    pub width: usize,
    pub height: usize,
}

impl Default for DirectoryTreeOptions {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            selected: None,
            expanded: None,
            on_select: None,
            on_toggle: None,
            on_open: None,
            show_hidden: false,
            icons: None,
            indent_size: 2,
            line_style: "unicode",
            max_depth: usize::MAX,
            width: 80,
            height: 0,
        }
    }
}

/// File list component options.
pub struct FileListOptions {
    pub items: Vec<FileItem>,
    pub selected: Option<String>,
    pub selected_items: Option<HashSet<String>>,
    pub multi_select: bool,
    pub on_select: Option<fn(&FileItem)>,
    pub on_open: Option<fn(&FileItem)>,
    pub on_selection_change: Option<fn(&[FileItem])>,
    pub show_hidden: bool,
    pub sort: Option<FileSorter>,
    pub filter: Option<FileFilter>,
    pub view_mode: &'static str,
    pub icons: Option<FileIcons>,
    pub columns: Vec<FileListColumn>,
    pub width: usize,
    pub height: usize,
    pub show_size: bool,
    pub show_modified: bool,
    pub show_permissions: bool,
}

impl Default for FileListOptions {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            selected: None,
            selected_items: None,
            multi_select: false,
            on_select: None,
            on_open: None,
            on_selection_change: None,
            show_hidden: false,
            sort: None,
            filter: None,
            view_mode: "list",
            icons: None,
            columns: Vec::new(),
            width: 80,
            height: 0,
            show_size: true,
            show_modified: true,
            show_permissions: false,
        }
    }
}

/// Path-breadcrumb options.
pub struct PathBreadcrumbsOptions {
    pub path: String,
    pub separator: &'static str,
    pub on_navigate: Option<fn(String)>,
    pub max_segments: usize,
    pub home_path: Option<String>,
    pub show_home_as: &'static str,
}

impl Default for PathBreadcrumbsOptions {
    fn default() -> Self {
        Self {
            path: String::new(),
            separator: "/",
            on_navigate: None,
            max_segments: 5,
            home_path: None,
            show_home_as: "~",
        }
    }
}

/// FileBrowser component options.
pub struct FileBrowserOptions {
    pub path: String,
    pub items: Vec<FileItem>,
    pub on_path_change: Option<fn(String)>,
    pub on_select: Option<fn(&FileItem)>,
    pub on_open: Option<fn(&FileItem)>,
    pub show_hidden: bool,
    pub sort: Option<FileSorter>,
    pub filter: Option<FileFilter>,
    pub view_mode: &'static str,
    pub show_breadcrumbs: bool,
    pub show_toolbar: bool,
    pub show_status_bar: bool,
    pub show_preview: bool,
    pub preview_width: usize,
    pub icons: Option<FileIcons>,
    pub width: usize,
    pub height: usize,
    pub split_view: bool,
    pub tree_width: usize,
}

impl Default for FileBrowserOptions {
    fn default() -> Self {
        Self {
            path: String::from("/"),
            items: Vec::new(),
            on_path_change: None,
            on_select: None,
            on_open: None,
            show_hidden: false,
            sort: None,
            filter: None,
            view_mode: "list",
            show_breadcrumbs: true,
            show_toolbar: false,
            show_status_bar: true,
            show_preview: false,
            preview_width: 40,
            icons: None,
            width: 80,
            height: 0,
            split_view: false,
            tree_width: 30,
        }
    }
}

/// File details options.
pub struct FileDetailsOptions {
    pub item: FileItem,
    pub show_permissions: bool,
    pub show_timestamps: bool,
    pub show_owner: bool,
    pub show_mime_type: bool,
    pub icons: Option<FileIcons>,
}

impl Default for FileDetailsOptions {
    fn default() -> Self {
        Self {
            item: FileItem::file("", ""),
            show_permissions: true,
            show_timestamps: true,
            show_owner: true,
            show_mime_type: true,
            icons: None,
        }
    }
}

/// File preview options.
pub struct FilePreviewOptions {
    pub item: FileItem,
    pub content: Option<String>,
    pub max_lines: usize,
    pub syntax_highlight: bool,
    pub line_numbers: bool,
    pub width: usize,
    pub height: usize,
    pub placeholder: &'static str,
    pub binary_message: &'static str,
    pub large_file_message: &'static str,
    pub max_preview_size: u64,
}

impl Default for FilePreviewOptions {
    fn default() -> Self {
        Self {
            item: FileItem::file("", ""),
            content: None,
            max_lines: 100,
            syntax_highlight: false,
            line_numbers: true,
            width: 80,
            height: 0,
            placeholder: "No preview available",
            binary_message: "Binary file - preview not available",
            large_file_message: "File too large for preview",
            max_preview_size: 1024 * 1024,
        }
    }
}

/// Icon set compatibility map.
#[derive(Debug, Clone, Copy)]
pub struct FileIcons {
    pub file: &'static str,
    pub directory: &'static str,
    pub directory_open: &'static str,
    pub symlink: &'static str,
    pub device: &'static str,
    pub socket: &'static str,
    pub fifo: &'static str,
    pub unknown: &'static str,
    pub tree_vertical: &'static str,
    pub tree_branch: &'static str,
    pub tree_corner: &'static str,
    pub tree_space: &'static str,
    pub js: &'static str,
    pub ts: &'static str,
    pub jsx: &'static str,
    pub tsx: &'static str,
    pub json: &'static str,
    pub md: &'static str,
    pub txt: &'static str,
    pub html: &'static str,
    pub css: &'static str,
    pub scss: &'static str,
    pub py: &'static str,
    pub rs: &'static str,
    pub go: &'static str,
    pub yml: &'static str,
    pub yaml: &'static str,
    pub toml: &'static str,
    pub png: &'static str,
    pub jpg: &'static str,
    pub gif: &'static str,
    pub exe: &'static str,
    pub bin: &'static str,
    pub zip: &'static str,
    pub tar: &'static str,
    pub gz: &'static str,
    pub mp3: &'static str,
    pub mp4: &'static str,
    pub pdf: &'static str,
    pub svg: &'static str,
    pub rb: &'static str,
    pub java: &'static str,
    pub c: &'static str,
    pub cpp: &'static str,
    pub h: &'static str,
    pub sh: &'static str,
    pub docker: &'static str,
}

pub const unicodeIcons: FileIcons = FileIcons {
    file: "📄",
    directory: "📁",
    directory_open: "📂",
    symlink: "🔗",
    device: "💾",
    socket: "🔌",
    fifo: "📤",
    unknown: "❓",
    tree_vertical: "│",
    tree_branch: "├",
    tree_corner: "└",
    tree_space: " ",
    js: "🟨",
    ts: "🔷",
    jsx: "⚛️",
    tsx: "⚛️",
    json: "📋",
    md: "📝",
    txt: "📝",
    html: "🌐",
    css: "🎨",
    scss: "🎨",
    py: "🐍",
    rs: "🦀",
    go: "🐹",
    yml: "⚙️",
    yaml: "⚙️",
    toml: "⚙️",
    png: "🖼️",
    jpg: "🖼️",
    gif: "🎞️",
    exe: "⚙️",
    bin: "⚙️",
    zip: "📦",
    tar: "📦",
    gz: "📦",
    mp3: "🎵",
    mp4: "🎬",
    pdf: "📕",
    svg: "🖼️",
    rb: "💎",
    java: "☕",
    c: "🔵",
    cpp: "🔵",
    h: "📑",
    sh: "⚡",
    docker: "🐳",
};

pub const asciiIcons: FileIcons = FileIcons {
    file: "[F]",
    directory: "[D]",
    directory_open: "[D]",
    symlink: "[L]",
    device: "[V]",
    socket: "[S]",
    fifo: "[P]",
    unknown: "[?]",
    tree_vertical: "|",
    tree_branch: "+",
    tree_corner: "`",
    tree_space: " ",
    js: "[JS]",
    ts: "[TS]",
    jsx: "[JX]",
    tsx: "[TX]",
    json: "[{}]",
    md: "[MD]",
    txt: "[T]",
    html: "[H]",
    css: "[C]",
    scss: "[S]",
    py: "[PY]",
    rs: "[RS]",
    go: "[GO]",
    yml: "[YM]",
    yaml: "[YM]",
    toml: "[TM]",
    png: "[IM]",
    jpg: "[IM]",
    gif: "[IM]",
    exe: "[EX]",
    bin: "[BN]",
    zip: "[ZP]",
    tar: "[TR]",
    gz: "[GZ]",
    mp3: "[AU]",
    mp4: "[VD]",
    pdf: "[PD]",
    svg: "[SV]",
    rb: "[RB]",
    java: "[JV]",
    c: "[C]",
    cpp: "[C+]",
    h: "[H]",
    sh: "[SH]",
    docker: "[DK]",
};

pub const nerdIcons: FileIcons = FileIcons {
    file: "",
    directory: "",
    directory_open: "",
    symlink: "",
    device: "",
    socket: "",
    fifo: "",
    unknown: "",
    tree_vertical: "│",
    tree_branch: "├",
    tree_corner: "└",
    tree_space: " ",
    js: "",
    ts: "",
    jsx: "",
    tsx: "",
    json: "",
    md: "",
    txt: "",
    html: "",
    css: "",
    scss: "",
    py: "",
    rs: "",
    go: "",
    yml: "",
    yaml: "",
    toml: "",
    png: "",
    jpg: "",
    gif: "",
    exe: "",
    bin: "",
    zip: "",
    tar: "",
    gz: "",
    mp3: "",
    mp4: "",
    pdf: "",
    svg: "",
    rb: "",
    java: "",
    c: "",
    cpp: "",
    h: "",
    sh: "",
    docker: "",
};

fn icon_set(icons: Option<FileIcons>) -> FileIcons {
    icons.unwrap_or(unicodeIcons)
}

fn is_directory(item: &FileItem) -> bool {
    matches!(item.item_type, FileItemType::Directory)
}

fn has_extension(item: &FileItem, extension: &str) -> bool {
    getExtension(&item.name) == extension
}

/// Get icon for a file item.
pub fn getFileIcon(item: &FileItem, icons: Option<FileIcons>) -> String {
    let set = icon_set(icons);

    let icon = match item.item_type {
        FileItemType::Directory => {
            if item.expanded {
                set.directory_open
            } else {
                set.directory
            }
        }
        FileItemType::Symlink => set.symlink,
        FileItemType::Device => set.device,
        FileItemType::Socket => set.socket,
        FileItemType::Fifo => set.fifo,
        FileItemType::File => {
            if let Some(ext) = item.extension.as_deref() {
                match ext {
                    "js" => set.js,
                    "ts" => set.ts,
                    "jsx" => set.jsx,
                    "tsx" => set.tsx,
                    "json" => set.json,
                    "md" => set.md,
                    "txt" => set.txt,
                    "html" => set.html,
                    "css" => set.css,
                    "scss" => set.scss,
                    "py" => set.py,
                    "rs" => set.rs,
                    "go" => set.go,
                    "yml" => set.yml,
                    "yaml" => set.yaml,
                    "toml" => set.toml,
                    "png" | "jpg" | "gif" | "svg" => set.png,
                    "exe" => set.exe,
                    "bin" => set.bin,
                    "zip" | "tar" | "gz" => set.zip,
                    "mp3" => set.mp3,
                    "mp4" => set.mp4,
                    "pdf" => set.pdf,
                    "rb" => set.rb,
                    "java" => set.java,
                    "c" => set.c,
                    "cpp" => set.cpp,
                    "h" => set.h,
                    "sh" => set.sh,
                    _ => set.file,
                }
            } else {
                set.file
            }
        }
        FileItemType::Unknown => set.unknown,
    };

    if icon.is_empty() {
        set.file.to_string()
    } else {
        icon.to_string()
    }
}

/// Format file size.
pub fn formatFileSize(bytes: Option<u64>) -> String {
    let size = match bytes {
        Some(value) => value as f64,
        None => return String::new(),
    };

    let units = ["B", "KB", "MB", "GB", "TB"];
    let mut value = size;
    let mut unit = 0usize;

    while value >= 1024.0 && unit < units.len() - 1 {
        value /= 1024.0;
        unit += 1;
    }

    format!("{value:.1} {}", units[unit])
}

/// Format a unix timestamp in a compact relative format.
pub fn formatDate(date: Option<u64>) -> String {
    let Some(date) = date else {
        return String::new();
    };

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |d| d.as_secs());
    let diff = now.saturating_sub(date);
    let minute = 60;
    let hour = minute * 60;
    let day = hour * 24;

    if diff < minute {
        "just now".to_string()
    } else if diff < hour {
        format!("{}m", diff / minute)
    } else if diff < day {
        format!("{}h", diff / hour)
    } else {
        format!("{}d", diff / day)
    }
}

/// Sort file items using a sorter.
pub fn sortFileItems(items: &Vec<FileItem>, sorter: &FileSorter) -> Vec<FileItem> {
    let mut output = items.clone();
    output.sort_by(|a, b| {
        if is_directory(a) && !is_directory(b) {
            std::cmp::Ordering::Less
        } else if !is_directory(a) && is_directory(b) {
            std::cmp::Ordering::Greater
        } else {
            let ord = match sorter.field {
                FileSortField::Name => a.name.cmp(&b.name),
                FileSortField::Size => a.size.cmp(&b.size),
                FileSortField::Modified => a.modified.cmp(&b.modified),
                FileSortField::Type => a.item_type.to_string().cmp(&b.item_type.to_string()),
                FileSortField::Extension => a.extension.cmp(&b.extension),
            };

            if sorter.direction == FileSortDirection::Descending {
                ord.reverse()
            } else {
                ord
            }
        }
    });

    output
}

#[derive(Debug)]
pub struct FileBrowserFilter {
    pub show_hidden: bool,
    pub filter: Option<FileFilter>,
}

impl Default for FileBrowserFilter {
    fn default() -> Self {
        Self {
            show_hidden: false,
            filter: None,
        }
    }
}

/// Filter file items using visibility and predicate.
pub fn filterFileItems(items: &Vec<FileItem>, options: FileBrowserFilter) -> Vec<FileItem> {
    items
        .iter()
        .filter(|item| {
            if !options.show_hidden && item.hidden {
                return false;
            }

            if let Some(filter_fn) = options.filter {
                filter_fn(item)
            } else {
                true
            }
        })
        .cloned()
        .collect()
}

/// Parse path to segments.
pub fn parsePath(path: &str, separator: &str) -> Vec<String> {
    let sep = if separator.is_empty() { "/" } else { separator };
    path.split(sep)
        .filter(|part| !part.is_empty())
        .map(|part| part.to_string())
        .collect()
}

/// Build path from segments.
pub fn buildPath(segments: &[String], separator: &str) -> String {
    let sep = if separator.is_empty() { "/" } else { separator };
    let joined = segments.join(sep);
    if joined.starts_with('/') {
        joined
    } else {
        format!("{sep}{joined}")
    }
}

/// Get parent path for a given path.
pub fn getParentPath(path: &str, separator: &str) -> String {
    let parts = parsePath(path, separator);
    if parts.len() <= 1 {
        separator.to_string()
    } else {
        buildPath(&parts[..parts.len() - 1], separator)
    }
}

/// File extension helper.
pub fn getExtension(filename: &str) -> String {
    let normalized = if filename.starts_with('.') {
        let rest = &filename[1..];
        if !rest.contains('.') {
            rest
        } else {
            filename
        }
    } else {
        filename
    };

    normalized
        .rsplitn(2, '.')
        .next()
        .unwrap_or(normalized)
        .to_lowercase()
}

/// Directory tree component.
pub fn DirectoryTree(options: DirectoryTreeOptions) -> VNode {
    let options_icons = icon_set(options.icons);
    let expanded = options.expanded.unwrap_or_default();

    fn walk(
        items: &[FileItem],
        depth: usize,
        max_depth: usize,
        expanded: &HashSet<String>,
        out: &mut Vec<VNode>,
        options_icons: FileIcons,
        selected: &Option<String>,
        show_hidden: bool,
    ) {
        if depth > max_depth {
            return;
        }

        for item in items.iter() {
            if !show_hidden && item.hidden {
                continue;
            }

            let is_selected = selected
                .as_ref()
                .is_some_and(|selected_path| selected_path == &item.path);
            let icon = getFileIcon(item, Some(options_icons));
            let prefix = "  ".repeat(depth);
            let style = if is_selected {
                TextStyle {
                    color: Some(Color::Named(NamedColor::Black)),
                    background: Some(Color::Named(NamedColor::Blue)),
                    ..Default::default()
                }
            } else {
                TextStyle::default()
            };

            out.push(VNode::styled_text(
                format!("{prefix}{icon} {}", item.name),
                style,
            ));

            if item.item_type == FileItemType::Directory && expanded.contains(&item.path) {
                if let Some(children) = &item.children {
                    walk(
                        children,
                        depth + 1,
                        max_depth,
                        expanded,
                        out,
                        options_icons,
                        selected,
                        show_hidden,
                    );
                }
            }
        }
    }

    let mut rows: Vec<VNode> = Vec::new();
    walk(
        &options.items,
        0,
        options.max_depth,
        &expanded,
        &mut rows,
        options_icons,
        &options.selected,
        options.show_hidden,
    );

    VNode::Box(BoxNode {
        children: rows,
        id: None,
        style: BoxStyle {
            width: if options.width == 0 {
                None
            } else {
                Some(Size::Fixed(options.width as u16))
            },
            flex_direction: Some(FlexDirection::Column),
            ..Default::default()
        },
        handlers: Default::default(),
    })
}

/// File list component.
pub fn FileList(options: FileListOptions) -> VNode {
    let options_icons = icon_set(options.icons);
    let sorter = options.sort.unwrap_or(FileSorter {
        field: FileSortField::Name,
        direction: FileSortDirection::Ascending,
    });
    let mut items = filterFileItems(
        &options.items,
        FileBrowserFilter {
            show_hidden: options.show_hidden,
            filter: options.filter,
        },
    );

    if !matches!(sorter.field, FileSortField::Name)
        || sorter.direction != FileSortDirection::Ascending
    {
        items = sortFileItems(&items, &sorter);
    }

    let rows: Vec<VNode> = items
        .iter()
        .map(|item| {
            let icon = getFileIcon(item, Some(options_icons));
            if options.view_mode == "compact" {
                VNode::styled_text(format!("{} {}", icon, item.name), TextStyle::default())
            } else {
                let mut cols: Vec<String> = Vec::new();
                cols.push(format!("{icon} {}", item.name));

                if options.show_size {
                    cols.push(formatFileSize(item.size));
                }
                if options.show_modified {
                    cols.push(formatDate(item.modified));
                }

                VNode::styled_text(cols.join("  "), TextStyle::default())
            }
        })
        .collect();

    VNode::Box(BoxNode {
        children: rows,
        id: None,
        style: BoxStyle {
            width: if options.width == 0 {
                None
            } else {
                Some(Size::Fixed(options.width as u16))
            },
            flex_direction: Some(FlexDirection::Column),
            ..Default::default()
        },
        handlers: Default::default(),
    })
}

/// Path breadcrumbs.
pub fn PathBreadcrumbs(options: PathBreadcrumbsOptions) -> VNode {
    let parts = parsePath(&options.path, options.separator);
    let mut rows = Vec::new();

    if parts.is_empty() {
        rows.push(VNode::styled_text("/".to_string(), TextStyle::default()));
    } else {
        let visible: Vec<&str> = if parts.len() > options.max_segments {
            let offset = parts.len() - options.max_segments;
            let trimmed = &parts[offset..];
            trimmed.iter().map(|s| s.as_str()).collect()
        } else {
            parts.iter().map(|s| s.as_str()).collect()
        };

        for (idx, part) in visible.iter().enumerate() {
            let text = if idx == 0 {
                part.to_string()
            } else {
                format!("{sep}{part}", sep = options.separator)
            };
            rows.push(VNode::styled_text(text, TextStyle::default()));
        }
    }

    VNode::Box(BoxNode {
        children: rows,
        id: None,
        style: BoxStyle {
            flex_direction: Some(FlexDirection::Row),
            ..Default::default()
        },
        handlers: Default::default(),
    })
}

/// File details component.
pub fn FileDetails(options: FileDetailsOptions) -> VNode {
    let mut lines = vec![VNode::styled_text(
        format!(
            "{} {}",
            getFileIcon(&options.item, Some(icon_set(options.icons))),
            options.item.name
        ),
        TextStyle::default(),
    )];
    if options.show_permissions {
        if let Some(permissions) = &options.item.permissions {
            lines.push(VNode::styled_text(
                format!("Permissions: {permissions}"),
                TextStyle::default(),
            ));
        }
    }
    if options.show_owner {
        if let Some(owner) = &options.item.owner {
            lines.push(VNode::styled_text(
                format!("Owner: {owner}"),
                TextStyle::default(),
            ));
        }
        if let Some(group) = &options.item.group {
            lines.push(VNode::styled_text(
                format!("Group: {group}"),
                TextStyle::default(),
            ));
        }
    }
    if options.show_timestamps {
        if let Some(modified) = options.item.modified {
            lines.push(VNode::styled_text(
                format!("Modified: {}", formatDate(Some(modified))),
                TextStyle::default(),
            ));
        }
        if let Some(created) = options.item.created {
            lines.push(VNode::styled_text(
                format!("Created: {}", formatDate(Some(created))),
                TextStyle::default(),
            ));
        }
        if let Some(accessed) = options.item.accessed {
            lines.push(VNode::styled_text(
                format!("Accessed: {}", formatDate(Some(accessed))),
                TextStyle::default(),
            ));
        }
    }

    VNode::Box(BoxNode {
        children: lines,
        style: BoxStyle {
            flex_direction: Some(FlexDirection::Column),
            ..Default::default()
        },
        ..Default::default()
    })
}

/// File preview component.
pub fn FilePreview(options: FilePreviewOptions) -> VNode {
    if options.item.size.unwrap_or(0) > options.max_preview_size {
        return VNode::styled_text(options.large_file_message.to_string(), TextStyle::default());
    }

    if options.content.as_ref().is_none() {
        return VNode::styled_text(options.placeholder.to_string(), TextStyle::default());
    }

    let content = options.content.unwrap_or_default();
    let lines: Vec<_> = content
        .lines()
        .take(options.max_lines)
        .map(|line| VNode::styled_text(line.to_string(), TextStyle::default()))
        .collect();

    VNode::Box(BoxNode {
        children: lines,
        style: BoxStyle {
            flex_direction: Some(FlexDirection::Column),
            width: if options.width == 0 {
                None
            } else {
                Some(Size::Fixed(options.width as u16))
            },
            ..Default::default()
        },
        ..Default::default()
    })
}

/// File icon component.
pub fn FileIcon(item: &FileItem, icons: Option<FileIcons>, style: Option<TextStyle>) -> VNode {
    let final_style = style.unwrap_or_default();
    VNode::styled_text(getFileIcon(item, icons), final_style)
}

/// Directory expand/collapse indicator.
pub fn DirectoryIndicator(is_expanded: bool) -> VNode {
    let glyph = if is_expanded { "▾" } else { "▸" };
    VNode::styled_text(glyph.to_string(), TextStyle::default())
}

/// FileBrowser compatibility constructor using options.
pub fn FileBrowser(options: FileBrowserOptions) -> VNode {
    let filtered = filterFileItems(
        &options.items,
        FileBrowserFilter {
            show_hidden: options.show_hidden,
            filter: options.filter,
        },
    );

    let main = match options.view_mode {
        "tree" => DirectoryTree(DirectoryTreeOptions {
            items: filtered.clone(),
            show_hidden: options.show_hidden,
            icons: options.icons,
            selected: None,
            expanded: Some(HashSet::new()),
            ..Default::default()
        }),
        "details" => FileList(FileListOptions {
            items: filtered.clone(),
            view_mode: "details",
            show_hidden: options.show_hidden,
            icons: options.icons,
            width: options.width,
            ..Default::default()
        }),
        _ => FileList(FileListOptions {
            items: filtered.clone(),
            view_mode: "list",
            show_hidden: options.show_hidden,
            icons: options.icons,
            width: options.width,
            ..Default::default()
        }),
    };

    let mut children = Vec::new();
    if options.show_breadcrumbs {
        children.push(PathBreadcrumbs(PathBreadcrumbsOptions {
            path: options.path.clone(),
            max_segments: 5,
            ..Default::default()
        }));
    }

    children.push(main);

    if options.show_status_bar {
        let status = VNode::styled_text(
            format!(
                "{} items • {} dirs",
                filtered.len(),
                filtered
                    .iter()
                    .filter(|item| item.item_type == FileItemType::Directory)
                    .count(),
            ),
            TextStyle::default(),
        );
        children.push(status);
    }

    VNode::Box(BoxNode {
        children,
        id: None,
        style: BoxStyle {
            flex_direction: Some(FlexDirection::Column),
            width: if options.width == 0 {
                None
            } else {
                Some(Size::Fixed(options.width as u16))
            },
            ..Default::default()
        },
        handlers: Default::default(),
    })
}

pub fn get_extension(filename: &str) -> String {
    getExtension(filename)
}

impl FileItemType {
    fn to_string(&self) -> &str {
        match self {
            FileItemType::File => "file",
            FileItemType::Directory => "directory",
            FileItemType::Symlink => "symlink",
            FileItemType::Device => "device",
            FileItemType::Socket => "socket",
            FileItemType::Fifo => "fifo",
            FileItemType::Unknown => "unknown",
        }
    }
}
