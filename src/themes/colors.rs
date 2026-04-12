//! Tailwind CSS Color Palette
//!
//! Complete color palette with all Tailwind colors and shades.
//! Each color has shades from 50 (lightest) to 950 (darkest).

/// Color shades available in the palette
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Shade {
    S50,
    S100,
    S200,
    S300,
    S400,
    S500,
    S600,
    S700,
    S800,
    S900,
    S950,
}

impl Shade {
    /// Get shade index (0-10)
    pub fn index(&self) -> usize {
        match self {
            Shade::S50 => 0,
            Shade::S100 => 1,
            Shade::S200 => 2,
            Shade::S300 => 3,
            Shade::S400 => 4,
            Shade::S500 => 5,
            Shade::S600 => 6,
            Shade::S700 => 7,
            Shade::S800 => 8,
            Shade::S900 => 9,
            Shade::S950 => 10,
        }
    }
}

/// A color scale with all shades
#[derive(Debug, Clone, Copy)]
pub struct ColorScale {
    pub s50: &'static str,
    pub s100: &'static str,
    pub s200: &'static str,
    pub s300: &'static str,
    pub s400: &'static str,
    pub s500: &'static str,
    pub s600: &'static str,
    pub s700: &'static str,
    pub s800: &'static str,
    pub s900: &'static str,
    pub s950: &'static str,
}

impl ColorScale {
    /// Get a specific shade
    pub fn shade(&self, shade: Shade) -> &'static str {
        match shade {
            Shade::S50 => self.s50,
            Shade::S100 => self.s100,
            Shade::S200 => self.s200,
            Shade::S300 => self.s300,
            Shade::S400 => self.s400,
            Shade::S500 => self.s500,
            Shade::S600 => self.s600,
            Shade::S700 => self.s700,
            Shade::S800 => self.s800,
            Shade::S900 => self.s900,
            Shade::S950 => self.s950,
        }
    }

    /// Get all shades as an array
    pub fn all(&self) -> [&'static str; 11] {
        [
            self.s50, self.s100, self.s200, self.s300, self.s400, self.s500, self.s600, self.s700,
            self.s800, self.s900, self.s950,
        ]
    }
}

// =============================================================================
// Base Colors
// =============================================================================

pub const WHITE: &str = "#ffffff";
pub const BLACK: &str = "#000000";
pub const TRANSPARENT: &str = "transparent";

// =============================================================================
// Neutral Colors
// =============================================================================

pub const SLATE: ColorScale = ColorScale {
    s50: "#f8fafc",
    s100: "#f1f5f9",
    s200: "#e2e8f0",
    s300: "#cbd5e1",
    s400: "#94a3b8",
    s500: "#64748b",
    s600: "#475569",
    s700: "#334155",
    s800: "#1e293b",
    s900: "#0f172a",
    s950: "#020617",
};

pub const GRAY: ColorScale = ColorScale {
    s50: "#f9fafb",
    s100: "#f3f4f6",
    s200: "#e5e7eb",
    s300: "#d1d5db",
    s400: "#9ca3af",
    s500: "#6b7280",
    s600: "#4b5563",
    s700: "#374151",
    s800: "#1f2937",
    s900: "#111827",
    s950: "#030712",
};

pub const ZINC: ColorScale = ColorScale {
    s50: "#fafafa",
    s100: "#f4f4f5",
    s200: "#e4e4e7",
    s300: "#d4d4d8",
    s400: "#a1a1aa",
    s500: "#71717a",
    s600: "#52525b",
    s700: "#3f3f46",
    s800: "#27272a",
    s900: "#18181b",
    s950: "#09090b",
};

pub const NEUTRAL: ColorScale = ColorScale {
    s50: "#fafafa",
    s100: "#f5f5f5",
    s200: "#e5e5e5",
    s300: "#d4d4d4",
    s400: "#a3a3a3",
    s500: "#737373",
    s600: "#525252",
    s700: "#404040",
    s800: "#262626",
    s900: "#171717",
    s950: "#0a0a0a",
};

pub const STONE: ColorScale = ColorScale {
    s50: "#fafaf9",
    s100: "#f5f5f4",
    s200: "#e7e5e4",
    s300: "#d6d3d1",
    s400: "#a8a29e",
    s500: "#78716c",
    s600: "#57534e",
    s700: "#44403c",
    s800: "#292524",
    s900: "#1c1917",
    s950: "#0c0a09",
};

// =============================================================================
// Color Palette
// =============================================================================

pub const RED: ColorScale = ColorScale {
    s50: "#fef2f2",
    s100: "#fee2e2",
    s200: "#fecaca",
    s300: "#fca5a5",
    s400: "#f87171",
    s500: "#ef4444",
    s600: "#dc2626",
    s700: "#b91c1c",
    s800: "#991b1b",
    s900: "#7f1d1d",
    s950: "#450a0a",
};

pub const ORANGE: ColorScale = ColorScale {
    s50: "#fff7ed",
    s100: "#ffedd5",
    s200: "#fed7aa",
    s300: "#fdba74",
    s400: "#fb923c",
    s500: "#f97316",
    s600: "#ea580c",
    s700: "#c2410c",
    s800: "#9a3412",
    s900: "#7c2d12",
    s950: "#431407",
};

pub const AMBER: ColorScale = ColorScale {
    s50: "#fffbeb",
    s100: "#fef3c7",
    s200: "#fde68a",
    s300: "#fcd34d",
    s400: "#fbbf24",
    s500: "#f59e0b",
    s600: "#d97706",
    s700: "#b45309",
    s800: "#92400e",
    s900: "#78350f",
    s950: "#451a03",
};

pub const YELLOW: ColorScale = ColorScale {
    s50: "#fefce8",
    s100: "#fef9c3",
    s200: "#fef08a",
    s300: "#fde047",
    s400: "#facc15",
    s500: "#eab308",
    s600: "#ca8a04",
    s700: "#a16207",
    s800: "#854d0e",
    s900: "#713f12",
    s950: "#422006",
};

pub const LIME: ColorScale = ColorScale {
    s50: "#f7fee7",
    s100: "#ecfccb",
    s200: "#d9f99d",
    s300: "#bef264",
    s400: "#a3e635",
    s500: "#84cc16",
    s600: "#65a30d",
    s700: "#4d7c0f",
    s800: "#3f6212",
    s900: "#365314",
    s950: "#1a2e05",
};

pub const GREEN: ColorScale = ColorScale {
    s50: "#f0fdf4",
    s100: "#dcfce7",
    s200: "#bbf7d0",
    s300: "#86efac",
    s400: "#4ade80",
    s500: "#22c55e",
    s600: "#16a34a",
    s700: "#15803d",
    s800: "#166534",
    s900: "#14532d",
    s950: "#052e16",
};

pub const EMERALD: ColorScale = ColorScale {
    s50: "#ecfdf5",
    s100: "#d1fae5",
    s200: "#a7f3d0",
    s300: "#6ee7b7",
    s400: "#34d399",
    s500: "#10b981",
    s600: "#059669",
    s700: "#047857",
    s800: "#065f46",
    s900: "#064e3b",
    s950: "#022c22",
};

pub const TEAL: ColorScale = ColorScale {
    s50: "#f0fdfa",
    s100: "#ccfbf1",
    s200: "#99f6e4",
    s300: "#5eead4",
    s400: "#2dd4bf",
    s500: "#14b8a6",
    s600: "#0d9488",
    s700: "#0f766e",
    s800: "#115e59",
    s900: "#134e4a",
    s950: "#042f2e",
};

pub const CYAN: ColorScale = ColorScale {
    s50: "#ecfeff",
    s100: "#cffafe",
    s200: "#a5f3fc",
    s300: "#67e8f9",
    s400: "#22d3ee",
    s500: "#06b6d4",
    s600: "#0891b2",
    s700: "#0e7490",
    s800: "#155e75",
    s900: "#164e63",
    s950: "#083344",
};

pub const SKY: ColorScale = ColorScale {
    s50: "#f0f9ff",
    s100: "#e0f2fe",
    s200: "#bae6fd",
    s300: "#7dd3fc",
    s400: "#38bdf8",
    s500: "#0ea5e9",
    s600: "#0284c7",
    s700: "#0369a1",
    s800: "#075985",
    s900: "#0c4a6e",
    s950: "#082f49",
};

pub const BLUE: ColorScale = ColorScale {
    s50: "#eff6ff",
    s100: "#dbeafe",
    s200: "#bfdbfe",
    s300: "#93c5fd",
    s400: "#60a5fa",
    s500: "#3b82f6",
    s600: "#2563eb",
    s700: "#1d4ed8",
    s800: "#1e40af",
    s900: "#1e3a8a",
    s950: "#172554",
};

pub const INDIGO: ColorScale = ColorScale {
    s50: "#eef2ff",
    s100: "#e0e7ff",
    s200: "#c7d2fe",
    s300: "#a5b4fc",
    s400: "#818cf8",
    s500: "#6366f1",
    s600: "#4f46e5",
    s700: "#4338ca",
    s800: "#3730a3",
    s900: "#312e81",
    s950: "#1e1b4b",
};

pub const VIOLET: ColorScale = ColorScale {
    s50: "#f5f3ff",
    s100: "#ede9fe",
    s200: "#ddd6fe",
    s300: "#c4b5fd",
    s400: "#a78bfa",
    s500: "#8b5cf6",
    s600: "#7c3aed",
    s700: "#6d28d9",
    s800: "#5b21b6",
    s900: "#4c1d95",
    s950: "#2e1065",
};

pub const PURPLE: ColorScale = ColorScale {
    s50: "#faf5ff",
    s100: "#f3e8ff",
    s200: "#e9d5ff",
    s300: "#d8b4fe",
    s400: "#c084fc",
    s500: "#a855f7",
    s600: "#9333ea",
    s700: "#7e22ce",
    s800: "#6b21a8",
    s900: "#581c87",
    s950: "#3b0764",
};

pub const FUCHSIA: ColorScale = ColorScale {
    s50: "#fdf4ff",
    s100: "#fae8ff",
    s200: "#f5d0fe",
    s300: "#f0abfc",
    s400: "#e879f9",
    s500: "#d946ef",
    s600: "#c026d3",
    s700: "#a21caf",
    s800: "#86198f",
    s900: "#701a75",
    s950: "#4a044e",
};

pub const PINK: ColorScale = ColorScale {
    s50: "#fdf2f8",
    s100: "#fce7f3",
    s200: "#fbcfe8",
    s300: "#f9a8d4",
    s400: "#f472b6",
    s500: "#ec4899",
    s600: "#db2777",
    s700: "#be185d",
    s800: "#9d174d",
    s900: "#831843",
    s950: "#500724",
};

pub const ROSE: ColorScale = ColorScale {
    s50: "#fff1f2",
    s100: "#ffe4e6",
    s200: "#fecdd3",
    s300: "#fda4af",
    s400: "#fb7185",
    s500: "#f43f5e",
    s600: "#e11d48",
    s700: "#be123c",
    s800: "#9f1239",
    s900: "#881337",
    s950: "#4c0519",
};

// =============================================================================
// Helper Functions
// =============================================================================

/// Get a color by name
pub fn get_color(name: &str) -> Option<&'static ColorScale> {
    match name.to_lowercase().as_str() {
        "slate" => Some(&SLATE),
        "gray" => Some(&GRAY),
        "zinc" => Some(&ZINC),
        "neutral" => Some(&NEUTRAL),
        "stone" => Some(&STONE),
        "red" => Some(&RED),
        "orange" => Some(&ORANGE),
        "amber" => Some(&AMBER),
        "yellow" => Some(&YELLOW),
        "lime" => Some(&LIME),
        "green" => Some(&GREEN),
        "emerald" => Some(&EMERALD),
        "teal" => Some(&TEAL),
        "cyan" => Some(&CYAN),
        "sky" => Some(&SKY),
        "blue" => Some(&BLUE),
        "indigo" => Some(&INDIGO),
        "violet" => Some(&VIOLET),
        "purple" => Some(&PURPLE),
        "fuchsia" => Some(&FUCHSIA),
        "pink" => Some(&PINK),
        "rose" => Some(&ROSE),
        _ => None,
    }
}

/// Parse a color string like "blue-500" into a hex value
pub fn parse_color(color_str: &str) -> Option<&'static str> {
    let parts: Vec<&str> = color_str.split('-').collect();
    if parts.len() != 2 {
        return None;
    }

    let scale = get_color(parts[0])?;
    let shade = match parts[1] {
        "50" => Shade::S50,
        "100" => Shade::S100,
        "200" => Shade::S200,
        "300" => Shade::S300,
        "400" => Shade::S400,
        "500" => Shade::S500,
        "600" => Shade::S600,
        "700" => Shade::S700,
        "800" => Shade::S800,
        "900" => Shade::S900,
        "950" => Shade::S950,
        _ => return None,
    };

    Some(scale.shade(shade))
}

/// List all available color names
pub fn list_colors() -> &'static [&'static str] {
    &[
        "slate", "gray", "zinc", "neutral", "stone", "red", "orange", "amber", "yellow", "lime",
        "green", "emerald", "teal", "cyan", "sky", "blue", "indigo", "violet", "purple", "fuchsia",
        "pink", "rose",
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_scale_shade() {
        assert_eq!(BLUE.shade(Shade::S500), "#3b82f6");
        assert_eq!(SLATE.shade(Shade::S900), "#0f172a");
    }

    #[test]
    fn test_parse_color() {
        assert_eq!(parse_color("blue-500"), Some("#3b82f6"));
        assert_eq!(parse_color("red-600"), Some("#dc2626"));
        assert_eq!(parse_color("invalid"), None);
    }

    #[test]
    fn test_get_color() {
        assert!(get_color("blue").is_some());
        assert!(get_color("BLUE").is_some()); // case insensitive
        assert!(get_color("invalid").is_none());
    }
}
