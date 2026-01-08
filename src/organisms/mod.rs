//! Organisms - Complex UI Components
//!
//! Organisms are self-contained UI components that combine multiple atoms
//! and molecules to provide complete functionality. They include modals,
//! dialogs, tables, file browsers, and other complex widgets.

// =============================================================================
// Implemented Organisms
// =============================================================================

mod modal;
mod toast;
mod confirm_dialog;
mod alert_box;
mod scroll_area;
mod virtual_list;
mod split_panel;
mod data_table;
mod command_palette;
mod overlay_stack;
mod grid;
mod scroll_list;
mod file_browser;

pub use modal::{Modal, ModalSize, ModalState};
pub use toast::{
    Toast, ToastContainer, ToastEntry, ToastId, ToastPosition, ToastState, ToastVariant,
};
pub use confirm_dialog::{ConfirmDialog, ConfirmDialogState, ConfirmResult, ConfirmVariant};
pub use alert_box::{AlertBox, AlertVariant, error_alert, info_alert, success_alert, warning_alert};
pub use scroll_area::{ScrollArea, ScrollAreaState, ScrollbarVisibility};
pub use virtual_list::{ItemRenderer, VirtualList, VirtualListState};
pub use split_panel::{
    DividerStyle, SplitOrientation, SplitPanel, SplitPanelState, ThreePanel,
};
pub use data_table::{
    Column, ColumnAlign, DataTable, DataTableState, SelectionMode, SortDirection, SortState,
};
pub use command_palette::{
    Command, CommandGroup, CommandPalette, CommandPaletteState,
};
pub use overlay_stack::{
    OverlayEntry, OverlayStack, OverlayStackState, create_overlay_stack_state,
};
pub use grid::{Grid, GridCell, GridColumn, GridGap, GridSize};
pub use scroll_list::{ScrollList, ScrollListState, create_scroll_list_state, simple_scroll_list};
pub use file_browser::{
    FileBrowser, FileBrowserState, FileItem, FileItemType,
    FileSortDirection, FileSortField, create_file_browser_state,
};

// =============================================================================
// Stub Types (to be implemented)
// =============================================================================

/// Notification system (like toast but persistent).
///
/// TODO: Implement with notification queue and actions.
pub struct Notification;
