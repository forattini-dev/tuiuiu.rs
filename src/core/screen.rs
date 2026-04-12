//! Screen Navigation
//!
//! Stack-based screen navigation with lifecycle hooks and global helpers.

use crate::core::event::EventEmitter;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

// ============================================================================
// Types
// ============================================================================

/// Screen component placeholder type.
pub type ScreenComponent = &'static str;

/// Generic screen state.
pub type ScreenState = HashMap<String, String>;

/// Unique screen identifier.
pub type ScreenId = String;

/// Direction used for screen transitions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransitionDirection {
    /// Navigate forward to a new screen.
    Forward,
    /// Navigate back in the stack.
    Back,
    /// Replace the current screen.
    Replace,
    /// No transition.
    None,
}

/// Screen definition.
#[derive(Clone)]
pub struct Screen {
    /// Unique identifier.
    pub id: ScreenId,
    /// Component entry-point.
    pub component: ScreenComponent,
    /// Screen title.
    pub title: Option<String>,
    /// Screen subtitle.
    pub subtitle: Option<String>,
    /// Props placeholder (kept simple for the core port).
    pub props: Option<String>,
    /// Keep screen alive when not active.
    pub keep_alive: bool,
    /// Show back affordance for this screen.
    pub show_back: bool,
    /// Custom back handler.
    pub on_back: Option<OnScreenActionBool>,
    /// Called when screen becomes active.
    pub on_enter: Option<OnScreenAction>,
    /// Called when screen becomes inactive.
    pub on_exit: Option<OnScreenAction>,
    /// Guard before entering.
    pub on_before_enter: Option<OnScreenActionBool>,
    /// Guard before exiting.
    pub on_before_exit: Option<OnScreenActionBool>,
}

impl fmt::Debug for Screen {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Screen")
            .field("id", &self.id)
            .field("component", &self.component)
            .field("title", &self.title)
            .field("subtitle", &self.subtitle)
            .field("props", &self.props)
            .field("keep_alive", &self.keep_alive)
            .field("show_back", &self.show_back)
            .finish()
    }
}

/// Lifecycle hooks that only return side effects.
pub type OnScreenAction = Arc<dyn Fn() + Send + Sync + 'static>;

/// Lifecycle hooks that can veto navigation.
pub type OnScreenActionBool = Arc<dyn Fn() -> bool + Send + Sync + 'static>;

impl Screen {
    /// Create a new screen.
    pub fn new(component: ScreenComponent) -> Self {
        Self {
            id: generate_screen_id(),
            component,
            title: None,
            subtitle: None,
            props: None,
            keep_alive: false,
            show_back: false,
            on_back: None,
            on_enter: None,
            on_exit: None,
            on_before_enter: None,
            on_before_exit: None,
        }
    }

    /// Create a new screen with configuration.
    pub fn with_options(component: ScreenComponent, options: ScreenOptions) -> Self {
        let mut screen = Self::new(component);
        screen.title = options.title;
        screen.subtitle = options.subtitle;
        screen.props = options.props;
        screen.keep_alive = options.keep_alive;
        screen.show_back = options.show_back;
        screen.on_back = options.on_back;
        screen.on_enter = options.on_enter;
        screen.on_exit = options.on_exit;
        screen.on_before_enter = options.on_before_enter;
        screen.on_before_exit = options.on_before_exit;
        screen
    }
}

/// Creation options for a screen.
#[derive(Default)]
pub struct ScreenOptions {
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub props: Option<String>,
    pub keep_alive: bool,
    pub show_back: bool,
    pub on_back: Option<OnScreenActionBool>,
    pub on_enter: Option<OnScreenAction>,
    pub on_exit: Option<OnScreenAction>,
    pub on_before_enter: Option<OnScreenActionBool>,
    pub on_before_exit: Option<OnScreenActionBool>,
}

/// Entry stored in navigation stack.
#[derive(Debug, Clone)]
pub struct ScreenStackEntry {
    /// Screen instance.
    pub screen: Screen,
    /// Cached state for the screen.
    pub state: Option<ScreenState>,
    /// Unix timestamp (ms) when entered.
    pub entered_at: u128,
}

impl ScreenStackEntry {
    fn new(screen: Screen) -> Self {
        Self {
            screen,
            state: None,
            entered_at: unix_now_ms(),
        }
    }
}

/// Screen navigation event data.
#[derive(Debug, Clone)]
pub struct ScreenNavigationEvent {
    pub from: Option<Screen>,
    pub to: Screen,
    pub direction: TransitionDirection,
}

/// Screen manager state snapshot.
#[derive(Debug, Clone)]
pub struct ScreenManagerState {
    /// Current active screen.
    pub current: Option<Screen>,
    /// Stack of screens.
    pub stack: Vec<ScreenStackEntry>,
    /// Whether a transition is in progress.
    pub transitioning: bool,
    /// Current transition direction.
    pub transition_direction: TransitionDirection,
}

impl Default for ScreenManagerState {
    fn default() -> Self {
        Self {
            current: None,
            stack: Vec::new(),
            transitioning: false,
            transition_direction: TransitionDirection::None,
        }
    }
}

/// Public screen manager options.
#[derive(Debug, Clone)]
pub struct ScreenManagerOptions {
    pub initial_screen: Option<Screen>,
    pub default_keep_alive: bool,
    pub escape_goes_back: bool,
    pub max_stack_size: usize,
    pub transition_duration_ms: u64,
}

impl Default for ScreenManagerOptions {
    fn default() -> Self {
        Self {
            initial_screen: None,
            default_keep_alive: false,
            escape_goes_back: true,
            max_stack_size: 0,
            transition_duration_ms: 300,
        }
    }
}

/// Convenience result returned by [`use_screen`].
#[derive(Debug, Clone)]
pub struct UseScreenResult {
    /// Current screen.
    pub current: Option<Screen>,
    /// Can navigate backward.
    pub can_go_back: bool,
    /// Can navigate forward.
    pub can_go_forward: bool,
    /// Is manager transitioning.
    pub transitioning: bool,
    /// Full current state.
    pub state: ScreenManagerState,
}

impl Default for UseScreenResult {
    fn default() -> Self {
        Self {
            current: None,
            can_go_back: false,
            can_go_forward: false,
            transitioning: false,
            state: ScreenManagerState::default(),
        }
    }
}

// ============================================================================
// Manager
// ============================================================================

/// Screen manager implementation.
pub struct ScreenManager {
    options: ScreenManagerOptions,
    stack: Vec<ScreenStackEntry>,
    current_index: isize,
    max_stack_size: usize,
    transitioning: bool,
    transition_direction: TransitionDirection,
    events: EventEmitter,
}

impl ScreenManager {
    /// Create a new manager.
    pub fn new(options: Option<ScreenManagerOptions>) -> Self {
        let mut manager = Self {
            options: options.unwrap_or_default(),
            stack: Vec::new(),
            current_index: -1,
            max_stack_size: 0,
            transitioning: false,
            transition_direction: TransitionDirection::None,
            events: EventEmitter::new(),
        };

        manager.max_stack_size = manager.options.max_stack_size;

        if let Some(initial_screen) = manager.options.initial_screen.clone() {
            let _ = manager.push(initial_screen);
        }

        manager
    }

    /// Current screen snapshot.
    pub fn current(&self) -> Option<Screen> {
        self.current_entry().map(|entry| entry.screen.clone())
    }

    /// Current stack snapshot.
    pub fn screen_stack(&self) -> Vec<ScreenStackEntry> {
        self.stack.clone()
    }

    /// Whether a transition is in progress.
    pub fn is_transitioning(&self) -> bool {
        self.transitioning
    }

    /// Current transition direction.
    pub fn transition_direction(&self) -> TransitionDirection {
        self.transition_direction
    }

    /// Number of screens in stack.
    pub fn stack_size(&self) -> usize {
        self.stack.len()
    }

    /// Can go back.
    pub fn can_go_back(&self) -> bool {
        self.current_index > 0
    }

    /// Can go forward.
    pub fn can_go_forward(&self) -> bool {
        if self.current_index < 0 {
            return false;
        }

        (self.current_index as usize) + 1 < self.stack.len()
    }

    /// Return full state snapshot.
    pub fn get_state(&self) -> ScreenManagerState {
        ScreenManagerState {
            current: self.current(),
            stack: self.stack.clone(),
            transitioning: self.transitioning,
            transition_direction: self.transition_direction,
        }
    }

    fn current_entry(&self) -> Option<&ScreenStackEntry> {
        if self.current_index < 0 {
            return None;
        }
        self.stack.get(self.current_index as usize)
    }

    fn current_entry_mut(&mut self) -> Option<&mut ScreenStackEntry> {
        if self.current_index < 0 {
            return None;
        }
        self.stack.get_mut(self.current_index as usize)
    }

    fn clamp_stack_size(&mut self) {
        if self.max_stack_size == 0 || self.stack.len() <= self.max_stack_size {
            return;
        }

        let overflow = self.stack.len() - self.max_stack_size;
        if overflow > 0 {
            self.stack.drain(0..overflow);
            self.current_index = (self.current_index - overflow as isize).max(0);
        }
    }

    fn check_transition_guard(&self, to: &Screen) -> bool {
        if let Some(current) = self.current() {
            if let Some(before_exit) = current.on_before_exit.as_ref() {
                if !(before_exit)() {
                    return false;
                }
            }
        }

        if let Some(before_enter) = to.on_before_enter.as_ref() {
            if !(before_enter)() {
                return false;
            }
        }

        true
    }

    fn apply_enter_exit(
        &mut self,
        from: Option<&Screen>,
        to: &Screen,
        direction: TransitionDirection,
    ) {
        if let Some(from_screen) = from {
            if let Some(on_exit) = from_screen.on_exit.as_ref() {
                on_exit();
            }

            self.events.emit(
                "screenExit",
                &ScreenNavigationEvent {
                    from: Some(from_screen.clone()),
                    to: to.clone(),
                    direction,
                },
            );
        }

        if let Some(on_enter) = to.on_enter.as_ref() {
            on_enter();
        }

        self.events.emit(
            "screenEnter",
            &ScreenNavigationEvent {
                from: from.cloned(),
                to: to.clone(),
                direction,
            },
        );
    }

    fn update_current_index(&mut self, next_index: isize) {
        self.current_index = next_index;
    }

    /// Register an event listener.
    pub fn on<F>(&self, event: &str, handler: F)
    where
        F: Fn(&dyn std::any::Any) + 'static,
    {
        self.events.on(event, handler);
    }

    /// Emit an event payload.
    pub fn emit<T: 'static>(&self, event: &str, data: &T) {
        self.events.emit(event, data);
    }

    /// Push new screen.
    pub fn push(&mut self, screen: Screen) -> bool {
        self.navigate(screen, TransitionDirection::Forward)
    }

    /// Replace current screen.
    pub fn replace(&mut self, screen: Screen) -> bool {
        if self.current_index < 0 {
            return self.push(screen);
        }

        let from = self.current();
        let _ = self.stack.pop();
        self.update_current_index((self.current_index - 1).max(-1));
        if let Some(from_screen) = from {
            if let Some(on_exit) = from_screen.on_exit.as_ref() {
                on_exit();
            }
            self.emit(
                "screenExit",
                &ScreenNavigationEvent {
                    from: Some(from_screen.clone()),
                    to: screen.clone(),
                    direction: TransitionDirection::Replace,
                },
            );
        }

        self.navigate(screen, TransitionDirection::Replace)
    }

    /// Pop current screen.
    pub fn pop(&mut self) -> bool {
        if !self.can_go_back() {
            return false;
        }

        let from = self.current();
        let next_index = self.current_index - 1;
        let target = self
            .stack
            .get(next_index as usize)
            .map(|entry| entry.screen.clone());

        if let Some(target) = target {
            if !self.check_transition_guard(&target) {
                self.events.emit(
                    "navigateCancelled",
                    &ScreenNavigationEvent {
                        from: from.clone(),
                        to: target.clone(),
                        direction: TransitionDirection::Back,
                    },
                );
                return false;
            }

            self.transitioning = true;
            self.transition_direction = TransitionDirection::Back;

            if self.options.transition_duration_ms > 0 {
                // Deterministic short pause to preserve old timing behavior.
                std::thread::sleep(Duration::from_millis(self.options.transition_duration_ms));
            }

            let popped = self.stack.pop();
            if let Some(popped) = popped {
                let from_screen = popped.screen;
                self.apply_enter_exit(Some(&from_screen), &target, self.transition_direction);
            }

            self.update_current_index(next_index);
            self.transitioning = false;
            self.transition_direction = TransitionDirection::None;

            self.events.emit(
                "navigate",
                &ScreenNavigationEvent {
                    from,
                    to: target.clone(),
                    direction: TransitionDirection::Back,
                },
            );
            self.events.emit("stackChange", &self.get_state());
            return true;
        }

        false
    }

    /// Go back helper.
    pub fn go_back(&mut self) -> bool {
        if let Some(current) = self.current() {
            if let Some(on_back) = current.on_back.as_ref() {
                if on_back() {
                    return false;
                }
            }
        }
        self.pop()
    }

    /// Pop to root screen.
    pub fn pop_to_root(&mut self) -> bool {
        if self.stack.is_empty() {
            return false;
        }

        while self.current_index > 0 {
            if !self.pop() {
                return false;
            }
        }

        true
    }

    /// Pop to screen id.
    pub fn pop_to(&mut self, id: &str) -> bool {
        let target_index = self
            .stack
            .iter()
            .position(|entry| entry.screen.id == id)
            .and_then(|idx| Some(idx as isize));

        let Some(target_index) = target_index else {
            return false;
        };

        if target_index >= self.current_index {
            return false;
        }

        while self.current_index > target_index {
            if !self.pop() {
                return false;
            }
        }

        true
    }

    /// Reset stack and push a single screen.
    pub fn reset(&mut self, screen: Screen) {
        if let Some(current) = self.current() {
            self.emit(
                "screenExit",
                &ScreenNavigationEvent {
                    from: Some(current.clone()),
                    to: screen.clone(),
                    direction: TransitionDirection::None,
                },
            );
        }

        self.stack.clear();
        self.current_index = -1;

        let _ = self.push(screen);
    }

    /// Check if screen exists by id.
    pub fn has_screen(&self, id: &str) -> bool {
        self.stack.iter().any(|entry| entry.screen.id == id)
    }

    /// Get screen by id.
    pub fn get_screen(&self, id: &str) -> Option<Screen> {
        self.stack
            .iter()
            .find(|entry| entry.screen.id == id)
            .map(|entry| entry.screen.clone())
    }

    /// Set/merge current screen state.
    pub fn set_screen_state(&mut self, state: ScreenState) {
        if let Some(entry) = self.current_entry_mut() {
            match entry.state.as_mut() {
                Some(existing) => {
                    for (k, v) in state {
                        existing.insert(k, v);
                    }
                }
                None => entry.state = Some(state),
            }
        }
    }

    /// Get current screen state.
    pub fn get_screen_state(&self) -> Option<ScreenState> {
        self.current_entry().and_then(|entry| entry.state.clone())
    }

    /// Escape handling hook.
    pub fn handle_escape(&mut self) -> bool {
        if !self.options.escape_goes_back {
            return false;
        }
        self.go_back()
    }

    /// Update manager options.
    pub fn set_max_stack_size(&mut self, max_stack_size: usize) {
        self.max_stack_size = max_stack_size;
        self.options.max_stack_size = max_stack_size;
        self.clamp_stack_size();
    }

    fn navigate(&mut self, screen: Screen, direction: TransitionDirection) -> bool {
        if self.transitioning {
            return false;
        }

        if !self.check_transition_guard(&screen) {
            let from = self.current();
            self.events.emit(
                "navigateCancelled",
                &ScreenNavigationEvent {
                    from,
                    to: screen,
                    direction,
                },
            );
            return false;
        }

        let from = self.current();
        self.transitioning = true;
        self.transition_direction = direction;

        if direction == TransitionDirection::Forward && self.can_go_forward() {
            self.stack.truncate(self.current_index as usize + 1);
        }

        if self.options.transition_duration_ms > 0 {
            std::thread::sleep(Duration::from_millis(self.options.transition_duration_ms));
        }

        self.apply_enter_exit(from.as_ref(), &screen, direction);

        self.stack.push(ScreenStackEntry::new(screen.clone()));
        self.update_current_index((self.stack.len() - 1) as isize);
        self.clamp_stack_size();

        self.transitioning = false;
        self.transition_direction = TransitionDirection::None;

        self.events.emit(
            "navigate",
            &ScreenNavigationEvent {
                from,
                to: screen,
                direction,
            },
        );
        self.events.emit("stackChange", &self.get_state());

        true
    }
}

// ============================================================================
// Global Manager
// ============================================================================

thread_local! {
    static SCREEN_MANAGER: RefCell<ScreenManager> = RefCell::new(ScreenManager::new(None));
}

static SCREEN_ID_COUNTER: AtomicU64 = AtomicU64::new(0);

/// Reset the screen ID counter.
pub fn reset_screen_id_counter() {
    SCREEN_ID_COUNTER.store(0, Ordering::SeqCst);
}

/// Generate a unique screen id.
pub fn generate_screen_id() -> String {
    let id = SCREEN_ID_COUNTER.fetch_add(1, Ordering::SeqCst) + 1;
    format!("screen-{id}")
}

/// Create a new screen manager.
pub fn create_screen_manager(options: Option<ScreenManagerOptions>) -> ScreenManager {
    ScreenManager::new(options)
}

/// Create a screen definition.
pub fn create_screen(component: ScreenComponent) -> Screen {
    Screen::new(component)
}

/// Create a screen with options.
pub fn create_screen_with_options(component: ScreenComponent, options: ScreenOptions) -> Screen {
    Screen::with_options(component, options)
}

/// Alias for compatibility with older names.
pub fn generate_screen_id_and_create_screen(component: ScreenComponent) -> Screen {
    create_screen(component)
}

/// Get global manager thread-local key.
pub fn get_screen_manager() -> &'static std::thread::LocalKey<RefCell<ScreenManager>> {
    &SCREEN_MANAGER
}

/// Reset global manager state.
pub fn reset_screen_manager() {
    get_screen_manager().with(|manager| {
        *manager.borrow_mut() = ScreenManager::new(None);
    });
}

/// Push a screen to global manager.
pub fn push_screen(screen: Screen) -> bool {
    get_screen_manager().with(|manager| manager.borrow_mut().push(screen))
}

/// Pop global manager.
pub fn pop_screen() -> bool {
    get_screen_manager().with(|manager| manager.borrow_mut().pop())
}

/// Replace global manager screen.
pub fn replace_screen(screen: Screen) -> bool {
    get_screen_manager().with(|manager| manager.borrow_mut().replace(screen))
}

/// Go back in global manager.
pub fn go_back() -> bool {
    get_screen_manager().with(|manager| manager.borrow_mut().go_back())
}

/// Hook-like read of global state.
pub fn use_screen(manager: Option<&ScreenManager>) -> UseScreenResult {
    let state = match manager {
        Some(manager) => manager.get_state(),
        None => get_screen_manager().with(|global| global.borrow().get_state()),
    };

    let current = state.current.as_ref();
    let can_go_back = current.is_some() && state.stack.len() > 1;
    let can_go_forward = current.is_some()
        && !state.stack.is_empty()
        && state.stack.last().is_some_and(|entry| {
            entry.screen.id != current.map(|screen| screen.id.clone()).unwrap_or_default()
        });

    UseScreenResult {
        current: state.current.clone(),
        can_go_back,
        can_go_forward,
        transitioning: state.transitioning,
        state,
    }
}

/// Hook-style state accessor.
pub fn use_screen_state<T, F>(manager: Option<&ScreenManager>, transform: F) -> T
where
    F: FnOnce(Option<&ScreenState>) -> T,
{
    let state = match manager {
        Some(manager) => manager.get_screen_state(),
        None => get_screen_manager().with(|global| global.borrow().get_screen_state()),
    };

    transform(state.as_ref())
}

fn unix_now_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dummy() -> Screen {
        create_screen("home")
    }

    #[test]
    fn create_screen_ids_and_manager() {
        reset_screen_id_counter();
        let mut manager = create_screen_manager(None);
        let home = create_screen_with_options(
            "home",
            ScreenOptions {
                title: Some("Home".to_string()),
                subtitle: Some("Welcome".to_string()),
                props: Some("{}".to_string()),
                keep_alive: true,
                ..Default::default()
            },
        );

        assert_eq!(home.id, "screen-1");

        assert!(manager.push(home));
        let next = create_screen("settings");

        assert!(manager.push(next));
        assert_eq!(manager.stack_size(), 2);
        assert!(manager.can_go_back());
        assert!(!manager.is_transitioning());

        let result = manager.pop();
        assert!(result);
        assert_eq!(manager.stack_size(), 1);
    }

    #[test]
    fn global_state_accessor() {
        reset_screen_id_counter();
        reset_screen_manager();

        let home = dummy();
        assert!(!use_screen(None).transitioning);
        let pushed = push_screen(home);
        assert!(pushed);

        let current = use_screen(None);
        assert_eq!(current.can_go_back, false);
        assert!(current.current.is_some());
    }
}
