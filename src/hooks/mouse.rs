//! Mouse Hooks
//!
//! Mouse event handling.

use std::cell::RefCell;
use std::rc::Rc;
use crate::core::terminal::MouseEvent;

/// Mouse handler function type.
pub type MouseHandler = Box<dyn Fn(&MouseEvent)>;

thread_local! {
    static MOUSE_HANDLERS: RefCell<Vec<Rc<dyn Fn(&MouseEvent)>>> = 
        const { RefCell::new(Vec::new()) };
}

/// Register a mouse event handler.
///
/// # Example
///
/// ```rust
/// use tuiuiu::hooks::use_mouse;
/// use tuiuiu::core::terminal::MouseEventKind;
///
/// use_mouse(|event| {
///     match event.kind {
///         MouseEventKind::Down(btn) => println!("Click at {}, {}", event.x, event.y),
///         MouseEventKind::ScrollUp => println!("Scroll up"),
///         MouseEventKind::ScrollDown => println!("Scroll down"),
///         _ => {}
///     }
/// });
/// ```
pub fn use_mouse<F>(handler: F)
where
    F: Fn(&MouseEvent) + 'static,
{
    MOUSE_HANDLERS.with(|handlers| {
        handlers.borrow_mut().push(Rc::new(handler));
    });
}

/// Dispatch a mouse event to all handlers.
pub fn dispatch_mouse_event(event: &MouseEvent) {
    MOUSE_HANDLERS.with(|handlers| {
        for handler in handlers.borrow().iter() {
            handler(event);
        }
    });
}

/// Clear all mouse handlers.
pub fn clear_mouse_handlers() {
    MOUSE_HANDLERS.with(|handlers| {
        handlers.borrow_mut().clear();
    });
}

/// Track mouse position.
pub struct MousePosition {
    pub x: u16,
    pub y: u16,
}

/// Create a mouse position tracker.
pub fn use_mouse_position() -> Rc<RefCell<MousePosition>> {
    let pos = Rc::new(RefCell::new(MousePosition { x: 0, y: 0 }));

    {
        let pos = Rc::clone(&pos);
        use_mouse(move |event| {
            let mut p = pos.borrow_mut();
            p.x = event.x;
            p.y = event.y;
        });
    }

    pos
}
