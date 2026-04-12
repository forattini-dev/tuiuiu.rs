//! Organisms - Complex UI Components
//!
//! Organisms are self-contained UI components that combine multiple atoms
//! and molecules to provide complete functionality. They include modals,
//! dialogs, tables, file browsers, and other complex widgets.

// =============================================================================
// Implemented Organisms
// =============================================================================

mod alert_box;
mod command_palette;
mod confirm_dialog;
mod data_table;
mod file_browser;
mod grid;
mod modal;
mod overlay_stack;
mod scroll_area;
mod scroll_list;
mod split_panel;
mod toast;
mod virtual_list;

pub use alert_box::{
    error_alert, info_alert, success_alert, warning_alert, AlertBox, AlertVariant,
};
pub use command_palette::{
    createCommandPalette, createGoToDialog, create_command_palette, create_go_to_dialog, Command,
    CommandGroup, CommandPalette, CommandPaletteProps, CommandPaletteState,
    CreateCommandPaletteOptions, CreateGoToDialogOptions, GoToDialog, GoToDialogProps,
    GoToDialogState,
};
pub use confirm_dialog::{ConfirmDialog, ConfirmDialogState, ConfirmResult, ConfirmVariant};
pub use data_table::{
    createDataTable,
    // JS-compatible compatibility layer
    create_data_table,
    useDataTableState,
    Column,
    ColumnAlign,
    DataTable,
    DataTableColumn,
    DataTableOptions,
    DataTableProps,
    DataTableState,
    EditableDataTable,
    EditableDataTableOptions,
    SelectionMode,
    SortDirection,
    SortState,
    TableSelectionMode,
    TableSortDirection,
    VirtualDataTable,
    VirtualDataTableOptions,
};
pub use file_browser::{
    asciiIcons, buildPath, create_file_browser_state, filterFileItems, formatDate, formatFileSize,
    getExtension, getFileIcon, getParentPath, get_extension, nerdIcons, parsePath, sortFileItems,
    unicodeIcons, DirectoryIndicator, DirectoryTree, DirectoryTreeOptions, FileBrowser,
    FileBrowserFilter, FileBrowserOptions, FileBrowserState, FileDetails, FileDetailsOptions,
    FileDirectoryTreeOptions, FileFilter, FileIcon, FileIcons, FileItem, FileItemField,
    FileItemType, FileList, FileListColumn, FileListOptions, FilePreview, FilePreviewOptions,
    FileSortDirection, FileSortField, FileSorter, PathBreadcrumbs, PathBreadcrumbsOptions,
};
pub use grid::{Grid, GridCell, GridColumn, GridGap, GridSize};
pub use modal::{Modal, ModalSize, ModalState};
pub use overlay_stack::{
    create_overlay_stack_state, OverlayEntry, OverlayStack, OverlayStackState,
};
pub use scroll_area::{ScrollArea, ScrollAreaState, ScrollbarVisibility};
pub use scroll_list::{create_scroll_list_state, simple_scroll_list, ScrollList, ScrollListState};
pub use split_panel::{DividerStyle, SplitOrientation, SplitPanel, SplitPanelState, ThreePanel};
pub use toast::{
    Toast, ToastContainer, ToastEntry, ToastId, ToastPosition, ToastState, ToastVariant,
};
pub use virtual_list::{ItemRenderer, VirtualList, VirtualListState};

// =============================================================================
// Stub Types (to be implemented)
// =============================================================================

/// Notification system (like toast but persistent).
///
/// TODO: Implement with notification queue and actions.
pub struct Notification;
