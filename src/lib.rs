//! # Tuiuiu - Zero-dependency Terminal UI Framework
//!
//! A minimal, reactive terminal UI framework with signal-based reactivity,
//! flexbox layout, and zero external dependencies.
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use tuiuiu::prelude::*;
//!
//! fn counter() -> impl Component {
//!     let (count, set_count) = create_signal(0);
//!
//!     use_input(move |key| {
//!         match key {
//!             Key::Up => set_count.update(|c| *c += 1),
//!             Key::Down => set_count.update(|c| *c -= 1),
//!             Key::Escape => app::exit(),
//!             _ => {}
//!         }
//!     });
//!
//!     Box::new()
//!         .flex_direction(FlexDirection::Column)
//!         .padding(1)
//!         .border(BorderStyle::Round)
//!         .children([
//!             Text::new("🐦 Tuiuiu Counter")
//!                 .color(Color::Cyan)
//!                 .bold(true),
//!             Text::new(move || format!("Count: {}", count.get())),
//!             Text::new("↑/↓: change • Esc: exit")
//!                 .color(Color::Gray)
//!                 .dim(true),
//!         ])
//! }
//!
//! fn main() {
//!     let app = render(counter);
//!     app.wait_until_exit();
//! }
//! ```
//!
//! ## Architecture
//!
//! Tuiuiu follows the Atomic Design methodology:
//!
//! - **Core**: Signals, Layout engine, Renderer, Event system
//! - **Primitives**: Box, Text, Spacer, Newline, Fragment, Divider
//! - **Atoms**: Button, TextInput, Switch, Slider, Spinner, ProgressBar
//! - **Molecules**: Select, Table, Tabs, Tree, Calendar, Charts
//! - **Organisms**: Modal, CommandPalette, DataTable, FileManager
//! - **Templates**: AppShell, Page, Header, StatusBar
//!
//! ## Features
//!
//! - `full` (default): All components
//! - `core`: Only core functionality
//! - `primitives`: Core + primitives
//! - `atoms`: Primitives + atoms
//! - `molecules`: Atoms + molecules
//! - `organisms`: Molecules + organisms
//! - `templates`: Organisms + templates
//! - `themes`: Theme system
//! - `mcp`: Model Context Protocol server
//! - `dev-tools`: Development and debugging tools

#![doc(html_root_url = "https://docs.rs/tuiuiu/0.1.0")]
#![allow(missing_docs)] // TODO: Add documentation for all public items
#![warn(rustdoc::missing_crate_level_docs)]

// =============================================================================
// Core Module
// =============================================================================

pub mod core;

// Re-export core types
pub use core::signals::{
    batch, create_effect, create_memo, create_signal, untrack, Effect, Memo, ReadSignal,
    WriteSignal,
};

pub use core::layout::{
    AlignContent, AlignItems, AlignSelf, FlexDirection, FlexWrap, JustifyContent, LayoutNode,
    calculate_layout,
};

pub use core::renderer::{OutputBuffer, RenderContext, render_to_string};

pub use core::app::{App, RenderOptions, render, render_once};

pub use core::terminal::{
    Key, KeyModifiers, MouseButton, MouseEvent, Terminal, TerminalEvent,
};

pub use core::event::{Event, EventEmitter, EventHandler, EventPhase};

pub use core::focus::{FocusManager, Focusable, FocusZoneState};

pub use core::tick::{
    Tick, get_tick, on_tick, start_tick, stop_tick, pause_tick, resume_tick,
    get_frame, oscillate,
};

// =============================================================================
// Hooks Module
// =============================================================================

pub mod hooks;

pub use hooks::{
    use_state, use_effect, use_memo, use_input, use_mouse, use_focus, use_app,
    use_terminal_size, use_fps, use_hotkeys,
};

// =============================================================================
// Utils Module
// =============================================================================

pub mod utils;

pub use utils::ansi::{
    strip_ansi, colorize, style, Color, Style,
};

pub use utils::text::{
    measure_text, visible_width, wrap_text, truncate_text, slice_ansi,
};

pub use utils::cursor::{
    show_cursor, hide_cursor, move_cursor, save_cursor, restore_cursor,
};

pub use utils::border::{BorderStyle, BorderChars, BORDER_STYLES};

// =============================================================================
// Primitives Module
// =============================================================================

#[cfg(feature = "primitives")]
pub mod primitives;

#[cfg(feature = "primitives")]
pub use primitives::{
    BoxComponent, Text, Spacer, Newline, Fragment, Divider, Canvas,
    When, Each, Transform, Static, Slot,
};

// =============================================================================
// Atoms Module
// =============================================================================

#[cfg(feature = "atoms")]
pub mod atoms;

#[cfg(feature = "atoms")]
pub use atoms::{
    Button, TextInput, Switch, Slider, Spinner, ProgressBar, Timer,
    Checkbox, Badge, Link, Tooltip, Icon,
};

// =============================================================================
// Molecules Module
// =============================================================================

#[cfg(feature = "molecules")]
pub mod molecules;

#[cfg(feature = "molecules")]
pub use molecules::{
    Select, MultiSelect, RadioGroup, Autocomplete, Table, Tabs, Tree,
    Calendar, CodeBlock, Markdown,
    // Data visualization
    Sparkline, BarChart, LineChart, Gauge, Heatmap,
};

// =============================================================================
// Organisms Module
// =============================================================================

#[cfg(feature = "organisms")]
pub mod organisms;

#[cfg(feature = "organisms")]
pub use organisms::{
    Modal, CommandPalette, DataTable, FileManager, SplitPanel, ScrollArea,
    Grid, OverlayStack, Notification, Toast,
};

// =============================================================================
// Templates Module
// =============================================================================

#[cfg(feature = "templates")]
pub mod templates;

#[cfg(feature = "templates")]
pub use templates::{
    AppShell, Page, Header, StatusBar, Footer, Sidebar,
    VStack, HStack, Center, FullScreen, Container,
};

// =============================================================================
// Themes Module
// =============================================================================

#[cfg(feature = "themes")]
pub mod themes;

#[cfg(feature = "themes")]
pub use themes::{
    Theme, ThemeMode, use_theme, get_theme, set_theme, create_theme,
    dark_theme, light_theme, monokai_theme, dracula_theme, nord_theme,
};

// =============================================================================
// MCP Module (Model Context Protocol)
// =============================================================================

#[cfg(feature = "mcp")]
pub mod mcp;

// =============================================================================
// Dev Tools Module
// =============================================================================

#[cfg(feature = "dev-tools")]
pub mod dev_tools;

#[cfg(feature = "dev-tools")]
pub use dev_tools::{
    inspect_layout, log_event, get_event_log,
    TerminalSimulator, create_snapshot, compare_snapshots,
};

// =============================================================================
// Prelude - Common imports
// =============================================================================

/// Commonly used types and traits for convenient importing.
///
/// ```rust
/// use tuiuiu::prelude::*;
/// ```
pub mod prelude {
    // Core
    pub use crate::core::signals::{
        batch, create_effect, create_memo, create_signal, Effect, Memo, ReadSignal, WriteSignal,
    };
    pub use crate::core::layout::{FlexDirection, JustifyContent, AlignItems};
    pub use crate::core::app::{render, render_once, App};
    pub use crate::core::terminal::{Key, KeyModifiers};

    // Hooks
    pub use crate::hooks::{use_state, use_effect, use_input, use_app};

    // Utils
    pub use crate::utils::ansi::Color;
    pub use crate::utils::border::BorderStyle;

    // Primitives
    #[cfg(feature = "primitives")]
    pub use crate::primitives::{BoxComponent as Box, Text, Spacer, Fragment};

    // Component trait
    pub use crate::core::component::Component;
}

// =============================================================================
// Version Info
// =============================================================================

/// Returns the library version.
pub const fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

/// Returns detailed version information.
pub fn version_info() -> VersionInfo {
    VersionInfo {
        version: version(),
        rust_version: env!("CARGO_PKG_RUST_VERSION"),
        features: get_enabled_features(),
    }
}

/// Version information structure.
#[derive(Debug, Clone)]
pub struct VersionInfo {
    /// Package version
    pub version: &'static str,
    /// Minimum Rust version
    pub rust_version: &'static str,
    /// Enabled features
    pub features: Vec<&'static str>,
}

fn get_enabled_features() -> Vec<&'static str> {
    let mut features = Vec::new();

    #[cfg(feature = "core")]
    features.push("core");

    #[cfg(feature = "primitives")]
    features.push("primitives");

    #[cfg(feature = "atoms")]
    features.push("atoms");

    #[cfg(feature = "molecules")]
    features.push("molecules");

    #[cfg(feature = "organisms")]
    features.push("organisms");

    #[cfg(feature = "templates")]
    features.push("templates");

    #[cfg(feature = "themes")]
    features.push("themes");

    #[cfg(feature = "mcp")]
    features.push("mcp");

    #[cfg(feature = "dev-tools")]
    features.push("dev-tools");

    features
}
