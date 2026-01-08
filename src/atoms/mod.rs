//! Atoms - Smallest functional UI components
//!
//! Basic interactive components:
//! - Button, TextInput, Switch, Slider
//! - Spinner, ProgressBar, Timer
//! - Checkbox, Badge, Link, Tooltip, Icon, Tag
//! - StatusIndicator, BigText, Digits
//! - MetricDisplay, Picture, Skeleton

mod button;
mod text_input;
mod spinner;
mod progress;
mod switch;
mod badge;
mod checkbox;
mod slider;
mod scrollbar;
mod timer;
mod tag;
mod tooltip;
mod link;
mod icon;
mod status_indicator;
mod big_text;
mod digits;
mod metric_display;
mod picture;
mod skeleton;

pub use button::Button;
pub use text_input::TextInput;
pub use spinner::Spinner;
pub use progress::ProgressBar;
pub use switch::{Switch, SwitchState, SwitchSize};
pub use badge::{Badge, BadgeVariant};
pub use checkbox::{Checkbox, CheckboxState, CheckboxValue};
pub use slider::{Slider, SliderState, SliderMode};
pub use scrollbar::{Scrollbar, ScrollbarMode};
pub use timer::{Timer, TimerMode, TimerFormat, TimerState, create_timer_state};
pub use tag::{Tag, TagVariant, TagSize};
pub use tooltip::{Tooltip, TooltipPosition};
pub use link::{Link, LinkVariant};
pub use icon::{Icon, IconSize, icons};
pub use status_indicator::{StatusIndicator, StatusState, StatusSize};
pub use big_text::{BigText, BigTextFont, BigTextAlign};
pub use digits::{Digits, DigitStyle};
pub use metric_display::{
    MetricDisplay, MetricState, MetricLayout, MetricSize,
    ThresholdConfig, ThresholdRange, get_threshold_color, create_metric,
};
pub use picture::{
    Picture, PictureProps, PictureFit, PictureAlignX, PictureAlignY,
    ColoredPicture, Pixel, PixelGrid, ColorPalette,
    create_pixel_grid, create_pixel_grid_from_colors,
    AsciiPatterns, create_banner, rainbow_text,
};
pub use skeleton::{
    Skeleton, SkeletonText, SkeletonCard,
    SkeletonChars, SKELETON_ASCII, SKELETON_BLOCKS, SKELETON_DOTS,
};
