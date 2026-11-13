//! Extra feedback sound themes: bell, chime, pluck and six Kenney UI sounds
//! (switcher, click, mouse, soft switch, bright switch, minimal).
//!
//! They are plain `SoundTheme` variants (settings.rs) and play through
//! upstream's `audio_feedback` exactly like marimba and pop:
//! `resources/<name>_start.wav` and `resources/<name>_stop.wav`, resolved
//! from the bundled resources. Where the files come from:
//! `resources/voice_sound_themes_note.txt`.
//!
//! The picker sits in General settings next to the audio feedback toggle
//! (`src/components/settings/voice/SoundThemePicker.tsx`).

#[cfg(test)]
mod tests {
    use crate::settings::SoundTheme;
    use std::path::Path;

    /// Every built-in theme with its stored name (also the file prefix).
    const BUILT_IN: [(SoundTheme, &str); 11] = [
        (SoundTheme::Marimba, "marimba"),
        (SoundTheme::Pop, "pop"),
        (SoundTheme::Bell, "bell"),
        (SoundTheme::Chime, "chime"),
        (SoundTheme::Pluck, "pluck"),
        (SoundTheme::Switcher, "switcher"),
        (SoundTheme::Click, "click"),
        (SoundTheme::Mouse, "mouse"),
        (SoundTheme::SoftSwitch, "soft_switch"),
        (SoundTheme::BrightSwitch, "bright_switch"),
        (SoundTheme::Minimal, "minimal"),
    ];

    #[test]
    fn built_in_themes_store_their_names() {
        for (theme, name) in BUILT_IN {
            assert_eq!(serde_json::to_value(theme).unwrap(), name);
            let parsed: SoundTheme = serde_json::from_value(name.into()).unwrap();
            assert_eq!(parsed, theme);
        }
    }

    #[test]
    fn built_in_themes_have_bundled_start_and_stop_sounds() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"));
        for (theme, name) in BUILT_IN {
            let start = theme.to_start_path();
            let stop = theme.to_stop_path();
            assert_eq!(start, format!("resources/{name}_start.wav"));
            assert_eq!(stop, format!("resources/{name}_stop.wav"));
            assert!(root.join(&start).is_file(), "{start} is missing");
            assert!(root.join(&stop).is_file(), "{stop} is missing");
        }
    }
}
