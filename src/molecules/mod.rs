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

mod accordion;
mod annotations;
mod autocomplete;
mod calendar;
mod charts;
mod code;
mod collapsible;
mod combobox;
mod confirm_button;
mod form_field;
mod gantt_chart;
mod legend;
mod select;
mod splash_screen;
mod table;
mod tabs;
mod tag_input;
mod time_heatmap;
mod tree;
mod vertical_tabs;
mod waveform;

// Selection components
pub use autocomplete::{Autocomplete, Suggestion};
pub use combobox::{create_combobox_state, Combobox, ComboboxOption, ComboboxState};
pub use select::{MultiSelect, RadioGroup, Select, SelectOption};
pub use tag_input::{create_tag_input_state, TagInput, TagInputState, TagItem};

// Data display
pub use calendar::Calendar;
pub use table::{Align, Column, Table};
pub use tree::{file_tree, Tree, TreeNode};

// Navigation
pub use tabs::{Tab, TabStyle, Tabs};
pub use vertical_tabs::{
    create_vertical_tabs_state, TabListPosition, VerticalTab, VerticalTabs, VerticalTabsState,
};

// Content
pub use code::{CodeBlock, CodeTheme, Markdown};

// Charts
pub use charts::{
    BarChart, BarItem, BarOrientation, Gauge, GaugeStyle, Heatmap, LineChart, RadarAxis,
    RadarChart, RadarSeries, ScatterPlot, ScatterPoint, ScatterSeries, Sparkline,
};
pub use gantt_chart::{GanttChart, GanttTask, TaskStatus};
pub use legend::{Legend, LegendItem, LegendLayout, LegendMarker};
pub use time_heatmap::{
    create_monthly_heatmap, create_weekly_heatmap, HeatmapCell, HeatmapData, TimeHeatmap, TimeScale,
};

// Annotations
pub use annotations::{
    format_annotation_label, get_annotation_color, point, range, text, threshold, Annotation,
    AnnotationType, LineStyle, PointAnnotation, RangeAnnotation, TextAnnotation,
    ThresholdAnnotation,
};

// Waveform
pub use waveform::{
    generate_spectrum_data, generate_waveform_data, Waveform, WaveformBuffer, WaveformStyle,
};

// Collapsible components
pub use accordion::{Accordion, AccordionItem, AccordionState};
pub use collapsible::{create_collapsible_state, Collapsible, CollapsibleState};

// Form components
pub use confirm_button::{
    create_confirm_button_state, ConfirmButton, ConfirmButtonState, ConfirmState, ConfirmVariant,
};
pub use form_field::{FieldState, FormField, FormGroup};

// Feedback
pub use splash_screen::SplashScreen;
