//! Fragment Component
//!
//! Groups children without a wrapper element.

use crate::core::component::{VNode, Child, children_to_vnodes};

/// Fragment - groups multiple children without a wrapper.
#[derive(Debug, Clone, Default)]
pub struct Fragment {
    children: Vec<VNode>,
}

impl Fragment {
    /// Create a new fragment.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add children.
    pub fn children<I, C>(mut self, children: I) -> Self
    where
        I: IntoIterator<Item = C>,
        C: Into<Child>,
    {
        self.children = children_to_vnodes(children);
        self
    }

    /// Build into a VNode.
    pub fn build(self) -> VNode {
        VNode::Fragment(self.children)
    }
}

impl From<Fragment> for VNode {
    fn from(f: Fragment) -> VNode {
        f.build()
    }
}

/// Create a fragment from children.
pub fn fragment<I, C>(children: I) -> Fragment
where
    I: IntoIterator<Item = C>,
    C: Into<Child>,
{
    Fragment::new().children(children)
}
