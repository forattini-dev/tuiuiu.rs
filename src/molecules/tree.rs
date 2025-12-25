//! Tree Component
//!
//! Hierarchical tree view.

use crate::core::component::{VNode, BoxNode, BoxStyle, TextStyle, Color, NamedColor};

/// A tree node.
#[derive(Debug, Clone)]
pub struct TreeNode {
    /// Node label
    pub label: String,
    /// Child nodes
    pub children: Vec<TreeNode>,
    /// Whether expanded
    pub expanded: bool,
    /// Whether selected
    pub selected: bool,
    /// Icon (optional)
    pub icon: Option<String>,
    /// Node ID (optional)
    pub id: Option<String>,
}

impl TreeNode {
    /// Create a new tree node.
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            children: Vec::new(),
            expanded: false,
            selected: false,
            icon: None,
            id: None,
        }
    }

    /// Add a child node.
    pub fn child(mut self, child: TreeNode) -> Self {
        self.children.push(child);
        self
    }

    /// Add multiple children.
    pub fn children<I>(mut self, children: I) -> Self
    where
        I: IntoIterator<Item = TreeNode>,
    {
        self.children.extend(children);
        self
    }

    /// Set expanded state.
    pub fn expanded(mut self, expanded: bool) -> Self {
        self.expanded = expanded;
        self
    }

    /// Set selected state.
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    /// Set icon.
    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Set ID.
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Create a leaf node (no children).
    pub fn leaf(label: impl Into<String>) -> Self {
        Self::new(label)
    }

    /// Create a folder-like node.
    pub fn folder(label: impl Into<String>) -> Self {
        Self::new(label).icon("📁")
    }

    /// Create a file-like node.
    pub fn file(label: impl Into<String>) -> Self {
        Self::new(label).icon("📄")
    }
}

/// Tree component.
#[derive(Debug, Clone)]
pub struct Tree {
    nodes: Vec<TreeNode>,
    indent: u16,
    show_lines: bool,
    selected_id: Option<String>,
    expand_all: bool,
    folder_icons: (String, String),
}

impl Default for Tree {
    fn default() -> Self {
        Self {
            nodes: Vec::new(),
            indent: 2,
            show_lines: true,
            selected_id: None,
            expand_all: false,
            folder_icons: ("▶".to_string(), "▼".to_string()),
        }
    }
}

impl Tree {
    /// Create a new tree.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set root nodes.
    pub fn nodes<I>(mut self, nodes: I) -> Self
    where
        I: IntoIterator<Item = TreeNode>,
    {
        self.nodes = nodes.into_iter().collect();
        self
    }

    /// Set single root node.
    pub fn root(mut self, node: TreeNode) -> Self {
        self.nodes = vec![node];
        self
    }

    /// Set indent size.
    pub fn indent(mut self, indent: u16) -> Self {
        self.indent = indent;
        self
    }

    /// Show/hide connecting lines.
    pub fn lines(mut self, show: bool) -> Self {
        self.show_lines = show;
        self
    }

    /// Set selected node ID.
    pub fn selected(mut self, id: impl Into<String>) -> Self {
        self.selected_id = Some(id.into());
        self
    }

    /// Expand all nodes.
    pub fn expand_all(mut self) -> Self {
        self.expand_all = true;
        self
    }

    /// Set folder icons (collapsed, expanded).
    pub fn folder_icons(mut self, collapsed: impl Into<String>, expanded: impl Into<String>) -> Self {
        self.folder_icons = (collapsed.into(), expanded.into());
        self
    }

    /// Render a node and its children.
    fn render_node(
        &self,
        node: &TreeNode,
        prefix: &str,
        is_last: bool,
        depth: usize,
    ) -> Vec<VNode> {
        let mut result = Vec::new();

        // Build the line prefix
        let connector = if depth == 0 {
            ""
        } else if is_last {
            if self.show_lines { "└─" } else { "  " }
        } else {
            if self.show_lines { "├─" } else { "  " }
        };

        // Expand/collapse icon
        let expand_icon = if node.children.is_empty() {
            "  "
        } else if node.expanded || self.expand_all {
            &self.folder_icons.1
        } else {
            &self.folder_icons.0
        };

        // Node icon
        let icon = node.icon.as_deref().unwrap_or("");

        // Build label
        let label = if icon.is_empty() {
            format!("{}{}{} {}", prefix, connector, expand_icon, node.label)
        } else {
            format!("{}{}{} {} {}", prefix, connector, expand_icon, icon, node.label)
        };

        // Check if selected
        let is_selected = node.id.as_ref()
            .map(|id| self.selected_id.as_ref() == Some(id))
            .unwrap_or(false) || node.selected;

        let color = if is_selected {
            Color::Named(NamedColor::Cyan)
        } else if !node.children.is_empty() {
            Color::Named(NamedColor::Yellow)
        } else {
            Color::Named(NamedColor::White)
        };

        result.push(VNode::styled_text(
            label,
            TextStyle { color: Some(color), bold: is_selected, inverse: is_selected, ..Default::default() }
        ));

        // Render children if expanded
        if (node.expanded || self.expand_all) && !node.children.is_empty() {
            let child_prefix = if depth == 0 {
                String::new()
            } else {
                let continuation = if is_last { "  " } else { if self.show_lines { "│ " } else { "  " } };
                format!("{}{}", prefix, continuation)
            };

            let child_prefix = format!("{}{}", child_prefix, " ".repeat(self.indent as usize));

            for (i, child) in node.children.iter().enumerate() {
                let is_last_child = i == node.children.len() - 1;
                result.extend(self.render_node(child, &child_prefix, is_last_child, depth + 1));
            }
        }

        result
    }

    /// Build the VNode.
    pub fn build(self) -> VNode {
        let mut children = Vec::new();

        for (i, node) in self.nodes.iter().enumerate() {
            let is_last = i == self.nodes.len() - 1;
            children.extend(self.render_node(node, "", is_last, 0));
        }

        VNode::Box(BoxNode {
            children,
            style: BoxStyle::default(),
            ..Default::default()
        })
    }
}

/// Convenience function to create a tree from a directory-like structure.
pub fn file_tree<I, S>(files: I) -> Tree
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    use std::collections::HashMap;

    let mut root: HashMap<String, TreeNode> = HashMap::new();

    for path in files {
        let parts: Vec<&str> = path.as_ref().split('/').collect();
        let mut current = &mut root;

        for (i, part) in parts.iter().enumerate() {
            let is_file = i == parts.len() - 1;
            let key = part.to_string();

            if !current.contains_key(&key) {
                let node = if is_file {
                    TreeNode::file(*part)
                } else {
                    TreeNode::folder(*part).expanded(true)
                };
                current.insert(key.clone(), node);
            }

            // For directories, we'd need to recurse into children
            // This is a simplified implementation
        }
    }

    Tree::new().nodes(root.into_values())
}
