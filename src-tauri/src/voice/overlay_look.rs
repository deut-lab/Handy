//! Recording overlay look, as in the Handy Voice fork: color theme, icon set,
//! transcribing icon, button style and transparency.
//!
//! Only the settings live here. The overlay reads them each time it is shown
//! (`src/overlay/voice/OverlayLook.tsx`), the picker is
//! `src/components/settings/voice/OverlayLookPicker.tsx`. Store names and
//! values are the fork's, so a fork `settings_store.json` keeps its choices.
//!
//! - `overlay_theme`: `classic` is Handy's own look (follows the app's light
//!   or dark theme); `calm`, `gray` and `dark` are the fork's fixed palettes.
//! - `overlay_icon_set`: `original` keeps Handy's dot, spinner and thin
//!   buttons; `line` uses line icons (microphone, check, cross).
//! - `overlay_transcribing_icon`: the line icon shown while transcribing,
//!   named after its Lucide icon.
//! - `overlay_button_style`: `circle` (filled round buttons) or `simple`.
//! - `overlay_opacity`: transparency in percent, 0-100 in steps of 10. The
//!   fork's first versions stored `solid` / `medium` / `light`.
//! - `overlay_size`: size of the compact pill in percent, 80-110 in steps of
//!   5 (Handy Voice 0.10; larger would not fit the overlay window).

use crate::settings;
use serde::{de, Deserialize, Deserializer, Serialize};
use specta::Type;
use tauri::AppHandle;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Type, Default)]
#[serde(rename_all = "snake_case")]
pub enum OverlayTheme {
    #[default]
    Calm,
    Classic,
    Dark,
    Gray,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Type, Default)]
#[serde(rename_all = "snake_case")]
pub enum OverlayIconSet {
    Original,
    #[default]
    Line,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Type, Default)]
#[serde(rename_all = "snake_case")]
pub enum OverlayTranscribingIcon {
    #[default]
    ScanText,
    SquareDashedText,
    ScrollText,
    TextCursorInput,
    MessageSquareText,
    TextInitial,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Type, Default)]
#[serde(rename_all = "snake_case")]
pub enum OverlayButtonStyle {
    Simple,
    #[default]
    Circle,
}

pub fn default_opacity() -> u8 {
    30
}

fn normalize_opacity(percent: u64) -> u8 {
    (((percent.min(100) + 5) / 10) * 10) as u8
}

/// Reads `overlay_opacity` as a percentage or one of the fork's old names.
pub fn deserialize_opacity<'de, D>(deserializer: D) -> Result<u8, D::Error>
where
    D: Deserializer<'de>,
{
    match serde_json::Value::deserialize(deserializer)? {
        serde_json::Value::Number(number) => number
            .as_f64()
            .filter(|percent| percent.is_finite() && *percent >= 0.0)
            .map(|percent| normalize_opacity(percent.round() as u64))
            .ok_or_else(|| de::Error::custom("overlay opacity must be 0-100")),
        serde_json::Value::String(text) => {
            let percent = match text.as_str() {
                "solid" => 0,
                "medium" => 20,
                "light" => 30,
                other => other.trim().parse::<u64>().map_err(de::Error::custom)?,
            };
            Ok(normalize_opacity(percent))
        }
        _ => Err(de::Error::custom("overlay opacity must be a number or a string")),
    }
}

pub fn default_size() -> u8 {
    100
}

/// 80-110 % in steps of 5: the pill must fit the 266x50 overlay window.
fn normalize_size(percent: u64) -> u8 {
    ((percent.clamp(80, 110) + 2) / 5 * 5) as u8
}

fn update(app: &AppHandle, change: impl FnOnce(&mut settings::AppSettings)) {
    let mut settings = settings::get_settings(app);
    change(&mut settings);
    settings::write_settings(app, settings);
}

#[tauri::command]
#[specta::specta]
pub fn change_overlay_theme_setting(app: AppHandle, theme: OverlayTheme) -> Result<(), String> {
    update(&app, |settings| settings.overlay_theme = theme);
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub fn change_overlay_icon_set_setting(
    app: AppHandle,
    icon_set: OverlayIconSet,
) -> Result<(), String> {
    update(&app, |settings| settings.overlay_icon_set = icon_set);
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub fn change_overlay_transcribing_icon_setting(
    app: AppHandle,
    icon: OverlayTranscribingIcon,
) -> Result<(), String> {
    update(&app, |settings| settings.overlay_transcribing_icon = icon);
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub fn change_overlay_button_style_setting(
    app: AppHandle,
    button_style: OverlayButtonStyle,
) -> Result<(), String> {
    update(&app, |settings| settings.overlay_button_style = button_style);
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub fn change_overlay_opacity_setting(app: AppHandle, opacity: u8) -> Result<(), String> {
    update(&app, |settings| settings.overlay_opacity = normalize_opacity(opacity.into()));
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub fn change_overlay_size_setting(app: AppHandle, size: u8) -> Result<(), String> {
    update(&app, |settings| settings.overlay_size = normalize_size(size.into()));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::settings::{get_default_settings, AppSettings};

    #[test]
    fn defaults_are_the_forks() {
        let settings = get_default_settings();
        assert_eq!(settings.overlay_theme, OverlayTheme::Calm);
        assert_eq!(settings.overlay_icon_set, OverlayIconSet::Line);
        assert_eq!(
            settings.overlay_transcribing_icon,
            OverlayTranscribingIcon::ScanText
        );
        assert_eq!(settings.overlay_button_style, OverlayButtonStyle::Circle);
        assert_eq!(settings.overlay_opacity, 30);
        assert_eq!(settings.overlay_size, 100);
    }

    #[test]
    fn size_stays_in_the_window_in_steps_of_five() {
        assert_eq!(normalize_size(0), 80);
        assert_eq!(normalize_size(82), 80);
        assert_eq!(normalize_size(83), 85);
        assert_eq!(normalize_size(100), 100);
        assert_eq!(normalize_size(110), 110);
        assert_eq!(normalize_size(250), 110);
    }

    #[test]
    fn store_names_are_the_forks() {
        for (theme, name) in [
            (OverlayTheme::Calm, "calm"),
            (OverlayTheme::Classic, "classic"),
            (OverlayTheme::Dark, "dark"),
            (OverlayTheme::Gray, "gray"),
        ] {
            assert_eq!(serde_json::to_value(theme).unwrap(), name);
        }
        for (icon_set, name) in [
            (OverlayIconSet::Original, "original"),
            (OverlayIconSet::Line, "line"),
        ] {
            assert_eq!(serde_json::to_value(icon_set).unwrap(), name);
        }
        for (icon, name) in [
            (OverlayTranscribingIcon::ScanText, "scan_text"),
            (OverlayTranscribingIcon::SquareDashedText, "square_dashed_text"),
            (OverlayTranscribingIcon::ScrollText, "scroll_text"),
            (OverlayTranscribingIcon::TextCursorInput, "text_cursor_input"),
            (OverlayTranscribingIcon::MessageSquareText, "message_square_text"),
            (OverlayTranscribingIcon::TextInitial, "text_initial"),
        ] {
            assert_eq!(serde_json::to_value(icon).unwrap(), name);
        }
        for (style, name) in [
            (OverlayButtonStyle::Simple, "simple"),
            (OverlayButtonStyle::Circle, "circle"),
        ] {
            assert_eq!(serde_json::to_value(style).unwrap(), name);
        }
    }

    /// The overlay keys as the fork's `settings_store.json` holds them.
    #[test]
    fn fork_settings_are_read_as_they_are() {
        let stored = serde_json::json!({
            "overlay_button_style": "circle",
            "overlay_icon_set": "line",
            "overlay_opacity": 10,
            "overlay_position": "bottom",
            "overlay_theme": "gray",
            "overlay_transcribing_icon": "scan_text"
        });
        let settings: AppSettings = serde_json::from_value(stored).unwrap();
        assert_eq!(settings.overlay_theme, OverlayTheme::Gray);
        assert_eq!(settings.overlay_icon_set, OverlayIconSet::Line);
        assert_eq!(
            settings.overlay_transcribing_icon,
            OverlayTranscribingIcon::ScanText
        );
        assert_eq!(settings.overlay_button_style, OverlayButtonStyle::Circle);
        assert_eq!(settings.overlay_opacity, 10);

        // And written back unchanged.
        let written = serde_json::to_value(&settings).unwrap();
        assert_eq!(written["overlay_theme"], "gray");
        assert_eq!(written["overlay_icon_set"], "line");
        assert_eq!(written["overlay_transcribing_icon"], "scan_text");
        assert_eq!(written["overlay_button_style"], "circle");
        assert_eq!(written["overlay_opacity"], 10);
    }

    #[test]
    fn a_store_without_overlay_keys_gets_the_defaults() {
        let settings: AppSettings = serde_json::from_value(serde_json::json!({})).unwrap();
        assert_eq!(settings.overlay_theme, OverlayTheme::Calm);
        assert_eq!(settings.overlay_opacity, default_opacity());
        assert_eq!(settings.overlay_size, default_size());
    }

    #[test]
    fn opacity_accepts_old_names_and_rounds_numbers() {
        assert_eq!(normalize_opacity(0), 0);
        assert_eq!(normalize_opacity(14), 10);
        assert_eq!(normalize_opacity(15), 20);
        assert_eq!(normalize_opacity(101), 100);

        for (stored, percent) in [
            (serde_json::json!("solid"), 0),
            (serde_json::json!("medium"), 20),
            (serde_json::json!("light"), 30),
            (serde_json::json!("40"), 40),
            (serde_json::json!(57), 60),
            (serde_json::json!(12.4), 10),
        ] {
            let stored = serde_json::json!({ "overlay_opacity": stored });
            let settings: AppSettings = serde_json::from_value(stored).unwrap();
            assert_eq!(settings.overlay_opacity, percent);
        }
    }

    /// A broken value fails the parse, so the store's salvage resets only this
    /// field (`settings::salvage_settings`).
    #[test]
    fn broken_opacity_is_left_to_the_salvage() {
        for stored in [
            serde_json::json!(-10),
            serde_json::json!("lots"),
            serde_json::json!(true),
        ] {
            let stored = serde_json::json!({ "overlay_opacity": stored });
            assert!(serde_json::from_value::<AppSettings>(stored).is_err());
        }
    }
}
