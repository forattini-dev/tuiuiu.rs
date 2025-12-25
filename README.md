<div align="center">

# 🐦 Tuiuiu.rs

### Terminal UI Framework — Rust Edition

Build beautiful, reactive terminal apps with blazing-fast performance.
<br>
**Zero dependencies** • **Signal-based** • **Flexbox layout** • **Native speed** • **Memory safe**
<br>
50+ components. Pure Rust. No unsafe chaos. Maximum performance.

[![Crates.io](https://img.shields.io/crates/v/tuiuiu.svg?style=flat-square&color=F5A623)](https://crates.io/crates/tuiuiu)
[![Downloads](https://img.shields.io/crates/d/tuiuiu.svg?style=flat-square&color=34C759)](https://crates.io/crates/tuiuiu)
[![Rust](https://img.shields.io/badge/Rust-1.85+-DEA584?style=flat-square&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License](https://img.shields.io/crates/l/tuiuiu.svg?style=flat-square&color=007AFF)](LICENSE)
[![Zero Deps](https://img.shields.io/badge/dependencies-0*-success?style=flat-square)](https://crates.io/crates/tuiuiu)

[📖 Documentation](https://github.com/forattini-dev/tuiuiu.js) · [🚀 Quick Start](#quick-start) · [🦀 Rust API](#rust-api) · [⚡ Performance](#performance)

<sub>* Only `libc` for raw terminal mode</sub>

</div>

---

> **📚 Full Documentation**: This is the Rust port of [tuiuiu.js](https://github.com/forattini-dev/tuiuiu.js). For detailed concepts, architecture, component API, and design philosophy, see the **[JavaScript documentation](https://github.com/forattini-dev/tuiuiu.js#readme)** — the APIs are intentionally similar.

---

## Quick Start

```toml
# Cargo.toml
[dependencies]
tuiuiu = "0.1"
```

```rust
use tuiuiu::prelude::*;

fn main() {
    let app = render(counter);
    app.wait_until_exit();
}

fn counter() -> impl Component {
    let (count, set_count) = create_signal(0);

    use_input(move |key| match key {
        Key::Up => set_count.update(|c| *c += 1),
        Key::Down => set_count.update(|c| *c -= 1),
        Key::Escape => app::exit(),
        _ => {}
    });

    Box::new()
        .border(BorderStyle::Round)
        .padding(1)
        .children([
            Text::new("🐦 Tuiuiu Counter").color(Color::Cyan).bold(true),
            Text::new(move || format!("Count: {}", count.get())),
            Text::new("↑/↓: change • Esc: exit").color(Color::Gray).dim(true),
        ])
}
```

```bash
cargo run
```

## What's Inside

| Category | Components |
|:---------|:-----------|
| **Core** | Signals, Flexbox layout, Focus management, Event system |
| **Primitives** | `Box`, `Text`, `Spacer`, `Newline`, `Fragment`, `Divider`, `Canvas` |
| **Atoms** | `Button`, `TextInput`, `Switch`, `Slider`, `Spinner`, `ProgressBar` |
| **Molecules** | `Select`, `MultiSelect`, `RadioGroup`, `Autocomplete`, `Table`, `Tabs`, `Tree`, `Calendar`, `CodeBlock`, `Markdown` |
| **Organisms** | `Modal`, `CommandPalette`, `DataTable`, `FileManager`, `SplitPanel` |
| **Templates** | `AppShell`, `Page`, `Header`, `StatusBar`, `VStack`, `HStack` |
| **Data Viz** | `BarChart`, `LineChart`, `Sparkline`, `Heatmap`, `Gauge` |

## Rust API

### ⚡ Signals

Fine-grained reactivity — only what changes gets updated.

```rust
use tuiuiu::prelude::*;

let (count, set_count) = create_signal(0);
let doubled = create_memo(move || count.get() * 2);

create_effect(move || {
    println!("Count: {}, Doubled: {}", count.get(), doubled.get());
});

set_count.set(5);  // → "Count: 5, Doubled: 10"

// Batch multiple updates
batch(|| {
    set_count.set(1);
    set_count.set(2);
    set_count.set(3);
}); // Only one render!
```

### 📦 Flexbox Layout

CSS Flexbox model for terminals.

```rust
Box::new()
    .flex_direction(FlexDirection::Row)
    .justify_content(JustifyContent::SpaceBetween)
    .align_items(AlignItems::Center)
    .gap(2)
    .padding(1)
    .border(BorderStyle::Round)
    .children([
        Text::new("Left").color(Color::Blue),
        Text::new("Center"),
        Text::new("Right").color(Color::Red),
    ])
```

### 🎨 Components

```rust
use tuiuiu::molecules::*;

// Select dropdown
Select::new()
    .items(["Option A", "Option B", "Option C"])
    .selected(0)
    .build()

// Table with columns
Table::new()
    .columns([
        Column::new("Name").width(20),
        Column::new("Age").width(10).align(Align::Right),
    ])
    .rows([
        vec!["Alice".into(), "30".into()],
        vec!["Bob".into(), "25".into()],
    ])
    .build()

// Tree view
Tree::new()
    .nodes([
        TreeNode::folder("src").expanded(true).children([
            TreeNode::file("main.rs"),
            TreeNode::file("lib.rs"),
        ]),
    ])
    .build()

// Calendar
Calendar::new()
    .date(2025, 1)
    .selected(24)
    .today(2025, 1, 24)
    .build()

// Charts
Sparkline::new().data([1.0, 4.0, 2.0, 8.0, 5.0]).build()
Gauge::new().value(75.0).max(100.0).label("CPU").build()
```

### ⌨️ Input Handling

```rust
// Keyboard
use_input(|key| match key {
    Key::Char('q') => app::exit(),
    Key::Up => { /* ... */ },
    Key::Enter => { /* ... */ },
    _ => {}
});

// With modifiers
use_hotkeys(|key, mods| {
    if mods.ctrl && key == Key::Char('c') {
        app::exit();
    }
});

// Mouse
use_mouse(|event| match event.kind {
    MouseEventKind::Click(MouseButton::Left) => {
        println!("Click at ({}, {})", event.x, event.y);
    },
    _ => {}
});
```

## Feature Flags

Pick what you need — unused code is not compiled.

```toml
# Full (default)
tuiuiu = "0.1"

# Minimal core only
tuiuiu = { version = "0.1", default-features = false, features = ["core"] }

# Just what you need
tuiuiu = { version = "0.1", default-features = false, features = ["molecules"] }
```

| Feature | Contents | Includes |
|:--------|:---------|:---------|
| `full` | Everything | atoms, molecules, organisms, templates, themes |
| `core` | Signals, layout, renderer | — |
| `primitives` | Box, Text, Spacer, etc. | core |
| `atoms` | Button, Input, Spinner, etc. | primitives |
| `molecules` | Select, Table, Tabs, etc. | atoms |
| `organisms` | Modal, DataTable, etc. | molecules |
| `templates` | AppShell, Page, etc. | organisms |
| `themes` | Dark, Light, Monokai, etc. | — |

## Performance

Why Rust?

| Metric | tuiuiu.rs | tuiuiu.js |
|:-------|----------:|----------:|
| Startup time | ~1ms | ~50ms |
| Memory usage | ~2MB | ~30MB |
| Binary size | ~500KB | N/A |
| Dependencies | 0* | 0 |
| Type safety | Compile-time | Runtime |

<sub>* Only `libc` for raw terminal mode</sub>

## Examples

```bash
# Clone the repo
git clone https://github.com/tetis-io/tuiuiu.rs
cd tuiuiu.rs

# Run examples
cargo run --example counter      # Simple counter
cargo run --example dashboard    # Full dashboard
cargo run --example 03_molecules # Component showcase
```

## Comparison

| Feature | tuiuiu.rs | tuiuiu.js | Ratatui |
|:--------|:---------:|:---------:|:-------:|
| Signal reactivity | ✅ | ✅ | ❌ |
| Flexbox layout | ✅ | ✅ | ❌ |
| 50+ components | ✅ | ✅ | ❌ |
| Zero deps | ✅* | ✅ | ❌ |
| Mouse support | ✅ | ✅ | ✅ |
| Native binary | ✅ | ❌ | ✅ |
| Memory safe | ✅ | ✅ | ✅ |

## Documentation

> 📖 **For full documentation, concepts, and detailed API reference, see [tuiuiu.js](https://github.com/forattini-dev/tuiuiu.js#readme)** — the authoritative source.

| Topic | Description |
|:------|:------------|
| [Quick Start](https://github.com/forattini-dev/tuiuiu.js#quick-start) | Get up and running |
| [Signals](https://github.com/forattini-dev/tuiuiu.js#-signal-based-reactivity) | Reactive state management |
| [Layout](https://github.com/forattini-dev/tuiuiu.js#-flexbox-layout) | Flexbox for terminals |
| [Components](https://github.com/forattini-dev/tuiuiu.js#-50-ready-to-use-components) | All 50+ components |
| [Mouse](https://github.com/forattini-dev/tuiuiu.js#%EF%B8%8F-full-mouse-support) | Click, hover, scroll |
| [Charts](https://github.com/forattini-dev/tuiuiu.js#-data-visualization) | Data visualization |

## Numbers

| Metric | Value |
|:-------|------:|
| Components | 50+ |
| Dependencies | 0* |
| Hooks | 10 |
| Border styles | 9 |
| Named colors | 18 |
| Feature flags | 8 |
| Binary size | ~500KB |

## Why "Tuiuiu"?

The [Tuiuiu](https://en.wikipedia.org/wiki/Jabiru) (Jabiru mycteria) is a majestic Brazilian bird — the tallest flying bird in South America. Just like this bird stands out in its environment, Tuiuiu stands out in the terminal UI landscape: elegant, powerful, and distinctly Brazilian.

🇧🇷 Made with ❤️ in Brazil

## License

MIT © [Tetis](https://tetis.io)

---

<div align="center">

**[tuiuiu.js](https://github.com/forattini-dev/tuiuiu.js)** — JavaScript/TypeScript version
•
**tuiuiu.rs** — Rust version (you are here)

</div>
