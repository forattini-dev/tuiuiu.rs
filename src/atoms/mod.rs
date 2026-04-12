//! Atoms - Smallest functional UI components
//!
//! Basic interactive components:
//! - Button, TextInput, Switch, Slider
//! - Spinner, ProgressBar, Timer
//! - Checkbox, Badge, Link, Tooltip, Icon, Tag
//! - StatusIndicator, BigText, Digits
//! - MetricDisplay, Picture, Skeleton

mod badge;
mod big_text;
mod button;
mod checkbox;
mod digits;
mod icon;
mod link;
mod metric_display;
mod picture;
mod progress;
mod scrollbar;
mod skeleton;
mod slider;
mod spinner;
mod status_indicator;
mod switch;
mod tag;
mod text_input;
mod timer;
mod tooltip;

pub use badge::{Badge, BadgeVariant};
pub use big_text::{BigText, BigTextAlign, BigTextFont};
pub use button::Button;
pub use checkbox::{Checkbox, CheckboxState, CheckboxValue};
pub use digits::{DigitStyle, Digits};
pub use icon::{icons, Icon, IconSize};
pub use link::{Link, LinkVariant};
pub use metric_display::{
    create_metric, get_threshold_color, MetricDisplay, MetricLayout, MetricSize, MetricState,
    ThresholdConfig, ThresholdRange,
};
pub use picture::{
    create_banner, create_pixel_grid, create_pixel_grid_from_colors, rainbow_text, AsciiPatterns,
    ColorPalette, ColoredPicture, Picture, PictureAlignX, PictureAlignY, PictureFit, PictureProps,
    Pixel, PixelGrid,
};
pub use progress::ProgressBar;
pub use scrollbar::{Scrollbar, ScrollbarMode};
pub use skeleton::{
    Skeleton, SkeletonCard, SkeletonChars, SkeletonText, SKELETON_ASCII, SKELETON_BLOCKS,
    SKELETON_DOTS,
};
pub use slider::{Slider, SliderMode, SliderState};
pub use spinner::Spinner;
pub use status_indicator::{StatusIndicator, StatusSize, StatusState};
pub use switch::{Switch, SwitchSize, SwitchState};
pub use tag::{Tag, TagSize, TagVariant};
pub use text_input::TextInput;
pub use timer::{create_timer_state, Timer, TimerFormat, TimerMode, TimerState};
pub use tooltip::{Tooltip, TooltipPosition};
