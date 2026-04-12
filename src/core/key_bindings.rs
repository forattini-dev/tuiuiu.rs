//! Key Binding Registry
//!
//! Context-aware keyboard shortcut registry with optional vim/emacs mode helpers.

use crate::core::terminal::{Key, KeyModifiers as TerminalKeyModifiers};
use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::Hash;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

// ============================================================================
// Types
// ============================================================================

/// Key modifiers used for key-binding matching.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub struct KeyModifiers {
    /// Ctrl key is pressed.
    pub ctrl: bool,
    /// Alt/Option key is pressed.
    pub alt: bool,
    /// Shift key is pressed.
    pub shift: bool,
    /// Meta/Cmd/Super key is pressed.
    pub meta: bool,
}

impl KeyModifiers {
    fn from_terminal(mods: &TerminalKeyModifiers) -> Self {
        Self {
            ctrl: mods.ctrl,
            alt: mods.alt,
            shift: mods.shift,
            meta: mods.meta,
        }
    }
}

/// Parsed key combination.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyCombo {
    /// Main key name.
    pub key: String,
    /// Modifier state.
    pub modifiers: KeyModifiers,
}

/// Action executed when a binding matches.
pub type KeyAction = Arc<dyn Fn() + Send + Sync + 'static>;

/// Key binding definition.
pub struct KeyBinding {
    /// Unique identifier.
    pub id: String,
    /// Human typed key string.
    pub key: String,
    /// Parsed representation.
    pub combo: KeyCombo,
    /// Action callback.
    pub action: KeyAction,
    /// Optional description.
    pub description: Option<String>,
    /// Binding context.
    pub context: String,
    /// Priority used in conflict resolution.
    pub priority: i32,
    /// Enabled flag.
    pub enabled: bool,
    /// Optional command id.
    pub command_id: Option<String>,
}

impl Clone for KeyBinding {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            key: self.key.clone(),
            combo: self.combo.clone(),
            action: Arc::clone(&self.action),
            description: self.description.clone(),
            context: self.context.clone(),
            priority: self.priority,
            enabled: self.enabled,
            command_id: self.command_id.clone(),
        }
    }
}

/// Registration options.
pub struct KeyBindingOptions {
    /// Key combo string.
    pub key: String,
    /// Action to execute.
    pub action: KeyAction,
    /// Optional description.
    pub description: Option<String>,
    /// Optional context.
    pub context: Option<String>,
    /// Optional priority.
    pub priority: Option<i32>,
    /// Optional command identifier.
    pub command_id: Option<String>,
}

/// Supported interaction modes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeyMode {
    /// Default mode.
    Default,
    /// Vim mode preset.
    Vim,
    /// Emacs mode preset.
    Emacs,
}

impl Default for KeyMode {
    fn default() -> Self {
        Self::Default
    }
}

/// Conflict info.
pub struct KeyConflict {
    /// Human-friendly key representation.
    pub key: String,
    /// Context where conflict occurs.
    pub context: String,
    /// Conflicting bindings.
    pub bindings: Vec<KeyBinding>,
}

// ============================================================================
// Parsing helpers
// ============================================================================

fn normalize_key(raw: &str) -> String {
    match raw.to_ascii_lowercase().as_str() {
        "esc" | "escape" => "Escape".to_string(),
        "enter" | "return" => "Enter".to_string(),
        "space" | "spacebar" => " ".to_string(),
        "up" => "ArrowUp".to_string(),
        "down" => "ArrowDown".to_string(),
        "left" => "ArrowLeft".to_string(),
        "right" => "ArrowRight".to_string(),
        "tab" => "Tab".to_string(),
        "backspace" => "Backspace".to_string(),
        "delete" | "del" => "Delete".to_string(),
        "home" => "Home".to_string(),
        "end" => "End".to_string(),
        "pageup" => "PageUp".to_string(),
        "pagedown" => "PageDown".to_string(),
        "insert" | "ins" => "Insert".to_string(),
        other => other.to_string(),
    }
}

/// Parse a string like `ctrl+k` or `shift+ArrowUp`.
pub fn parse_key_combo(key_string: &str) -> KeyCombo {
    let mut modifiers = KeyModifiers::default();
    let mut main_key = String::new();

    for part in key_string
        .split('+')
        .map(str::trim)
        .filter(|p| !p.is_empty())
    {
        match part.to_ascii_lowercase().as_str() {
            "ctrl" | "control" => modifiers.ctrl = true,
            "alt" | "option" => modifiers.alt = true,
            "shift" => modifiers.shift = true,
            "meta" | "cmd" | "command" | "win" | "windows" => modifiers.meta = true,
            key => main_key = normalize_key(key),
        }
    }

    if main_key.is_empty() {
        main_key = String::new();
    }

    KeyCombo {
        key: main_key,
        modifiers,
    }
}

/// Parse terminal event into key combo.
pub fn key_combo_from_terminal_key(key: &Key, modifiers: &TerminalKeyModifiers) -> KeyCombo {
    KeyCombo {
        key: terminal_key_to_string(key),
        modifiers: KeyModifiers::from_terminal(modifiers),
    }
}

fn terminal_key_to_string(key: &Key) -> String {
    match key {
        Key::Char(c) => c.to_string(),
        Key::F(n) => format!("F{n}"),
        Key::Backspace => "Backspace".to_string(),
        Key::Enter => "Enter".to_string(),
        Key::Left => "ArrowLeft".to_string(),
        Key::Right => "ArrowRight".to_string(),
        Key::Up => "ArrowUp".to_string(),
        Key::Down => "ArrowDown".to_string(),
        Key::Home => "Home".to_string(),
        Key::End => "End".to_string(),
        Key::PageUp => "PageUp".to_string(),
        Key::PageDown => "PageDown".to_string(),
        Key::Tab => "Tab".to_string(),
        Key::BackTab => "BackTab".to_string(),
        Key::Delete => "Delete".to_string(),
        Key::Insert => "Insert".to_string(),
        Key::Escape => "Escape".to_string(),
        Key::Null => "Null".to_string(),
    }
}

/// Convert combo to normalized string.
pub fn key_combo_to_string(combo: &KeyCombo) -> String {
    let mut parts: Vec<String> = Vec::with_capacity(5);
    if combo.modifiers.ctrl {
        parts.push("ctrl".to_string());
    }
    if combo.modifiers.alt {
        parts.push("alt".to_string());
    }
    if combo.modifiers.shift {
        parts.push("shift".to_string());
    }
    if combo.modifiers.meta {
        parts.push("meta".to_string());
    }
    parts.push(combo.key.to_ascii_lowercase());
    parts.join("+")
}

/// Compare two combos.
pub fn key_combo_equals(a: &KeyCombo, b: &KeyCombo) -> bool {
    a.key.eq_ignore_ascii_case(&b.key) && a.modifiers == b.modifiers
}

/// Convenience helper to validate if an input combo string matches a modifier state.
pub fn combo_from_key_input(key: &str, modifiers: &KeyModifiers) -> bool {
    let combo = parse_key_combo(key);
    combo.modifiers == *modifiers
}

// ============================================================================
// Registry
// ============================================================================

/// Binding ID counter used for deterministic test output.
static BINDING_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

fn next_binding_id() -> String {
    format!(
        "binding_{}",
        BINDING_ID_COUNTER.fetch_add(1, Ordering::SeqCst) + 1
    )
}

/// Registry for keyboard shortcuts.
pub struct KeyBindingRegistry {
    bindings: HashMap<String, KeyBinding>,
    context_stack: Vec<String>,
    mode: KeyMode,
    mode_bindings: HashMap<KeyMode, Vec<String>>,
}

impl Default for KeyBindingRegistry {
    fn default() -> Self {
        let mut registry = Self {
            bindings: HashMap::new(),
            context_stack: vec!["global".to_string()],
            mode: KeyMode::Default,
            mode_bindings: HashMap::new(),
        };
        registry.mode_bindings.insert(KeyMode::Vim, Vec::new());
        registry.mode_bindings.insert(KeyMode::Emacs, Vec::new());
        registry
    }
}

impl KeyBindingRegistry {
    /// Register a binding and return its id.
    pub fn register(&mut self, options: KeyBindingOptions) -> String {
        let combo = parse_key_combo(&options.key);
        let id = next_binding_id();

        let binding = KeyBinding {
            id: id.clone(),
            key: options.key,
            combo,
            action: options.action,
            description: options.description,
            context: options.context.unwrap_or_else(|| "global".to_string()),
            priority: options.priority.unwrap_or(0),
            enabled: true,
            command_id: options.command_id,
        };

        #[cfg(not(test))]
        {
            let _ = self.find_conflicts(&binding);
        }

        self.bindings.insert(id.clone(), binding);
        id
    }

    /// Remove a binding.
    pub fn unregister(&mut self, id: &str) -> bool {
        if self.bindings.remove(id).is_none() {
            return false;
        }

        for ids in self.mode_bindings.values_mut() {
            ids.retain(|stored| stored != id);
        }
        true
    }

    /// Enable/disable a binding by id.
    pub fn set_enabled(&mut self, id: &str, enabled: bool) {
        if let Some(binding) = self.bindings.get_mut(id) {
            binding.enabled = enabled;
        }
    }

    /// Get a binding by id.
    pub fn get(&self, id: &str) -> Option<&KeyBinding> {
        self.bindings.get(id)
    }

    /// List all bindings.
    pub fn get_all(&self) -> Vec<KeyBinding> {
        self.bindings.values().cloned().collect()
    }

    /// List all bindings for a context.
    pub fn get_by_context(&self, context: &str) -> Vec<KeyBinding> {
        self.bindings
            .values()
            .filter(|binding| binding.context == context && binding.enabled)
            .cloned()
            .collect()
    }

    /// Resolve best match for a key combo.
    pub fn resolve(&self, combo: &KeyCombo) -> Option<KeyBinding> {
        let mut candidates: Vec<KeyBinding> = Vec::new();

        for context in &self.context_stack {
            for binding in self.get_by_context(context) {
                if key_combo_equals(&binding.combo, combo) {
                    candidates.push(binding);
                }
            }
        }

        if candidates.is_empty() {
            return None;
        }

        candidates.sort_by(|a, b| {
            let a_ctx = self
                .context_stack
                .iter()
                .position(|ctx| ctx == &a.context)
                .unwrap_or(0);
            let b_ctx = self
                .context_stack
                .iter()
                .position(|ctx| ctx == &b.context)
                .unwrap_or(0);

            if a_ctx != b_ctx {
                return b_ctx.cmp(&a_ctx);
            }
            b.priority.cmp(&a.priority)
        });

        candidates.into_iter().next()
    }

    /// Handle one key string event.
    pub fn handle(&mut self, key: &str, modifiers: &KeyModifiers) -> bool {
        let combo = KeyCombo {
            key: normalize_key(key),
            modifiers: *modifiers,
        };

        if let Some(binding) = self.resolve(&combo) {
            (binding.action)();
            return true;
        }

        false
    }

    /// Handle terminal key events.
    pub fn handle_terminal(&mut self, key: &Key, modifiers: &TerminalKeyModifiers) -> bool {
        let combo = key_combo_from_terminal_key(key, modifiers);
        if let Some(binding) = self.resolve(&combo) {
            (binding.action)();
            return true;
        }
        false
    }

    /// Enter a context.
    pub fn push_context(&mut self, context: impl Into<String>) {
        self.context_stack.push(context.into());
    }

    /// Leave current context.
    pub fn pop_context(&mut self) -> Option<String> {
        if self.context_stack.len() > 1 {
            self.context_stack.pop()
        } else {
            None
        }
    }

    /// Current context stack copy.
    pub fn get_context_stack(&self) -> Vec<String> {
        self.context_stack.clone()
    }

    /// Current context (top of stack).
    pub fn get_current_context(&self) -> &str {
        self.context_stack
            .last()
            .map(String::as_str)
            .unwrap_or("global")
    }

    /// Conflicts for one binding.
    pub fn find_conflicts(&self, binding: &KeyBinding) -> Vec<KeyBinding> {
        self.bindings
            .values()
            .filter(|candidate| {
                candidate.context == binding.context
                    && candidate.enabled
                    && candidate.id != binding.id
                    && key_combo_equals(&candidate.combo, &binding.combo)
            })
            .cloned()
            .collect()
    }

    /// All conflicts grouped by context+combo.
    pub fn get_all_conflicts(&self) -> Vec<KeyConflict> {
        let mut map: HashMap<String, Vec<KeyBinding>> = HashMap::new();

        for binding in self.bindings.values() {
            if !binding.enabled {
                continue;
            }
            let key = format!(
                "{}:{}",
                binding.context,
                key_combo_to_string(&binding.combo)
            );
            map.entry(key).or_default().push(binding.clone());
        }

        let mut conflicts = Vec::new();
        for (composed, list) in map {
            if list.len() > 1 {
                let mut parts = composed.splitn(2, ':');
                conflicts.push(KeyConflict {
                    context: parts.next().unwrap_or_default().to_string(),
                    key: parts.next().unwrap_or_default().to_string(),
                    bindings: list,
                });
            }
        }

        conflicts
    }

    /// Current mode.
    pub fn get_mode(&self) -> KeyMode {
        self.mode
    }

    /// Set interaction mode.
    pub fn set_mode(&mut self, mode: KeyMode) {
        if self.mode == mode {
            return;
        }

        let previous = self.mode;
        if let Some(previous_bindings) = self.mode_bindings.get(&previous).cloned() {
            for binding_id in previous_bindings {
                let _ = self.unregister(&binding_id);
            }
            if let Some(target) = self.mode_bindings.get_mut(&previous) {
                target.clear();
            }
        }

        self.mode = mode;
        match mode {
            KeyMode::Default => {}
            KeyMode::Vim => self.enable_vim_bindings(),
            KeyMode::Emacs => self.enable_emacs_bindings(),
        }
    }

    fn enable_vim_bindings(&mut self) {
        let defs = [
            ("j", "Move down", 10),
            ("k", "Move up", 10),
            ("h", "Move left", 9),
            ("l", "Move right", 9),
            ("g", "Go to first", 8),
            ("shift+g", "Go to last", 8),
            ("/", "Search", 7),
            ("n", "Next match", 7),
            ("shift+n", "Previous match", 7),
        ];

        let ids: Vec<String> = defs
            .into_iter()
            .map(|(key, description, priority)| {
                self.register(KeyBindingOptions {
                    key: key.to_string(),
                    action: Arc::new(|| {}),
                    description: Some(description.to_string()),
                    context: Some("vim-nav".to_string()),
                    priority: Some(priority),
                    command_id: None,
                })
            })
            .collect();
        self.mode_bindings.insert(KeyMode::Vim, ids);
    }

    fn enable_emacs_bindings(&mut self) {
        let defs = [
            ("ctrl+n", "Move down", 10),
            ("ctrl+p", "Move up", 10),
            ("ctrl+b", "Move left", 9),
            ("ctrl+f", "Move right", 9),
            ("ctrl+a", "Start of line", 8),
            ("ctrl+e", "End of line", 8),
            ("ctrl+s", "Search forward", 7),
            ("ctrl+r", "Search backward", 7),
            ("ctrl+w", "Delete previous word", 7),
        ];

        let ids: Vec<String> = defs
            .into_iter()
            .map(|(key, description, priority)| {
                self.register(KeyBindingOptions {
                    key: key.to_string(),
                    action: Arc::new(|| {}),
                    description: Some(description.to_string()),
                    context: Some("emacs-nav".to_string()),
                    priority: Some(priority),
                    command_id: None,
                })
            })
            .collect();
        self.mode_bindings.insert(KeyMode::Emacs, ids);
    }

    /// Clear registry.
    pub fn clear(&mut self) {
        self.bindings.clear();
        self.context_stack.clear();
        self.context_stack.push("global".to_string());
        self.mode = KeyMode::Default;
        self.mode_bindings.insert(KeyMode::Vim, Vec::new());
        self.mode_bindings.insert(KeyMode::Emacs, Vec::new());
    }
}

// ============================================================================
// Global Registry
// ============================================================================

thread_local! {
    static KEY_BINDING_REGISTRY: RefCell<KeyBindingRegistry> =
        RefCell::new(KeyBindingRegistry::default());
}

/// Access the global registry.
pub fn get_key_binding_registry() -> &'static std::thread::LocalKey<RefCell<KeyBindingRegistry>> {
    &KEY_BINDING_REGISTRY
}

/// Reset global registry and ID counter.
pub fn reset_key_binding_registry() {
    BINDING_ID_COUNTER.store(0, Ordering::SeqCst);
    KEY_BINDING_REGISTRY.with(|registry| *registry.borrow_mut() = KeyBindingRegistry::default());
}

/// Register in global registry.
pub fn register_key_binding(options: KeyBindingOptions) -> String {
    KEY_BINDING_REGISTRY.with(|registry| registry.borrow_mut().register(options))
}

/// Remove from global registry.
pub fn unregister_key_binding(id: &str) -> bool {
    KEY_BINDING_REGISTRY.with(|registry| registry.borrow_mut().unregister(id))
}

/// Resolve and execute from key string.
pub fn handle_key(combo: &str, modifiers: KeyModifiers) -> bool {
    KEY_BINDING_REGISTRY.with(|registry| registry.borrow_mut().handle(combo, &modifiers))
}

/// Resolve and execute from terminal event.
pub fn handle_terminal_key(key: &Key, modifiers: &TerminalKeyModifiers) -> bool {
    KEY_BINDING_REGISTRY.with(|registry| registry.borrow_mut().handle_terminal(key, modifiers))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::AtomicBool;

    #[test]
    fn parse_combo() {
        let combo = parse_key_combo("ctrl+shift+k");
        assert_eq!(combo.key, "k");
        assert!(combo.modifiers.ctrl);
        assert!(combo.modifiers.shift);
    }

    #[test]
    fn register_resolve_and_handle() {
        reset_key_binding_registry();

        let called = Arc::new(AtomicBool::new(false));
        register_key_binding(KeyBindingOptions {
            key: "ctrl+s".to_string(),
            action: {
                let called = called.clone();
                Arc::new(move || {
                    called.store(true, Ordering::SeqCst);
                })
            },
            description: Some("Save".to_string()),
            context: Some("global".to_string()),
            priority: Some(1),
            command_id: None,
        });

        let flag = Arc::new(AtomicBool::new(false));
        register_key_binding(KeyBindingOptions {
            key: "s".to_string(),
            action: {
                let flag = flag.clone();
                Arc::new(move || {
                    flag.store(true, Ordering::SeqCst);
                })
            },
            description: None,
            context: Some("local".to_string()),
            priority: Some(1),
            command_id: None,
        });

        let handled = handle_key(
            "s",
            KeyModifiers {
                ctrl: true,
                alt: false,
                shift: false,
                meta: false,
            },
        );

        assert!(handled);
        assert!(called.load(Ordering::SeqCst));
        assert!(!flag.load(Ordering::SeqCst));
    }

    #[test]
    fn mode_binding_registration() {
        reset_key_binding_registry();
        KEY_BINDING_REGISTRY.with(|registry| {
            let mut registry = registry.borrow_mut();
            registry.set_mode(KeyMode::Vim);
            assert_eq!(registry.get_mode(), KeyMode::Vim);
            assert!(
                registry
                    .mode_bindings
                    .get(&KeyMode::Vim)
                    .unwrap_or(&Vec::new())
                    .len()
                    > 0
            );
        });
    }
}
