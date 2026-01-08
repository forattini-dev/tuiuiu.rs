//! Molecules - Composite Components
//!
//! Components built from atoms that provide richer functionality:
//! - Selection: Select, MultiSelect, RadioGroup, Autocomplete, TagInput
//! - Data Display: Table, Tree, Calendar
//! - Navigation: Tabs
//! - Content: CodeBlock, Markdown
//! - Data Visualization: Sparkline, BarChart, LineChart, Gauge, Heatmap, Legend, GanttChart, TimeHeatmap, Waveform
//! - Annotations: Chart annotations (threshold, range, text, point)
//! - Collapsible: Collapsible, Accordion
//! - Forms: FormField, FormGroup, ConfirmButton
//! - Feedback: SplashScreen

mod select;
mod table;
mod tabs;
mod tree;
mod calendar;
mod code;
mod charts;
mod autocomplete;
mod collapsible;
mod accordion;
mod form_field;
mod tag_input;
mod combobox;
mod vertical_tabs;
mod confirm_button;
mod legend;
mod splash_screen;
mod gantt_chart;
mod time_heatmap;
mod annotations;
mod waveform;

// Selection components
pub use select::{Select, SelectOption, MultiSelect, RadioGroup};
pub use autocomplete::{Autocomplete, Suggestion};
pub use tag_input::{TagInput, TagItem, TagInputState, create_tag_input_state};
pub use combobox::{Combobox, ComboboxOption, ComboboxState, create_combobox_state};

// Data display
pub use table::{Table, Column, Align};
pub use tree::{Tree, TreeNode, file_tree};
pub use calendar::Calendar;

// Navigation
pub use tabs::{Tabs, Tab, TabStyle};
pub use vertical_tabs::{VerticalTabs, VerticalTab, VerticalTabsState, TabListPosition, create_vertical_tabs_state};

// Content
pub use code::{CodeBlock, CodeTheme, Markdown};

// Charts
pub use charts::{
    Sparkline,
    BarChart, BarItem, BarOrientation,
    LineChart,
    Gauge, GaugeStyle,
    Heatmap,
    RadarChart, RadarAxis, RadarSeries,
    ScatterPlot, ScatterPoint, ScatterSeries,
};
pub use legend::{Legend, LegendItem, LegendLayout, LegendMarker};
pub use gantt_chart::{GanttChart, GanttTask, TaskStatus};
pub use time_heatmap::{TimeHeatmap, HeatmapData, HeatmapCell, TimeScale, create_weekly_heatmap, create_monthly_heatmap};

// Annotations
pub use annotations::{
    Annotation, AnnotationType, LineStyle,
    ThresholdAnnotation, RangeAnnotation, TextAnnotation, PointAnnotation,
    format_annotation_label, get_annotation_color,
    threshold, range, text, point,
};

// Waveform
pub use waveform::{
    Waveform, WaveformStyle, WaveformBuffer,
    generate_waveform_data, generate_spectrum_data,
};

// Collapsible components
pub use collapsible::{Collapsible, CollapsibleState, create_collapsible_state};
pub use accordion::{Accordion, AccordionItem, AccordionState};

// Form components
pub use form_field::{FormField, FormGroup, FieldState};
pub use confirm_button::{ConfirmButton, ConfirmState, ConfirmVariant, ConfirmButtonState, create_confirm_button_state};

// Feedback
pub use splash_screen::SplashScreen;
