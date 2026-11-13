//! "Insert without Enter": the ✓ button next to ✕ on the recording overlay.
//!
//! ✓ ends the recording as a stopping press would and pastes the text, but
//! skips auto-submit (the key sent after the paste) for this one dictation;
//! the next one submits again as set. With auto-submit off it is an ordinary
//! stop. As the Handy Voice fork's `finish_without_submit`.
//!
//! The choice travels with the stop: the coordinator runs the stop inside
//! [`with_submit`], `TranscribeAction::stop` reads it with
//! [`submit_allowed`] and hands it to its paste thread ([`set_submit`]),
//! where `clipboard::paste` applies it through [`paste_settings`].

use crate::settings::AppSettings;
use crate::transcription_coordinator::TranscriptionCoordinator;
use std::cell::Cell;
use tauri::{AppHandle, Manager};

thread_local! {
    /// Whether the stop or paste running on this thread may auto-submit.
    static SUBMIT: Cell<bool> = const { Cell::new(true) };
}

/// Puts the previous value back, also if the stop panics.
struct Restore(bool);

impl Drop for Restore {
    fn drop(&mut self) {
        SUBMIT.with(|s| s.set(self.0));
    }
}

/// Runs `f` (a stop) with auto-submit allowed or not on this thread.
pub fn with_submit<R>(submit: bool, f: impl FnOnce() -> R) -> R {
    let _restore = Restore(SUBMIT.with(|s| s.replace(submit)));
    f()
}

/// Whether the stop or paste running on this thread may auto-submit.
pub fn submit_allowed() -> bool {
    SUBMIT.with(Cell::get)
}

/// For a thread of its own, such as one dictation's paste thread.
pub fn set_submit(submit: bool) {
    SUBMIT.with(|s| s.set(submit));
}

/// `settings` for the paste on this thread: auto-submit off when it skips it.
pub fn paste_settings(mut settings: AppSettings) -> AppSettings {
    if !submit_allowed() {
        settings.auto_submit = false;
    }
    settings
}

/// The overlay's ✓: stop, paste, no auto-submit this time.
#[tauri::command]
#[specta::specta]
pub fn finish_without_submit(app: AppHandle) {
    if let Some(coordinator) = app.try_state::<TranscriptionCoordinator>() {
        coordinator.request_stop("overlay-insert-without-enter", false);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::settings::get_default_settings;

    fn submitting() -> AppSettings {
        let mut settings = get_default_settings();
        settings.auto_submit = true;
        settings
    }

    #[test]
    fn submits_by_default() {
        assert!(submit_allowed());
        assert!(paste_settings(submitting()).auto_submit);
    }

    #[test]
    fn a_stop_without_submit_skips_it_only_inside() {
        let inside = with_submit(false, || {
            (submit_allowed(), paste_settings(submitting()).auto_submit)
        });
        assert_eq!(inside, (false, false));
        assert!(submit_allowed());
        assert!(paste_settings(submitting()).auto_submit);
    }

    #[test]
    fn auto_submit_off_stays_off() {
        let mut settings = get_default_settings();
        settings.auto_submit = false;
        assert!(!with_submit(true, || paste_settings(settings.clone())).auto_submit);
        assert!(!with_submit(false, || paste_settings(settings)).auto_submit);
    }

    #[test]
    fn restored_after_a_panicking_stop() {
        let failing_stop = || -> bool { panic!("stop failed") };
        let result = std::panic::catch_unwind(|| with_submit(false, failing_stop));
        assert!(result.is_err());
        assert!(submit_allowed());
    }

    #[test]
    fn the_paste_thread_carries_the_choice() {
        let skipped = std::thread::spawn(|| {
            set_submit(false);
            paste_settings(submitting()).auto_submit
        })
        .join()
        .unwrap();
        assert!(!skipped);
        // Another thread, e.g. the next dictation's paste, submits again.
        let next = std::thread::spawn(|| paste_settings(submitting()).auto_submit)
            .join()
            .unwrap();
        assert!(next);
    }
}
