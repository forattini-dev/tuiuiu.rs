//! Canvas Component
//!
//! Low-level drawing primitives using Braille or block characters.

use crate::core::component::VNode;

/// Canvas drawing mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CanvasMode {
    #[default]
    Braille,
    Block,
    Ascii,
}

/// Canvas for low-level drawing.
#[derive(Debug, Clone)]
pub struct Canvas {
    width: u16,
    height: u16,
    mode: CanvasMode,
    buffer: Vec<bool>,
}

impl Canvas {
    /// Create a new canvas.
    pub fn new(width: u16, height: u16) -> Self {
        let pixel_w = (width * 2) as usize;
        let pixel_h = (height * 4) as usize;
        Self {
            width,
            height,
            mode: CanvasMode::Braille,
            buffer: vec![false; pixel_w * pixel_h],
        }
    }

    /// Set the drawing mode.
    pub fn mode(mut self, mode: CanvasMode) -> Self {
        self.mode = mode;
        self
    }

    /// Set a pixel.
    pub fn set(&mut self, x: u16, y: u16) {
        let idx = self.pixel_index(x, y);
        if idx < self.buffer.len() {
            self.buffer[idx] = true;
        }
    }

    /// Clear a pixel.
    pub fn clear(&mut self, x: u16, y: u16) {
        let idx = self.pixel_index(x, y);
        if idx < self.buffer.len() {
            self.buffer[idx] = false;
        }
    }

    /// Toggle a pixel.
    pub fn toggle(&mut self, x: u16, y: u16) {
        let idx = self.pixel_index(x, y);
        if idx < self.buffer.len() {
            self.buffer[idx] = !self.buffer[idx];
        }
    }

    fn pixel_index(&self, x: u16, y: u16) -> usize {
        let pw = (self.width * 2) as usize;
        (y as usize) * pw + (x as usize)
    }

    /// Draw a line using Bresenham's algorithm.
    pub fn line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32) {
        let dx = (x1 - x0).abs();
        let dy = -(y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;
        let mut x = x0;
        let mut y = y0;

        loop {
            if x >= 0 && y >= 0 {
                self.set(x as u16, y as u16);
            }
            if x == x1 && y == y1 { break; }
            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x += sx;
            }
            if e2 <= dx {
                err += dx;
                y += sy;
            }
        }
    }

    /// Draw a rectangle.
    pub fn rect(&mut self, x: u16, y: u16, w: u16, h: u16) {
        for dx in 0..w {
            self.set(x + dx, y);
            self.set(x + dx, y + h - 1);
        }
        for dy in 0..h {
            self.set(x, y + dy);
            self.set(x + w - 1, y + dy);
        }
    }

    /// Render to a string.
    pub fn render(&self) -> String {
        match self.mode {
            CanvasMode::Braille => self.render_braille(),
            CanvasMode::Block => self.render_block(),
            CanvasMode::Ascii => self.render_ascii(),
        }
    }

    fn render_braille(&self) -> String {
        let mut output = String::new();
        let pw = (self.width * 2) as usize;

        for row in 0..self.height {
            for col in 0..self.width {
                let mut code: u32 = 0x2800; // Braille base
                let bx = (col * 2) as usize;
                let by = (row * 4) as usize;

                // Braille dot pattern
                let dots = [
                    (0, 0, 0x01), (0, 1, 0x02), (0, 2, 0x04), (0, 3, 0x40),
                    (1, 0, 0x08), (1, 1, 0x10), (1, 2, 0x20), (1, 3, 0x80),
                ];

                for (dx, dy, bit) in dots {
                    let idx = (by + dy) * pw + (bx + dx);
                    if idx < self.buffer.len() && self.buffer[idx] {
                        code |= bit;
                    }
                }

                if let Some(c) = char::from_u32(code) {
                    output.push(c);
                }
            }
            if row < self.height - 1 {
                output.push('\n');
            }
        }
        output
    }

    fn render_block(&self) -> String {
        let mut output = String::new();
        let pw = (self.width * 2) as usize;

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = (row as usize) * pw + (col as usize);
                let filled = idx < self.buffer.len() && self.buffer[idx];
                output.push(if filled { '█' } else { ' ' });
            }
            if row < self.height - 1 {
                output.push('\n');
            }
        }
        output
    }

    fn render_ascii(&self) -> String {
        let mut output = String::new();
        let pw = (self.width * 2) as usize;

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = (row as usize) * pw + (col as usize);
                let filled = idx < self.buffer.len() && self.buffer[idx];
                output.push(if filled { '#' } else { '.' });
            }
            if row < self.height - 1 {
                output.push('\n');
            }
        }
        output
    }

    /// Build into a VNode.
    pub fn build(self) -> VNode {
        VNode::Text(crate::core::component::TextNode {
            content: self.render(),
            style: Default::default(),
        })
    }
}

impl From<Canvas> for VNode {
    fn from(c: Canvas) -> VNode {
        c.build()
    }
}

/// Create a canvas.
pub fn canvas(width: u16, height: u16) -> Canvas {
    Canvas::new(width, height)
}
