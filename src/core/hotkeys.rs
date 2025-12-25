//! Hotkey System
//!
//! Parse and match keyboard shortcuts.

use crate::core::terminal::{Key, KeyModifiers};

/// Check if a key event matches a hotkey string.
pub fn is_hotkey(key: &Key, mods: &KeyModifiers, hotkey: &str) -> bool {
    let parsed = parse_hotkey(hotkey);
    matches_hotkey(key, mods, &parsed)
}

/// Parsed hotkey representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedHotkey {
    pub key: Key,
    pub modifiers: KeyModifiers,
}

/// Parse a hotkey string like "ctrl+s" or "cmd+shift+p".
pub fn parse_hotkey(s: &str) -> ParsedHotkey {
    let lowercase = s.to_lowercase();
    let parts: Vec<_> = lowercase.split('+').collect();
    let mut modifiers = KeyModifiers::default();
    let mut key = Key::Null;

    for part in parts {
        match part.trim() {
            "ctrl" | "control" => modifiers.ctrl = true,
            "alt" | "option" => modifiers.alt = true,
            "shift" => modifiers.shift = true,
            "cmd" | "meta" | "super" | "win" => modifiers.meta = true,
            "enter" | "return" => key = Key::Enter,
            "esc" | "escape" => key = Key::Escape,
            "tab" => key = Key::Tab,
            "space" => key = Key::Char(' '),
            "backspace" => key = Key::Backspace,
            "delete" | "del" => key = Key::Delete,
            "up" => key = Key::Up,
            "down" => key = Key::Down,
            "left" => key = Key::Left,
            "right" => key = Key::Right,
            "home" => key = Key::Home,
            "end" => key = Key::End,
            "pageup" => key = Key::PageUp,
            "pagedown" => key = Key::PageDown,
            s if s.len() == 1 => key = Key::Char(s.chars().next().unwrap()),
            s if s.starts_with('f') => {
                if let Ok(n) = s[1..].parse::<u8>() {
                    key = Key::F(n);
                }
            }
            _ => {}
        }
    }

    ParsedHotkey { key, modifiers }
}

/// Check if a key event matches a parsed hotkey.
pub fn matches_hotkey(key: &Key, mods: &KeyModifiers, hotkey: &ParsedHotkey) -> bool {
    key == &hotkey.key
        && mods.ctrl == hotkey.modifiers.ctrl
        && mods.alt == hotkey.modifiers.alt
        && mods.shift == hotkey.modifiers.shift
        && mods.meta == hotkey.modifiers.meta
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_hotkey() {
        let h = parse_hotkey("ctrl+s");
        assert!(h.modifiers.ctrl);
        assert_eq!(h.key, Key::Char('s'));
    }
}
