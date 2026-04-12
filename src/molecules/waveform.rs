//! Waveform Component - Audio Visualization
//!
//! Renders audio waveforms, spectrum analyzers, and equalizer-style
//! visualizations using Unicode block characters.
//!
//! # Example
//!
//! ```rust,ignore
//! use tuiuiu::molecules::{Waveform, WaveformStyle};
//!
//! let audio_data = vec![0.2, 0.5, 0.8, 0.6, 0.3, 0.9, 0.4, 0.7];
//!
//! let waveform = Waveform::new(audio_data)
//!     .width(40)
//!     .height(8)
//!     .style(WaveformStyle::Bars)
//!     .show_peaks(true);
//! ```

use crate::core::component::{
    BorderStyle, BoxNode, BoxStyle, Color, NamedColor, TextNode, TextStyle, VNode,
};

// =============================================================================
// Types
// =============================================================================

/// Waveform visualization style.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum WaveformStyle {
    /// Vertical bars (equalizer style).
    #[default]
    Bars,
    /// Classic waveform (centered line).
    Waveform,
    /// Mirrored waveform (SoundCloud style).
    Mirrored,
    /// Spectrum analyzer with frequency-based coloring.
    Spectrum,
    /// Oscilloscope (continuous line).
    Oscilloscope,
}

/// Waveform component configuration.
#[derive(Debug, Clone)]
pub struct Waveform {
    /// Amplitude data (0-1 range or will be normalized).
    data: Vec<f64>,
    /// Width in characters.
    width: usize,
    /// Height in rows.
    height: usize,
    /// Visualization style.
    style: WaveformStyle,
    /// Primary color.
    color: Color,
    /// Secondary color (top of bars).
    color_high: Option<Color>,
    /// Background color.
    background: Option<Color>,
    /// Show peak indicators.
    show_peaks: bool,
    /// Peak color.
    peak_color: Color,
    /// Minimum value for normalization.
    min: Option<f64>,
    /// Maximum value for normalization.
    max: Option<f64>,
    /// Gap between bars (for 'bars' style).
    bar_gap: usize,
    /// Show border around visualization.
    border: bool,
    /// Border color.
    border_color: Color,
    /// Peak values (for real-time updates).
    peaks: Vec<f64>,
}

impl Waveform {
    /// Create a new waveform visualization.
    pub fn new(data: Vec<f64>) -> Self {
        Self {
            data,
            width: 40,
            height: 8,
            style: WaveformStyle::default(),
            color: Color::Named(NamedColor::Green),
            color_high: None,
            background: None,
            show_peaks: false,
            peak_color: Color::Named(NamedColor::Red),
            min: None,
            max: None,
            bar_gap: 0,
            border: false,
            border_color: Color::Named(NamedColor::Gray),
            peaks: Vec::new(),
        }
    }

    /// Set the width.
    pub fn width(mut self, width: usize) -> Self {
        self.width = width;
        self
    }

    /// Set the height.
    pub fn height(mut self, height: usize) -> Self {
        self.height = height;
        self
    }

    /// Set the visualization style.
    pub fn style(mut self, style: WaveformStyle) -> Self {
        self.style = style;
        self
    }

    /// Set the primary color.
    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    /// Set the secondary (high) color for gradient effect.
    pub fn color_high(mut self, color: Color) -> Self {
        self.color_high = Some(color);
        self
    }

    /// Set the background color.
    pub fn background(mut self, color: Color) -> Self {
        self.background = Some(color);
        self
    }

    /// Enable or disable peak indicators.
    pub fn show_peaks(mut self, show: bool) -> Self {
        self.show_peaks = show;
        self
    }

    /// Set the peak color.
    pub fn peak_color(mut self, color: Color) -> Self {
        self.peak_color = color;
        self
    }

    /// Set the minimum value for normalization.
    pub fn min(mut self, min: f64) -> Self {
        self.min = Some(min);
        self
    }

    /// Set the maximum value for normalization.
    pub fn max(mut self, max: f64) -> Self {
        self.max = Some(max);
        self
    }

    /// Set the gap between bars.
    pub fn bar_gap(mut self, gap: usize) -> Self {
        self.bar_gap = gap;
        self
    }

    /// Enable or disable border.
    pub fn border(mut self, border: bool) -> Self {
        self.border = border;
        self
    }

    /// Set the border color.
    pub fn border_color(mut self, color: Color) -> Self {
        self.border_color = color;
        self
    }

    /// Set peak data for real-time visualization.
    pub fn peaks(mut self, peaks: Vec<f64>) -> Self {
        self.peaks = peaks;
        self
    }

    /// Build the VNode.
    pub fn build(self) -> VNode {
        let rows = match self.style {
            WaveformStyle::Bars => self.render_bars(),
            WaveformStyle::Waveform => self.render_waveform(),
            WaveformStyle::Mirrored => self.render_mirrored(),
            WaveformStyle::Spectrum => self.render_spectrum(),
            WaveformStyle::Oscilloscope => self.render_oscilloscope(),
        };

        let content = join_rows(rows);

        if self.border {
            wrap_with_border(content, self.border_color)
        } else {
            content
        }
    }

    /// Normalize data to 0-1 range.
    fn normalize_data(&self) -> Vec<f64> {
        if self.data.is_empty() {
            return vec![];
        }

        let valid_data: Vec<f64> = self
            .data
            .iter()
            .copied()
            .filter(|v| v.is_finite())
            .collect();

        if valid_data.is_empty() {
            return self.data.iter().map(|_| 0.0).collect();
        }

        let min = self
            .min
            .unwrap_or_else(|| valid_data.iter().copied().fold(f64::INFINITY, f64::min));
        let max = self
            .max
            .unwrap_or_else(|| valid_data.iter().copied().fold(f64::NEG_INFINITY, f64::max));

        let range = max - min;
        if range.abs() < f64::EPSILON {
            return self.data.iter().map(|_| 0.5).collect();
        }

        self.data
            .iter()
            .map(|&v| {
                if !v.is_finite() {
                    0.0
                } else {
                    ((v - min) / range).clamp(0.0, 1.0)
                }
            })
            .collect()
    }

    /// Resample data to fit target width.
    fn resample_data(&self, normalized: &[f64]) -> Vec<f64> {
        if normalized.is_empty() {
            return vec![];
        }

        if normalized.len() == self.width {
            return normalized.to_vec();
        }

        let ratio = normalized.len() as f64 / self.width as f64;
        let mut result = Vec::with_capacity(self.width);

        for i in 0..self.width {
            let start = (i as f64 * ratio).floor() as usize;
            let end = ((i + 1) as f64 * ratio).floor() as usize;

            let mut sum = 0.0;
            let mut count = 0;

            for j in start..end.min(normalized.len()) {
                if let Some(&val) = normalized.get(j) {
                    if !val.is_nan() {
                        sum += val;
                        count += 1;
                    }
                }
            }

            result.push(if count > 0 { sum / count as f64 } else { 0.0 });
        }

        result
    }

    /// Render vertical bars style (equalizer).
    fn render_bars(&self) -> Vec<String> {
        let normalized = self.normalize_data();
        let resampled = self.resample_data(&normalized);
        let _color_high = self
            .color_high
            .clone()
            .unwrap_or(Color::Named(NamedColor::Yellow));

        let mut rows = Vec::with_capacity(self.height);

        for row in 0..self.height {
            let row_threshold = 1.0 - (row + 1) as f64 / self.height as f64;
            let mut line = String::new();

            for (col, &value) in resampled.iter().enumerate() {
                let is_bar_gap = self.bar_gap > 0 && (col + 1) % (self.bar_gap + 1) == 0;

                if is_bar_gap {
                    line.push(' ');
                } else if self.show_peaks && !self.peaks.is_empty() {
                    let peak_value = self.peaks.get(col).copied().unwrap_or(0.0);
                    let peak_row = ((1.0 - peak_value) * self.height as f64).floor() as usize;
                    if row == peak_row {
                        line.push('▀');
                    } else if value > row_threshold {
                        line.push('█');
                    } else {
                        line.push(' ');
                    }
                } else if value > row_threshold {
                    line.push('█');
                } else {
                    line.push(' ');
                }
            }

            rows.push(line);
        }

        rows
    }

    /// Render classic waveform style (centered line).
    fn render_waveform(&self) -> Vec<String> {
        let normalized = self.normalize_data();
        let resampled = self.resample_data(&normalized);
        let center_row = self.height / 2;

        let mut rows = Vec::with_capacity(self.height);

        for row in 0..self.height {
            let mut line = String::new();

            for &value in &resampled {
                // Map value to row offset from center
                let amplitude = (value - 0.5) * self.height as f64;
                let target_row = center_row as isize - amplitude.round() as isize;

                if row as isize == target_row {
                    line.push('█');
                } else if row == center_row {
                    line.push('─');
                } else {
                    line.push(' ');
                }
            }

            rows.push(line);
        }

        rows
    }

    /// Render mirrored waveform style (SoundCloud-like).
    fn render_mirrored(&self) -> Vec<String> {
        let normalized = self.normalize_data();
        let resampled = self.resample_data(&normalized);
        let half_height = self.height / 2;

        let mut rows = Vec::with_capacity(self.height);

        for row in 0..self.height {
            let mut line = String::new();

            for &value in &resampled {
                let bar_height = (value * half_height as f64).floor() as usize;
                let dist_from_center = (row as isize - half_height as isize).unsigned_abs();
                let is_in_bar = dist_from_center <= bar_height;

                if is_in_bar {
                    line.push('█');
                } else {
                    line.push(' ');
                }
            }

            rows.push(line);
        }

        rows
    }

    /// Render spectrum analyzer style.
    fn render_spectrum(&self) -> Vec<String> {
        let normalized = self.normalize_data();
        let resampled = self.resample_data(&normalized);

        let mut rows = Vec::with_capacity(self.height);

        for row in 0..self.height {
            let row_threshold = 1.0 - (row + 1) as f64 / self.height as f64;
            let mut line = String::new();

            for &value in &resampled {
                if value > row_threshold {
                    line.push('█');
                } else {
                    line.push(' ');
                }
            }

            rows.push(line);
        }

        rows
    }

    /// Render oscilloscope style (continuous line).
    fn render_oscilloscope(&self) -> Vec<String> {
        let normalized = self.normalize_data();
        let resampled = self.resample_data(&normalized);

        // Build a 2D grid
        let mut grid: Vec<Vec<char>> = vec![vec![' '; self.width]; self.height];

        // Plot the waveform
        for (col, &value) in resampled.iter().enumerate() {
            let row = ((1.0 - value) * (self.height - 1) as f64).round() as usize;
            let row = row.min(self.height - 1);

            // Draw line segment between points
            if col > 0 {
                let prev_value = resampled.get(col - 1).copied().unwrap_or(0.5);
                let prev_row = ((1.0 - prev_value) * (self.height - 1) as f64).round() as usize;
                let prev_row = prev_row.min(self.height - 1);

                let min_row = row.min(prev_row);
                let max_row = row.max(prev_row);

                for r in min_row..=max_row {
                    if r < self.height && col < self.width {
                        grid[r][col] = '│';
                    }
                }
            }

            // Draw the point
            if row < self.height && col < self.width {
                grid[row][col] = '●';
            }
        }

        grid.into_iter()
            .map(|row| row.into_iter().collect())
            .collect()
    }
}

// =============================================================================
// Waveform Buffer for Real-time Visualization
// =============================================================================

/// Rolling buffer for audio visualization with smoothing and peak detection.
#[derive(Debug, Clone)]
pub struct WaveformBuffer {
    /// Number of frequency bins/bars.
    bins: usize,
    /// Smoothing factor (0-1, higher = smoother).
    smoothing: f64,
    /// Peak decay rate (0-1, per frame).
    peak_decay: f64,
    /// Current data values.
    data: Vec<f64>,
    /// Peak values.
    peaks: Vec<f64>,
    /// History for waveform display.
    history: Vec<Vec<f64>>,
    /// Maximum history frames.
    max_history: usize,
}

impl WaveformBuffer {
    /// Create a new waveform buffer.
    pub fn new() -> Self {
        Self::with_config(32, 0.8, 0.95)
    }

    /// Create a buffer with custom configuration.
    pub fn with_config(bins: usize, smoothing: f64, peak_decay: f64) -> Self {
        Self {
            bins,
            smoothing: smoothing.clamp(0.0, 1.0),
            peak_decay: peak_decay.clamp(0.0, 1.0),
            data: vec![0.0; bins],
            peaks: vec![0.0; bins],
            history: Vec::new(),
            max_history: 60,
        }
    }

    /// Update with new audio data.
    pub fn update(&mut self, new_data: &[f64]) {
        // Resample to match bins
        let resampled = resample_to_bins(new_data, self.bins);

        // Apply smoothing
        for i in 0..self.bins {
            let new_value = resampled.get(i).copied().unwrap_or(0.0);
            self.data[i] = self.data[i] * self.smoothing + new_value * (1.0 - self.smoothing);

            // Update peaks
            if self.data[i] > self.peaks[i] {
                self.peaks[i] = self.data[i];
            } else {
                self.peaks[i] *= self.peak_decay;
            }
        }

        // Store in history
        self.history.push(self.data.clone());
        if self.history.len() > self.max_history {
            self.history.remove(0);
        }
    }

    /// Get current data.
    pub fn data(&self) -> &[f64] {
        &self.data
    }

    /// Get peak data.
    pub fn peaks(&self) -> &[f64] {
        &self.peaks
    }

    /// Get flattened history for waveform display.
    pub fn history_flat(&self) -> Vec<f64> {
        self.history
            .iter()
            .map(|frame| {
                let sum: f64 = frame.iter().sum();
                sum / frame.len() as f64
            })
            .collect()
    }

    /// Reset peaks.
    pub fn reset_peaks(&mut self) {
        self.peaks = vec![0.0; self.bins];
    }

    /// Clear all data.
    pub fn clear(&mut self) {
        self.data = vec![0.0; self.bins];
        self.peaks = vec![0.0; self.bins];
        self.history.clear();
    }

    /// Create a Waveform from current data.
    pub fn to_waveform(&self) -> Waveform {
        Waveform::new(self.data.clone()).peaks(self.peaks.clone())
    }
}

impl Default for WaveformBuffer {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// Data Generation Utilities
// =============================================================================

/// Generate audio-like data for testing.
/// Uses a simple deterministic pseudo-random for noise.
pub fn generate_waveform_data(
    length: usize,
    frequency: f64,
    noise: f64,
    amplitude: f64,
) -> Vec<f64> {
    (0..length)
        .map(|i| {
            let sine = (i as f64 * frequency * std::f64::consts::PI * 2.0).sin() * amplitude;
            // Simple deterministic noise based on index
            let pseudo_random = ((i as f64 * 12.9898).sin() * 43758.5453).fract();
            let random_noise = (pseudo_random - 0.5) * noise;
            (0.5 + sine * 0.5 + random_noise).clamp(0.0, 1.0)
        })
        .collect()
}

/// Generate spectrum-like data (higher frequencies = lower amplitude).
/// Uses a simple deterministic pseudo-random for variation.
pub fn generate_spectrum_data(
    bins: usize,
    peak_bin: usize,
    spread: f64,
    variation: f64,
) -> Vec<f64> {
    (0..bins)
        .map(|i| {
            let distance = (i as f64 - peak_bin as f64).abs();
            let gaussian = (-distance * distance / (2.0 * spread * spread)).exp();
            // Simple deterministic noise based on index
            let pseudo_random = ((i as f64 * 78.233).sin() * 43758.5453).fract();
            let random = (pseudo_random - 0.5) * variation;
            (gaussian + random).clamp(0.0, 1.0)
        })
        .collect()
}

// =============================================================================
// Helper Functions
// =============================================================================

fn resample_to_bins(data: &[f64], bins: usize) -> Vec<f64> {
    if data.is_empty() || bins == 0 {
        return vec![0.0; bins];
    }

    if data.len() == bins {
        return data.to_vec();
    }

    let ratio = data.len() as f64 / bins as f64;
    let mut result = Vec::with_capacity(bins);

    for i in 0..bins {
        let start = (i as f64 * ratio).floor() as usize;
        let end = ((i + 1) as f64 * ratio).floor() as usize;

        let mut sum = 0.0;
        let mut count = 0;

        for j in start..end.min(data.len()) {
            if let Some(&val) = data.get(j) {
                if !val.is_nan() {
                    sum += val;
                    count += 1;
                }
            }
        }

        result.push(if count > 0 { sum / count as f64 } else { 0.0 });
    }

    result
}

fn join_rows(rows: Vec<String>) -> VNode {
    let content = rows.join("\n");
    VNode::Box(BoxNode {
        children: vec![VNode::Text(TextNode {
            content,
            style: TextStyle::default(),
        })],
        ..Default::default()
    })
}

fn wrap_with_border(content: VNode, _border_color: Color) -> VNode {
    // Simple border implementation
    VNode::Box(BoxNode {
        style: BoxStyle {
            border_style: Some(BorderStyle::Round),
            padding_left: Some(1),
            padding_right: Some(1),
            ..Default::default()
        },
        children: vec![content],
        ..Default::default()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_waveform_new() {
        let data = vec![0.2, 0.5, 0.8, 0.6, 0.3];
        let waveform = Waveform::new(data.clone());
        assert_eq!(waveform.data, data);
        assert_eq!(waveform.width, 40);
        assert_eq!(waveform.height, 8);
    }

    #[test]
    fn test_waveform_builder() {
        let waveform = Waveform::new(vec![0.5; 10])
            .width(60)
            .height(12)
            .style(WaveformStyle::Spectrum)
            .show_peaks(true);

        assert_eq!(waveform.width, 60);
        assert_eq!(waveform.height, 12);
        assert_eq!(waveform.style, WaveformStyle::Spectrum);
        assert!(waveform.show_peaks);
    }

    #[test]
    fn test_normalize_data() {
        let waveform = Waveform::new(vec![0.0, 50.0, 100.0]);
        let normalized = waveform.normalize_data();

        assert_eq!(normalized.len(), 3);
        assert!((normalized[0] - 0.0).abs() < 0.01);
        assert!((normalized[1] - 0.5).abs() < 0.01);
        assert!((normalized[2] - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_waveform_buffer() {
        let mut buffer = WaveformBuffer::with_config(4, 0.0, 0.9); // No smoothing for predictable test

        buffer.update(&[0.5, 0.6, 0.7, 0.8]);
        assert_eq!(buffer.data().len(), 4);

        // With smoothing = 0, new values replace old directly
        assert!((buffer.data()[0] - 0.5).abs() < 0.01);

        buffer.update(&[0.9, 0.9, 0.9, 0.9]);
        assert!((buffer.data()[0] - 0.9).abs() < 0.01);
    }

    #[test]
    fn test_generate_waveform_data() {
        let data = generate_waveform_data(100, 0.1, 0.1, 0.8);
        assert_eq!(data.len(), 100);

        for &val in &data {
            assert!(val >= 0.0 && val <= 1.0);
        }
    }

    #[test]
    fn test_generate_spectrum_data() {
        let data = generate_spectrum_data(32, 8, 10.0, 0.1);
        assert_eq!(data.len(), 32);

        // Peak should be near the peak_bin
        let max_idx = data
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .map(|(i, _)| i)
            .unwrap();

        assert!((max_idx as i32 - 8).abs() <= 3);
    }
}
