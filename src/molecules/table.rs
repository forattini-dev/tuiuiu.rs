//! Table Component
//!
//! Data table with headers and rows.

use crate::core::component::{VNode, BoxNode, BoxStyle, TextStyle, Color, NamedColor, BorderStyle};

/// Table column definition.
#[derive(Debug, Clone)]
pub struct Column {
    /// Column header
    pub header: String,
    /// Column width (characters)
    pub width: Option<usize>,
    /// Alignment
    pub align: Align,
}

/// Text alignment.
#[derive(Debug, Clone, Copy, Default)]
pub enum Align {
    #[default]
    Left,
    Center,
    Right,
}

impl Column {
    /// Create a new column.
    pub fn new(header: impl Into<String>) -> Self {
        Self {
            header: header.into(),
            width: None,
            align: Align::Left,
        }
    }

    /// Set column width.
    pub fn width(mut self, width: usize) -> Self {
        self.width = Some(width);
        self
    }

    /// Set alignment.
    pub fn align(mut self, align: Align) -> Self {
        self.align = align;
        self
    }

    /// Right align.
    pub fn right(self) -> Self {
        self.align(Align::Right)
    }

    /// Center align.
    pub fn center(self) -> Self {
        self.align(Align::Center)
    }
}

/// Table component.
#[derive(Debug, Clone)]
pub struct Table {
    columns: Vec<Column>,
    rows: Vec<Vec<String>>,
    header_color: Color,
    row_color: Color,
    alt_row_color: Option<Color>,
    border: bool,
    selected_row: Option<usize>,
    striped: bool,
}

impl Default for Table {
    fn default() -> Self {
        Self {
            columns: Vec::new(),
            rows: Vec::new(),
            header_color: Color::Named(NamedColor::Cyan),
            row_color: Color::Named(NamedColor::White),
            alt_row_color: None,
            border: true,
            selected_row: None,
            striped: false,
        }
    }
}

impl Table {
    /// Create a new table.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set columns.
    pub fn columns<I>(mut self, columns: I) -> Self
    where
        I: IntoIterator<Item = Column>,
    {
        self.columns = columns.into_iter().collect();
        self
    }

    /// Set columns from headers only.
    pub fn headers<I, S>(mut self, headers: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.columns = headers.into_iter().map(|h| Column::new(h)).collect();
        self
    }

    /// Add a row.
    pub fn row<I, S>(mut self, cells: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.rows.push(cells.into_iter().map(Into::into).collect());
        self
    }

    /// Set all rows at once.
    pub fn rows<R, I, S>(mut self, rows: R) -> Self
    where
        R: IntoIterator<Item = I>,
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.rows = rows
            .into_iter()
            .map(|row| row.into_iter().map(Into::into).collect())
            .collect();
        self
    }

    /// Set header color.
    pub fn header_color(mut self, color: Color) -> Self {
        self.header_color = color;
        self
    }

    /// Enable striped rows.
    pub fn striped(mut self) -> Self {
        self.striped = true;
        self.alt_row_color = Some(Color::Named(NamedColor::BrightBlack));
        self
    }

    /// Set selected row.
    pub fn selected(mut self, row: usize) -> Self {
        self.selected_row = Some(row);
        self
    }

    /// Disable border.
    pub fn borderless(mut self) -> Self {
        self.border = false;
        self
    }

    /// Build the VNode.
    pub fn build(self) -> VNode {
        let mut children = Vec::new();

        // Calculate column widths
        let col_widths: Vec<usize> = self.columns.iter().enumerate().map(|(i, col)| {
            let header_len = col.header.len();
            let max_data_len = self.rows.iter()
                .filter_map(|row| row.get(i))
                .map(|s| s.len())
                .max()
                .unwrap_or(0);
            col.width.unwrap_or_else(|| header_len.max(max_data_len).max(4))
        }).collect();

        // Header row
        let header_cells: Vec<String> = self.columns.iter().enumerate()
            .map(|(i, col)| {
                let width = col_widths.get(i).copied().unwrap_or(8);
                format!("{:<width$}", col.header, width = width)
            })
            .collect();

        let header_text = header_cells.join(" │ ");
        children.push(VNode::styled_text(
            header_text,
            TextStyle { color: Some(self.header_color), bold: true, ..Default::default() }
        ));

        // Separator
        let separator: String = col_widths.iter()
            .map(|&w| "─".repeat(w))
            .collect::<Vec<_>>()
            .join("─┼─");
        children.push(VNode::styled_text(separator, TextStyle::color(Color::Named(NamedColor::Gray))));

        // Data rows
        for (row_idx, row) in self.rows.iter().enumerate() {
            let cells: Vec<String> = (0..self.columns.len())
                .map(|i| {
                    let text = row.get(i).map(|s| s.as_str()).unwrap_or("");
                    let width = col_widths.get(i).copied().unwrap_or(8);
                    let align = self.columns.get(i).map(|c| c.align).unwrap_or(Align::Left);
                    match align {
                        Align::Left => format!("{:<width$}", text, width = width),
                        Align::Center => format!("{:^width$}", text, width = width),
                        Align::Right => format!("{:>width$}", text, width = width),
                    }
                })
                .collect();

            let row_text = cells.join(" │ ");

            let is_selected = self.selected_row == Some(row_idx);
            let color = if is_selected {
                Color::Named(NamedColor::Black)
            } else if self.striped && row_idx % 2 == 1 {
                self.alt_row_color.clone().unwrap_or(self.row_color)
            } else {
                self.row_color
            };

            children.push(VNode::styled_text(
                row_text,
                TextStyle { color: Some(color), inverse: is_selected, ..Default::default() }
            ));
        }

        VNode::Box(BoxNode {
            children,
            style: BoxStyle {
                border_style: if self.border { Some(BorderStyle::Single) } else { None },
                padding_left: Some(1),
                padding_right: Some(1),
                ..Default::default()
            },
            ..Default::default()
        })
    }
}
