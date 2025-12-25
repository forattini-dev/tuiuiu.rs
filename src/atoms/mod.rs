//! Atoms - Smallest functional UI components
//!
//! Basic interactive components:
//! - Button, TextInput, Switch, Slider
//! - Spinner, ProgressBar, Timer
//! - Checkbox, Badge, Link, Tooltip, Icon

mod button;
mod text_input;
mod spinner;
mod progress;

pub use button::Button;
pub use text_input::TextInput;
pub use spinner::Spinner;
pub use progress::ProgressBar;

// Stubs for other atoms
pub struct Switch;
pub struct Slider;
pub struct Timer;
pub struct Checkbox;
pub struct Badge;
pub struct Link;
pub struct Tooltip;
pub struct Icon;
