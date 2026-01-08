//! Layout Reference Hook
//!
//! Hook for measuring and tracking component layout bounds.
//!
//! # Example
//!
//! ```rust,ignore
//! use tuiuiu::hooks::layout::{use_layout_ref, LayoutRef};
//!
//! let layout_ref = use_layout_ref();
//! // After render, layout_ref will contain the measured bounds
//! let bounds = layout_ref.get();
//! println!("Component at ({}, {}) with size {}x{}",
//!     bounds.x, bounds.y, bounds.width, bounds.height);
//! ```

use crate::core::signals::{create_signal, ReadSignal, WriteSignal};

// =============================================================================
// Types
// =============================================================================

/// Layout rectangle representing bounds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct LayoutRect {
    /// X position (left edge).
    pub x: u16,
    /// Y position (top edge).
    pub y: u16,
    /// Width of the bounds.
    pub width: u16,
    /// Height of the bounds.
    pub height: u16,
}

impl LayoutRect {
    /// Create a new layout rect.
    pub fn new(x: u16, y: u16, width: u16, height: u16) -> Self {
        Self { x, y, width, height }
    }

    /// Create an empty layout rect.
    pub fn empty() -> Self {
        Self::default()
    }

    /// Check if a point is inside this rect.
    pub fn contains(&self, x: u16, y: u16) -> bool {
        x >= self.x
            && x < self.x.saturating_add(self.width)
            && y >= self.y
            && y < self.y.saturating_add(self.height)
    }

    /// Get the right edge x coordinate.
    pub fn right(&self) -> u16 {
        self.x.saturating_add(self.width)
    }

    /// Get the bottom edge y coordinate.
    pub fn bottom(&self) -> u16 {
        self.y.saturating_add(self.height)
    }

    /// Get the center x coordinate.
    pub fn center_x(&self) -> u16 {
        self.x.saturating_add(self.width / 2)
    }

    /// Get the center y coordinate.
    pub fn center_y(&self) -> u16 {
        self.y.saturating_add(self.height / 2)
    }

    /// Check if this rect is empty.
    pub fn is_empty(&self) -> bool {
        self.width == 0 || self.height == 0
    }

    /// Calculate area.
    pub fn area(&self) -> u32 {
        self.width as u32 * self.height as u32
    }

    /// Check if this rect intersects with another.
    pub fn intersects(&self, other: &LayoutRect) -> bool {
        self.x < other.right()
            && self.right() > other.x
            && self.y < other.bottom()
            && self.bottom() > other.y
    }

    /// Get the intersection of two rects.
    pub fn intersection(&self, other: &LayoutRect) -> Option<LayoutRect> {
        if !self.intersects(other) {
            return None;
        }

        let x = self.x.max(other.x);
        let y = self.y.max(other.y);
        let right = self.right().min(other.right());
        let bottom = self.bottom().min(other.bottom());

        Some(LayoutRect::new(
            x,
            y,
            right.saturating_sub(x),
            bottom.saturating_sub(y),
        ))
    }
}

/// A layout reference that tracks component bounds.
#[derive(Clone)]
pub struct LayoutRef {
    rect: ReadSignal<LayoutRect>,
    set_rect: WriteSignal<LayoutRect>,
}

impl LayoutRef {
    /// Create a new layout ref with initial bounds.
    pub fn new(initial: LayoutRect) -> Self {
        let (rect, set_rect) = create_signal(initial);
        Self { rect, set_rect }
    }

    /// Create a new layout ref with empty bounds.
    pub fn empty() -> Self {
        Self::new(LayoutRect::empty())
    }

    /// Get the current layout bounds.
    pub fn get(&self) -> LayoutRect {
        self.rect.get()
    }

    /// Get the x position.
    pub fn x(&self) -> u16 {
        self.rect.get().x
    }

    /// Get the y position.
    pub fn y(&self) -> u16 {
        self.rect.get().y
    }

    /// Get the width.
    pub fn width(&self) -> u16 {
        self.rect.get().width
    }

    /// Get the height.
    pub fn height(&self) -> u16 {
        self.rect.get().height
    }

    /// Update the layout bounds.
    ///
    /// Only triggers updates if the bounds have changed.
    pub fn update(&self, new_rect: LayoutRect) {
        let current = self.rect.get_untracked();
        if current != new_rect {
            self.set_rect.set(new_rect);
        }
    }

    /// Update individual position.
    pub fn set_position(&self, x: u16, y: u16) {
        let current = self.rect.get_untracked();
        if current.x != x || current.y != y {
            self.set_rect.set(LayoutRect {
                x,
                y,
                ..current
            });
        }
    }

    /// Update individual size.
    pub fn set_size(&self, width: u16, height: u16) {
        let current = self.rect.get_untracked();
        if current.width != width || current.height != height {
            self.set_rect.set(LayoutRect {
                width,
                height,
                ..current
            });
        }
    }

    /// Check if a point is inside the bounds.
    pub fn contains(&self, x: u16, y: u16) -> bool {
        self.rect.get().contains(x, y)
    }

    /// Get the read signal for reactive access.
    pub fn signal(&self) -> ReadSignal<LayoutRect> {
        self.rect.clone()
    }
}

// =============================================================================
// Hook
// =============================================================================

/// Create a layout reference for measuring component bounds.
///
/// This hook creates a stable layout ref that persists across re-renders
/// and can be updated by the layout system.
///
/// # Returns
///
/// A `LayoutRef` that will be populated with bounds after render.
///
/// # Example
///
/// ```rust,ignore
/// use tuiuiu::hooks::layout::use_layout_ref;
///
/// let layout_ref = use_layout_ref();
///
/// // Later, after layout is computed:
/// let bounds = layout_ref.get();
/// if bounds.contains(mouse_x, mouse_y) {
///     // Handle click inside component
/// }
/// ```
pub fn use_layout_ref() -> LayoutRef {
    LayoutRef::empty()
}

/// Create a layout reference with initial bounds.
pub fn use_layout_ref_with(initial: LayoutRect) -> LayoutRef {
    LayoutRef::new(initial)
}

/// Create a layout ref outside of component context.
///
/// Useful for creating refs in state management code.
pub fn create_layout_ref(initial: Option<LayoutRect>) -> LayoutRef {
    LayoutRef::new(initial.unwrap_or_default())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_layout_rect_contains() {
        let rect = LayoutRect::new(10, 10, 20, 20);
        assert!(rect.contains(15, 15));
        assert!(rect.contains(10, 10)); // Top-left corner
        assert!(!rect.contains(30, 30)); // Right edge is exclusive
        assert!(!rect.contains(5, 5)); // Outside
    }

    #[test]
    fn test_layout_rect_edges() {
        let rect = LayoutRect::new(10, 20, 30, 40);
        assert_eq!(rect.right(), 40);
        assert_eq!(rect.bottom(), 60);
        assert_eq!(rect.center_x(), 25);
        assert_eq!(rect.center_y(), 40);
    }

    #[test]
    fn test_layout_rect_intersection() {
        let a = LayoutRect::new(0, 0, 20, 20);
        let b = LayoutRect::new(10, 10, 20, 20);

        assert!(a.intersects(&b));
        let intersection = a.intersection(&b).unwrap();
        assert_eq!(intersection, LayoutRect::new(10, 10, 10, 10));

        let c = LayoutRect::new(50, 50, 10, 10);
        assert!(!a.intersects(&c));
        assert!(a.intersection(&c).is_none());
    }

    #[test]
    fn test_layout_ref() {
        let layout_ref = use_layout_ref();
        assert_eq!(layout_ref.get(), LayoutRect::empty());

        layout_ref.update(LayoutRect::new(10, 20, 100, 50));
        assert_eq!(layout_ref.x(), 10);
        assert_eq!(layout_ref.y(), 20);
        assert_eq!(layout_ref.width(), 100);
        assert_eq!(layout_ref.height(), 50);
    }

    #[test]
    fn test_layout_ref_partial_update() {
        let layout_ref = use_layout_ref_with(LayoutRect::new(0, 0, 100, 100));

        layout_ref.set_position(10, 20);
        assert_eq!(layout_ref.x(), 10);
        assert_eq!(layout_ref.y(), 20);
        assert_eq!(layout_ref.width(), 100); // Unchanged

        layout_ref.set_size(50, 50);
        assert_eq!(layout_ref.width(), 50);
        assert_eq!(layout_ref.height(), 50);
        assert_eq!(layout_ref.x(), 10); // Unchanged
    }
}
