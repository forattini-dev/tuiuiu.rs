//! Border Style Utilities

pub use crate::core::component::BorderStyle;
pub use crate::core::renderer::BorderChars;

/// All border styles.
pub const BORDER_STYLES: [BorderStyle; 9] = [
    BorderStyle::None,
    BorderStyle::Single,
    BorderStyle::Double,
    BorderStyle::Round,
    BorderStyle::Bold,
    BorderStyle::Dashed,
    BorderStyle::Dotted,
    BorderStyle::Hidden,
    BorderStyle::Classic,
];

/// Get border characters for a style.
pub fn get_border_chars(style: BorderStyle) -> BorderChars {
    crate::core::renderer::get_border_chars(style)
}
