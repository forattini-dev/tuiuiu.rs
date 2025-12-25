//! Control Flow Components
//!
//! Conditional rendering and iteration helpers.

use crate::core::component::{VNode, Child, children_to_vnodes};

/// Conditional rendering - shows children only when condition is true.
pub struct When {
    condition: bool,
    children: Vec<VNode>,
    fallback: Option<Vec<VNode>>,
}

impl When {
    /// Create a When component.
    pub fn new(condition: bool) -> Self {
        Self {
            condition,
            children: Vec::new(),
            fallback: None,
        }
    }

    /// Set children to show when true.
    pub fn children<I, C>(mut self, children: I) -> Self
    where
        I: IntoIterator<Item = C>,
        C: Into<Child>,
    {
        self.children = children_to_vnodes(children);
        self
    }

    /// Set fallback to show when false.
    pub fn fallback<I, C>(mut self, children: I) -> Self
    where
        I: IntoIterator<Item = C>,
        C: Into<Child>,
    {
        self.fallback = Some(children_to_vnodes(children));
        self
    }

    /// Build into a VNode.
    pub fn build(self) -> VNode {
        if self.condition {
            VNode::Fragment(self.children)
        } else if let Some(fallback) = self.fallback {
            VNode::Fragment(fallback)
        } else {
            VNode::Empty
        }
    }
}

impl From<When> for VNode {
    fn from(w: When) -> VNode {
        w.build()
    }
}

/// Iterate and render items.
pub struct Each<T, F>
where
    F: Fn(&T, usize) -> VNode,
{
    items: Vec<T>,
    render: F,
}

impl<T, F> Each<T, F>
where
    F: Fn(&T, usize) -> VNode,
{
    /// Create an Each component.
    pub fn new<I: IntoIterator<Item = T>>(items: I, render: F) -> Self {
        Self {
            items: items.into_iter().collect(),
            render,
        }
    }

    /// Build into a VNode.
    pub fn build(self) -> VNode {
        let children: Vec<_> = self.items
            .iter()
            .enumerate()
            .map(|(i, item)| (self.render)(item, i))
            .collect();
        VNode::Fragment(children)
    }
}

impl<T, F> From<Each<T, F>> for VNode
where
    F: Fn(&T, usize) -> VNode,
{
    fn from(e: Each<T, F>) -> VNode {
        e.build()
    }
}

/// Transform children with a function.
pub struct Transform<F>
where
    F: Fn(VNode) -> VNode,
{
    transform: F,
    children: Vec<VNode>,
}

impl<F> Transform<F>
where
    F: Fn(VNode) -> VNode,
{
    /// Create a Transform component.
    pub fn new(transform: F) -> Self {
        Self {
            transform,
            children: Vec::new(),
        }
    }

    /// Set children.
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
        let transformed: Vec<_> = self.children
            .into_iter()
            .map(|c| (self.transform)(c))
            .collect();
        VNode::Fragment(transformed)
    }
}

/// Static content that never re-renders.
pub struct Static {
    children: Vec<VNode>,
}

impl Static {
    /// Create a Static component.
    pub fn new() -> Self {
        Self { children: Vec::new() }
    }

    /// Set children.
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

impl Default for Static {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Static> for VNode {
    fn from(s: Static) -> VNode {
        s.build()
    }
}

/// Named slot for content injection.
pub struct Slot {
    #[allow(dead_code)]
    name: String,
    default: Option<Vec<VNode>>,
}

impl Slot {
    /// Create a named slot.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            default: None,
        }
    }

    /// Set default content.
    pub fn default<I, C>(mut self, children: I) -> Self
    where
        I: IntoIterator<Item = C>,
        C: Into<Child>,
    {
        self.default = Some(children_to_vnodes(children));
        self
    }

    /// Build into a VNode (returns default if no injection).
    pub fn build(self) -> VNode {
        if let Some(default) = self.default {
            VNode::Fragment(default)
        } else {
            VNode::Empty
        }
    }
}

impl From<Slot> for VNode {
    fn from(s: Slot) -> VNode {
        s.build()
    }
}

/// Helper function for conditional rendering.
pub fn when(condition: bool) -> When {
    When::new(condition)
}

/// Helper function for iteration.
pub fn each<T, I, F>(items: I, render: F) -> Each<T, F>
where
    I: IntoIterator<Item = T>,
    F: Fn(&T, usize) -> VNode,
{
    Each::new(items, render)
}
