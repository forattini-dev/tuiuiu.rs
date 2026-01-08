//! Grid Layout Component
//!
//! CSS Grid-like layout for terminal UIs.

use crate::core::component::{VNode, BoxNode, BoxStyle, BorderStyle};
use crate::core::layout::{FlexDirection, Size};

/// Grid cell content.
#[derive(Debug, Clone)]
pub struct GridCell {
    /// Cell content
    pub content: VNode,
    /// Column span
    pub col_span: usize,
    /// Row span
    pub row_span: usize,
}

impl GridCell {
    /// Create a new grid cell.
    pub fn new(content: VNode) -> Self {
        Self {
            content,
            col_span: 1,
            row_span: 1,
        }
    }

    /// Set column span.
    pub fn col_span(mut self, span: usize) -> Self {
        self.col_span = span.max(1);
        self
    }

    /// Set row span.
    pub fn row_span(mut self, span: usize) -> Self {
        self.row_span = span.max(1);
        self
    }
}

/// Grid column definition.
#[derive(Debug, Clone)]
pub struct GridColumn {
    /// Column width (in characters)
    pub width: GridSize,
    /// Minimum width
    pub min_width: Option<u16>,
    /// Maximum width
    pub max_width: Option<u16>,
}

impl GridColumn {
    /// Fixed width column.
    pub fn fixed(width: u16) -> Self {
        Self {
            width: GridSize::Fixed(width),
            min_width: None,
            max_width: None,
        }
    }

    /// Flexible width column.
    pub fn flex(factor: f32) -> Self {
        Self {
            width: GridSize::Flex(factor),
            min_width: None,
            max_width: None,
        }
    }

    /// Auto-sized column.
    pub fn auto() -> Self {
        Self {
            width: GridSize::Auto,
            min_width: None,
            max_width: None,
        }
    }

    /// Set minimum width.
    pub fn min(mut self, width: u16) -> Self {
        self.min_width = Some(width);
        self
    }

    /// Set maximum width.
    pub fn max(mut self, width: u16) -> Self {
        self.max_width = Some(width);
        self
    }
}

/// Grid size unit.
#[derive(Debug, Clone, Copy)]
pub enum GridSize {
    /// Fixed number of characters
    Fixed(u16),
    /// Flexible fraction (like CSS fr)
    Flex(f32),
    /// Auto-size based on content
    Auto,
}

impl Default for GridSize {
    fn default() -> Self {
        GridSize::Flex(1.0)
    }
}

/// Grid gap.
#[derive(Debug, Clone, Copy, Default)]
pub struct GridGap {
    pub row: u16,
    pub column: u16,
}

impl GridGap {
    /// Create uniform gap.
    pub fn all(gap: u16) -> Self {
        Self { row: gap, column: gap }
    }

    /// Create with separate row and column gaps.
    pub fn new(row: u16, column: u16) -> Self {
        Self { row, column }
    }
}

/// Grid component.
#[derive(Debug, Clone)]
pub struct Grid {
    cells: Vec<Vec<Option<GridCell>>>,
    columns: Vec<GridColumn>,
    rows: usize,
    gap: GridGap,
    border: bool,
    width: Option<u16>,
}

impl Default for Grid {
    fn default() -> Self {
        Self {
            cells: Vec::new(),
            columns: Vec::new(),
            rows: 0,
            gap: GridGap::default(),
            border: false,
            width: None,
        }
    }
}

impl Grid {
    /// Create a new grid.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create with column count.
    pub fn columns(count: usize) -> Self {
        Self {
            columns: (0..count).map(|_| GridColumn::flex(1.0)).collect(),
            ..Default::default()
        }
    }

    /// Create with column definitions.
    pub fn with_columns<I>(columns: I) -> Self
    where
        I: IntoIterator<Item = GridColumn>,
    {
        Self {
            columns: columns.into_iter().collect(),
            ..Default::default()
        }
    }

    /// Set column definitions.
    pub fn cols<I>(mut self, columns: I) -> Self
    where
        I: IntoIterator<Item = GridColumn>,
    {
        self.columns = columns.into_iter().collect();
        self
    }

    /// Add a row of cells.
    pub fn row<I>(mut self, cells: I) -> Self
    where
        I: IntoIterator<Item = VNode>,
    {
        let row: Vec<Option<GridCell>> = cells
            .into_iter()
            .map(|c| Some(GridCell::new(c)))
            .collect();
        self.cells.push(row);
        self.rows += 1;
        self
    }

    /// Add a row with grid cells (for spans).
    pub fn row_cells<I>(mut self, cells: I) -> Self
    where
        I: IntoIterator<Item = GridCell>,
    {
        let row: Vec<Option<GridCell>> = cells.into_iter().map(Some).collect();
        self.cells.push(row);
        self.rows += 1;
        self
    }

    /// Set gap.
    pub fn gap(mut self, gap: u16) -> Self {
        self.gap = GridGap::all(gap);
        self
    }

    /// Set row and column gaps separately.
    pub fn gaps(mut self, row: u16, column: u16) -> Self {
        self.gap = GridGap::new(row, column);
        self
    }

    /// Enable border.
    pub fn border(mut self, show: bool) -> Self {
        self.border = show;
        self
    }

    /// Set width.
    pub fn width(mut self, width: u16) -> Self {
        self.width = Some(width);
        self
    }

    /// Calculate column widths.
    fn calculate_widths(&self, total_width: u16) -> Vec<u16> {
        if self.columns.is_empty() {
            return vec![];
        }

        let gap_space = (self.columns.len().saturating_sub(1) as u16) * self.gap.column;
        let available = total_width.saturating_sub(gap_space);

        let mut widths = vec![0u16; self.columns.len()];
        let mut remaining = available;
        let mut flex_total = 0.0;

        // First pass: allocate fixed and auto widths
        for (i, col) in self.columns.iter().enumerate() {
            match col.width {
                GridSize::Fixed(w) => {
                    let w = w.min(col.max_width.unwrap_or(w)).max(col.min_width.unwrap_or(0));
                    widths[i] = w;
                    remaining = remaining.saturating_sub(w);
                }
                GridSize::Auto => {
                    // For now, treat auto as a small fixed size
                    let w = 10u16.min(col.max_width.unwrap_or(10)).max(col.min_width.unwrap_or(5));
                    widths[i] = w;
                    remaining = remaining.saturating_sub(w);
                }
                GridSize::Flex(f) => {
                    flex_total += f;
                }
            }
        }

        // Second pass: allocate flex widths
        if flex_total > 0.0 {
            for (i, col) in self.columns.iter().enumerate() {
                if let GridSize::Flex(f) = col.width {
                    let w = ((remaining as f32 * f) / flex_total) as u16;
                    let w = w.min(col.max_width.unwrap_or(w)).max(col.min_width.unwrap_or(0));
                    widths[i] = w;
                }
            }
        }

        widths
    }

    /// Build the VNode.
    pub fn build(self) -> VNode {
        let total_width = self.width.unwrap_or(80);
        let widths = self.calculate_widths(total_width);

        let mut row_children: Vec<VNode> = Vec::new();

        for (row_idx, row) in self.cells.iter().enumerate() {
            let mut cell_children: Vec<VNode> = Vec::new();

            for (col_idx, cell_opt) in row.iter().enumerate() {
                if col_idx > 0 && self.gap.column > 0 {
                    // Add gap spacer
                    cell_children.push(VNode::text(" ".repeat(self.gap.column as usize)));
                }

                let cell_width = widths.get(col_idx).copied().unwrap_or(10);

                if let Some(cell) = cell_opt {
                    cell_children.push(VNode::Box(BoxNode {
                        children: vec![cell.content.clone()],
                        style: BoxStyle {
                            width: Some(Size::Fixed(cell_width)),
                            ..Default::default()
                        },
                        ..Default::default()
                    }));
                } else {
                    // Empty cell
                    cell_children.push(VNode::text(" ".repeat(cell_width as usize)));
                }
            }

            // Add row gap
            if row_idx > 0 && self.gap.row > 0 {
                for _ in 0..self.gap.row {
                    row_children.push(VNode::text(String::new()));
                }
            }

            row_children.push(VNode::Box(BoxNode {
                children: cell_children,
                style: BoxStyle {
                    flex_direction: Some(FlexDirection::Row),
                    ..Default::default()
                },
                ..Default::default()
            }));
        }

        let mut container_style = BoxStyle {
            flex_direction: Some(FlexDirection::Column),
            ..Default::default()
        };

        if self.border {
            container_style.border_style = Some(BorderStyle::Single);
            container_style.padding_left = Some(1);
            container_style.padding_right = Some(1);
        }

        VNode::Box(BoxNode {
            children: row_children,
            style: container_style,
            ..Default::default()
        })
    }
}
