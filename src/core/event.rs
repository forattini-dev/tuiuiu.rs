//! Event System
//!
//! Event creation, bubbling, and delegation for the component tree.

use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

// =============================================================================
// Event Types
// =============================================================================

/// Event propagation phase.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventPhase {
    /// Capturing phase (root to target)
    Capture,
    /// At target
    Target,
    /// Bubbling phase (target to root)
    Bubble,
}

/// Base event trait.
pub trait Event: Any {
    /// Get the event type name.
    fn event_type(&self) -> &'static str;

    /// Check if propagation should stop.
    fn propagation_stopped(&self) -> bool;

    /// Stop propagation.
    fn stop_propagation(&mut self);

    /// Prevent default behavior.
    fn prevent_default(&mut self);

    /// Check if default is prevented.
    fn default_prevented(&self) -> bool;

    /// Get the event phase.
    fn phase(&self) -> EventPhase;

    /// Set the event phase.
    fn set_phase(&mut self, phase: EventPhase);
}

/// Generic TUI event.
#[derive(Debug)]
pub struct TuiEvent<T> {
    /// Event type name
    pub type_name: &'static str,
    /// Event data
    pub data: T,
    /// Target node ID
    pub target: Option<u64>,
    /// Current phase
    phase: EventPhase,
    /// Whether propagation is stopped
    stopped: bool,
    /// Whether default is prevented
    prevented: bool,
}

impl<T> TuiEvent<T> {
    /// Create a new event.
    pub fn new(type_name: &'static str, data: T) -> Self {
        Self {
            type_name,
            data,
            target: None,
            phase: EventPhase::Bubble,
            stopped: false,
            prevented: false,
        }
    }

    /// Set the target.
    pub fn with_target(mut self, target: u64) -> Self {
        self.target = Some(target);
        self
    }
}

impl<T: 'static> Event for TuiEvent<T> {
    fn event_type(&self) -> &'static str {
        self.type_name
    }

    fn propagation_stopped(&self) -> bool {
        self.stopped
    }

    fn stop_propagation(&mut self) {
        self.stopped = true;
    }

    fn prevent_default(&mut self) {
        self.prevented = true;
    }

    fn default_prevented(&self) -> bool {
        self.prevented
    }

    fn phase(&self) -> EventPhase {
        self.phase
    }

    fn set_phase(&mut self, phase: EventPhase) {
        self.phase = phase;
    }
}

/// Create a typed event.
pub fn create_event<T: 'static>(type_name: &'static str, data: T) -> TuiEvent<T> {
    TuiEvent::new(type_name, data)
}

// =============================================================================
// Event Handler
// =============================================================================

/// Event handler function type.
pub type EventHandler<T> = Box<dyn Fn(&mut TuiEvent<T>)>;

/// Event handler options.
#[derive(Debug, Clone, Default)]
pub struct EventListenerOptions {
    /// Handle during capture phase
    pub capture: bool,
    /// Remove after first invocation
    pub once: bool,
    /// Passive (won't call preventDefault)
    pub passive: bool,
}

// =============================================================================
// Event Emitter
// =============================================================================

/// A simple event emitter.
pub struct EventEmitter {
    listeners: RefCell<HashMap<String, Vec<Rc<dyn Fn(&dyn Any)>>>>,
}

impl EventEmitter {
    /// Create a new emitter.
    pub fn new() -> Self {
        Self {
            listeners: RefCell::new(HashMap::new()),
        }
    }

    /// Add an event listener.
    pub fn on<F>(&self, event: &str, handler: F)
    where
        F: Fn(&dyn Any) + 'static,
    {
        self.listeners
            .borrow_mut()
            .entry(event.to_string())
            .or_default()
            .push(Rc::new(handler));
    }

    /// Remove all listeners for an event.
    pub fn off(&self, event: &str) {
        self.listeners.borrow_mut().remove(event);
    }

    /// Emit an event.
    pub fn emit<T: 'static>(&self, event: &str, data: &T) {
        if let Some(handlers) = self.listeners.borrow().get(event) {
            for handler in handlers {
                handler(data);
            }
        }
    }

    /// Check if there are listeners for an event.
    pub fn has_listeners(&self, event: &str) -> bool {
        self.listeners
            .borrow()
            .get(event)
            .map(|h| !h.is_empty())
            .unwrap_or(false)
    }
}

impl Default for EventEmitter {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// Global Event Bus
// =============================================================================

thread_local! {
    static EVENT_BUS: EventEmitter = EventEmitter::new();
}

/// Get the global event bus.
pub fn get_event_bus() -> &'static std::thread::LocalKey<EventEmitter> {
    &EVENT_BUS
}

// =============================================================================
// Delegation Helpers
// =============================================================================

/// Options for event delegation.
#[derive(Debug, Clone, Default)]
pub struct DelegateOptions {
    /// Stop propagation after handling
    pub stop_propagation: bool,
    /// Prevent default after handling
    pub prevent_default: bool,
}

/// Create a delegated event handler.
pub fn delegate<F, T>(selector: impl Fn(u64) -> bool + 'static, handler: F) -> impl Fn(&mut TuiEvent<T>)
where
    F: Fn(&mut TuiEvent<T>) + 'static,
    T: 'static,
{
    move |event: &mut TuiEvent<T>| {
        if let Some(target) = event.target {
            if selector(target) {
                handler(event);
            }
        }
    }
}

// =============================================================================
// Async Helpers
// =============================================================================

/// Wait for an event to occur.
pub async fn wait_for_event<T: Clone + 'static>(
    _emitter: &EventEmitter,
    _event: &str,
) -> Option<T> {
    // In a real implementation, this would use async channels
    None
}

/// Create an iterator over events.
pub fn event_iterator<T: Clone + 'static>(
    _emitter: &EventEmitter,
    _event: &str,
) -> impl Iterator<Item = T> {
    std::iter::empty()
}

// =============================================================================
// Composition Helpers
// =============================================================================

/// Combine multiple handlers into one.
pub fn combine_handlers<T: 'static>(
    handlers: Vec<Box<dyn Fn(&mut TuiEvent<T>)>>,
) -> Box<dyn Fn(&mut TuiEvent<T>)> {
    Box::new(move |event: &mut TuiEvent<T>| {
        for handler in &handlers {
            handler(event);
            if event.propagation_stopped() {
                break;
            }
        }
    })
}

/// Create a conditional handler.
pub fn conditional_handler<T: 'static, P>(
    predicate: P,
    handler: Box<dyn Fn(&mut TuiEvent<T>)>,
) -> Box<dyn Fn(&mut TuiEvent<T>)>
where
    P: Fn(&TuiEvent<T>) -> bool + 'static,
{
    Box::new(move |event: &mut TuiEvent<T>| {
        if predicate(event) {
            handler(event);
        }
    })
}

/// Create a debounced handler.
pub fn debounce_handler<T: 'static>(
    handler: Box<dyn Fn(&mut TuiEvent<T>)>,
    _delay_ms: u64,
) -> Box<dyn Fn(&mut TuiEvent<T>)> {
    // Simplified - in real implementation would track timing
    handler
}

/// Create a throttled handler.
pub fn throttle_handler<T: 'static>(
    handler: Box<dyn Fn(&mut TuiEvent<T>)>,
    _delay_ms: u64,
) -> Box<dyn Fn(&mut TuiEvent<T>)> {
    // Simplified - in real implementation would track timing
    handler
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_creation() {
        let event = create_event("click", 42);
        assert_eq!(event.event_type(), "click");
        assert_eq!(event.data, 42);
    }

    #[test]
    fn test_event_propagation() {
        let mut event: TuiEvent<()> = create_event("test", ());
        assert!(!event.propagation_stopped());

        event.stop_propagation();
        assert!(event.propagation_stopped());
    }

    #[test]
    fn test_event_emitter() {
        let emitter = EventEmitter::new();
        let called = Rc::new(RefCell::new(false));

        {
            let called = Rc::clone(&called);
            emitter.on("test", move |_| {
                *called.borrow_mut() = true;
            });
        }

        emitter.emit("test", &());
        assert!(*called.borrow());
    }
}
