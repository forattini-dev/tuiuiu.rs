//! Terminal Renderer
//!
//! Converts the component tree into terminal output with ANSI escape codes.
//! Handles double-buffering, diffing, and efficient updates.

use std::collections::HashMap;
use crate::core::layout::{ComputedLayout, LayoutNode, calculate_layout};
use crate::core::component::{VNode, Color, NamedColor, BorderStyle, TextStyle};

// =============================================================================
// Render Context
// =============================================================================

/// Context passed during rendering.
#[derive(Debug, Clone)]
pub struct RenderContext {
    /// Terminal width
    pub width: u16,
    /// Terminal height
    pub height: u16,
    /// Current cursor X
    pub cursor_x: u16,
    /// Current cursor Y
    pub cursor_y: u16,
    /// Whether to use colors
    pub use_colors: bool,
    /// Whether to use Unicode
    pub use_unicode: bool,
}

impl RenderContext {
    /// Create a new render context.
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            width,
            height,
            cursor_x: 0,
            cursor_y: 0,
            use_colors: true,
            use_unicode: true,
        }
    }
}

// =============================================================================
// Output Buffer
// =============================================================================

/// A cell in the output buffer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cell {
    /// Character to display
    pub char: char,
    /// Foreground color
    pub fg: Color,
    /// Background color
    pub bg: Color,
    /// Style flags
    pub bold: bool,
    pub dim: bool,
    pub italic: bool,
    pub underline: bool,
    pub inverse: bool,
    pub strikethrough: bool,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            char: ' ',
            fg: Color::Default,
            bg: Color::Default,
            bold: false,
            dim: false,
            italic: false,
            underline: false,
            inverse: false,
            strikethrough: false,
        }
    }
}

impl Cell {
    /// Create a cell with a character.
    pub fn new(c: char) -> Self {
        Self {
            char: c,
            ..Default::default()
        }
    }

    /// Check if this cell differs from another (needs redraw).
    pub fn differs_from(&self, other: &Cell) -> bool {
        self != other
    }
}

/// Output buffer for double-buffering.
#[derive(Debug, Clone)]
pub struct OutputBuffer {
    /// Width of the buffer
    width: u16,
    /// Height of the buffer
    height: u16,
    /// Cell data
    cells: Vec<Cell>,
}

impl OutputBuffer {
    /// Create a new buffer.
    pub fn new(width: u16, height: u16) -> Self {
        let size = (width as usize) * (height as usize);
        Self {
            width,
            height,
            cells: vec![Cell::default(); size],
        }
    }

    /// Get buffer dimensions.
    pub fn size(&self) -> (u16, u16) {
        (self.width, self.height)
    }

    /// Get a cell at position.
    pub fn get(&self, x: u16, y: u16) -> Option<&Cell> {
        if x < self.width && y < self.height {
            let idx = (y as usize) * (self.width as usize) + (x as usize);
            self.cells.get(idx)
        } else {
            None
        }
    }

    /// Get a mutable cell at position.
    pub fn get_mut(&mut self, x: u16, y: u16) -> Option<&mut Cell> {
        if x < self.width && y < self.height {
            let idx = (y as usize) * (self.width as usize) + (x as usize);
            self.cells.get_mut(idx)
        } else {
            None
        }
    }

    /// Set a cell at position.
    pub fn set(&mut self, x: u16, y: u16, cell: Cell) {
        if x < self.width && y < self.height {
            let idx = (y as usize) * (self.width as usize) + (x as usize);
            if idx < self.cells.len() {
                self.cells[idx] = cell;
            }
        }
    }

    /// Set a character at position.
    pub fn set_char(&mut self, x: u16, y: u16, c: char) {
        if let Some(cell) = self.get_mut(x, y) {
            cell.char = c;
        }
    }

    /// Write a string at position.
    pub fn write_str(&mut self, x: u16, y: u16, s: &str, style: &TextStyle) {
        let mut curr_x = x;
        for c in s.chars() {
            if curr_x >= self.width {
                break;
            }
            if let Some(cell) = self.get_mut(curr_x, y) {
                cell.char = c;
                if let Some(color) = style.color {
                    cell.fg = color;
                }
                if let Some(bg) = style.background {
                    cell.bg = bg;
                }
                cell.bold = style.bold;
                cell.dim = style.dim;
                cell.italic = style.italic;
                cell.underline = style.underline;
                cell.inverse = style.inverse;
                cell.strikethrough = style.strikethrough;
            }
            curr_x += 1;
        }
    }

    /// Fill a rectangle with a cell.
    pub fn fill_rect(&mut self, x: u16, y: u16, width: u16, height: u16, cell: Cell) {
        for dy in 0..height {
            for dx in 0..width {
                self.set(x + dx, y + dy, cell.clone());
            }
        }
    }

    /// Draw a border around a rectangle.
    pub fn draw_border(
        &mut self,
        x: u16,
        y: u16,
        width: u16,
        height: u16,
        style: BorderStyle,
        color: Color,
    ) {
        if width < 2 || height < 2 {
            return;
        }

        let chars = get_border_chars(style);

        // Corners
        self.set_styled_char(x, y, chars.top_left, color);
        self.set_styled_char(x + width - 1, y, chars.top_right, color);
        self.set_styled_char(x, y + height - 1, chars.bottom_left, color);
        self.set_styled_char(x + width - 1, y + height - 1, chars.bottom_right, color);

        // Horizontal lines
        for dx in 1..width - 1 {
            self.set_styled_char(x + dx, y, chars.horizontal, color);
            self.set_styled_char(x + dx, y + height - 1, chars.horizontal, color);
        }

        // Vertical lines
        for dy in 1..height - 1 {
            self.set_styled_char(x, y + dy, chars.vertical, color);
            self.set_styled_char(x + width - 1, y + dy, chars.vertical, color);
        }
    }

    fn set_styled_char(&mut self, x: u16, y: u16, c: char, color: Color) {
        if let Some(cell) = self.get_mut(x, y) {
            cell.char = c;
            cell.fg = color;
        }
    }

    /// Clear the buffer.
    pub fn clear(&mut self) {
        for cell in &mut self.cells {
            *cell = Cell::default();
        }
    }

    /// Generate diff between two buffers.
    pub fn diff(&self, other: &OutputBuffer) -> Vec<(u16, u16, Cell)> {
        let mut changes = Vec::new();

        for y in 0..self.height.min(other.height) {
            for x in 0..self.width.min(other.width) {
                if let (Some(a), Some(b)) = (self.get(x, y), other.get(x, y)) {
                    if a.differs_from(b) {
                        changes.push((x, y, b.clone()));
                    }
                }
            }
        }

        changes
    }

    /// Render buffer to a string.
    pub fn to_string(&self) -> String {
        let mut output = String::new();
        let mut last_fg = Color::Default;
        let mut last_bg = Color::Default;
        let mut last_bold = false;
        let mut last_dim = false;

        for y in 0..self.height {
            if y > 0 {
                output.push('\n');
            }

            for x in 0..self.width {
                if let Some(cell) = self.get(x, y) {
                    // Apply style changes
                    if cell.fg != last_fg || cell.bg != last_bg
                        || cell.bold != last_bold || cell.dim != last_dim
                    {
                        output.push_str("\x1B[0m"); // Reset

                        if cell.bold {
                            output.push_str("\x1B[1m");
                        }
                        if cell.dim {
                            output.push_str("\x1B[2m");
                        }
                        if cell.italic {
                            output.push_str("\x1B[3m");
                        }
                        if cell.underline {
                            output.push_str("\x1B[4m");
                        }
                        if cell.inverse {
                            output.push_str("\x1B[7m");
                        }
                        if cell.strikethrough {
                            output.push_str("\x1B[9m");
                        }

                        output.push_str(&color_to_ansi_fg(cell.fg));
                        output.push_str(&color_to_ansi_bg(cell.bg));

                        last_fg = cell.fg;
                        last_bg = cell.bg;
                        last_bold = cell.bold;
                        last_dim = cell.dim;
                    }

                    output.push(cell.char);
                }
            }
        }

        // Reset at end
        output.push_str("\x1B[0m");
        output
    }
}

// =============================================================================
// Border Characters
// =============================================================================

/// Border character set.
#[derive(Debug, Clone, Copy)]
pub struct BorderChars {
    pub top_left: char,
    pub top_right: char,
    pub bottom_left: char,
    pub bottom_right: char,
    pub horizontal: char,
    pub vertical: char,
    pub t_left: char,
    pub t_right: char,
    pub t_top: char,
    pub t_bottom: char,
    pub cross: char,
}

/// Get border characters for a style.
pub fn get_border_chars(style: BorderStyle) -> BorderChars {
    match style {
        BorderStyle::None | BorderStyle::Hidden => BorderChars {
            top_left: ' ',
            top_right: ' ',
            bottom_left: ' ',
            bottom_right: ' ',
            horizontal: ' ',
            vertical: ' ',
            t_left: ' ',
            t_right: ' ',
            t_top: ' ',
            t_bottom: ' ',
            cross: ' ',
        },
        BorderStyle::Single => BorderChars {
            top_left: '┌',
            top_right: '┐',
            bottom_left: '└',
            bottom_right: '┘',
            horizontal: '─',
            vertical: '│',
            t_left: '├',
            t_right: '┤',
            t_top: '┬',
            t_bottom: '┴',
            cross: '┼',
        },
        BorderStyle::Double => BorderChars {
            top_left: '╔',
            top_right: '╗',
            bottom_left: '╚',
            bottom_right: '╝',
            horizontal: '═',
            vertical: '║',
            t_left: '╠',
            t_right: '╣',
            t_top: '╦',
            t_bottom: '╩',
            cross: '╬',
        },
        BorderStyle::Round => BorderChars {
            top_left: '╭',
            top_right: '╮',
            bottom_left: '╰',
            bottom_right: '╯',
            horizontal: '─',
            vertical: '│',
            t_left: '├',
            t_right: '┤',
            t_top: '┬',
            t_bottom: '┴',
            cross: '┼',
        },
        BorderStyle::Bold => BorderChars {
            top_left: '┏',
            top_right: '┓',
            bottom_left: '┗',
            bottom_right: '┛',
            horizontal: '━',
            vertical: '┃',
            t_left: '┣',
            t_right: '┫',
            t_top: '┳',
            t_bottom: '┻',
            cross: '╋',
        },
        BorderStyle::Dashed => BorderChars {
            top_left: '┌',
            top_right: '┐',
            bottom_left: '└',
            bottom_right: '┘',
            horizontal: '╌',
            vertical: '╎',
            t_left: '├',
            t_right: '┤',
            t_top: '┬',
            t_bottom: '┴',
            cross: '┼',
        },
        BorderStyle::Dotted => BorderChars {
            top_left: '·',
            top_right: '·',
            bottom_left: '·',
            bottom_right: '·',
            horizontal: '·',
            vertical: '·',
            t_left: '·',
            t_right: '·',
            t_top: '·',
            t_bottom: '·',
            cross: '·',
        },
        BorderStyle::Classic => BorderChars {
            top_left: '+',
            top_right: '+',
            bottom_left: '+',
            bottom_right: '+',
            horizontal: '-',
            vertical: '|',
            t_left: '+',
            t_right: '+',
            t_top: '+',
            t_bottom: '+',
            cross: '+',
        },
    }
}

// =============================================================================
// Color Conversion
// =============================================================================

fn color_to_ansi_fg(color: Color) -> String {
    match color {
        Color::Default => String::new(),
        Color::Named(c) => format!("\x1B[{}m", named_color_to_fg_code(c)),
        Color::Ansi256(n) => format!("\x1B[38;5;{}m", n),
        Color::Rgb(r, g, b) => format!("\x1B[38;2;{};{};{}m", r, g, b),
    }
}

fn color_to_ansi_bg(color: Color) -> String {
    match color {
        Color::Default => String::new(),
        Color::Named(c) => format!("\x1B[{}m", named_color_to_bg_code(c)),
        Color::Ansi256(n) => format!("\x1B[48;5;{}m", n),
        Color::Rgb(r, g, b) => format!("\x1B[48;2;{};{};{}m", r, g, b),
    }
}

fn named_color_to_fg_code(color: NamedColor) -> u8 {
    match color {
        NamedColor::Black => 30,
        NamedColor::Red => 31,
        NamedColor::Green => 32,
        NamedColor::Yellow => 33,
        NamedColor::Blue => 34,
        NamedColor::Magenta => 35,
        NamedColor::Cyan => 36,
        NamedColor::White => 37,
        NamedColor::BrightBlack | NamedColor::Gray => 90,
        NamedColor::BrightRed => 91,
        NamedColor::BrightGreen => 92,
        NamedColor::BrightYellow => 93,
        NamedColor::BrightBlue => 94,
        NamedColor::BrightMagenta => 95,
        NamedColor::BrightCyan => 96,
        NamedColor::BrightWhite => 97,
    }
}

fn named_color_to_bg_code(color: NamedColor) -> u8 {
    named_color_to_fg_code(color) + 10
}

// =============================================================================
// Render Functions
// =============================================================================

/// Render a VNode tree to a string.
pub fn render_to_string(node: &VNode, width: u16, height: u16) -> String {
    let mut buffer = OutputBuffer::new(width, height);
    let layout_node = vnode_to_layout_node(node, 0);
    let layouts = calculate_layout(&layout_node, width, height);

    render_vnode_to_buffer(node, &layouts, 0, &mut buffer);

    buffer.to_string()
}

/// Render a VNode to an output buffer.
pub fn render_vnode_to_buffer(
    node: &VNode,
    layouts: &HashMap<u64, ComputedLayout>,
    id: u64,
    buffer: &mut OutputBuffer,
) {
    match node {
        VNode::Box(box_node) => {
            let node_id = box_node.id.unwrap_or(id);
            if let Some(layout) = layouts.get(&node_id) {
                // Draw background
                if let Some(bg) = box_node.style.background {
                    buffer.fill_rect(
                        layout.x,
                        layout.y,
                        layout.width,
                        layout.height,
                        Cell {
                            char: ' ',
                            bg,
                            ..Default::default()
                        },
                    );
                }

                // Draw border
                if let Some(border_style) = box_node.style.border_style {
                    if !matches!(border_style, BorderStyle::None | BorderStyle::Hidden) {
                        buffer.draw_border(
                            layout.x,
                            layout.y,
                            layout.width,
                            layout.height,
                            border_style,
                            box_node.style.border_color.unwrap_or(Color::Default),
                        );
                    }
                }

                // Render children
                for (i, child) in box_node.children.iter().enumerate() {
                    render_vnode_to_buffer(child, layouts, node_id * 1000 + i as u64, buffer);
                }
            }
        }
        VNode::Text(text_node) => {
            if let Some(layout) = layouts.get(&id) {
                buffer.write_str(layout.x, layout.y, &text_node.content, &text_node.style);
            }
        }
        VNode::Spacer(_) => {
            // Spacers don't render anything visible
        }
        VNode::Fragment(children) => {
            for (i, child) in children.iter().enumerate() {
                render_vnode_to_buffer(child, layouts, id * 1000 + i as u64, buffer);
            }
        }
        VNode::Empty => {}
    }
}

/// Convert VNode to LayoutNode for layout calculation.
fn vnode_to_layout_node(node: &VNode, id: u64) -> LayoutNode {
    match node {
        VNode::Box(box_node) => {
            let node_id = box_node.id.unwrap_or(id);
            let mut layout = LayoutNode::new(node_id);

            // Apply style
            if let Some(fd) = box_node.style.flex_direction {
                layout.style.flex_direction = fd;
            }
            if let Some(jc) = box_node.style.justify_content {
                layout.style.justify_content = jc;
            }
            if let Some(ai) = box_node.style.align_items {
                layout.style.align_items = ai;
            }
            if let Some(gap) = box_node.style.gap {
                layout.style.gap = gap;
            }
            if let Some(fg) = box_node.style.flex_grow {
                layout.style.flex_grow = fg;
            }
            if let Some(w) = box_node.style.width {
                layout.style.width = w;
            }
            if let Some(h) = box_node.style.height {
                layout.style.height = h;
            }

            // Padding
            if let Some(p) = box_node.style.padding {
                layout.style.padding = crate::core::layout::Edges::all(p);
            }

            // Border
            if box_node.style.border_style.is_some() {
                layout.style.border_width = 1;
            }

            // Children
            for (i, child) in box_node.children.iter().enumerate() {
                layout.children.push(vnode_to_layout_node(child, node_id * 1000 + i as u64));
            }

            layout
        }
        VNode::Text(text_node) => {
            let width = text_node.content.chars().count() as u16;
            LayoutNode::text(id, width, 1)
        }
        VNode::Spacer(spacer) => {
            LayoutNode::text(id, spacer.x, spacer.y.max(1))
        }
        VNode::Fragment(children) => {
            let mut layout = LayoutNode::new(id);
            for (i, child) in children.iter().enumerate() {
                layout.children.push(vnode_to_layout_node(child, id * 1000 + i as u64));
            }
            layout
        }
        VNode::Empty => LayoutNode::new(id),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_output_buffer() {
        let mut buffer = OutputBuffer::new(10, 5);
        buffer.set_char(0, 0, 'X');
        assert_eq!(buffer.get(0, 0).map(|c| c.char), Some('X'));
    }

    #[test]
    fn test_buffer_write_str() {
        let mut buffer = OutputBuffer::new(20, 5);
        buffer.write_str(0, 0, "Hello", &TextStyle::default());

        assert_eq!(buffer.get(0, 0).map(|c| c.char), Some('H'));
        assert_eq!(buffer.get(4, 0).map(|c| c.char), Some('o'));
    }

    #[test]
    fn test_border_chars() {
        let chars = get_border_chars(BorderStyle::Round);
        assert_eq!(chars.top_left, '╭');
        assert_eq!(chars.top_right, '╮');
    }
}
