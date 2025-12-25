//! Spacer and Newline Components

use crate::core::component::{VNode, SpacerNode};

/// Spacer component - creates empty space.
#[derive(Debug, Clone, Default)]
pub struct Spacer {
    x: u16,
    y: u16,
}

impl Spacer {
    /// Create a new spacer.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set horizontal space.
    pub fn x(mut self, value: u16) -> Self {
        self.x = value;
        self
    }

    /// Set vertical space.
    pub fn y(mut self, value: u16) -> Self {
        self.y = value;
        self
    }

    /// Create a horizontal spacer.
    pub fn horizontal(width: u16) -> Self {
        Self::new().x(width)
    }

    /// Create a vertical spacer.
    pub fn vertical(height: u16) -> Self {
        Self::new().y(height)
    }

    /// Create a flex spacer (takes remaining space).
    pub fn flex() -> Self {
        Self::new()
    }

    /// Build into a VNode.
    pub fn build(self) -> VNode {
        VNode::Spacer(SpacerNode {
            x: self.x,
            y: self.y.max(1),
        })
    }
}

impl From<Spacer> for VNode {
    fn from(s: Spacer) -> VNode {
        s.build()
    }
}

/// Newline component - line break.
#[derive(Debug, Clone, Default)]
pub struct Newline {
    count: u16,
}

impl Newline {
    /// Create a newline.
    pub fn new() -> Self {
        Self { count: 1 }
    }

    /// Set number of newlines.
    pub fn count(mut self, n: u16) -> Self {
        self.count = n;
        self
    }

    /// Build into a VNode.
    pub fn build(self) -> VNode {
        VNode::Spacer(SpacerNode {
            x: 0,
            y: self.count,
        })
    }
}

impl From<Newline> for VNode {
    fn from(n: Newline) -> VNode {
        n.build()
    }
}

/// Create a spacer.
pub fn spacer() -> Spacer {
    Spacer::new()
}

/// Create a newline.
pub fn newline() -> Newline {
    Newline::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spacer() {
        let s = Spacer::horizontal(5);
        assert_eq!(s.x, 5);
    }

    #[test]
    fn test_newline() {
        let n = Newline::new().count(2);
        assert_eq!(n.count, 2);
    }
}
