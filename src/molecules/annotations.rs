//! Chart Annotations - Visual annotations for charts
//!
//! Features:
//! - Threshold lines
//! - Shaded regions
//! - Text labels
//! - Point markers
//! - Custom styling

use crate::core::component::Color;

// =============================================================================
// Types
// =============================================================================

/// Annotation type enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnnotationType {
    Threshold,
    Range,
    Text,
    Point,
}

/// Line style for threshold annotations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LineStyle {
    #[default]
    Solid,
    Dashed,
    Dotted,
}

impl LineStyle {
    /// Get the character pattern for this line style.
    pub fn pattern(&self) -> &'static [char] {
        match self {
            LineStyle::Solid => &['─'],
            LineStyle::Dashed => &['─', '─', ' '],
            LineStyle::Dotted => &['·', ' '],
        }
    }

    /// Render a line of given width.
    pub fn render(&self, width: usize) -> String {
        let pattern = self.pattern();
        let mut result = String::with_capacity(width);
        let mut idx = 0;

        for _ in 0..width {
            result.push(pattern[idx % pattern.len()]);
            idx += 1;
        }

        result
    }
}

/// Threshold annotation - a horizontal line at a specific value.
#[derive(Debug, Clone)]
pub struct ThresholdAnnotation {
    /// Value on axis.
    pub value: f64,
    /// Label for the line.
    pub label: Option<String>,
    /// Color.
    pub color: Option<Color>,
    /// Line style.
    pub line_style: LineStyle,
}

impl ThresholdAnnotation {
    /// Create a new threshold annotation.
    pub fn new(value: f64) -> Self {
        Self {
            value,
            label: None,
            color: None,
            line_style: LineStyle::default(),
        }
    }

    /// Set the label.
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Set the color.
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    /// Set the line style.
    pub fn line_style(mut self, style: LineStyle) -> Self {
        self.line_style = style;
        self
    }

    /// Set dashed line style.
    pub fn dashed(mut self) -> Self {
        self.line_style = LineStyle::Dashed;
        self
    }

    /// Set dotted line style.
    pub fn dotted(mut self) -> Self {
        self.line_style = LineStyle::Dotted;
        self
    }
}

/// Range annotation - a shaded region between two values.
#[derive(Debug, Clone)]
pub struct RangeAnnotation {
    /// Start value.
    pub start: f64,
    /// End value.
    pub end: f64,
    /// Label for the range.
    pub label: Option<String>,
    /// Color.
    pub color: Option<Color>,
    /// Opacity (0.0-1.0).
    pub opacity: f64,
}

impl RangeAnnotation {
    /// Create a new range annotation.
    pub fn new(start: f64, end: f64) -> Self {
        Self {
            start,
            end,
            label: None,
            color: None,
            opacity: 0.3,
        }
    }

    /// Set the label.
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Set the color.
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    /// Set the opacity.
    pub fn opacity(mut self, opacity: f64) -> Self {
        self.opacity = opacity.clamp(0.0, 1.0);
        self
    }

    /// Check if a value is within this range.
    pub fn contains(&self, value: f64) -> bool {
        value >= self.start && value <= self.end
    }
}

/// Text annotation - a label at specific coordinates.
#[derive(Debug, Clone)]
pub struct TextAnnotation {
    /// X coordinate.
    pub x: f64,
    /// Y coordinate.
    pub y: f64,
    /// Text content.
    pub text: String,
    /// Text color.
    pub color: Option<Color>,
    /// Background color.
    pub background_color: Option<Color>,
}

impl TextAnnotation {
    /// Create a new text annotation.
    pub fn new(x: f64, y: f64, text: impl Into<String>) -> Self {
        Self {
            x,
            y,
            text: text.into(),
            color: None,
            background_color: None,
        }
    }

    /// Set the color.
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    /// Set the background color.
    pub fn background_color(mut self, color: Color) -> Self {
        self.background_color = Some(color);
        self
    }
}

/// Point annotation - a marker at specific coordinates.
#[derive(Debug, Clone)]
pub struct PointAnnotation {
    /// X coordinate.
    pub x: f64,
    /// Y coordinate.
    pub y: f64,
    /// Marker character.
    pub marker: char,
    /// Color.
    pub color: Option<Color>,
    /// Label.
    pub label: Option<String>,
}

impl PointAnnotation {
    /// Create a new point annotation.
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            x,
            y,
            marker: '●',
            color: None,
            label: None,
        }
    }

    /// Set the marker character.
    pub fn marker(mut self, marker: char) -> Self {
        self.marker = marker;
        self
    }

    /// Set the color.
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    /// Set the label.
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }
}

/// Unified annotation enum.
#[derive(Debug, Clone)]
pub enum Annotation {
    Threshold(ThresholdAnnotation),
    Range(RangeAnnotation),
    Text(TextAnnotation),
    Point(PointAnnotation),
}

impl Annotation {
    /// Get the annotation type.
    pub fn annotation_type(&self) -> AnnotationType {
        match self {
            Annotation::Threshold(_) => AnnotationType::Threshold,
            Annotation::Range(_) => AnnotationType::Range,
            Annotation::Text(_) => AnnotationType::Text,
            Annotation::Point(_) => AnnotationType::Point,
        }
    }

    /// Get the annotation color.
    pub fn color(&self) -> Option<&Color> {
        match self {
            Annotation::Threshold(a) => a.color.as_ref(),
            Annotation::Range(a) => a.color.as_ref(),
            Annotation::Text(a) => a.color.as_ref(),
            Annotation::Point(a) => a.color.as_ref(),
        }
    }

    /// Get the annotation label.
    pub fn label(&self) -> Option<&str> {
        match self {
            Annotation::Threshold(a) => a.label.as_deref(),
            Annotation::Range(a) => a.label.as_deref(),
            Annotation::Text(a) => Some(&a.text),
            Annotation::Point(a) => a.label.as_deref(),
        }
    }
}

// =============================================================================
// Helpers
// =============================================================================

/// Format annotation label.
pub fn format_annotation_label(annotation: &Annotation) -> String {
    match annotation {
        Annotation::Threshold(a) => {
            a.label.clone().unwrap_or_else(|| format!("{}", a.value))
        }
        Annotation::Range(a) => {
            a.label
                .clone()
                .unwrap_or_else(|| format!("{} - {}", a.start, a.end))
        }
        Annotation::Text(a) => a.text.clone(),
        Annotation::Point(a) => a.label.clone().unwrap_or_else(|| "●".to_string()),
    }
}

/// Get annotation color with default fallback.
pub fn get_annotation_color(annotation: &Annotation, default: Color) -> Color {
    annotation.color().cloned().unwrap_or(default)
}

// =============================================================================
// Builder helpers
// =============================================================================

/// Create a threshold annotation.
pub fn threshold(value: f64) -> ThresholdAnnotation {
    ThresholdAnnotation::new(value)
}

/// Create a range annotation.
pub fn range(start: f64, end: f64) -> RangeAnnotation {
    RangeAnnotation::new(start, end)
}

/// Create a text annotation.
pub fn text(x: f64, y: f64, content: impl Into<String>) -> TextAnnotation {
    TextAnnotation::new(x, y, content)
}

/// Create a point annotation.
pub fn point(x: f64, y: f64) -> PointAnnotation {
    PointAnnotation::new(x, y)
}

// =============================================================================
// Conversion implementations
// =============================================================================

impl From<ThresholdAnnotation> for Annotation {
    fn from(a: ThresholdAnnotation) -> Self {
        Annotation::Threshold(a)
    }
}

impl From<RangeAnnotation> for Annotation {
    fn from(a: RangeAnnotation) -> Self {
        Annotation::Range(a)
    }
}

impl From<TextAnnotation> for Annotation {
    fn from(a: TextAnnotation) -> Self {
        Annotation::Text(a)
    }
}

impl From<PointAnnotation> for Annotation {
    fn from(a: PointAnnotation) -> Self {
        Annotation::Point(a)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_threshold_annotation() {
        let ann = ThresholdAnnotation::new(50.0)
            .label("Midpoint")
            .dashed();

        assert_eq!(ann.value, 50.0);
        assert_eq!(ann.label, Some("Midpoint".to_string()));
        assert_eq!(ann.line_style, LineStyle::Dashed);
    }

    #[test]
    fn test_range_annotation() {
        let ann = RangeAnnotation::new(10.0, 90.0)
            .label("Safe Zone")
            .opacity(0.5);

        assert!(ann.contains(50.0));
        assert!(!ann.contains(5.0));
        assert_eq!(ann.opacity, 0.5);
    }

    #[test]
    fn test_line_style_render() {
        let solid = LineStyle::Solid.render(5);
        assert_eq!(solid, "─────");

        let dashed = LineStyle::Dashed.render(6);
        assert_eq!(dashed, "── ── ");
    }

    #[test]
    fn test_format_annotation_label() {
        let ann = Annotation::Threshold(ThresholdAnnotation::new(100.0));
        assert_eq!(format_annotation_label(&ann), "100");

        let ann_with_label = Annotation::Range(
            RangeAnnotation::new(0.0, 50.0).label("Low"),
        );
        assert_eq!(format_annotation_label(&ann_with_label), "Low");
    }
}
