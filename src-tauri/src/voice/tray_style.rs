//! Tray icon styles and the short tray menu.
//!
//! Styles (setting `tray_icon_style`):
//! - `states` (default): a neutral microphone that changes with the state
//!   (idle, recording, transcribing), `resources/tray_state_*.png`;
//! - `original`: upstream Handy's icons;
//! - `logo`: one fixed icon whatever the state.
//!
//! Which variant (light icon for a dark taskbar, dark icon for a light one,
//! colored on Linux) comes from upstream's `tray::get_current_theme`, which
//! already follows the Windows taskbar theme (`SystemUsesLightTheme`). While
//! something needs attention (a notice, macOS Secure Input) the idle icon is
//! upstream's warning badge in every style, so the warning is never hidden.
//!
//! The menu is upstream's without the version line, "Check for Updates",
//! "Unload Model", the keyboard accelerators and the trailing "..." on
//! Settings, as in the Handy Voice fork. The tooltip says "Handy Voice" and
//! the version from the build ([`version_label`]).

use crate::settings;
use crate::tray::{self, AppTheme, TrayIconState};
use serde::{Deserialize, Serialize};
use specta::Type;
use tauri::menu::{Menu, MenuItemKind};
use tauri::{AppHandle, Wry};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Type, Default)]
#[serde(rename_all = "snake_case")]
pub enum TrayIconStyle {
    Original,
    #[default]
    States,
    Logo,
}

/// The tray icon for the chosen style; replaces upstream's
/// [`tray::get_icon_path`] (same `badge`) in `tray::compute_desired`.
pub fn icon_path(
    settings: &settings::AppSettings,
    theme: AppTheme,
    state: TrayIconState,
    badge: bool,
) -> &'static str {
    style_icon_path(settings.tray_icon_style, theme, state, badge)
}

fn style_icon_path(
    style: TrayIconStyle,
    theme: AppTheme,
    state: TrayIconState,
    badge: bool,
) -> &'static str {
    if style == TrayIconStyle::Original || (badge && state == TrayIconState::Idle) {
        return tray::get_icon_path(theme, state, badge);
    }
    let state = match style {
        TrayIconStyle::Logo => TrayIconState::Idle,
        _ => state,
    };
    match (theme, state) {
        (AppTheme::Dark, TrayIconState::Idle) => "resources/tray_state_idle.png",
        (AppTheme::Dark, TrayIconState::Recording) => "resources/tray_state_recording.png",
        (AppTheme::Dark, TrayIconState::Transcribing) => "resources/tray_state_transcribing.png",
        (AppTheme::Light, TrayIconState::Idle) => "resources/tray_state_idle_dark.png",
        (AppTheme::Light, TrayIconState::Recording) => "resources/tray_state_recording_dark.png",
        (AppTheme::Light, TrayIconState::Transcribing) => {
            "resources/tray_state_transcribing_dark.png"
        }
        (AppTheme::Colored, TrayIconState::Idle) => "resources/tray_state_idle_color.png",
        (AppTheme::Colored, TrayIconState::Recording) => {
            "resources/tray_state_recording_color.png"
        }
        (AppTheme::Colored, TrayIconState::Transcribing) => {
            "resources/tray_state_transcribing_color.png"
        }
    }
}

/// The idle icon the tray is created with, before the first sync.
pub fn initial_icon_path(app: &AppHandle, theme: AppTheme) -> &'static str {
    icon_path(&settings::get_settings(app), theme, TrayIconState::Idle, false)
}

#[tauri::command]
#[specta::specta]
pub fn change_tray_icon_style_setting(app: AppHandle, style: TrayIconStyle) -> Result<(), String> {
    let mut settings = settings::get_settings(&app);
    settings.tray_icon_style = style;
    settings::write_settings(&app, settings);
    tray::refresh_tray_icon(&app);
    Ok(())
}

/// Trims upstream's tray menu to the short one. Runs on the finished menu,
/// so upstream's layout logic (warnings, notices, language submenu) stays as
/// it is. Upstream starts both menus with `[version, separator, ...]`.
pub fn simplify_menu(menu: &Menu<Wry>) -> tauri::Result<()> {
    for id in ["version", "check_updates", "unload_model"] {
        if let Some(item) = menu.get(id) {
            menu.remove(&item)?;
        }
    }
    // The separator that followed the version line now leads the menu.
    if let Some(MenuItemKind::Predefined(_)) = menu.items()?.first() {
        menu.remove_at(0)?;
    }
    for id in ["settings", "quit"] {
        if let Some(MenuItemKind::MenuItem(item)) = menu.get(id) {
            item.set_accelerator(None::<&str>)?;
            let text = item.text()?;
            let trimmed = text.trim_end_matches("...").trim_end_matches('…');
            if trimmed.len() != text.len() {
                item.set_text(trimmed.trim_end())?;
            }
        }
    }
    Ok(())
}

/// The tray tooltip, for upstream's `tray::version_label`: "Handy Voice
/// v0.10.0", with the version from the build; a build number ("0.10.0+1")
/// shows as "0.10.0.1", as in About and the footer.
pub fn version_label() -> String {
    let version = env!("CARGO_PKG_VERSION").replace('+', ".");
    if cfg!(debug_assertions) {
        format!("Handy Voice v{version} (Dev)")
    } else {
        format!("Handy Voice v{version}")
    }
}

#[cfg(test)]
mod tests {
    use super::{style_icon_path as icon_path, version_label, TrayIconStyle};
    use crate::settings::get_default_settings;
    use crate::tray::{get_icon_path, AppTheme, TrayIconState};
    use std::path::Path;

    const THEMES: [AppTheme; 3] = [AppTheme::Dark, AppTheme::Light, AppTheme::Colored];
    const STATES: [TrayIconState; 3] = [
        TrayIconState::Idle,
        TrayIconState::Recording,
        TrayIconState::Transcribing,
    ];

    #[test]
    fn default_style_is_states_and_store_names_are_stable() {
        assert_eq!(get_default_settings().tray_icon_style, TrayIconStyle::States);
        for (style, name) in [
            (TrayIconStyle::Original, "original"),
            (TrayIconStyle::States, "states"),
            (TrayIconStyle::Logo, "logo"),
        ] {
            assert_eq!(serde_json::to_value(style).unwrap(), name);
        }
    }

    #[test]
    fn every_icon_is_bundled() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"));
        for style in [
            TrayIconStyle::Original,
            TrayIconStyle::States,
            TrayIconStyle::Logo,
        ] {
            for theme in THEMES {
                for state in STATES {
                    for badge in [false, true] {
                        let path = icon_path(style, theme.clone(), state, badge);
                        assert!(root.join(path).is_file(), "{path} is missing");
                    }
                }
            }
        }
    }

    #[test]
    fn original_style_is_upstream() {
        for theme in THEMES {
            for state in STATES {
                assert_eq!(
                    icon_path(TrayIconStyle::Original, theme.clone(), state, false),
                    get_icon_path(theme.clone(), state, false)
                );
            }
        }
    }

    #[test]
    fn states_style_follows_the_state() {
        assert_eq!(
            icon_path(TrayIconStyle::States, AppTheme::Dark, TrayIconState::Idle, false),
            "resources/tray_state_idle.png"
        );
        assert_eq!(
            icon_path(
                TrayIconStyle::States,
                AppTheme::Light,
                TrayIconState::Recording,
                false
            ),
            "resources/tray_state_recording_dark.png"
        );
        assert_eq!(
            icon_path(
                TrayIconStyle::States,
                AppTheme::Dark,
                TrayIconState::Transcribing,
                false
            ),
            "resources/tray_state_transcribing.png"
        );
    }

    #[test]
    fn logo_style_does_not_change_by_state() {
        for theme in THEMES {
            for state in STATES {
                assert_eq!(
                    icon_path(TrayIconStyle::Logo, theme.clone(), state, false),
                    icon_path(TrayIconStyle::Logo, theme.clone(), TrayIconState::Idle, false)
                );
            }
        }
    }

    #[test]
    fn the_warning_badge_shows_in_every_style() {
        for style in [TrayIconStyle::States, TrayIconStyle::Logo] {
            assert_eq!(
                icon_path(style, AppTheme::Dark, TrayIconState::Idle, true),
                get_icon_path(AppTheme::Dark, TrayIconState::Idle, true)
            );
            // Busy states keep the style's own icon, as upstream does.
            assert_eq!(
                icon_path(style, AppTheme::Dark, TrayIconState::Recording, true),
                icon_path(style, AppTheme::Dark, TrayIconState::Recording, false)
            );
        }
    }

    #[test]
    fn tooltip_names_handy_voice_and_the_built_version() {
        let label = version_label();
        assert!(label.starts_with("Handy Voice v"));
        assert!(label.contains(&env!("CARGO_PKG_VERSION").replace('+', ".")));
    }
}
