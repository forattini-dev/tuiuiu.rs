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
//! fn counter() -> impl Into<tuiuiu::core::component::VNode> {
//!     let (count, set_count) = create_signal(0);
//!
//!     use_input(move |key, _mods| {
//!         match key {
//!             Key::Up => set_count.update(|c| *c += 1),
//!             Key::Down => set_count.update(|c| *c -= 1),
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
//!                 .cyan()
//!                 .bold()
//!                 .build(),
//!             Text::new(format!("Count: {}", count.get())).build(),
//!             Text::new("↑/↓: change • Esc: exit")
//!                 .gray()
//!                 .dim()
//!                 .build(),
//!         ])
//! }
//!
//! fn main() -> std::io::Result<()> {
//!     let mut app = render(counter)?;
//!     app.wait_until_exit()?;
//!     Ok(())
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
    batch, create_debounced, create_debounced as createDebounced,
    create_deferred as createDeferred, create_deferred, create_effect,
    create_effect as createEffect, create_id, create_id as createId, create_memo,
    create_memo as createMemo, create_previous, create_previous as createPrevious, create_reducer,
    create_reducer as createReducer, create_ref, create_ref as createRef, create_signal,
    create_signal as createSignal, create_throttled as createThrottled, create_throttled,
    reset_id_counter, reset_id_counter as resetIdCounter, untrack, Effect, Memo, ReadSignal,
    Signal, WriteSignal,
};

pub use core::layout::{
    calculate_layout, calculate_layout as calculateLayout, AlignContent, AlignItems, AlignSelf,
    FlexDirection, FlexWrap, JustifyContent, LayoutNode,
};

pub use core::renderer::{
    measure_height, measure_height as measureHeight, render_to_string as renderToString,
    render_to_string, OutputBuffer, RenderContext,
};

pub use core::app::{render, render_once, App, RenderOptions};

pub use core::terminal::{Key, KeyModifiers, MouseButton, MouseEvent, Terminal, TerminalEvent};

pub use core::event::{
    combine_handlers, combine_handlers as combineHandlers, conditional_handler,
    conditional_handler as conditionalHandler, create_event, create_event as createEvent,
    debounce_handler, debounce_handler as debounceHandler, delegate as delegateHandler,
    event_iterator, event_iterator as eventIterator, wait_for_event,
    wait_for_event as waitForEvent, DelegateOptions, Event, EventEmitter, EventHandler,
    EventListenerOptions, EventPhase,
};

pub use core::focus::{
    blur_focus as blurFocus, blur_focus, create_focus_trap as createFocusTrap, create_focus_trap,
    create_focus_zone as createFocusZone, create_focus_zone, focus_element as focusElement,
    focus_element, focus_first as focusFirst, focus_first, focus_last as focusLast, focus_last,
    focus_next as focusNext, focus_next, focus_previous as focusPrevious, focus_previous,
    get_active_id as getActiveId, get_active_id, get_focus_zone_manager as getFocusZoneManager,
    get_focus_zone_manager, is_focused as isFocused, is_focused, on_focus_change as onFocusChange,
    on_focus_change, register_focusable as registerFocusable, register_focusable,
    reset_focus_zone_manager as resetFocusZoneManager, reset_focus_zone_manager, FocusManager,
    FocusZoneState, Focusable,
};

pub use core::hotkeys::{is_hotkey, is_hotkey as isHotkey};

pub use core::command_palette::{
    create_command_palette_state, create_command_palette_state as createCommandPaletteState,
    execute_command, execute_command as executeCommand, format_command,
    format_command as formatCommand, fuzzy_match, fuzzy_match as fuzzyMatch, get_command_registry,
    get_command_registry as getCommandRegistry, group_by_category,
    group_by_category as groupByCategory, highlight_matches, highlight_matches as highlightMatches,
    register_command, register_command as registerCommand, reset_command_id_counter,
    reset_command_id_counter as resetCommandIdCounter, reset_command_registry,
    reset_command_registry as resetCommandRegistry, search_commands,
    search_commands as searchCommands, search_commands_default as searchCommandsDefault,
    search_global_commands, search_global_commands as searchGlobalCommands,
    search_global_commands_default as searchGlobalCommandsDefault, subscribe, subscribeListener,
    subscribe_and_return_unsubscribe,
    subscribe_and_return_unsubscribe as subscribeAndReturnUnsubscribe, unregister_command,
    unregister_command as unregisterCommand, unsubscribe, Command, CommandAction,
    CommandAsyncAction, CommandOptions, CommandPaletteOptions, CommandPaletteState,
    CommandRegistry, CommandRegistryHandle, FuzzyMatch, HighlightSegment, PaletteState,
    RegistryUnsubscribe,
};

pub use core::tick::{
    advance_tick, advance_tick as advanceTick, every_n_ticks as everyNTicks, every_n_ticks,
    get_elapsed_seconds, get_elapsed_seconds as getElapsedSeconds, get_fps, get_fps as getFps,
    get_fps_color as getFpsColor, get_fps_color, get_fps_metrics as getFpsMetrics, get_fps_metrics,
    get_frame as getFrame, get_frame, get_frame_item, get_frame_item as getFrameItem,
    get_tick as getTick, get_tick, get_tick_rate as getTickRate, get_tick_rate,
    is_tick_running as isTickRunning, is_tick_running, on_tick, on_tick as onTick, oscillate,
    pause_tick as pauseTick, pause_tick, reset_fps, reset_fps as resetFps, reset_tick as resetTick,
    reset_tick, resume_tick as resumeTick, resume_tick, set_tick_rate as setTickRate,
    set_tick_rate, set_tick_value as setTickValue, set_tick_value, start_tick as startTick,
    start_tick, stop_tick as stopTick, stop_tick, track_frame as trackFrame, track_frame, Tick,
};

// =============================================================================
// Hooks Module
// =============================================================================

pub mod hooks;

pub use hooks::{
    auto_threshold_color, binary_thresholds, cleanup_interval, cleanup_interval as cleanupInterval,
    cleanup_timeout, cleanup_timeout as cleanupTimeout, clear_clipboard, clear_input_handlers,
    clear_mouse_handlers, color_gradient, copy_to_clipboard, create_form, create_layout_ref,
    create_layout_ref as createLayoutRef, create_navigation, create_navigation as createNavigation,
    dispatch_key_event, dispatch_mouse_event, format_hotkey, format_hotkey_platform,
    get_hotkey_scope, get_registered_hotkeys, health_thresholds, inverted_percentage_thresholds,
    is_mac, key_matches, lerp_color, matches_hotkey, parse_hotkey, parse_hotkeys, parse_keypress,
    parse_keypress as parseKeypress, percentage_thresholds, read_clipboard, register_hotkey,
    reset_hotkey_scope, set_hotkey_scope, simple_steps, temperature_thresholds, trigger_hotkey,
    use_animation, use_animation as useAnimation, use_app, use_app as useApp, use_callback,
    use_changed, use_cleanup, use_cleanup as onCleanup, use_clipboard, use_counter,
    use_counter as useCounter, use_debounce, use_debounce as useDebounce,
    use_debounce as useDebounced, use_effect, use_effect as useEffect, use_fade_in, use_fade_out,
    use_focus, use_focus as useFocus, use_focus_manager, use_focus_manager as useFocusManager,
    use_form, use_form as useForm, use_format_bytes, use_format_bytes_si, use_format_compact,
    use_format_currency, use_format_delta, use_format_duration, use_format_duration_compact,
    use_format_number, use_format_percent, use_format_relative, use_fps, use_fps as useFps,
    use_hotkeys, use_hotkeys as useHotkeys, use_input, use_input as useInput, use_interval,
    use_interval as useInterval, use_interval_with_options, use_key, use_key as useKey,
    use_layout_ref, use_layout_ref as useLayoutRef, use_layout_ref_with,
    use_layout_ref_with as useLayoutRefWith, use_lazy_state, use_lazy_state as useLazyState,
    use_local_mouse, use_local_mouse as useLocalMouse, use_local_mouse_with,
    use_local_mouse_with as useLocalMouseWith, use_memo, use_memo as useMemo, use_mount, use_mouse,
    use_mouse as useMouse, use_navigation, use_navigation as useNavigation, use_previous,
    use_previous as usePrevious, use_reducer, use_reducer as useReducer, use_ref,
    use_ref as useRef, use_slide, use_state, use_state as useState, use_terminal_size,
    use_terminal_size as useTerminalSize, use_threshold_color, use_throttle,
    use_throttle as useThrottle, use_timeout, use_timeout as useTimeout, use_timeout_paused,
    use_toggle, use_toggle as useToggle, validators, AnimationHandle, AnimationState, Bounds,
    ClipboardHandle, Easing, FieldValue, FormField, FormHandle, FormState, HotkeyBinding,
    HotkeyHandler, HotkeyOptions, InputHandler, LayoutRect, LayoutRef, LocalMouseEvent,
    LocalMouseHandler, LocalMouseOptions, Modifiers, MouseAction, MouseHandler, MousePosition,
    NavigationResult, NavigationState, NavigationStep, PreviousHandle, RawMouseEvent,
    ThresholdColor, ThresholdConfig, ThresholdRange, ValidationResult,
};

// =============================================================================
// Utils Module
// =============================================================================

pub mod utils;

pub use utils::ansi::{colorize, strip_ansi, style, Color, Style};

pub use utils::text::{
    clear_text_measure_cache as clearTextMeasureCache, clear_text_measure_cache, measure_text,
    measure_text as measureText, slice_ansi, truncate_text, visible_width,
    visible_width as getVisibleWidth, visible_width as stringWidth, visible_width as visibleWidth,
    wrap_text,
};

pub use utils::cursor::{hide_cursor, move_cursor, restore_cursor, save_cursor, show_cursor};

pub use utils::border::{BorderChars, BorderStyle, BORDER_STYLES};

// =============================================================================
// Primitives Module
// =============================================================================

#[cfg(feature = "primitives")]
pub mod primitives;

#[cfg(feature = "primitives")]
pub use primitives::{
    applyMiddleware, apply_middleware, createLoggerMiddleware, createPersistMiddleware,
    createPersistedStore, createReactiveStore, createStore, create_logger_middleware,
    create_persist_middleware, create_persisted_store, create_reactive_store, create_store, Action,
    AnyAction, BoxComponent, Canvas, Dispatch, Divider, Each, Fragment, Middleware, MiddlewareAPI,
    Newline, PersistDeserializer, PersistOptions, PersistSerializer, PersistedStoreOptions,
    ReactiveStore, Reducer, Slot, Spacer, Static, Store, StoreCreator, StoreEnhancer,
    SyncStorageAdapter, Text, Transform, When,
};

// =============================================================================
// Atoms Module
// =============================================================================

#[cfg(feature = "atoms")]
pub mod atoms;

#[cfg(feature = "atoms")]
pub use atoms::{
    Badge, BadgeVariant, Button, Checkbox, CheckboxState, CheckboxValue, Icon, Link, ProgressBar,
    Scrollbar, ScrollbarMode, Slider, SliderMode, SliderState, Spinner, Switch, SwitchSize,
    SwitchState, TextInput, Timer, Tooltip,
};

// =============================================================================
// Molecules Module
// =============================================================================

#[cfg(feature = "molecules")]
pub mod molecules;

#[cfg(feature = "molecules")]
pub use molecules::{
    Autocomplete,
    BarChart,
    Calendar,
    CodeBlock,
    Gauge,
    Heatmap,
    LineChart,
    Markdown,
    MultiSelect,
    RadioGroup,
    Select,
    // Data visualization
    Sparkline,
    Table,
    Tabs,
    Tree,
};

// =============================================================================
// Organisms Module
// =============================================================================

#[cfg(feature = "organisms")]
pub mod organisms;

#[cfg(feature = "organisms")]
pub use organisms::{
    asciiIcons,
    buildPath,
    createCommandPalette,
    createDataTable,
    createGoToDialog,
    // Command palette
    create_command_palette,
    // DataTable compatibility
    create_data_table,
    // File browser compatibility
    create_file_browser_state,
    create_go_to_dialog,
    create_overlay_stack_state,
    create_scroll_list_state,
    filterFileItems,
    formatDate,
    formatFileSize,
    getExtension,
    getFileIcon,
    getParentPath,
    get_extension,
    nerdIcons,
    parsePath,
    simple_scroll_list,
    sortFileItems,
    unicodeIcons,
    useDataTableState,
    Column,
    ColumnAlign,
    CreateGoToDialogOptions,
    DataTable,
    DataTableColumn,
    DataTableOptions,
    DataTableProps,
    DataTableState,
    DirectoryIndicator,
    DirectoryTree,
    DirectoryTreeOptions,
    DividerStyle,
    EditableDataTable,
    EditableDataTableOptions,
    FileBrowser,
    FileBrowserFilter,
    FileBrowserOptions,
    FileBrowserState,
    FileDetails,
    FileDetailsOptions,
    FileDirectoryTreeOptions,
    FileFilter,
    FileIcon,
    FileIcons,
    FileItem,
    // Additional organism exports for parity
    FileItemField,
    FileItemType,
    FileList,
    FileListColumn,
    FileListOptions,
    FilePreview,
    FilePreviewOptions,
    FileSortDirection,
    FileSortField,
    FileSorter,
    GoToDialog,
    GoToDialogProps,
    GoToDialogState,
    // Base organism components
    Grid,
    Modal,
    Notification,
    OverlayEntry,
    OverlayStack,
    OverlayStackState,
    PathBreadcrumbs,
    PathBreadcrumbsOptions,
    ScrollArea,
    ScrollAreaState,
    ScrollList,
    ScrollListState,
    ScrollbarVisibility,
    SelectionMode,
    SortDirection,
    SortState,
    SplitOrientation,
    SplitPanel,
    SplitPanelState,
    TableSelectionMode,
    TableSortDirection,
    ThreePanel,
    Toast,
    ToastContainer,
    ToastEntry,
    ToastId,
    ToastPosition,
    ToastState,
    ToastVariant,
    VirtualDataTable,
    VirtualDataTableOptions,
    VirtualList,
    VirtualListState,
};
// Templates Module
// =============================================================================

#[cfg(feature = "templates")]
pub mod templates;

#[cfg(feature = "templates")]
pub use templates::{
    AppShell, Center, Container, Footer, FullScreen, HAlign, HJustify, HStack, Header, LayoutProps,
    Page, Sidebar, StatusBar, VAlign, VStack,
};

// =============================================================================
// Themes Module
// =============================================================================

#[cfg(feature = "themes")]
pub mod themes;

#[cfg(feature = "themes")]
pub use themes::{
    create_theme,
    // Built-in themes
    dark_theme,
    dracula_theme,
    get_color,
    get_contrast_color,
    get_theme,
    get_theme_by_name,
    light_theme,
    list_themes,
    parse_color,
    resolve_color,
    set_theme,
    // Theme management
    use_theme,
    // Colors
    ColorScale,
    Shade,
    // Core types
    Theme,
    ThemeAccents,
    ThemeBackground,
    ThemeBorders,
    ThemeForeground,
    ThemeMeta,
    ThemeMode,
    ThemePalette,
    ThemeStates,
    AMBER,
    BLACK,
    BLUE,
    CYAN,
    GREEN,
    RED,
    SLATE,
    WHITE,
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
    compare_snapshots, create_snapshot, get_event_log, inspect_layout, log_event, TerminalSimulator,
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
    pub use crate::core::app::{render, render_once, App};
    pub use crate::core::layout::{AlignItems, FlexDirection, JustifyContent};
    pub use crate::core::signals::{
        batch, create_effect, create_memo, create_signal, Effect, Memo, ReadSignal, WriteSignal,
    };
    pub use crate::core::terminal::{Key, KeyModifiers};

    // Command palette utilities
    pub use crate::core::command_palette::{
        create_command_palette_state as createCommandPaletteState, create_command_palette_state,
        execute_command as executeCommand, execute_command, format_command as formatCommand,
        format_command, fuzzy_match as fuzzyMatch, fuzzy_match,
        get_command_registry as getCommandRegistry, get_command_registry,
        group_by_category as groupByCategory, group_by_category,
        highlight_matches as highlightMatches, highlight_matches,
        register_command as registerCommand, register_command,
        reset_command_id_counter as resetCommandIdCounter, reset_command_id_counter,
        reset_command_registry as resetCommandRegistry, reset_command_registry, search_commands,
        search_commands as searchCommands, search_commands_default as searchCommandsDefault,
        search_global_commands as searchGlobalCommands,
        search_global_commands_default as searchGlobalCommandsDefault, subscribe,
        subscribeListener, subscribe_and_return_unsubscribe as subscribeAndReturnUnsubscribe,
        unregister_command as unregisterCommand, unregister_command, unsubscribe, Command,
        CommandAction, CommandAsyncAction, CommandOptions, CommandPaletteOptions,
        CommandPaletteState, CommandRegistryHandle, FuzzyMatch, PaletteState, RegistryUnsubscribe,
    };

    // Hooks
    pub use crate::hooks::{use_app, use_effect, use_input, use_state};

    // Utils
    pub use crate::utils::ansi::Color;
    pub use crate::utils::border::BorderStyle;

    // Primitives
    #[cfg(feature = "primitives")]
    pub use crate::primitives::{BoxComponent as Box, Fragment, Spacer, Text};

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

#[allow(non_snake_case)]
/// JS compatibility alias for [`version`].
pub fn getVersion() -> &'static str {
    version()
}

#[allow(non_snake_case)]
/// JS compatibility alias for [`version`].
pub fn getVersionSync() -> &'static str {
    version()
}

#[allow(non_snake_case)]
/// JS compatibility alias for [`version_info`].
pub fn getVersionInfo() -> VersionInfo {
    version_info()
}

#[allow(non_snake_case)]
/// JS compatibility helper for formatting version metadata.
pub fn formatVersionInfo(info: &VersionInfo) -> String {
    let features = info.features.join(", ");
    format!(
        "version: {}, rust_version: {}, features: [{}]",
        info.version, info.rust_version, features
    )
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
