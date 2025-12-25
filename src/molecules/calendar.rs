//! Calendar Component
//!
//! Date picker and calendar view.

use crate::core::component::{VNode, BoxNode, BoxStyle, TextStyle, Color, NamedColor};

/// Calendar component.
#[derive(Debug, Clone)]
pub struct Calendar {
    year: i32,
    month: u32,
    selected_day: Option<u32>,
    today: Option<(i32, u32, u32)>,
    show_week_numbers: bool,
    start_on_monday: bool,
}

impl Default for Calendar {
    fn default() -> Self {
        Self {
            year: 2025,
            month: 1,
            selected_day: None,
            today: None,
            show_week_numbers: false,
            start_on_monday: true,
        }
    }
}

impl Calendar {
    /// Create a new calendar.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set year and month.
    pub fn date(mut self, year: i32, month: u32) -> Self {
        self.year = year;
        self.month = month.clamp(1, 12);
        self
    }

    /// Set selected day.
    pub fn selected(mut self, day: u32) -> Self {
        self.selected_day = Some(day);
        self
    }

    /// Set today's date (for highlighting).
    pub fn today(mut self, year: i32, month: u32, day: u32) -> Self {
        self.today = Some((year, month, day));
        self
    }

    /// Show week numbers.
    pub fn week_numbers(mut self) -> Self {
        self.show_week_numbers = true;
        self
    }

    /// Start week on Sunday instead of Monday.
    pub fn start_on_sunday(mut self) -> Self {
        self.start_on_monday = false;
        self
    }

    /// Get days in month.
    fn days_in_month(&self) -> u32 {
        match self.month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => {
                if self.year % 4 == 0 && (self.year % 100 != 0 || self.year % 400 == 0) {
                    29
                } else {
                    28
                }
            }
            _ => 30,
        }
    }

    /// Get the day of week for the first day (0 = Mon, 6 = Sun).
    fn first_day_of_week(&self) -> u32 {
        // Zeller's formula (simplified)
        let y = if self.month <= 2 { self.year - 1 } else { self.year };
        let m = if self.month <= 2 { self.month as i32 + 12 } else { self.month as i32 };

        let q = 1i32; // First day
        let k = y % 100;
        let j = y / 100;

        let h = (q + (13 * (m + 1)) / 5 + k + k / 4 + j / 4 - 2 * j) % 7;
        let dow = ((h + 5) % 7) as u32; // Convert to 0=Mon

        if self.start_on_monday {
            dow
        } else {
            (dow + 1) % 7
        }
    }

    /// Get month name.
    fn month_name(&self) -> &'static str {
        match self.month {
            1 => "January",
            2 => "February",
            3 => "March",
            4 => "April",
            5 => "May",
            6 => "June",
            7 => "July",
            8 => "August",
            9 => "September",
            10 => "October",
            11 => "November",
            12 => "December",
            _ => "Unknown",
        }
    }

    /// Build the VNode.
    pub fn build(self) -> VNode {
        let mut children = Vec::new();

        // Header: Month Year
        children.push(VNode::styled_text(
            format!("  {} {}  ", self.month_name(), self.year),
            TextStyle { color: Some(Color::Named(NamedColor::Cyan)), bold: true, ..Default::default() }
        ));

        // Day headers
        let day_headers = if self.start_on_monday {
            "Mo Tu We Th Fr Sa Su"
        } else {
            "Su Mo Tu We Th Fr Sa"
        };

        let header = if self.show_week_numbers {
            format!("   {}", day_headers)
        } else {
            day_headers.to_string()
        };

        children.push(VNode::styled_text(header, TextStyle::color(Color::Named(NamedColor::Gray))));

        // Days grid
        let days_in_month = self.days_in_month();
        let first_dow = self.first_day_of_week();

        let mut current_day = 1u32;
        let mut week = 1;

        while current_day <= days_in_month {
            let mut row = String::new();

            if self.show_week_numbers {
                row.push_str(&format!("{:2} ", week));
            }

            for dow in 0..7 {
                if (week == 1 && dow < first_dow) || current_day > days_in_month {
                    row.push_str("   ");
                } else {
                    let is_selected = self.selected_day == Some(current_day);
                    let is_today = self.today == Some((self.year, self.month, current_day));

                    let day_str = if is_selected {
                        format!("[{:2}]", current_day)
                    } else if is_today {
                        format!("*{:2}", current_day)
                    } else {
                        format!("{:2} ", current_day)
                    };

                    row.push_str(&day_str);
                    current_day += 1;
                }
            }

            let is_today_week = self.today.map(|(y, m, d)| {
                y == self.year && m == self.month &&
                (d >= current_day.saturating_sub(7) && d < current_day)
            }).unwrap_or(false);

            let color = if is_today_week {
                Color::Named(NamedColor::Yellow)
            } else {
                Color::Named(NamedColor::White)
            };

            children.push(VNode::styled_text(row, TextStyle::color(color)));

            week += 1;
        }

        VNode::Box(BoxNode {
            children,
            style: BoxStyle {
                padding_left: Some(1),
                padding_right: Some(1),
                ..Default::default()
            },
            ..Default::default()
        })
    }
}
