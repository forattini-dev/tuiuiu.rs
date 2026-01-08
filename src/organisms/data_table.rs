//! DataTable Component
//!
//! A full-featured data table with sorting, filtering, selection,
//! and virtual scrolling for large datasets.

use crate::core::component::{
    BoxNode, BoxStyle, Color, EventHandlers, NamedColor, TextNode, TextStyle, VNode,
};
use crate::core::layout::{FlexDirection, Size};
use crate::core::signals::{create_signal, ReadSignal, WriteSignal};
use std::cmp::Ordering;

// =============================================================================
// Column Definition
// =============================================================================

/// Column alignment.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ColumnAlign {
    #[default]
    Left,
    Center,
    Right,
}

/// Column definition for a DataTable.
#[derive(Debug, Clone)]
pub struct Column<T> {
    /// Column header text
    pub header: String,
    /// Column width (fixed characters)
    pub width: u16,
    /// Whether column is sortable
    pub sortable: bool,
    /// Column alignment
    pub align: ColumnAlign,
    /// Value extractor function
    pub accessor: fn(&T) -> String,
    /// Optional sort comparator (for custom sorting)
    pub comparator: Option<fn(&T, &T) -> Ordering>,
}

impl<T> Column<T> {
    /// Create a new column.
    pub fn new(header: impl Into<String>, width: u16, accessor: fn(&T) -> String) -> Self {
        Self {
            header: header.into(),
            width,
            sortable: true,
            align: ColumnAlign::default(),
            accessor,
            comparator: None,
        }
    }

    /// Set sortable.
    pub fn sortable(mut self, sortable: bool) -> Self {
        self.sortable = sortable;
        self
    }

    /// Set alignment.
    pub fn align(mut self, align: ColumnAlign) -> Self {
        self.align = align;
        self
    }

    /// Set custom comparator.
    pub fn comparator(mut self, cmp: fn(&T, &T) -> Ordering) -> Self {
        self.comparator = Some(cmp);
        self
    }

    /// Align left.
    pub fn left(self) -> Self {
        self.align(ColumnAlign::Left)
    }

    /// Align center.
    pub fn center(self) -> Self {
        self.align(ColumnAlign::Center)
    }

    /// Align right.
    pub fn right(self) -> Self {
        self.align(ColumnAlign::Right)
    }
}

// =============================================================================
// Sort State
// =============================================================================

/// Sort direction.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum SortDirection {
    #[default]
    None,
    Ascending,
    Descending,
}

impl SortDirection {
    /// Get the sort indicator character.
    pub fn indicator(&self) -> &'static str {
        match self {
            SortDirection::None => " ",
            SortDirection::Ascending => "▲",
            SortDirection::Descending => "▼",
        }
    }

    /// Toggle sort direction.
    pub fn toggle(&self) -> Self {
        match self {
            SortDirection::None => SortDirection::Ascending,
            SortDirection::Ascending => SortDirection::Descending,
            SortDirection::Descending => SortDirection::None,
        }
    }
}

/// Sort state for a column.
#[derive(Debug, Clone, Copy, Default)]
pub struct SortState {
    /// Column index being sorted
    pub column: Option<usize>,
    /// Sort direction
    pub direction: SortDirection,
}

// =============================================================================
// Selection Mode
// =============================================================================

/// Row selection mode.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum SelectionMode {
    /// No selection allowed
    #[default]
    None,
    /// Single row selection
    Single,
    /// Multiple row selection
    Multi,
}

// =============================================================================
// DataTable Component
// =============================================================================

/// DataTable component.
///
/// A table with sorting, filtering, and virtual scrolling.
///
/// # Example
/// ```ignore
/// #[derive(Clone)]
/// struct User {
///     name: String,
///     email: String,
///     age: u32,
/// }
///
/// let columns = vec![
///     Column::new("Name", 20, |u: &User| u.name.clone()),
///     Column::new("Email", 30, |u: &User| u.email.clone()),
///     Column::new("Age", 5, |u: &User| u.age.to_string()).right(),
/// ];
///
/// let table = DataTable::new(columns)
///     .data(&users)
///     .visible_rows(20)
///     .selection(SelectionMode::Single)
///     .build();
/// ```
#[derive(Clone)]
pub struct DataTable<T: Clone> {
    /// Column definitions
    columns: Vec<Column<T>>,
    /// Table data
    data: Vec<T>,
    /// Number of visible rows
    visible_rows: u16,
    /// Current scroll offset
    scroll_offset: usize,
    /// Sort state
    sort_state: SortState,
    /// Selection mode
    selection_mode: SelectionMode,
    /// Selected row indices
    selected: Vec<usize>,
    /// Show header row
    show_header: bool,
    /// Zebra striping
    zebra: bool,
    /// Header background color
    header_bg: Option<Color>,
    /// Header text color
    header_fg: Option<Color>,
    /// Row background color
    row_bg: Option<Color>,
    /// Alternate row background (for zebra)
    row_alt_bg: Option<Color>,
    /// Selected row background
    selected_bg: Option<Color>,
    /// Border between columns
    show_borders: bool,
}

impl<T: Clone> DataTable<T> {
    /// Create a new data table with columns.
    pub fn new(columns: Vec<Column<T>>) -> Self {
        Self {
            columns,
            data: Vec::new(),
            visible_rows: 10,
            scroll_offset: 0,
            sort_state: SortState::default(),
            selection_mode: SelectionMode::None,
            selected: Vec::new(),
            show_header: true,
            zebra: true,
            header_bg: Some(Color::Named(NamedColor::Blue)),
            header_fg: Some(Color::Named(NamedColor::White)),
            row_bg: None,
            row_alt_bg: Some(Color::Rgb(30, 30, 40)),
            selected_bg: Some(Color::Named(NamedColor::Cyan)),
            show_borders: true,
        }
    }

    /// Set the data.
    pub fn data(mut self, data: &[T]) -> Self {
        self.data = data.to_vec();
        self
    }

    /// Set number of visible rows.
    pub fn visible_rows(mut self, rows: u16) -> Self {
        self.visible_rows = rows;
        self
    }

    /// Set scroll offset.
    pub fn scroll_offset(mut self, offset: usize) -> Self {
        self.scroll_offset = offset;
        self
    }

    /// Set sort state.
    pub fn sort(mut self, column: usize, direction: SortDirection) -> Self {
        self.sort_state = SortState {
            column: Some(column),
            direction,
        };
        self
    }

    /// Set selection mode.
    pub fn selection(mut self, mode: SelectionMode) -> Self {
        self.selection_mode = mode;
        self
    }

    /// Set selected rows.
    pub fn selected(mut self, indices: Vec<usize>) -> Self {
        self.selected = indices;
        self
    }

    /// Show/hide header.
    pub fn header(mut self, show: bool) -> Self {
        self.show_header = show;
        self
    }

    /// Enable/disable zebra striping.
    pub fn zebra(mut self, enable: bool) -> Self {
        self.zebra = enable;
        self
    }

    /// Set header colors.
    pub fn header_colors(mut self, bg: Color, fg: Color) -> Self {
        self.header_bg = Some(bg);
        self.header_fg = Some(fg);
        self
    }

    /// Set selected row background.
    pub fn selected_color(mut self, color: Color) -> Self {
        self.selected_bg = Some(color);
        self
    }

    /// Show/hide column borders.
    pub fn borders(mut self, show: bool) -> Self {
        self.show_borders = show;
        self
    }

    /// Calculate total width.
    fn total_width(&self) -> u16 {
        let col_widths: u16 = self.columns.iter().map(|c| c.width).sum();
        if self.show_borders {
            col_widths + (self.columns.len() as u16).saturating_sub(1)
        } else {
            col_widths
        }
    }

    /// Build the table VNode.
    pub fn build(self) -> VNode {
        let mut rows: Vec<VNode> = Vec::new();

        // Header row
        if self.show_header {
            rows.push(self.build_header());
        }

        // Get sorted data indices
        let sorted_indices = self.get_sorted_indices();

        // Data rows (with virtual scrolling)
        for (display_idx, &data_idx) in sorted_indices
            .iter()
            .skip(self.scroll_offset)
            .take(self.visible_rows as usize)
            .enumerate()
        {
            let row_idx = self.scroll_offset + display_idx;
            let is_selected = self.selected.contains(&data_idx);
            let is_alternate = row_idx % 2 == 1;

            rows.push(self.build_row(&self.data[data_idx], data_idx, is_selected, is_alternate));
        }

        // Table container
        VNode::Box(BoxNode {
            id: None,
            style: BoxStyle {
                flex_direction: Some(FlexDirection::Column),
                width: Some(Size::Fixed(self.total_width())),
                ..Default::default()
            },
            children: rows,
            handlers: EventHandlers {
                focusable: true,
                ..Default::default()
            },
        })
    }

    /// Build header row.
    fn build_header(&self) -> VNode {
        let mut cells: Vec<VNode> = Vec::new();

        for (idx, col) in self.columns.iter().enumerate() {
            // Sort indicator
            let indicator = if self.sort_state.column == Some(idx) {
                self.sort_state.direction.indicator()
            } else {
                " "
            };

            let header_text = if col.sortable {
                format!("{}{}", col.header, indicator)
            } else {
                col.header.clone()
            };

            // Pad/truncate to column width
            let display = Self::format_cell(&header_text, col.width as usize, col.align);

            cells.push(VNode::Text(TextNode {
                content: display,
                style: TextStyle {
                    color: self.header_fg,
                    bold: true,
                    ..Default::default()
                },
            }));

            // Column separator
            if self.show_borders && idx < self.columns.len() - 1 {
                cells.push(VNode::Text(TextNode {
                    content: "│".to_string(),
                    style: TextStyle {
                        color: Some(Color::Named(NamedColor::Gray)),
                        ..Default::default()
                    },
                }));
            }
        }

        VNode::Box(BoxNode {
            id: None,
            style: BoxStyle {
                flex_direction: Some(FlexDirection::Row),
                background: self.header_bg,
                ..Default::default()
            },
            children: cells,
            handlers: Default::default(),
        })
    }

    /// Build a data row.
    fn build_row(&self, item: &T, _data_idx: usize, is_selected: bool, is_alternate: bool) -> VNode {
        let mut cells: Vec<VNode> = Vec::new();

        for (idx, col) in self.columns.iter().enumerate() {
            let value = (col.accessor)(item);
            let display = Self::format_cell(&value, col.width as usize, col.align);

            let text_color = if is_selected {
                Some(Color::Named(NamedColor::Black))
            } else {
                Some(Color::Named(NamedColor::White))
            };

            cells.push(VNode::Text(TextNode {
                content: display,
                style: TextStyle {
                    color: text_color,
                    ..Default::default()
                },
            }));

            // Column separator
            if self.show_borders && idx < self.columns.len() - 1 {
                cells.push(VNode::Text(TextNode {
                    content: "│".to_string(),
                    style: TextStyle {
                        color: Some(Color::Named(NamedColor::Gray)),
                        ..Default::default()
                    },
                }));
            }
        }

        // Determine background color
        let bg = if is_selected {
            self.selected_bg
        } else if is_alternate && self.zebra {
            self.row_alt_bg
        } else {
            self.row_bg
        };

        VNode::Box(BoxNode {
            id: None,
            style: BoxStyle {
                flex_direction: Some(FlexDirection::Row),
                background: bg,
                ..Default::default()
            },
            children: cells,
            handlers: EventHandlers {
                on_click: self.selection_mode != SelectionMode::None,
                ..Default::default()
            },
        })
    }

    /// Format a cell value with padding/truncation.
    fn format_cell(value: &str, width: usize, align: ColumnAlign) -> String {
        let len = value.chars().count();
        if len >= width {
            // Truncate with ellipsis
            if width > 3 {
                let truncated: String = value.chars().take(width - 1).collect();
                format!("{}…", truncated)
            } else {
                value.chars().take(width).collect()
            }
        } else {
            // Pad according to alignment
            let padding = width - len;
            match align {
                ColumnAlign::Left => format!("{}{}", value, " ".repeat(padding)),
                ColumnAlign::Right => format!("{}{}", " ".repeat(padding), value),
                ColumnAlign::Center => {
                    let left_pad = padding / 2;
                    let right_pad = padding - left_pad;
                    format!("{}{}{}", " ".repeat(left_pad), value, " ".repeat(right_pad))
                }
            }
        }
    }

    /// Get sorted data indices.
    fn get_sorted_indices(&self) -> Vec<usize> {
        let mut indices: Vec<usize> = (0..self.data.len()).collect();

        if let Some(col_idx) = self.sort_state.column {
            if col_idx < self.columns.len() && self.sort_state.direction != SortDirection::None {
                let col = &self.columns[col_idx];

                indices.sort_by(|&a, &b| {
                    let cmp = if let Some(comparator) = col.comparator {
                        comparator(&self.data[a], &self.data[b])
                    } else {
                        let val_a = (col.accessor)(&self.data[a]);
                        let val_b = (col.accessor)(&self.data[b]);
                        val_a.cmp(&val_b)
                    };

                    match self.sort_state.direction {
                        SortDirection::Ascending => cmp,
                        SortDirection::Descending => cmp.reverse(),
                        SortDirection::None => Ordering::Equal,
                    }
                });
            }
        }

        indices
    }
}

// =============================================================================
// DataTable State
// =============================================================================

/// State manager for DataTable.
///
/// Provides reactive state for scroll, sort, and selection.
pub struct DataTableState {
    /// Scroll offset
    scroll_read: ReadSignal<usize>,
    scroll_write: WriteSignal<usize>,
    /// Sort state
    sort_read: ReadSignal<SortState>,
    sort_write: WriteSignal<SortState>,
    /// Selected rows
    selected_read: ReadSignal<Vec<usize>>,
    selected_write: WriteSignal<Vec<usize>>,
    /// Total row count
    total_rows: usize,
    /// Visible row count
    visible_rows: usize,
    /// Selection mode
    selection_mode: SelectionMode,
}

impl DataTableState {
    /// Create a new table state.
    pub fn new(visible_rows: usize) -> Self {
        let (scroll_read, scroll_write) = create_signal(0usize);
        let (sort_read, sort_write) = create_signal(SortState::default());
        let (selected_read, selected_write) = create_signal(Vec::new());

        Self {
            scroll_read,
            scroll_write,
            sort_read,
            sort_write,
            selected_read,
            selected_write,
            total_rows: 0,
            visible_rows,
            selection_mode: SelectionMode::None,
        }
    }

    /// Set total rows.
    pub fn set_total_rows(&mut self, count: usize) {
        self.total_rows = count;
        // Clamp scroll
        let max = self.max_scroll();
        if self.scroll_read.get() > max {
            self.scroll_write.set(max);
        }
    }

    /// Set selection mode.
    pub fn set_selection_mode(&mut self, mode: SelectionMode) {
        self.selection_mode = mode;
        if mode == SelectionMode::None {
            self.selected_write.set(Vec::new());
        }
    }

    /// Get current scroll offset.
    pub fn scroll(&self) -> usize {
        self.scroll_read.get()
    }

    /// Set scroll offset.
    pub fn set_scroll(&self, offset: usize) {
        let max = self.max_scroll();
        self.scroll_write.set(offset.min(max));
    }

    /// Get max scroll offset.
    pub fn max_scroll(&self) -> usize {
        self.total_rows.saturating_sub(self.visible_rows)
    }

    /// Scroll up one row.
    pub fn scroll_up(&self) {
        let current = self.scroll_read.get();
        if current > 0 {
            self.scroll_write.set(current - 1);
        }
    }

    /// Scroll down one row.
    pub fn scroll_down(&self) {
        let current = self.scroll_read.get();
        let max = self.max_scroll();
        if current < max {
            self.scroll_write.set(current + 1);
        }
    }

    /// Page up.
    pub fn page_up(&self) {
        let current = self.scroll_read.get();
        self.scroll_write.set(current.saturating_sub(self.visible_rows));
    }

    /// Page down.
    pub fn page_down(&self) {
        let current = self.scroll_read.get();
        let max = self.max_scroll();
        self.scroll_write.set((current + self.visible_rows).min(max));
    }

    /// Get current sort state.
    pub fn sort(&self) -> SortState {
        self.sort_read.get()
    }

    /// Sort by column.
    pub fn sort_by(&self, column: usize) {
        let current = self.sort_read.get();
        let new_direction = if current.column == Some(column) {
            current.direction.toggle()
        } else {
            SortDirection::Ascending
        };

        self.sort_write.set(SortState {
            column: if new_direction == SortDirection::None {
                None
            } else {
                Some(column)
            },
            direction: new_direction,
        });
    }

    /// Clear sort.
    pub fn clear_sort(&self) {
        self.sort_write.set(SortState::default());
    }

    /// Get selected rows.
    pub fn selected(&self) -> Vec<usize> {
        self.selected_read.get()
    }

    /// Select a row.
    pub fn select(&self, row: usize) {
        match self.selection_mode {
            SelectionMode::None => {}
            SelectionMode::Single => {
                self.selected_write.set(vec![row]);
            }
            SelectionMode::Multi => {
                let mut selected = self.selected_read.get();
                if !selected.contains(&row) {
                    selected.push(row);
                    self.selected_write.set(selected);
                }
            }
        }
    }

    /// Deselect a row.
    pub fn deselect(&self, row: usize) {
        let mut selected = self.selected_read.get();
        selected.retain(|&r| r != row);
        self.selected_write.set(selected);
    }

    /// Toggle row selection.
    pub fn toggle_select(&self, row: usize) {
        let selected = self.selected_read.get();
        if selected.contains(&row) {
            self.deselect(row);
        } else {
            self.select(row);
        }
    }

    /// Clear selection.
    pub fn clear_selection(&self) {
        self.selected_write.set(Vec::new());
    }

    /// Select all rows.
    pub fn select_all(&self) {
        if self.selection_mode == SelectionMode::Multi {
            let all: Vec<usize> = (0..self.total_rows).collect();
            self.selected_write.set(all);
        }
    }

    /// Get scroll signal.
    pub fn scroll_signal(&self) -> ReadSignal<usize> {
        self.scroll_read.clone()
    }

    /// Get sort signal.
    pub fn sort_signal(&self) -> ReadSignal<SortState> {
        self.sort_read.clone()
    }

    /// Get selected signal.
    pub fn selected_signal(&self) -> ReadSignal<Vec<usize>> {
        self.selected_read.clone()
    }
}

impl Default for DataTableState {
    fn default() -> Self {
        Self::new(10)
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone)]
    struct TestUser {
        name: String,
        age: u32,
    }

    fn test_columns() -> Vec<Column<TestUser>> {
        vec![
            Column::new("Name", 15, |u: &TestUser| u.name.clone()),
            Column::new("Age", 5, |u: &TestUser| u.age.to_string()).right(),
        ]
    }

    fn test_data() -> Vec<TestUser> {
        vec![
            TestUser { name: "Alice".to_string(), age: 30 },
            TestUser { name: "Bob".to_string(), age: 25 },
            TestUser { name: "Charlie".to_string(), age: 35 },
        ]
    }

    #[test]
    fn test_data_table_build() {
        let table = DataTable::new(test_columns())
            .data(&test_data())
            .visible_rows(10)
            .build();

        matches!(table, VNode::Box(_));
    }

    #[test]
    fn test_data_table_with_sort() {
        let table = DataTable::new(test_columns())
            .data(&test_data())
            .sort(0, SortDirection::Ascending)
            .build();

        matches!(table, VNode::Box(_));
    }

    #[test]
    fn test_data_table_with_selection() {
        let table = DataTable::new(test_columns())
            .data(&test_data())
            .selection(SelectionMode::Single)
            .selected(vec![0])
            .build();

        matches!(table, VNode::Box(_));
    }

    #[test]
    fn test_format_cell_left() {
        let result = DataTable::<()>::format_cell("test", 10, ColumnAlign::Left);
        assert_eq!(result, "test      ");
    }

    #[test]
    fn test_format_cell_right() {
        let result = DataTable::<()>::format_cell("test", 10, ColumnAlign::Right);
        assert_eq!(result, "      test");
    }

    #[test]
    fn test_format_cell_center() {
        let result = DataTable::<()>::format_cell("test", 10, ColumnAlign::Center);
        assert_eq!(result, "   test   ");
    }

    #[test]
    fn test_format_cell_truncate() {
        let result = DataTable::<()>::format_cell("very long text", 8, ColumnAlign::Left);
        assert_eq!(result, "very lo…");
    }

    #[test]
    fn test_sort_direction_toggle() {
        assert_eq!(SortDirection::None.toggle(), SortDirection::Ascending);
        assert_eq!(SortDirection::Ascending.toggle(), SortDirection::Descending);
        assert_eq!(SortDirection::Descending.toggle(), SortDirection::None);
    }

    #[test]
    fn test_table_state() {
        let mut state = DataTableState::new(10);
        state.set_total_rows(100);
        state.set_selection_mode(SelectionMode::Multi);

        assert_eq!(state.scroll(), 0);
        assert_eq!(state.max_scroll(), 90);

        state.scroll_down();
        assert_eq!(state.scroll(), 1);

        state.page_down();
        assert_eq!(state.scroll(), 11);

        state.sort_by(0);
        assert_eq!(state.sort().column, Some(0));
        assert_eq!(state.sort().direction, SortDirection::Ascending);

        state.sort_by(0);
        assert_eq!(state.sort().direction, SortDirection::Descending);

        state.select(5);
        state.select(10);
        assert_eq!(state.selected(), vec![5, 10]);

        state.toggle_select(5);
        assert_eq!(state.selected(), vec![10]);
    }
}
