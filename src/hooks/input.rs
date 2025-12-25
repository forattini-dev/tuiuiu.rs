//! Input Hooks
//!
//! Keyboard input handling.

use std::cell::RefCell;
use std::rc::Rc;
use crate::core::terminal::{Key, KeyModifiers, KeyEvent};

/// Input handler function type.
pub type InputHandler = Box<dyn Fn(&Key, &KeyModifiers)>;

thread_local! {
    static INPUT_HANDLERS: RefCell<Vec<Rc<dyn Fn(&Key, &KeyModifiers)>>> = 
        const { RefCell::new(Vec::new()) };
}

/// Register an input handler.
///
/// # Example
///
/// ```rust
/// use tuiuiu::hooks::use_input;
/// use tuiuiu::core::terminal::Key;
///
/// use_input(|key, mods| {
///     match key {
///         Key::Up => println!("Up pressed"),
///         Key::Down => println!("Down pressed"),
///         Key::Char('q') if mods.ctrl => println!("Quit"),
///         _ => {}
///     }
/// });
/// ```
pub fn use_input<F>(handler: F)
where
    F: Fn(&Key, &KeyModifiers) + 'static,
{
    INPUT_HANDLERS.with(|handlers| {
        handlers.borrow_mut().push(Rc::new(handler));
    });
}

/// Register a handler for a specific key.
pub fn use_key<F>(target: Key, handler: F)
where
    F: Fn() + 'static,
{
    use_input(move |key, _mods| {
        if key == &target {
            handler();
        }
    });
}

/// Dispatch a key event to all handlers.
pub fn dispatch_key_event(event: &KeyEvent) {
    INPUT_HANDLERS.with(|handlers| {
        for handler in handlers.borrow().iter() {
            handler(&event.key, &event.modifiers);
        }
    });
}

/// Clear all input handlers.
pub fn clear_input_handlers() {
    INPUT_HANDLERS.with(|handlers| {
        handlers.borrow_mut().clear();
    });
}

/// Check if a key matches a pattern.
pub fn key_matches(key: &Key, mods: &KeyModifiers, pattern: &str) -> bool {
    let parsed = crate::core::hotkeys::parse_hotkey(pattern);
    key == &parsed.key
        && mods.ctrl == parsed.modifiers.ctrl
        && mods.alt == parsed.modifiers.alt
        && mods.shift == parsed.modifiers.shift
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;

    #[test]
    fn test_use_input() {
        let called = Rc::new(Cell::new(false));

        {
            let called = Rc::clone(&called);
            use_input(move |key, _| {
                if matches!(key, Key::Enter) {
                    called.set(true);
                }
            });
        }

        let event = KeyEvent::simple(Key::Enter);
        dispatch_key_event(&event);

        assert!(called.get());

        clear_input_handlers();
    }
}
