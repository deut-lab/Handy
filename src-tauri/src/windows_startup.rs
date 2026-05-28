#[cfg(target_os = "windows")]
use std::path::{Path, PathBuf};
#[cfg(target_os = "windows")]
use std::process::Command;

#[cfg(target_os = "windows")]
use tauri::AppHandle;

pub const TASK_NAME: &str = "Handy Admin Startup";

#[cfg(target_os = "windows")]
fn ps_quote(value: &str) -> String {
    format!("'{}'", value.replace('\'', "''"))
}

#[cfg(target_os = "windows")]
fn run_script(script: &str) -> Result<(), String> {
    let output = Command::new("powershell.exe")
        .args([
            "-NoProfile",
            "-ExecutionPolicy",
            "Bypass",
            "-Command",
            script,
        ])
        .output()
        .map_err(|e| format!("failed to run PowerShell: {e}"))?;

    if output.status.success() {
        return Ok(());
    }

    let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let text = if stderr.is_empty() { stdout } else { stderr };
    Err(if text.is_empty() {
        "PowerShell task command failed".to_string()
    } else {
        text
    })
}

#[cfg(target_os = "windows")]
fn app_paths(_app: &AppHandle) -> Result<(PathBuf, PathBuf), String> {
    let exe = std::env::current_exe().map_err(|e| format!("failed to get app path: {e}"))?;
    let work = exe
        .parent()
        .ok_or_else(|| "failed to get app folder".to_string())?
        .to_path_buf();
    Ok((exe, work))
}

#[cfg(target_os = "windows")]
pub fn build_register_task_script(exe: &Path, work: &Path, admin: bool) -> String {
    let exe = ps_quote(&exe.to_string_lossy());
    let work = ps_quote(&work.to_string_lossy());
    let task_name = ps_quote(TASK_NAME);
    let run_level = if admin { "Highest" } else { "Limited" };
    let description = if admin {
        "Start Handy with admin rights when the user logs in."
    } else {
        "Start Handy when the user logs in."
    };
    let description = ps_quote(description);

    format!(
        "$ErrorActionPreference = 'Stop'; \
         $taskName = {task_name}; \
         $exe = {exe}; \
         $work = {work}; \
         $user = [System.Security.Principal.WindowsIdentity]::GetCurrent().Name; \
         $action = New-ScheduledTaskAction -Execute $exe -WorkingDirectory $work; \
         $trigger = New-ScheduledTaskTrigger -AtLogOn -User $user; \
         $settings = New-ScheduledTaskSettingsSet -AllowStartIfOnBatteries -DontStopIfGoingOnBatteries -ExecutionTimeLimit (New-TimeSpan -Hours 72); \
         $principal = New-ScheduledTaskPrincipal -UserId $user -LogonType Interactive -RunLevel {run_level}; \
         Register-ScheduledTask -TaskName $taskName -Action $action -Trigger $trigger -Settings $settings -Principal $principal -Description {description} -Force | Out-Null;"
    )
}

#[cfg(target_os = "windows")]
pub fn build_unregister_task_script() -> String {
    let task_name = ps_quote(TASK_NAME);
    format!(
        "$taskName = {task_name}; \
         Unregister-ScheduledTask -TaskName $taskName -Confirm:$false -ErrorAction SilentlyContinue;"
    )
}

#[cfg(target_os = "windows")]
pub fn register_task(app: &AppHandle, admin: bool) -> Result<(), String> {
    let (exe, work) = app_paths(app)?;
    let script = build_register_task_script(&exe, &work, admin);
    run_script(&script)
}

#[cfg(target_os = "windows")]
pub fn unregister_task() -> Result<(), String> {
    let script = build_unregister_task_script();
    run_script(&script)
}

#[cfg(target_os = "windows")]
pub fn sync_task(app: &AppHandle, enabled: bool, admin: bool) -> Result<(), String> {
    if enabled {
        register_task(app, admin)
    } else {
        unregister_task()
    }
}

#[cfg(not(target_os = "windows"))]
pub fn sync_task(_app: &tauri::AppHandle, _enabled: bool, _admin: bool) -> Result<(), String> {
    Ok(())
}

#[cfg(test)]
#[cfg(target_os = "windows")]
mod tests {
    use super::*;

    #[test]
    fn register_task_script_sets_highest_run_level() {
        let script = build_register_task_script(
            Path::new("C:\\Users\\Dima\\AppData\\Local\\Handy\\handy.exe"),
            Path::new("C:\\Users\\Dima\\AppData\\Local\\Handy"),
            true,
        );

        assert!(script.contains("-RunLevel Highest"));
        assert!(script.contains("New-ScheduledTaskAction -Execute $exe -WorkingDirectory $work"));
        assert!(script.contains("Register-ScheduledTask"));
        assert!(script.contains("Handy Admin Startup"));
    }

    #[test]
    fn register_task_script_escapes_single_quotes() {
        let script = build_register_task_script(
            Path::new("C:\\Users\\Dima's PC\\Handy\\handy.exe"),
            Path::new("C:\\Users\\Dima's PC\\Handy"),
            false,
        );

        assert!(script.contains("-RunLevel Limited"));
        assert!(script.contains("Dima''s PC"));
    }
}
