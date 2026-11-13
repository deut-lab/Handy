//! Handy Voice defaults for the first run, and the fork's compact overlay.
//!
//! `resources/default_settings.json` holds the fork's defaults where they
//! differ from upstream's (069fdab7, e7e10aef, e0904326), plus
//! `overlay_style: minimal` (the fork's overlay was the compact one). They
//! are used once, when there is no settings file yet (hook in
//! `settings::get_settings`); an existing settings file is not changed by
//! them. Keys are settings names; a `windows`, `macos` or `linux` section
//! applies on that system only (the fork's backtick shortcut was Windows
//! only); keys starting with `_` are notes. A file with an unknown key or a
//! wrong value is not used at all: upstream's defaults apply, and the log
//! says why.
//!
//! Language settings stay automatic, as in e0904326: recognition `auto`,
//! interface language from the system (both upstream's defaults). Fork
//! defaults of Handy Voice's own settings live with their features
//! (`silence_stop`, `overlay_look`, `tray_style`, `windows_startup`), so they
//! also fill those settings in a file that lacks them.
//!
//! [`keep_fork_overlay_style`] (hook in `settings::apply_settings_migrations`):
//! a settings file written by the fork has no `overlay_style`, which
//! upstream's migration turns into the Live overlay. When the file has the
//! fork's own keys and the overlay was shown, it becomes Minimal instead, the
//! compact overlay the fork had. Only that field changes, once.

use crate::settings::{AppSettings, OverlayStyle};
use log::{info, warn};
use serde_json::{Map, Value};
use tauri::{AppHandle, Manager};

const FILE: &str = "resources/default_settings.json";
const SYSTEMS: [&str; 3] = ["windows", "macos", "linux"];
/// Settings that only the fork's settings files have.
const FORK_KEYS: [&str; 5] = [
    "overlay_theme",
    "overlay_opacity",
    "auto_stop_silence_enabled",
    "windows_task_startup_enabled",
    "long_dictation_mode",
];

/// The settings for the first run: upstream's `defaults` with the values of
/// `default_settings.json`, or `defaults` as they are if the file can't be used.
pub fn first_run(app: &AppHandle, defaults: AppSettings) -> AppSettings {
    let file = app
        .path()
        .resolve(FILE, tauri::path::BaseDirectory::Resource)
        .map_err(|e| e.to_string())
        .and_then(|path| std::fs::read_to_string(path).map_err(|e| e.to_string()))
        .and_then(|text| serde_json::from_str::<Value>(&text).map_err(|e| e.to_string()));
    match file.and_then(|file| with_file(&defaults, &file, std::env::consts::OS)) {
        Ok(settings) => {
            info!("First run: Handy Voice default settings applied");
            settings
        }
        Err(e) => {
            warn!("Handy Voice default settings not applied ({e}); using upstream's");
            defaults
        }
    }
}

/// `defaults` with the values of `file` (as in `default_settings.json`) for
/// the system `os` (`std::env::consts::OS`).
pub fn with_file(defaults: &AppSettings, file: &Value, os: &str) -> Result<AppSettings, String> {
    let file = file.as_object().ok_or("the file is not a JSON object")?;
    let mut merged = serde_json::to_value(defaults).map_err(|e| e.to_string())?;
    let target = merged
        .as_object_mut()
        .ok_or("the defaults are not a JSON object")?;
    let system = file.get(os).and_then(Value::as_object);
    for (key, value) in file.iter().chain(system.into_iter().flatten()) {
        if key.starts_with('_') || SYSTEMS.contains(&key.as_str()) {
            continue;
        }
        if !target.contains_key(key) {
            return Err(format!("unknown setting \"{key}\""));
        }
        if key == "bindings" {
            merge_bindings(target, value)?;
        } else {
            target.insert(key.clone(), value.clone());
        }
    }
    serde_json::from_value(merged).map_err(|e| e.to_string())
}

/// Per shortcut, the given fields replace the default's; others stay.
fn merge_bindings(target: &mut Map<String, Value>, given: &Value) -> Result<(), String> {
    let given = given.as_object().ok_or("bindings is not a JSON object")?;
    let bindings = target
        .get_mut("bindings")
        .and_then(Value::as_object_mut)
        .ok_or("the defaults have no bindings")?;
    for (id, fields) in given {
        let fields = fields
            .as_object()
            .ok_or_else(|| format!("binding \"{id}\" is not a JSON object"))?;
        match bindings.get_mut(id).and_then(Value::as_object_mut) {
            Some(binding) => binding.extend(fields.clone()),
            None => {
                bindings.insert(id.clone(), Value::Object(fields.clone()));
            }
        }
    }
    Ok(())
}

/// Turns the Live overlay that upstream's migration gave a fork settings file
/// (`stored`, as read) into the fork's compact one. True if it changed.
pub fn keep_fork_overlay_style(settings: &mut AppSettings, stored: &Value) -> bool {
    let from_fork = FORK_KEYS.iter().any(|key| stored.get(key).is_some());
    if stored.get("overlay_style").is_some()
        || !from_fork
        || settings.overlay_style != OverlayStyle::Live
    {
        return false;
    }
    settings.overlay_style = OverlayStyle::Minimal;
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::settings::{
        get_default_settings, ClipboardHandling, LogLevel, ModelUnloadTimeout,
        RecordingRetentionPeriod, ShortcutActivation, SoundTheme,
    };
    use serde_json::json;

    fn shipped() -> Value {
        serde_json::from_str(include_str!("../../resources/default_settings.json")).unwrap()
    }

    #[test]
    fn shipped_file_gives_the_fork_defaults() {
        let defaults = get_default_settings();
        let settings = with_file(&defaults, &shipped(), "windows").unwrap();
        assert_eq!(settings.shortcut_activation, ShortcutActivation::Toggle);
        assert!(settings.audio_feedback);
        assert_eq!(settings.sound_theme, SoundTheme::Pop);
        assert!(settings.start_hidden);
        assert!(settings.debug_mode);
        assert_eq!(settings.log_level, LogLevel::Error);
        assert_eq!(settings.model_unload_timeout, ModelUnloadTimeout::Never);
        assert_eq!(settings.history_limit, 999);
        assert_eq!(
            settings.recording_retention_period,
            RecordingRetentionPeriod::Never
        );
        assert_eq!(
            settings.clipboard_handling,
            ClipboardHandling::CopyToClipboard
        );
        assert!(settings.auto_submit);
        assert!(settings.experimental_enabled);
        assert_eq!(settings.extra_recording_buffer_ms, 250);
        assert_eq!(settings.overlay_style, OverlayStyle::Minimal);
        // Languages stay automatic.
        assert_eq!(settings.selected_language, "auto");
        assert_eq!(settings.app_language, defaults.app_language);
        // The model is chosen in onboarding.
        assert_eq!(settings.selected_model, "");
        assert!(!settings.onboarding_completed);
    }

    #[test]
    fn backtick_shortcut_only_on_windows() {
        let defaults = get_default_settings();
        let default_transcribe = &defaults.bindings["transcribe"];

        let windows = with_file(&defaults, &shipped(), "windows").unwrap();
        let transcribe = &windows.bindings["transcribe"];
        assert_eq!(transcribe.default_binding, "`");
        assert_eq!(transcribe.current_binding, "`");
        assert_eq!(transcribe.name, default_transcribe.name);
        assert_eq!(windows.bindings.len(), defaults.bindings.len());

        for os in ["macos", "linux"] {
            let other = with_file(&defaults, &shipped(), os).unwrap();
            assert_eq!(
                other.bindings["transcribe"].current_binding,
                default_transcribe.current_binding
            );
        }
    }

    #[test]
    fn a_file_with_a_mistake_is_not_used() {
        let defaults = get_default_settings();
        assert!(with_file(&defaults, &json!({ "start_hiden": true }), "windows").is_err());
        assert!(with_file(&defaults, &json!({ "overlay_style": "big" }), "windows").is_err());
        assert!(with_file(&defaults, &json!(["start_hidden"]), "windows").is_err());
    }

    fn fork_store(extra: Value) -> Value {
        let mut stored = json!({
            "push_to_talk": false,
            "overlay_theme": "calm",
            "overlay_opacity": 30,
            "auto_stop_silence_enabled": true,
            "windows_task_startup_enabled": true,
            "selected_language": "auto"
        });
        stored
            .as_object_mut()
            .unwrap()
            .extend(extra.as_object().unwrap().clone());
        stored
    }

    #[test]
    fn fork_store_without_overlay_style_keeps_the_compact_overlay() {
        let stored = fork_store(json!({}));
        let mut settings: AppSettings = serde_json::from_value(stored.clone()).unwrap();
        // What upstream's migration sets for a shown overlay.
        settings.overlay_style = OverlayStyle::Live;
        let before = serde_json::to_value(&settings).unwrap();

        assert!(keep_fork_overlay_style(&mut settings, &stored));
        assert_eq!(settings.overlay_style, OverlayStyle::Minimal);

        let mut after = serde_json::to_value(&settings).unwrap();
        after["overlay_style"] = before["overlay_style"].clone();
        assert_eq!(after, before, "only overlay_style changes");
    }

    #[test]
    fn stored_overlay_style_is_not_changed() {
        for style in ["live", "minimal", "none"] {
            let stored = fork_store(json!({ "overlay_style": style }));
            let mut settings: AppSettings = serde_json::from_value(stored.clone()).unwrap();
            let kept = settings.overlay_style;
            assert!(!keep_fork_overlay_style(&mut settings, &stored));
            assert_eq!(settings.overlay_style, kept);
        }
    }

    #[test]
    fn hidden_fork_overlay_and_other_stores_are_left_alone() {
        // The fork's overlay was hidden: upstream's migration made it None.
        let stored = fork_store(json!({ "overlay_position": "none" }));
        let mut settings: AppSettings = serde_json::from_value(stored.clone()).unwrap();
        settings.overlay_style = OverlayStyle::None;
        assert!(!keep_fork_overlay_style(&mut settings, &stored));
        assert_eq!(settings.overlay_style, OverlayStyle::None);

        // Not the fork's file.
        let stored = json!({ "push_to_talk": true, "selected_model": "" });
        let mut settings: AppSettings = serde_json::from_value(stored.clone()).unwrap();
        settings.overlay_style = OverlayStyle::Live;
        assert!(!keep_fork_overlay_style(&mut settings, &stored));
        assert_eq!(settings.overlay_style, OverlayStyle::Live);
    }
}
