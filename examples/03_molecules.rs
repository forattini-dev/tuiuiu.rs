//! Molecules Showcase Example
//!
//! Demonstrates the molecule components in tuiuiu.

use tuiuiu::molecules::{
    // Select components
    Select, SelectOption, MultiSelect, RadioGroup,
    // Autocomplete
    Autocomplete, Suggestion,
    // Table
    Table, Column, Align,
    // Tree
    Tree, TreeNode,
    // Calendar
    Calendar,
    // Tabs
    Tabs, Tab, TabStyle,
    // Code
    CodeBlock, CodeTheme, Markdown,
    // Charts
    Sparkline, BarChart, BarItem, Gauge, GaugeStyle,
};
use tuiuiu::core::component::VNode;

fn main() {
    // Test Select
    let select = Select::new()
        .items(["Option 1", "Option 2", "Option 3"])
        .selected(1)
        .open(true)
        .build();

    println!("Select: {:?}\n", matches!(select, VNode::Box(_)));

    // Test RadioGroup
    let radio = RadioGroup::new()
        .items(["Small", "Medium", "Large"])
        .selected(1)
        .label("Size")
        .build();

    println!("RadioGroup: {:?}\n", matches!(radio, VNode::Box(_)));

    // Test Autocomplete
    let autocomplete = Autocomplete::new()
        .suggestions([
            Suggestion::new("Rust").description("Systems programming"),
            Suggestion::new("Python").description("Scripting"),
            Suggestion::new("JavaScript").description("Web development"),
        ])
        .value("Ru")
        .open(true)
        .build();

    println!("Autocomplete: {:?}\n", matches!(autocomplete, VNode::Box(_)));

    // Test Table
    let table = Table::new()
        .columns([
            Column::new("Name").width(20),
            Column::new("Age").width(10).align(Align::Right),
            Column::new("City").width(15),
        ])
        .rows([
            vec!["Alice".to_string(), "30".to_string(), "NYC".to_string()],
            vec!["Bob".to_string(), "25".to_string(), "LA".to_string()],
            vec!["Carol".to_string(), "35".to_string(), "SF".to_string()],
        ])
        .build();

    println!("Table: {:?}\n", matches!(table, VNode::Box(_)));

    // Test Tree
    let tree = Tree::new()
        .nodes([
            TreeNode::folder("src")
                .expanded(true)
                .children([
                    TreeNode::file("main.rs"),
                    TreeNode::file("lib.rs"),
                    TreeNode::folder("core")
                        .children([
                            TreeNode::file("app.rs"),
                            TreeNode::file("signals.rs"),
                        ]),
                ]),
            TreeNode::file("Cargo.toml"),
        ])
        .build();

    println!("Tree: {:?}\n", matches!(tree, VNode::Box(_)));

    // Test Calendar
    let calendar = Calendar::new()
        .date(2025, 1)
        .selected(15)
        .today(2025, 1, 24)
        .build();

    println!("Calendar: {:?}\n", matches!(calendar, VNode::Box(_)));

    // Test Tabs
    let tabs = Tabs::new()
        .tabs([
            Tab::new("Overview"),
            Tab::new("Details").disabled(),
            Tab::new("Settings"),
        ])
        .active(0)
        .build();

    println!("Tabs: {:?}\n", matches!(tabs, VNode::Box(_)));

    // Test CodeBlock
    let code = CodeBlock::new(r#"
fn main() {
    println!("Hello, Tuiuiu!");
}
"#)
        .rust()
        .highlight([2])
        .build();

    println!("CodeBlock: {:?}\n", matches!(code, VNode::Box(_)));

    // Test Markdown
    let md = Markdown::new(r#"
# Title
## Subtitle

- Item 1
- Item 2

> A quote

`code`
"#).build();

    println!("Markdown: {:?}\n", matches!(md, VNode::Box(_)));

    // Test Sparkline
    let sparkline = Sparkline::new()
        .data([1.0, 4.0, 2.0, 8.0, 5.0, 7.0, 3.0, 6.0])
        .build();

    println!("Sparkline: {:?}\n", matches!(sparkline, VNode::Box(_)));

    // Test BarChart
    let bar_chart = BarChart::new()
        .items([
            BarItem::new("A", 30.0),
            BarItem::new("B", 50.0),
            BarItem::new("C", 20.0),
        ])
        .build();

    println!("BarChart: {:?}\n", matches!(bar_chart, VNode::Box(_)));

    // Test Gauge (returns Text node, not Box)
    let gauge = Gauge::new()
        .value(75.0)
        .max(100.0)
        .label("Progress")
        .build();

    println!("Gauge: {:?}\n", matches!(gauge, VNode::Text(_)));

    println!("✅ All molecules compiled and instantiated successfully!");
}
