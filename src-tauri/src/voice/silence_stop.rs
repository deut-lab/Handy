//! Auto-stop on silence.
//!
//! Settings `auto_stop_silence_enabled` and `auto_stop_silence_seconds`
//! (1-30; Handy Voice default: on, 7 s). Once speech has been heard, a
//! recording stops by itself after that many seconds without speech, as in
//! the Handy Voice fork.
//!
//! The decision rides on the recording's own VAD: `CaptureSink` hands every
//! frame's verdict to [`SilenceWatch::observe`] (hook in
//! `audio_toolkit/audio/sink.rs`, armed per recording in `managers/audio.rs`).
//! Silence is counted in samples, so Silero's 32 ms and Earshot's 16 ms
//! frames give the same timing. The VAD's own tail after speech (0.45 s, or
//! 1.65 s for streaming models) still counts as speech, as the fork's did.
//!
//! When the time is up, the stop goes to the transcription coordinator as a
//! stopping press would, so overlay, sounds, paste and auto-submit behave as
//! for a stop by shortcut. With Voice Activity Detection off no frame is
//! judged, so nothing stops by itself.

use crate::audio_toolkit::constants::WHISPER_SAMPLE_RATE;
use crate::settings::{self, AppSettings};
use crate::transcription_coordinator::TranscriptionCoordinator;
use log::info;
use tauri::{AppHandle, Manager};

pub const MIN_SECONDS: u64 = 1;
pub const MAX_SECONDS: u64 = 30;

pub fn default_enabled() -> bool {
    true
}

pub fn default_seconds() -> u64 {
    7
}

fn clamp_seconds(seconds: u64) -> u64 {
    seconds.clamp(MIN_SECONDS, MAX_SECONDS)
}

/// Seconds of silence that end a recording, or `None` when auto-stop is off.
fn limit_seconds(settings: &AppSettings) -> Option<u64> {
    settings
        .auto_stop_silence_enabled
        .then(|| clamp_seconds(settings.auto_stop_silence_seconds))
}

/// Counts the silence after speech; fires once per recording.
#[derive(Debug)]
struct SilenceClock {
    limit_samples: u64,
    heard_speech: bool,
    quiet_samples: u64,
    fired: bool,
}

impl SilenceClock {
    fn new(seconds: u64) -> Self {
        Self {
            limit_samples: seconds * u64::from(WHISPER_SAMPLE_RATE),
            heard_speech: false,
            quiet_samples: 0,
            fired: false,
        }
    }

    /// One frame's VAD verdict and its length in 16 kHz samples. True exactly
    /// once: when the silence since the last speech reaches the limit.
    fn observe(&mut self, is_speech: bool, samples: usize) -> bool {
        if self.fired {
            return false;
        }
        if is_speech {
            self.heard_speech = true;
            self.quiet_samples = 0;
            return false;
        }
        if !self.heard_speech {
            return false;
        }
        self.quiet_samples += samples as u64;
        if self.quiet_samples >= self.limit_samples {
            self.fired = true;
        }
        self.fired
    }
}

/// One recording's silence clock and what to do when it runs out. Lives in
/// the recording's `CaptureSink`, on the recorder's delivery thread.
pub struct SilenceWatch {
    clock: SilenceClock,
    on_silence: Box<dyn FnMut() + Send>,
}

impl SilenceWatch {
    pub fn observe(&mut self, is_speech: bool, samples: usize) {
        if self.clock.observe(is_speech, samples) {
            (self.on_silence)();
        }
    }
}

/// A watch for the recording about to start, or `None` when auto-stop is off.
pub fn watch(app: &AppHandle) -> Option<SilenceWatch> {
    let seconds = limit_seconds(&settings::get_settings(app))?;
    let app = app.clone();
    Some(SilenceWatch {
        clock: SilenceClock::new(seconds),
        // Only queues the stop: the coordinator stops the recording on its
        // own thread, which drains this sink.
        on_silence: Box::new(move || {
            info!("Auto-stop: {seconds} s of silence after speech, stopping the recording");
            if let Some(coordinator) = app.try_state::<TranscriptionCoordinator>() {
                coordinator.request_stop("auto-stop-silence", true);
            }
        }),
    })
}

#[tauri::command]
#[specta::specta]
pub fn change_auto_stop_silence_enabled_setting(
    app: AppHandle,
    enabled: bool,
) -> Result<(), String> {
    let mut settings = settings::get_settings(&app);
    settings.auto_stop_silence_enabled = enabled;
    settings::write_settings(&app, settings);
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub fn change_auto_stop_silence_seconds_setting(
    app: AppHandle,
    seconds: u64,
) -> Result<(), String> {
    let mut settings = settings::get_settings(&app);
    settings.auto_stop_silence_seconds = clamp_seconds(seconds);
    settings::write_settings(&app, settings);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::settings::get_default_settings;

    /// Silero's frame; Earshot's is half of it.
    const FRAME: usize = 512;

    /// Frames of silence after speech until the clock fires (None: never
    /// within `max_frames`).
    fn quiet_frames_until_stop(
        clock: &mut SilenceClock,
        frame: usize,
        max_frames: usize,
    ) -> Option<usize> {
        (1..=max_frames).find(|_| clock.observe(false, frame))
    }

    #[test]
    fn silence_before_any_speech_never_stops() {
        let mut clock = SilenceClock::new(1);
        for _ in 0..10_000 {
            assert!(!clock.observe(false, FRAME));
        }
    }

    #[test]
    fn stops_after_the_set_seconds_of_silence_following_speech() {
        let mut clock = SilenceClock::new(7);
        assert!(!clock.observe(false, FRAME));
        assert!(!clock.observe(true, FRAME));
        // 7 s at 16 kHz = 112000 samples = 218.75 frames of 512.
        assert_eq!(quiet_frames_until_stop(&mut clock, FRAME, 1_000), Some(219));
    }

    #[test]
    fn fires_only_once() {
        let mut clock = SilenceClock::new(1);
        assert!(!clock.observe(true, FRAME));
        assert!(quiet_frames_until_stop(&mut clock, FRAME, 100).is_some());
        assert!(!clock.observe(false, FRAME));
        assert!(!clock.observe(true, FRAME));
        assert_eq!(quiet_frames_until_stop(&mut clock, FRAME, 1_000), None);
    }

    #[test]
    fn speech_restarts_the_count() {
        let mut clock = SilenceClock::new(1);
        assert!(!clock.observe(true, FRAME));
        // 1 s = 31.25 frames: 30 quiet frames are not enough...
        for _ in 0..30 {
            assert!(!clock.observe(false, FRAME));
        }
        // ...and speech starts the count again.
        assert!(!clock.observe(true, FRAME));
        assert_eq!(quiet_frames_until_stop(&mut clock, FRAME, 100), Some(32));
    }

    #[test]
    fn timing_does_not_depend_on_the_frame_size() {
        let mut silero = SilenceClock::new(2);
        let mut earshot = SilenceClock::new(2);
        assert!(!silero.observe(true, 512));
        assert!(!earshot.observe(true, 256));
        let silero_frames = quiet_frames_until_stop(&mut silero, 512, 1_000).unwrap();
        let earshot_frames = quiet_frames_until_stop(&mut earshot, 256, 1_000).unwrap();
        assert_eq!(silero_frames * 512, 32_256);
        assert_eq!(earshot_frames * 256, 32_000);
    }

    #[test]
    fn seconds_are_kept_between_1_and_30() {
        assert_eq!(clamp_seconds(0), 1);
        assert_eq!(clamp_seconds(1), 1);
        assert_eq!(clamp_seconds(7), 7);
        assert_eq!(clamp_seconds(30), 30);
        assert_eq!(clamp_seconds(31), 30);
        assert_eq!(clamp_seconds(u64::MAX), 30);
    }

    #[test]
    fn limit_follows_the_settings() {
        let mut settings = get_default_settings();
        settings.auto_stop_silence_enabled = false;
        assert_eq!(limit_seconds(&settings), None);
        settings.auto_stop_silence_enabled = true;
        settings.auto_stop_silence_seconds = 12;
        assert_eq!(limit_seconds(&settings), Some(12));
        settings.auto_stop_silence_seconds = 0;
        assert_eq!(limit_seconds(&settings), Some(1));
        settings.auto_stop_silence_seconds = 99;
        assert_eq!(limit_seconds(&settings), Some(30));
    }

    #[test]
    fn defaults_are_the_forks() {
        let settings = get_default_settings();
        assert!(settings.auto_stop_silence_enabled);
        assert_eq!(settings.auto_stop_silence_seconds, 7);
    }

    /// The fork's store keeps both keys at the top level of `settings`; they
    /// are read as they are, and a store without them gets the defaults.
    #[test]
    fn fork_store_keys_are_read_as_is() {
        let stored: AppSettings = serde_json::from_value(serde_json::json!({
            "auto_stop_silence_enabled": false,
            "auto_stop_silence_seconds": 12
        }))
        .unwrap();
        assert!(!stored.auto_stop_silence_enabled);
        assert_eq!(stored.auto_stop_silence_seconds, 12);

        let stored: AppSettings = serde_json::from_value(serde_json::json!({
            "auto_stop_silence_enabled": true,
            "auto_stop_silence_seconds": 7
        }))
        .unwrap();
        assert_eq!(limit_seconds(&stored), Some(7));

        let missing: AppSettings = serde_json::from_value(serde_json::json!({})).unwrap();
        assert!(missing.auto_stop_silence_enabled);
        assert_eq!(missing.auto_stop_silence_seconds, 7);
    }
}
