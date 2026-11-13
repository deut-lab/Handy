//! Start at sign-in through a Windows task.
//!
//! Settings `windows_task_startup_enabled` and `windows_task_startup_admin`
//! (store names from the Handy Voice fork; default on, on Windows). The task
//! "Handy Voice Admin Startup" starts Handy Voice when the user signs in,
//! with administrator rights when asked, so dictation also reaches windows
//! that run as administrator. Upstream's regular autostart (`autostart.rs`)
//! stays as it is, beside it.
//!
//! The task is made by `resources/windows_startup_task.ps1`, which the
//! installer uses too (`nsis/voice_hooks.nsh`). Its name and the exe path are
//! the fork's, so an install over the fork updates the user's task in place.
//!
//! At start (hook in `lib.rs`) the task is brought in line with the settings
//! in the background, without any prompt: a task that already matches is left
//! alone, and one the user disabled in Task Scheduler stays disabled. A change
//! of the switches may ask for administrator rights (UAC). Dev builds leave
//! the task alone, so they never point it at a dev exe. Nothing happens on
//! other systems.

use crate::settings;
use tauri::AppHandle;

pub fn default_enabled() -> bool {
    cfg!(target_os = "windows")
}

pub fn default_admin() -> bool {
    cfg!(target_os = "windows")
}

/// Brings the task in line with the settings, in the background, without
/// asking for administrator rights.
pub fn sync_at_start(app: &AppHandle) {
    sync_in_background(app.clone(), false);
}

#[tauri::command]
#[specta::specta]
pub fn change_windows_task_startup_setting(app: AppHandle, enabled: bool) -> Result<(), String> {
    let mut settings = settings::get_settings(&app);
    settings.windows_task_startup_enabled = enabled;
    settings::write_settings(&app, settings);
    sync_in_background(app, true);
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub fn change_windows_task_startup_admin_setting(
    app: AppHandle,
    enabled: bool,
) -> Result<(), String> {
    let mut settings = settings::get_settings(&app);
    settings.windows_task_startup_admin = enabled;
    settings::write_settings(&app, settings);
    sync_in_background(app, true);
    Ok(())
}

#[cfg(target_os = "windows")]
fn sync_in_background(app: AppHandle, may_elevate: bool) {
    std::thread::spawn(move || imp::sync(&app, may_elevate));
}

#[cfg(not(target_os = "windows"))]
fn sync_in_background(_app: AppHandle, _may_elevate: bool) {}

#[cfg(target_os = "windows")]
mod imp {
    use crate::settings;
    use log::{info, warn};
    use std::ffi::OsString;
    use std::os::windows::process::CommandExt;
    use std::path::Path;
    use std::process::Command;
    use std::sync::Mutex;
    use tauri::{AppHandle, Manager};

    const SCRIPT: &str = "resources/windows_startup_task.ps1";
    const CREATE_NO_WINDOW: u32 = 0x0800_0000;
    /// Exit code of the script when the change needs administrator rights.
    const NEEDS_ADMIN: i32 = 5;

    /// One change at a time; each reads the settings when it runs, so the
    /// last one leaves the task as the settings are now.
    static SYNC: Mutex<()> = Mutex::new(());

    pub fn sync(app: &AppHandle, may_elevate: bool) {
        if cfg!(debug_assertions) {
            info!("Dev build: Windows startup task left as it is");
            return;
        }
        let _one_at_a_time = SYNC.lock().unwrap_or_else(|e| e.into_inner());

        let settings = settings::get_settings(app);
        let script = match app
            .path()
            .resolve(SCRIPT, tauri::path::BaseDirectory::Resource)
        {
            Ok(path) => path,
            Err(e) => {
                warn!("Windows startup task: script not found: {e}");
                return;
            }
        };
        let exe = match std::env::current_exe() {
            Ok(path) => path,
            Err(e) => {
                warn!("Windows startup task: app path unknown: {e}");
                return;
            }
        };

        let args = sync_args(
            &script,
            &exe,
            settings.windows_task_startup_enabled,
            settings.windows_task_startup_admin,
            may_elevate,
        );
        let output = Command::new("powershell.exe")
            .args(args)
            .creation_flags(CREATE_NO_WINDOW)
            .output();
        match output {
            Ok(out) if out.status.success() => {
                info!(
                    "Windows startup task: {}",
                    String::from_utf8_lossy(&out.stdout).trim()
                );
            }
            Ok(out) if out.status.code() == Some(NEEDS_ADMIN) => {
                info!("Windows startup task not changed: administrator rights are needed");
            }
            Ok(out) => {
                let stderr = String::from_utf8_lossy(&out.stderr);
                let stdout = String::from_utf8_lossy(&out.stdout);
                warn!(
                    "Windows startup task not changed ({}): {} {}",
                    out.status,
                    stdout.trim(),
                    stderr.trim()
                );
            }
            Err(e) => warn!("Windows startup task: failed to run PowerShell: {e}"),
        }
    }

    pub(super) fn sync_args(
        script: &Path,
        exe: &Path,
        enabled: bool,
        admin: bool,
        may_elevate: bool,
    ) -> Vec<OsString> {
        let flag = |on: bool| if on { "1" } else { "0" };
        let mut args: Vec<OsString> = Vec::new();
        for arg in ["-NoProfile", "-NonInteractive", "-ExecutionPolicy", "Bypass"] {
            args.push(arg.into());
        }
        args.push("-File".into());
        args.push(script.into());
        args.push("-Mode".into());
        args.push("sync".into());
        args.push("-ExePath".into());
        args.push(exe.into());
        if let Some(dir) = exe.parent() {
            args.push("-WorkingDirectory".into());
            args.push(dir.into());
        }
        args.push("-Enabled".into());
        args.push(flag(enabled).into());
        args.push("-Admin".into());
        args.push(flag(admin).into());
        if may_elevate {
            args.push("-Elevate".into());
        }
        args
    }
}

#[cfg(all(test, target_os = "windows"))]
mod tests {
    use super::imp::sync_args;
    use std::ffi::OsString;
    use std::path::Path;

    fn args(enabled: bool, admin: bool, may_elevate: bool) -> Vec<OsString> {
        sync_args(
            Path::new("C:\\Users\\Dima's PC\\Handy Voice\\resources\\windows_startup_task.ps1"),
            Path::new("C:\\Users\\Dima's PC\\Handy Voice\\handy.exe"),
            enabled,
            admin,
            may_elevate,
        )
    }

    fn after<'a>(args: &'a [OsString], flag: &str) -> &'a OsString {
        let at = args.iter().position(|a| a == flag).expect(flag);
        &args[at + 1]
    }

    #[test]
    fn passes_settings_and_paths_as_they_are() {
        let args = args(true, false, false);
        assert_eq!(after(&args, "-Mode"), "sync");
        assert_eq!(after(&args, "-Enabled"), "1");
        assert_eq!(after(&args, "-Admin"), "0");
        assert_eq!(
            after(&args, "-ExePath"),
            "C:\\Users\\Dima's PC\\Handy Voice\\handy.exe"
        );
        assert_eq!(
            after(&args, "-WorkingDirectory"),
            "C:\\Users\\Dima's PC\\Handy Voice"
        );
        assert!(!args.iter().any(|a| a == "-Elevate"));
    }

    #[test]
    fn asks_for_rights_only_when_allowed() {
        assert!(args(false, true, true).iter().any(|a| a == "-Elevate"));
        assert_eq!(after(&args(false, true, true), "-Enabled"), "0");
    }
}
