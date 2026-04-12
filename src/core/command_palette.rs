//! Command Palette Core Utilities.
#![allow(non_snake_case)]

use crate::core::key_bindings::{self, KeyBindingOptions};
use futures::executor::block_on;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::future::Future;
use std::panic::{self, AssertUnwindSafe};
use std::pin::Pin;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::sync::Mutex;

const DEFAULT_MAX_RESULTS: usize = 50;

// ============================================================================
// Types
// ============================================================================

/// A command available in the palette.
#[derive(Clone)]
pub struct Command {
    /// Unique identifier.
    pub id: String,
    /// Main label.
    pub label: String,
    /// Optional description.
    pub description: Option<String>,
    /// Optional category.
    pub category: Option<String>,
    /// Optional shortcut hint.
    pub keybinding: Option<String>,
    /// Callback.
    pub action: CommandAction,
    /// Async callback.
    pub async_action: Option<CommandAsyncAction>,
    /// Enabled state.
    pub enabled: bool,
    /// Optional icon.
    pub icon: Option<String>,
    /// Search terms.
    pub tags: Vec<String>,
    /// Hide this command from recent list.
    pub hide_from_recent: bool,
}

impl fmt::Debug for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Command")
            .field("id", &self.id)
            .field("label", &self.label)
            .field("description", &self.description)
            .field("category", &self.category)
            .field("keybinding", &self.keybinding)
            .field("enabled", &self.enabled)
            .field("icon", &self.icon)
            .field("tags", &self.tags)
            .field("hide_from_recent", &self.hide_from_recent)
            .finish()
    }
}

/// Command callback type.
pub type CommandAction = Arc<dyn Fn() + Send + Sync + 'static>;
/// Async command callback type.
pub type CommandAsyncAction =
    Arc<dyn Fn() -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync + 'static>;

type RegistryListener = Box<dyn Fn() + Send + Sync + 'static>;
/// Unsubscribe callback for registry listeners.
pub type RegistryUnsubscribe = Box<dyn Fn() + Send + 'static>;

impl Default for Command {
    fn default() -> Self {
        Self {
            id: String::new(),
            label: String::new(),
            description: None,
            category: None,
            keybinding: None,
            action: Arc::new(|| {}),
            async_action: None,
            enabled: true,
            icon: None,
            tags: Vec::new(),
            hide_from_recent: false,
        }
    }
}

/// Register options.
#[derive(Clone)]
pub struct CommandOptions {
    /// Optional explicit id.
    pub id: Option<String>,
    /// Label.
    pub label: String,
    /// Optional description.
    pub description: Option<String>,
    /// Optional category.
    pub category: Option<String>,
    /// Optional keybinding.
    pub keybinding: Option<String>,
    /// Optional action.
    pub action: Option<CommandAction>,
    /// Optional async action.
    pub async_action: Option<CommandAsyncAction>,
    /// Enabled by default.
    pub enabled: bool,
    /// Optional icon.
    pub icon: Option<String>,
    /// Search tags.
    pub tags: Vec<String>,
    /// Hide from recent history.
    pub hide_from_recent: bool,
}

impl Default for CommandOptions {
    fn default() -> Self {
        Self {
            id: None,
            label: String::new(),
            description: None,
            category: None,
            keybinding: None,
            action: None,
            async_action: None,
            enabled: true,
            icon: None,
            tags: Vec::new(),
            hide_from_recent: false,
        }
    }
}

/// Fuzzy result.
#[derive(Debug, Clone)]
pub struct FuzzyMatch {
    /// Matched command.
    pub command: Command,
    /// Match score.
    pub score: f64,
    /// Matched character positions.
    pub matches: Vec<usize>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct HighlightSegment {
    pub text: String,
    pub highlight: bool,
}

#[derive(Clone)]
struct FuzzyMatchInternal {
    score: f64,
    matches: Vec<usize>,
}

// ============================================================================
// Fuzzy matching
// ============================================================================

fn fuzzy_match_internal(query: &str, text: &str) -> Option<FuzzyMatchInternal> {
    const SCORE_CONSECUTIVE: f64 = 15.0;
    const SCORE_WORD_START: f64 = 10.0;
    const SCORE_FIRST_CHAR: f64 = 8.0;
    const SCORE_SEPARATOR: f64 = 5.0;
    const SCORE_CASE_MATCH: f64 = 1.0;
    const PENALTY_DISTANCE: f64 = 1.0;

    if query.is_empty() {
        return Some(FuzzyMatchInternal {
            score: 0.0,
            matches: Vec::new(),
        });
    }

    let q_lower = query.to_lowercase();
    let q_chars: Vec<char> = q_lower.chars().collect();
    let q_chars_exact: Vec<char> = query.chars().collect();
    let text_lower = text.to_lowercase();
    let text_chars: Vec<char> = text_lower.chars().collect();
    let original_chars: Vec<char> = text.chars().collect();

    let mut pattern_pos = 0usize;
    for ch in text_chars.iter().copied() {
        if pattern_pos < q_chars.len() && ch == q_chars[pattern_pos] {
            pattern_pos += 1;
        }
    }
    if pattern_pos != q_chars.len() {
        return None;
    }

    let mut idx = 0usize;
    let mut score = 0f64;
    let mut prev: Option<usize> = None;
    let mut positions = Vec::new();
    let is_separator = |c: char| matches!(c, ' ' | '-' | '_' | '.' | '/' | ':' | '\\');

    for (char_index, ch) in text_chars.iter().copied().enumerate() {
        if idx >= q_chars.len() {
            break;
        }

        if Some(ch) == q_chars.get(idx).copied() {
            positions.push(char_index);
            score += 1.0;
            if let Some(prev_pos) = prev {
                if prev_pos + 1 == char_index {
                    score += SCORE_CONSECUTIVE;
                } else {
                    score -= (char_index - prev_pos - 1) as f64 * PENALTY_DISTANCE;
                }
            }
            prev = Some(char_index);
            idx += 1;

            if char_index == 0 {
                score += SCORE_FIRST_CHAR;
            } else if let Some(previous_char) = original_chars.get(char_index.saturating_sub(1)) {
                if is_separator(*previous_char) {
                    score += SCORE_WORD_START;
                }
            }

            if let Some(current_char) = original_chars.get(char_index) {
                if is_separator(*current_char) {
                    score += SCORE_SEPARATOR;
                }
            }

            if idx > 0 {
                if let Some(pattern_char) = q_chars_exact.get(idx.saturating_sub(1)) {
                    if ch.eq_ignore_ascii_case(pattern_char) {
                        score += SCORE_CASE_MATCH;
                    }
                }
            }
        }
    }

    if idx == q_chars.len() {
        score += (50i32 - text.chars().count() as i32).max(0) as f64;
        Some(FuzzyMatchInternal {
            score,
            matches: positions,
        })
    } else {
        None
    }
}

fn command_match_score(query: &str, command: &Command) -> Option<FuzzyMatchInternal> {
    let mut best: Option<FuzzyMatchInternal> = None;
    if !command.enabled {
        return None;
    }

    if let Some(base) = fuzzy_match_internal(query, command.label.as_str()) {
        best = Some(base);
    }

    if let Some(category) = command.category.as_deref() {
        let mut category_label = String::with_capacity(category.len() + command.label.len() + 2);
        category_label.push_str(category);
        category_label.push_str(": ");
        category_label.push_str(&command.label);

        if let Some(mut match_result) = fuzzy_match_internal(query, category_label.as_str()) {
            match_result.score *= 0.8;
            best = Some(match best {
                Some(current) if current.score >= match_result.score => current,
                _ => match_result,
            });
        }
    }

    for tag in &command.tags {
        if let Some(mut match_result) = fuzzy_match_internal(query, tag.as_str()) {
            match_result.score *= 0.6;
            best = Some(match best {
                Some(current) if current.score >= match_result.score => current,
                _ => match_result,
            });
            break;
        }
    }

    if let Some(desc) = command.description.as_deref() {
        if let Some(mut match_result) = fuzzy_match_internal(query, desc) {
            match_result.score *= 0.5;
            best = Some(match best {
                Some(current) if current.score >= match_result.score => current,
                _ => match_result,
            });
        }
    }

    best
}

/// Search command list with simple fuzzy matching.
pub fn search_commands(commands: &[Command], query: &str, max_results: usize) -> Vec<FuzzyMatch> {
    if query.is_empty() {
        let mut direct: Vec<FuzzyMatch> = commands
            .iter()
            .filter(|cmd| cmd.enabled)
            .map(|cmd| FuzzyMatch {
                command: cmd.clone(),
                score: 0.0,
                matches: Vec::new(),
            })
            .collect();
        direct.truncate(max_results);
        return direct;
    }

    let mut results: Vec<FuzzyMatch> = Vec::new();
    for command in commands.iter().filter(|cmd| cmd.enabled) {
        if let Some(m) = command_match_score(query, command) {
            results.push(FuzzyMatch {
                command: command.clone(),
                score: m.score,
                matches: m.matches,
            });
        }
    }
    results.sort_by(|a, b| b.score.total_cmp(&a.score));
    results.truncate(max_results);
    results
}

/// JS-compatible alias for [`search_commands`].
pub fn searchCommands(commands: &[Command], query: &str, max_results: usize) -> Vec<FuzzyMatch> {
    search_commands(commands, query, max_results)
}

/// Search commands with default max results.
pub fn search_commands_default(commands: &[Command], query: &str) -> Vec<FuzzyMatch> {
    search_commands(commands, query, DEFAULT_MAX_RESULTS)
}

/// Fuzzy match pattern against text and return score/matches.
pub fn fuzzy_match(query: &str, text: &str) -> Option<FuzzyMatch> {
    fuzzy_match_internal(query, text).map(|result| FuzzyMatch {
        command: Command::default(),
        score: result.score,
        matches: result.matches,
    })
}

/// JS-compatible alias for [`fuzzy_match`].
pub fn fuzzyMatch(query: &str, text: &str) -> Option<FuzzyMatch> {
    fuzzy_match(query, text)
}

/// JS-compatible alias for [`search_commands_default`].
pub fn searchCommandsDefault(commands: &[Command], query: &str) -> Vec<FuzzyMatch> {
    search_commands_default(commands, query)
}

// ============================================================================
// Registry
// ============================================================================

static COMMAND_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

fn next_command_id() -> String {
    let id = COMMAND_ID_COUNTER.fetch_add(1, Ordering::SeqCst) + 1;
    format!("command_{id}")
}

/// Command registry.
pub struct CommandRegistry {
    commands: HashMap<String, Command>,
    recent: Vec<String>,
    max_recent: usize,
    listeners: HashMap<usize, RegistryListener>,
    next_listener_id: usize,
}

impl Default for CommandRegistry {
    fn default() -> Self {
        Self {
            commands: HashMap::new(),
            recent: Vec::new(),
            max_recent: 10,
            listeners: HashMap::new(),
            next_listener_id: 0,
        }
    }
}

impl CommandRegistry {
    fn notify_listeners(&self) {
        for listener in self.listeners.values() {
            listener();
        }
    }

    /// Register a command and return its id.
    pub fn register(&mut self, options: CommandOptions) -> String {
        let id = options.id.unwrap_or_else(next_command_id);
        let action = options.action.unwrap_or_else(|| Arc::new(|| {}));
        let async_action = options.async_action;

        if let Some(ref binding) = options.keybinding {
            let command_id = id.clone();
            key_bindings::register_key_binding(KeyBindingOptions {
                key: binding.clone(),
                action: Arc::new(move || {
                    crate::core::command_palette::execute_command(&command_id);
                }),
                description: Some(options.label.clone()),
                context: Some("global".to_string()),
                priority: Some(100),
                command_id: Some(id.clone()),
            });
        }

        let command = Command {
            id: id.clone(),
            label: options.label,
            description: options.description,
            category: options.category,
            keybinding: options.keybinding,
            action,
            async_action,
            enabled: options.enabled,
            icon: options.icon,
            tags: options.tags,
            hide_from_recent: options.hide_from_recent,
        };
        self.commands.insert(id.clone(), command);
        self.notify_listeners();
        id
    }

    /// Unregister by id.
    pub fn unregister(&mut self, id: &str) -> bool {
        if self.commands.remove(id).is_none() {
            return false;
        }
        self.recent.retain(|value| value != id);
        self.notify_listeners();
        true
    }

    /// List all commands.
    pub fn all(&self) -> Vec<Command> {
        self.commands.values().cloned().collect()
    }

    /// Alias for JS parity.
    pub fn get_all(&self) -> Vec<Command> {
        self.all()
    }

    /// Get one command.
    pub fn get(&self, id: &str) -> Option<&Command> {
        self.commands.get(id)
    }

    /// Search.
    pub fn search(&self, query: &str, max_results: usize) -> Vec<FuzzyMatch> {
        search_commands(
            &self.commands.values().cloned().collect::<Vec<_>>(),
            query,
            max_results,
        )
    }

    /// Search using default max results.
    pub fn search_default(&self, query: &str) -> Vec<FuzzyMatch> {
        self.search(query, DEFAULT_MAX_RESULTS)
    }

    /// Execute command by id.
    pub fn execute(&mut self, id: &str) -> bool {
        if let Some(command) = self.commands.get(id) {
            if !command.enabled {
                return false;
            }
            let executed = if let Some(action) = command.async_action.as_ref() {
                panic::catch_unwind(AssertUnwindSafe(|| block_on(action()))).is_ok()
            } else {
                panic::catch_unwind(AssertUnwindSafe(|| (command.action)())).is_ok()
            };
            if !executed {
                return false;
            }
            if !command.hide_from_recent {
                self.recent.retain(|value| value != id);
                self.recent.insert(0, id.to_string());
                if self.recent.len() > self.max_recent {
                    self.recent.truncate(self.max_recent);
                }
            }
            true
        } else {
            false
        }
    }

    /// Set enabled state.
    pub fn set_enabled(&mut self, id: &str, enabled: bool) -> bool {
        if let Some(command) = self.commands.get_mut(id) {
            command.enabled = enabled;
            self.notify_listeners();
            true
        } else {
            false
        }
    }

    /// Subscribe to registry changes.
    pub fn subscribe<F>(&mut self, listener: F) -> usize
    where
        F: Fn() + Send + Sync + 'static,
    {
        let id = self.next_listener_id;
        self.next_listener_id = self.next_listener_id.checked_add(1).unwrap_or(0);
        self.listeners.insert(id, Box::new(listener));
        id
    }

    /// Remove a listener.
    pub fn unsubscribe(&mut self, id: usize) -> bool {
        self.listeners.remove(&id).is_some()
    }

    /// Recent commands.
    pub fn recent(&self) -> Vec<Command> {
        self.recent
            .iter()
            .filter_map(|id| self.commands.get(id).filter(|cmd| cmd.enabled).cloned())
            .collect()
    }

    /// Alias for JS parity.
    pub fn get_recent(&self) -> Vec<Command> {
        self.recent()
    }

    /// Commands by category.
    pub fn by_category(&self, category: &str) -> Vec<Command> {
        self.commands
            .values()
            .filter(|cmd| cmd.category.as_deref() == Some(category))
            .cloned()
            .collect()
    }

    /// Alias with a more explicit external naming style.
    pub fn get_by_category(&self, category: &str) -> Vec<Command> {
        self.by_category(category)
    }

    /// Unique category names.
    pub fn categories(&self) -> Vec<String> {
        let mut unique: HashSet<String> = HashSet::new();
        for cmd in self.commands.values() {
            if let Some(category) = cmd.category.as_deref() {
                unique.insert(category.to_string());
            }
        }
        let mut categories = unique.into_iter().collect::<Vec<_>>();
        categories.sort_unstable();
        categories
    }

    /// Alias with a more explicit external naming style.
    pub fn get_categories(&self) -> Vec<String> {
        self.categories()
    }

    /// Current size.
    pub fn size(&self) -> usize {
        self.commands.len()
    }

    /// Clear registry.
    pub fn clear(&mut self) {
        self.commands.clear();
        self.recent.clear();
        self.notify_listeners();
    }

    /// Clear recent list.
    pub fn clear_recent(&mut self) {
        self.recent.clear();
    }

    /// Set max recent.
    pub fn set_max_recent(&mut self, max: usize) {
        self.max_recent = max;
        if self.recent.len() > max {
            self.recent.truncate(max);
        }
    }
}

thread_local! {
    static COMMAND_REGISTRY: std::cell::RefCell<CommandRegistry> =
        std::cell::RefCell::new(CommandRegistry::default());
}

/// Reset global registry.
pub fn reset_command_registry() {
    COMMAND_ID_COUNTER.store(0, Ordering::SeqCst);
    COMMAND_REGISTRY.with(|registry| *registry.borrow_mut() = CommandRegistry::default());
}

/// JS-compatible alias for [`reset_command_registry`].
pub fn resetCommandRegistry() {
    reset_command_registry();
}

/// Reset only the global command id counter.
pub fn reset_command_id_counter() {
    COMMAND_ID_COUNTER.store(0, Ordering::SeqCst);
}

/// JS-compatible alias for [`reset_command_id_counter`].
pub fn resetCommandIdCounter() {
    reset_command_id_counter();
}

/// Register in global registry.
pub fn register_command(options: CommandOptions) -> String {
    COMMAND_REGISTRY.with(|registry| registry.borrow_mut().register(options))
}

/// JS-compatible alias for [`register_command`].
pub fn registerCommand(options: CommandOptions) -> String {
    register_command(options)
}

/// Remove from global registry.
pub fn unregister_command(id: &str) -> bool {
    COMMAND_REGISTRY.with(|registry| registry.borrow_mut().unregister(id))
}

/// JS-compatible alias for [`unregister_command`].
pub fn unregisterCommand(id: &str) -> bool {
    unregister_command(id)
}

/// Execute command by id from global registry.
pub fn execute_command(id: &str) -> bool {
    COMMAND_REGISTRY.with(|registry| registry.borrow_mut().execute(id))
}

/// JS-compatible alias for [`execute_command`].
pub fn executeCommand(id: &str) -> bool {
    execute_command(id)
}

/// Search in global registry.
pub fn search_global_commands(query: &str, max_results: usize) -> Vec<FuzzyMatch> {
    COMMAND_REGISTRY.with(|registry| registry.borrow().search(query, max_results))
}

/// JS-compatible alias for [`search_global_commands`].
pub fn searchGlobalCommands(query: &str, max_results: usize) -> Vec<FuzzyMatch> {
    search_global_commands(query, max_results)
}

/// Search in global registry using default max results.
pub fn search_global_commands_default(query: &str) -> Vec<FuzzyMatch> {
    COMMAND_REGISTRY.with(|registry| registry.borrow().search_default(query))
}

/// JS-compatible alias for [`search_global_commands_default`].
pub fn searchGlobalCommandsDefault(query: &str) -> Vec<FuzzyMatch> {
    search_global_commands_default(query)
}

/// Get the global command registry handle.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct CommandRegistryHandle;

impl CommandRegistryHandle {
    /// Register a command and return its id.
    pub fn register(&self, options: CommandOptions) -> String {
        register_command(options)
    }

    /// Unregister a command.
    pub fn unregister(&self, id: &str) -> bool {
        unregister_command(id)
    }

    /// Execute a command by id.
    pub fn execute(&self, id: &str) -> bool {
        execute_command(id)
    }

    /// Search commands with optional max-results.
    pub fn search(&self, query: &str, max_results: usize) -> Vec<FuzzyMatch> {
        search_global_commands(query, max_results)
    }

    /// Search commands with default max results.
    pub fn search_default(&self, query: &str) -> Vec<FuzzyMatch> {
        search_global_commands_default(query)
    }

    /// Get a command by id.
    pub fn get(&self, id: &str) -> Option<Command> {
        COMMAND_REGISTRY.with(|registry| registry.borrow().get(id).cloned())
    }

    /// Get all registered commands.
    pub fn get_all(&self) -> Vec<Command> {
        COMMAND_REGISTRY.with(|registry| registry.borrow().all())
    }

    /// Get commands by category.
    pub fn get_by_category(&self, category: &str) -> Vec<Command> {
        COMMAND_REGISTRY.with(|registry| registry.borrow().get_by_category(category))
    }

    /// Get sorted unique categories.
    pub fn get_categories(&self) -> Vec<String> {
        COMMAND_REGISTRY.with(|registry| registry.borrow().get_categories())
    }

    /// Get recent commands.
    pub fn get_recent(&self) -> Vec<Command> {
        COMMAND_REGISTRY.with(|registry| registry.borrow().get_recent())
    }

    /// Execute and register callback for registry changes.
    pub fn set_enabled(&self, id: &str, enabled: bool) -> bool {
        COMMAND_REGISTRY.with(|registry| registry.borrow_mut().set_enabled(id, enabled))
    }

    /// Clear all commands.
    pub fn clear(&self) {
        COMMAND_REGISTRY.with(|registry| registry.borrow_mut().clear())
    }

    /// Clear recent list.
    pub fn clear_recent(&self) {
        COMMAND_REGISTRY.with(|registry| registry.borrow_mut().clear_recent())
    }

    /// Set max recent commands.
    pub fn set_max_recent(&self, max: usize) {
        COMMAND_REGISTRY.with(|registry| registry.borrow_mut().set_max_recent(max))
    }

    /// Register a listener and return an id.
    pub fn subscribe<F>(&self, listener: F) -> usize
    where
        F: Fn() + Send + Sync + 'static,
    {
        COMMAND_REGISTRY.with(|registry| registry.borrow_mut().subscribe(listener))
    }

    /// Register a listener and return an unsubscribe callback (JS-style API).
    pub fn subscribe_and_return_unsubscribe<F>(&self, listener: F) -> RegistryUnsubscribe
    where
        F: Fn() + Send + Sync + 'static,
    {
        let id = COMMAND_REGISTRY.with(|registry| registry.borrow_mut().subscribe(listener));
        Box::new(move || {
            COMMAND_REGISTRY.with(|registry| {
                let _ = registry.borrow_mut().unsubscribe(id);
            });
        })
    }

    /// Remove a listener by id.
    pub fn unsubscribe(&self, id: usize) -> bool {
        COMMAND_REGISTRY.with(|registry| registry.borrow_mut().unsubscribe(id))
    }

    /// Number of registered commands.
    pub fn size(&self) -> usize {
        COMMAND_REGISTRY.with(|registry| registry.borrow().size())
    }
}

/// Return the global command registry handle (same as JS `getCommandRegistry` API).
pub fn get_command_registry() -> CommandRegistryHandle {
    CommandRegistryHandle
}

/// JS-compatible alias for [`get_command_registry`].
pub fn getCommandRegistry() -> CommandRegistryHandle {
    get_command_registry()
}

/// Subscribe to global registry changes.
pub fn subscribe<F>(listener: F) -> usize
where
    F: Fn() + Send + Sync + 'static,
{
    COMMAND_REGISTRY.with(|registry| registry.borrow_mut().subscribe(listener))
}

/// JS-compatible alias for [`subscribe`].
pub fn subscribeListener<F>(listener: F) -> usize
where
    F: Fn() + Send + Sync + 'static,
{
    subscribe(listener)
}

/// Subscribe to global registry changes and return an unsubscribe callback.
pub fn subscribe_and_return_unsubscribe<F>(listener: F) -> RegistryUnsubscribe
where
    F: Fn() + Send + Sync + 'static,
{
    let id = subscribe(listener);
    Box::new(move || {
        let _ = unsubscribe(id);
    })
}

/// JS-compatible alias for [`subscribe_and_return_unsubscribe`].
pub fn subscribeAndReturnUnsubscribe<F>(listener: F) -> RegistryUnsubscribe
where
    F: Fn() + Send + Sync + 'static,
{
    subscribe_and_return_unsubscribe(listener)
}

/// Unsubscribe from global registry changes.
pub fn unsubscribe(listener_id: usize) -> bool {
    COMMAND_REGISTRY.with(|registry| registry.borrow_mut().unsubscribe(listener_id))
}

// ============================================================================
// Palette state
// ============================================================================

/// Options for a palette instance.
#[derive(Default)]
pub struct CommandPaletteOptions {
    /// How many results are returned.
    pub max_results: Option<usize>,
    /// Open palette shortcut.
    pub open_key: Option<String>,
}

/// Palette state handle.
pub struct PaletteState {
    inner: Arc<Mutex<PaletteStateInner>>,
}

#[derive(Debug, Clone)]
struct PaletteStateInner {
    state: CommandPaletteState,
    max_results: usize,
}

/// Runtime state.
#[derive(Debug, Clone)]
pub struct CommandPaletteState {
    /// Open/closed.
    pub is_open: bool,
    /// Current query.
    pub query: String,
    /// Search results.
    pub results: Vec<FuzzyMatch>,
    /// Selected result index.
    pub selected_index: usize,
    /// Optional loading flag.
    pub is_loading: bool,
}

impl PaletteState {
    /// Borrow current state.
    pub fn state(&self) -> CommandPaletteState {
        self.inner.lock().expect("palette state lock").state.clone()
    }

    /// Open and run an empty query.
    pub fn open(&self) {
        let mut inner = self.inner.lock().expect("palette state lock");
        inner.state.is_open = true;
        inner.state.query.clear();
        inner.state.selected_index = 0;
        let max_results = inner.max_results;
        update_results_for_query(&mut inner.state, "", max_results);
    }

    /// Close palette.
    pub fn close(&self) {
        let mut inner = self.inner.lock().expect("palette state lock");
        inner.state.is_open = false;
        inner.state.query.clear();
        inner.state.results.clear();
        inner.state.selected_index = 0;
    }

    /// Toggle state.
    pub fn toggle(&self) {
        let is_open = self.inner.lock().expect("palette state lock").state.is_open;
        if is_open {
            self.close();
        } else {
            self.open();
        }
    }

    /// Set query and recompute results.
    pub fn set_query(&self, query: String) {
        let mut inner = self.inner.lock().expect("palette state lock");
        inner.state.query = query;
        inner.state.selected_index = 0;
        let query = inner.state.query.clone();
        let max_results = inner.max_results;
        update_results_for_query(&mut inner.state, &query, max_results);
    }

    /// Select next.
    pub fn select_next(&self) {
        let mut inner = self.inner.lock().expect("palette state lock");
        if inner.state.results.is_empty() {
            return;
        }
        inner.state.selected_index = if inner.state.selected_index + 1 >= inner.state.results.len()
        {
            inner.state.results.len() - 1
        } else {
            inner.state.selected_index + 1
        };
    }

    /// Select previous.
    pub fn select_previous(&self) {
        let mut inner = self.inner.lock().expect("palette state lock");
        if inner.state.results.is_empty() {
            return;
        }
        inner.state.selected_index = if inner.state.selected_index == 0 {
            0
        } else {
            inner.state.selected_index - 1
        };
    }

    /// Select first item.
    pub fn select_first(&self) {
        let mut inner = self.inner.lock().expect("palette state lock");
        if !inner.state.results.is_empty() {
            inner.state.selected_index = 0;
        }
    }

    /// Select last item.
    pub fn select_last(&self) {
        let mut inner = self.inner.lock().expect("palette state lock");
        if !inner.state.results.is_empty() {
            inner.state.selected_index = inner.state.results.len() - 1;
        }
    }

    /// Execute selected result.
    pub fn execute_selected(&self) -> bool {
        let state = {
            let inner = self.inner.lock().expect("palette state lock");
            inner.state.clone()
        };
        if state.results.is_empty() {
            return false;
        }
        let id = state.results[state.selected_index].command.id.clone();
        self.close();
        execute_command(&id)
    }
}

/// Create palette state.
pub fn create_command_palette_state(options: CommandPaletteOptions) -> PaletteState {
    let open_key = options.open_key.unwrap_or_else(|| "ctrl+k".to_string());

    let inner = Arc::new(Mutex::new(PaletteStateInner {
        state: CommandPaletteState {
            is_open: false,
            query: String::new(),
            results: Vec::new(),
            selected_index: 0,
            is_loading: false,
        },
        max_results: options.max_results.unwrap_or(50),
    }));

    let handle = Arc::clone(&inner);
    let _ = key_bindings::register_key_binding(KeyBindingOptions {
        key: open_key,
        action: Arc::new(move || {
            let palette_state = PaletteState {
                inner: Arc::clone(&handle),
            };
            palette_state.toggle();
        }),
        description: Some("Open Command Palette".to_string()),
        context: Some("global".to_string()),
        priority: Some(100),
        command_id: None,
    });

    PaletteState { inner }
}

/// JS-compatible alias for [`create_command_palette_state`].
pub fn createCommandPaletteState(options: CommandPaletteOptions) -> PaletteState {
    create_command_palette_state(options)
}

fn update_results_for_query(state: &mut CommandPaletteState, query: &str, max_results: usize) {
    if query.trim().is_empty() {
        let recent = COMMAND_REGISTRY.with(|registry| registry.borrow().recent());
        let recent_ids: HashSet<String> = recent.iter().map(|command| command.id.clone()).collect();
        let mut results = Vec::new();

        for command in recent.into_iter().filter(|command| command.enabled) {
            if results.len() >= max_results {
                break;
            }
            results.push(FuzzyMatch {
                command,
                score: 1000.0,
                matches: Vec::new(),
            });
        }

        let other_commands = COMMAND_REGISTRY.with(|registry| {
            let registry = registry.borrow();
            registry
                .all()
                .into_iter()
                .filter(|command| command.enabled)
                .filter(|command| !recent_ids.contains(&command.id))
                .collect::<Vec<_>>()
        });
        let remaining = max_results.saturating_sub(results.len());
        results.extend(
            other_commands
                .into_iter()
                .take(remaining)
                .map(|command| FuzzyMatch {
                    command,
                    score: 0.0,
                    matches: Vec::new(),
                }),
        );

        state.results = results;
    } else {
        let commands = COMMAND_REGISTRY.with(|registry| registry.borrow().all());
        state.results = search_commands(&commands, query, max_results);
    }
}

/// Group by category.
pub fn group_by_category(commands: &[Command]) -> HashMap<String, Vec<Command>> {
    let mut groups: HashMap<String, Vec<Command>> = HashMap::new();
    for command in commands {
        let key = command.category.clone().unwrap_or_default();
        groups.entry(key).or_default().push(command.clone());
    }
    groups
}

/// JS-compatible alias for [`group_by_category`].
pub fn groupByCategory(commands: &[Command]) -> HashMap<String, Vec<Command>> {
    group_by_category(commands)
}

/// Convert fuzzy match indices into highlighted segments.
pub fn highlight_matches(text: &str, matches: &[usize]) -> Vec<HighlightSegment> {
    if matches.is_empty() {
        return vec![HighlightSegment {
            text: text.to_string(),
            highlight: false,
        }];
    }

    let match_set: HashSet<usize> = matches.iter().copied().collect();
    let chars: Vec<char> = text.chars().collect();
    let mut segments = Vec::new();

    let mut cursor = 0usize;
    let mut current_highlight = match_set.contains(&0);
    let mut current: Vec<char> = Vec::new();

    while cursor < chars.len() {
        let is_highlight = match_set.contains(&cursor);
        if is_highlight != current_highlight {
            if !current.is_empty() {
                let text: String = current.iter().collect();
                segments.push(HighlightSegment {
                    text,
                    highlight: current_highlight,
                });
                current.clear();
            }
            current_highlight = is_highlight;
        }
        current.push(chars[cursor]);
        cursor += 1;
    }

    if !current.is_empty() {
        let text: String = current.iter().collect();
        segments.push(HighlightSegment {
            text,
            highlight: current_highlight,
        });
    }

    segments
}

/// JS-compatible alias for [`highlight_matches`].
pub fn highlightMatches(text: &str, matches: &[usize]) -> Vec<HighlightSegment> {
    highlight_matches(text, matches)
}

/// Format command for display.
pub fn format_command(command: &Command) -> String {
    let mut label = command.label.clone();
    if let Some(keybinding) = command.keybinding.as_ref() {
        let combo = key_bindings::parse_key_combo(keybinding);
        let mut parts: Vec<String> = Vec::with_capacity(5);
        if combo.modifiers.ctrl {
            parts.push("Ctrl".to_string());
        }
        if combo.modifiers.meta {
            parts.push("Cmd".to_string());
        }
        if combo.modifiers.alt {
            parts.push("Alt".to_string());
        }
        if combo.modifiers.shift {
            parts.push("Shift".to_string());
        }

        let key = if combo.key.len() == 1 {
            combo.key.to_ascii_uppercase()
        } else {
            combo.key
        };
        parts.push(key);

        label.push_str(" (");
        label.push_str(&parts.join("+"));
        label.push(')');
    }
    label
}

/// JS-compatible alias for [`format_command`].
pub fn formatCommand(command: &Command) -> String {
    format_command(command)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_and_execute() {
        reset_command_registry();
        register_command(CommandOptions {
            id: Some("open".to_string()),
            label: "Open File".to_string(),
            tags: vec!["open".to_string()],
            action: Some(Arc::new(|| {})),
            ..Default::default()
        });

        let palette = create_command_palette_state(CommandPaletteOptions::default());
        palette.open();
        palette.set_query("open".to_string());
        assert_eq!(palette.state().results.len(), 1);
        assert!(palette.execute_selected());
    }

    #[test]
    fn registry_executes_async_action() {
        let mut registry = CommandRegistry::default();
        let flag = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let inner_flag = Arc::clone(&flag);

        let _ = registry.register(CommandOptions {
            id: Some("async".to_string()),
            label: "Async Command".to_string(),
            async_action: Some(Arc::new(move || {
                let inner_flag = Arc::clone(&inner_flag);
                Box::pin(async move {
                    inner_flag.store(true, Ordering::SeqCst);
                })
            })),
            ..Default::default()
        });

        assert!(registry.execute("async"));
        assert!(flag.load(Ordering::SeqCst));
    }

    #[test]
    fn palette_navigation_without_wrapping() {
        reset_command_registry();
        let _ = register_command(CommandOptions {
            id: Some("a".to_string()),
            label: "A".to_string(),
            ..Default::default()
        });
        let _ = register_command(CommandOptions {
            id: Some("b".to_string()),
            label: "B".to_string(),
            ..Default::default()
        });
        let _ = register_command(CommandOptions {
            id: Some("c".to_string()),
            label: "C".to_string(),
            ..Default::default()
        });

        let palette = create_command_palette_state(CommandPaletteOptions::default());
        palette.open();
        assert_eq!(palette.state().results.len(), 3);

        palette.select_previous();
        assert_eq!(palette.state().selected_index, 0);

        palette.select_last();
        assert_eq!(palette.state().selected_index, 2);

        palette.select_next();
        assert_eq!(palette.state().selected_index, 2);

        palette.select_first();
        assert_eq!(palette.state().selected_index, 0);
    }

    #[test]
    fn registry_recent_alias_matches() {
        let mut registry = CommandRegistry::default();
        let _ = registry.register(CommandOptions {
            id: Some("cmd-a".to_string()),
            label: "Command A".to_string(),
            ..Default::default()
        });
        let _ = registry.register(CommandOptions {
            id: Some("cmd-b".to_string()),
            label: "Command B".to_string(),
            ..Default::default()
        });

        assert!(registry.execute("cmd-a"));
        let recent = registry.recent();
        let get_recent = registry.get_recent();
        let recent_ids: Vec<_> = recent.into_iter().map(|command| command.id).collect();
        let get_recent_ids: Vec<_> = get_recent.into_iter().map(|command| command.id).collect();
        assert_eq!(recent_ids, get_recent_ids);
    }

    #[test]
    fn fuzzy_match_matches_by_order() {
        let result = fuzzy_match("hlo", "hello");
        assert!(result.is_some());
        assert_eq!(result.unwrap().matches, vec![0, 2, 4]);
    }

    #[test]
    fn fuzzy_match_empty_query() {
        let result = fuzzy_match("", "hello");
        assert!(result.is_some());
        assert_eq!(result.unwrap().score, 0.0);
    }

    #[test]
    fn format_command_with_keybinding() {
        let command = Command {
            id: "cmd".to_string(),
            label: "Save".to_string(),
            keybinding: Some("ctrl+s".to_string()),
            ..Default::default()
        };
        assert_eq!(format_command(&command), "Save (Ctrl+S)");
    }

    #[test]
    fn highlight_matches_segments() {
        let result = highlight_matches("hello", &[0, 2, 4]);
        assert_eq!(
            result,
            vec![
                HighlightSegment {
                    text: "h".to_string(),
                    highlight: true
                },
                HighlightSegment {
                    text: "e".to_string(),
                    highlight: false
                },
                HighlightSegment {
                    text: "l".to_string(),
                    highlight: true
                },
                HighlightSegment {
                    text: "l".to_string(),
                    highlight: false
                },
                HighlightSegment {
                    text: "o".to_string(),
                    highlight: true
                },
            ]
        );
    }

    #[test]
    fn highlight_matches_empty() {
        let result = highlight_matches("hello", &[]);
        assert_eq!(
            result,
            vec![HighlightSegment {
                text: "hello".to_string(),
                highlight: false
            }]
        );
    }

    #[test]
    fn registry_subscribe_notifies_listeners() {
        let mut registry = CommandRegistry::default();
        let counter = Arc::new(Mutex::new(0usize));
        let counter_incr = {
            let counter = Arc::clone(&counter);
            move || {
                let mut value = counter.lock().unwrap_or_else(|e| e.into_inner());
                *value += 1;
            }
        };

        let sub = registry.subscribe(counter_incr);

        let _ = registry.register(CommandOptions {
            id: Some("a".to_string()),
            label: "A".to_string(),
            ..Default::default()
        });

        {
            let value = counter.lock().unwrap_or_else(|e| e.into_inner());
            assert_eq!(*value, 1);
        }

        assert!(registry.unsubscribe(sub));
        let _ = registry.register(CommandOptions {
            id: Some("b".to_string()),
            label: "B".to_string(),
            ..Default::default()
        });

        let value = counter.lock().unwrap_or_else(|e| e.into_inner());
        assert_eq!(*value, 1);
    }

    #[test]
    fn registry_subscribe_with_disposable_notifies_only_while_subscribed() {
        reset_command_registry();
        let callback_count = Arc::new(Mutex::new(0usize));
        let callback_count_inner = Arc::clone(&callback_count);

        let unsubscribe = get_command_registry().subscribe_and_return_unsubscribe(move || {
            let mut value = callback_count_inner
                .lock()
                .unwrap_or_else(|e| e.into_inner());
            *value += 1;
        });

        let first = register_command(CommandOptions {
            id: Some("notify-1".to_string()),
            label: "Notify".to_string(),
            ..Default::default()
        });
        let _ = first;

        unsubscribe();
        let second = register_command(CommandOptions {
            id: Some("notify-2".to_string()),
            label: "Ignored".to_string(),
            ..Default::default()
        });
        let _ = second;

        assert_eq!(*callback_count.lock().unwrap_or_else(|e| e.into_inner()), 1);
    }

    #[test]
    fn global_registry_handle_is_singleton() {
        let first = get_command_registry();
        let second = get_command_registry();
        assert_eq!(first, second);
    }

    #[test]
    fn registry_methods_have_get_all_alias() {
        let mut registry = CommandRegistry::default();
        let _ = registry.register(CommandOptions {
            id: Some("a".to_string()),
            label: "A".to_string(),
            ..Default::default()
        });
        let _ = registry.register(CommandOptions {
            id: Some("b".to_string()),
            label: "B".to_string(),
            ..Default::default()
        });

        assert_eq!(registry.all().len(), registry.get_all().len());
    }

    #[test]
    fn global_command_id_counter_can_be_reset_separately() {
        reset_command_id_counter();
        let _ = register_command(CommandOptions {
            label: "A".to_string(),
            ..Default::default()
        });

        reset_command_registry();
        let id = register_command(CommandOptions {
            label: "B".to_string(),
            ..Default::default()
        });
        assert_eq!(id, "command_1");
    }

    #[test]
    fn search_commands_default_uses_max_results_zero_and_empty_query() {
        let commands = vec![
            Command {
                id: "1".to_string(),
                label: "A".to_string(),
                ..Default::default()
            },
            Command {
                id: "2".to_string(),
                label: "B".to_string(),
                ..Default::default()
            },
        ];
        let results = search_commands(&commands, "", 1);
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn search_global_commands_default_for_query() {
        reset_command_registry();
        let _ = register_command(CommandOptions {
            label: "Save".to_string(),
            ..Default::default()
        });
        let results = search_global_commands_default("save");
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn registry_search_default_uses_default_max() {
        let mut registry = CommandRegistry::default();
        let _ = registry.register(CommandOptions {
            id: Some("a".to_string()),
            label: "A".to_string(),
            ..Default::default()
        });
        let _ = registry.register(CommandOptions {
            id: Some("b".to_string()),
            label: "B".to_string(),
            ..Default::default()
        });

        let results = registry.search_default("a");
        assert!(!results.is_empty());
    }

    #[test]
    fn format_command_keeps_cmd_modifier_as_cmd() {
        let command = Command {
            id: "cmd".to_string(),
            label: "Open".to_string(),
            keybinding: Some("cmd+o".to_string()),
            ..Default::default()
        };
        assert_eq!(format_command(&command), "Open (Cmd+O)");
    }

    #[test]
    fn command_palette_compat_aliases_are_callable() {
        reset_command_registry();
        let options = CommandOptions {
            id: Some("compat-id".to_string()),
            label: "Searchable".to_string(),
            ..Default::default()
        };
        let id_snake = register_command(options.clone());
        let id_camel = registerCommand(options);
        assert_eq!(id_snake, id_camel);
    }

    #[test]
    fn command_palette_search_aliases_behave_the_same() {
        reset_command_registry();
        let _ = registerCommand(CommandOptions {
            label: "Find Me".to_string(),
            ..Default::default()
        });

        let snake = search_global_commands("find", 10);
        let camel = searchGlobalCommands("find", 10);
        assert_eq!(snake.len(), camel.len());
    }

    #[test]
    fn command_palette_default_search_aliases_match() {
        reset_command_registry();
        let _ = registerCommand(CommandOptions {
            label: "Alpha".to_string(),
            ..Default::default()
        });

        let snake = search_global_commands_default("alpha");
        let camel = searchGlobalCommandsDefault("alpha");
        assert_eq!(snake.len(), camel.len());
        assert_eq!(
            snake.first().map(|m| m.command.id.clone()),
            camel.first().map(|m| m.command.id.clone())
        );
        assert_eq!(
            snake.first().map(|m| m.matches.clone()).unwrap_or_default(),
            camel.first().map(|m| m.matches.clone()).unwrap_or_default()
        );
    }

    #[test]
    fn command_palette_fuzzy_alias_is_same() {
        let result_snake = fuzzy_match("te", "test");
        let result_camel = fuzzyMatch("te", "test");
        assert_eq!(result_snake.is_some(), result_camel.is_some());
        assert_eq!(
            result_snake.map(|r| r.matches),
            result_camel.map(|r| r.matches)
        );
    }

    #[test]
    fn command_palette_format_alias_is_same() {
        let command = Command {
            id: "format".to_string(),
            label: "Save".to_string(),
            keybinding: Some("ctrl+s".to_string()),
            ..Default::default()
        };
        assert_eq!(format_command(&command), formatCommand(&command));
    }

    #[test]
    fn command_palette_group_alias_is_same() {
        let commands = vec![
            Command {
                id: "a".to_string(),
                label: "A".to_string(),
                category: Some("G".to_string()),
                ..Default::default()
            },
            Command {
                id: "b".to_string(),
                label: "B".to_string(),
                ..Default::default()
            },
        ];

        let snake = group_by_category(&commands);
        let camel = groupByCategory(&commands);
        assert_eq!(snake.len(), camel.len());
        assert_eq!(
            snake.get("G").map(|commands| commands.len()),
            camel.get("G").map(|commands| commands.len())
        );
    }

    #[test]
    fn command_palette_highlight_alias_is_same() {
        let chars = highlight_matches("hello", &[1, 3]);
        let alias = highlightMatches("hello", &[1, 3]);
        assert_eq!(chars, alias);
    }

    #[test]
    fn command_palette_global_subscribe_listener_alias() {
        let count = Arc::new(std::sync::Mutex::new(0usize));
        let count_alias = Arc::clone(&count);
        let id = subscribeListener(move || {
            let mut value = count_alias.lock().unwrap_or_else(|e| e.into_inner());
            *value += 1;
        });

        let _ = registerCommand(CommandOptions {
            label: "sub".to_string(),
            ..Default::default()
        });
        let value = *count.lock().unwrap_or_else(|e| e.into_inner());
        assert_eq!(value, 1);

        assert!(unsubscribe(id));
    }

    #[test]
    fn command_palette_disposable_subscribe_alias_works() {
        let count = Arc::new(std::sync::Mutex::new(0usize));
        let count_alias = Arc::clone(&count);
        let stop = subscribeAndReturnUnsubscribe(move || {
            let mut value = count_alias.lock().unwrap_or_else(|e| e.into_inner());
            *value += 1;
        });

        let _ = register_command(CommandOptions {
            label: "sub2".to_string(),
            ..Default::default()
        });
        assert_eq!(*count.lock().unwrap_or_else(|e| e.into_inner()), 1);

        stop();
        let _ = register_command(CommandOptions {
            label: "sub3".to_string(),
            ..Default::default()
        });
        assert_eq!(*count.lock().unwrap_or_else(|e| e.into_inner()), 1);
    }
}
