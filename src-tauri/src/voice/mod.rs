//! Handy Voice additions on top of upstream Handy.
//!
//! Each feature lives in its own file here; upstream files only carry short
//! hooks (a call, a settings field, an enum variant) into these modules, so
//! new Handy releases can be merged with few conflicts.

pub mod default_settings;
pub mod insert_without_enter;
pub mod overlay_look;
pub mod silence_stop;
mod sound_themes;
pub mod tray_style;
pub mod windows_startup;
