//! Molecules - Composite Components
//!
//! Components built from atoms that provide richer functionality:
//! - Selection: Select, MultiSelect, RadioGroup, Autocomplete
//! - Data Display: Table, Tree, Calendar
//! - Navigation: Tabs
//! - Content: CodeBlock, Markdown
//! - Data Visualization: Sparkline, BarChart, LineChart, Gauge, Heatmap

mod select;
mod table;
mod tabs;
mod tree;
mod calendar;
mod code;
mod charts;
mod autocomplete;

// Selection components
pub use select::{Select, SelectOption, MultiSelect, RadioGroup};
pub use autocomplete::{Autocomplete, Suggestion};

// Data display
pub use table::{Table, Column, Align};
pub use tree::{Tree, TreeNode, file_tree};
pub use calendar::Calendar;

// Navigation
pub use tabs::{Tabs, Tab, TabStyle};

// Content
pub use code::{CodeBlock, CodeTheme, Markdown};

// Charts
pub use charts::{
    Sparkline,
    BarChart, BarItem, BarOrientation,
    LineChart,
    Gauge, GaugeStyle,
    Heatmap,
};
