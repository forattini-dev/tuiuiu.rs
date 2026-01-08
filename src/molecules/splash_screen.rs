//! Splash Screen Component
//!
//! Full-screen splash/loading screen with logo and progress.

use crate::core::component::{VNode, BoxNode, BoxStyle, TextStyle, Color, NamedColor};
use crate::core::layout::{FlexDirection, JustifyContent, AlignItems};

/// Splash screen component.
#[derive(Debug, Clone)]
pub struct SplashScreen {
    title: String,
    subtitle: Option<String>,
    logo: Option<Vec<String>>,
    version: Option<String>,
    loading_text: Option<String>,
    progress: Option<f32>,
    show_spinner: bool,
    color: Color,
    background: Option<Color>,
}

impl Default for SplashScreen {
    fn default() -> Self {
        Self {
            title: "Application".to_string(),
            subtitle: None,
            logo: None,
            version: None,
            loading_text: Some("Loading...".to_string()),
            progress: None,
            show_spinner: true,
            color: Color::Named(NamedColor::Cyan),
            background: None,
        }
    }
}

impl SplashScreen {
    /// Create a new splash screen.
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            ..Default::default()
        }
    }

    /// Set subtitle.
    pub fn subtitle(mut self, subtitle: impl Into<String>) -> Self {
        self.subtitle = Some(subtitle.into());
        self
    }

    /// Set ASCII logo lines.
    pub fn logo<I, S>(mut self, lines: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.logo = Some(lines.into_iter().map(|s| s.into()).collect());
        self
    }

    /// Set version string.
    pub fn version(mut self, version: impl Into<String>) -> Self {
        self.version = Some(version.into());
        self
    }

    /// Set loading text.
    pub fn loading_text(mut self, text: impl Into<String>) -> Self {
        self.loading_text = Some(text.into());
        self
    }

    /// Set progress (0.0 - 1.0).
    pub fn progress(mut self, progress: f32) -> Self {
        self.progress = Some(progress.clamp(0.0, 1.0));
        self
    }

    /// Show/hide spinner.
    pub fn spinner(mut self, show: bool) -> Self {
        self.show_spinner = show;
        self
    }

    /// Set primary color.
    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    /// Set background color.
    pub fn background(mut self, color: Color) -> Self {
        self.background = Some(color);
        self
    }

    /// Build a simple progress bar.
    fn build_progress_bar(&self, progress: f32, width: usize) -> VNode {
        let filled = (progress * width as f32) as usize;
        let empty = width.saturating_sub(filled);

        let bar = format!(
            "[{}{}] {}%",
            "█".repeat(filled),
            "░".repeat(empty),
            (progress * 100.0) as u8
        );

        VNode::styled_text(
            bar,
            TextStyle {
                color: Some(self.color.clone()),
                ..Default::default()
            },
        )
    }

    /// Build spinner animation frame.
    fn build_spinner(&self) -> VNode {
        // Use a simple spinner character
        VNode::styled_text(
            "⠋".to_string(),
            TextStyle {
                color: Some(self.color.clone()),
                bold: true,
                ..Default::default()
            },
        )
    }

    /// Build the VNode.
    pub fn build(self) -> VNode {
        let mut children: Vec<VNode> = Vec::new();

        // Logo
        if let Some(logo_lines) = &self.logo {
            for line in logo_lines {
                children.push(VNode::styled_text(
                    line.clone(),
                    TextStyle {
                        color: Some(self.color.clone()),
                        bold: true,
                        ..Default::default()
                    },
                ));
            }
            children.push(VNode::text(String::new())); // Spacer
        }

        // Title
        children.push(VNode::styled_text(
            self.title.clone(),
            TextStyle {
                color: Some(self.color.clone()),
                bold: true,
                ..Default::default()
            },
        ));

        // Subtitle
        if let Some(subtitle) = &self.subtitle {
            children.push(VNode::styled_text(
                subtitle.clone(),
                TextStyle {
                    color: Some(Color::Named(NamedColor::Gray)),
                    ..Default::default()
                },
            ));
        }

        // Version
        if let Some(version) = &self.version {
            children.push(VNode::styled_text(
                format!("v{}", version),
                TextStyle {
                    color: Some(Color::Named(NamedColor::Gray)),
                    dim: true,
                    ..Default::default()
                },
            ));
        }

        children.push(VNode::text(String::new())); // Spacer

        // Progress or spinner
        if let Some(progress) = self.progress {
            children.push(self.build_progress_bar(progress, 30));
        } else if self.show_spinner {
            let spinner = self.build_spinner();
            let loading = if let Some(text) = &self.loading_text {
                text.clone()
            } else {
                String::new()
            };

            children.push(VNode::Box(BoxNode {
                children: vec![
                    spinner,
                    VNode::styled_text(
                        format!(" {}", loading),
                        TextStyle {
                            color: Some(Color::Named(NamedColor::Gray)),
                            ..Default::default()
                        },
                    ),
                ],
                style: BoxStyle {
                    flex_direction: Some(FlexDirection::Row),
                    ..Default::default()
                },
                ..Default::default()
            }));
        }

        VNode::Box(BoxNode {
            children,
            style: BoxStyle {
                flex_direction: Some(FlexDirection::Column),
                justify_content: Some(JustifyContent::Center),
                align_items: Some(AlignItems::Center),
                flex_grow: Some(1.0),
                ..Default::default()
            },
            ..Default::default()
        })
    }
}
