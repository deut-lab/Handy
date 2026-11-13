# Handy Voice: start the app at sign-in through a Windows Task Scheduler task.
#
# Shared by the app (src-tauri/src/voice/windows_startup.rs) and the installer
# (src-tauri/nsis/voice_hooks.nsh). Task name and exe path are the Handy Voice
# fork's, so an install over the fork updates the user's task in place.
#
#   -Mode sync    Make the task match the settings. On: create it, or point it
#                 at -ExePath with the asked rights. Off: remove it. A task that
#                 already matches is left alone, and a task the user disabled in
#                 Task Scheduler stays disabled. -Enabled / -Admin (1 or 0) win
#                 over the values read from -SettingsPath (only read, never
#                 written); without either, both are on (Handy Voice defaults).
#   -Mode remove  Remove the task.
#   -Mode start   Start the task. Exit 2 when it is missing or disabled, 3 when
#                 it starts another program than -ExePath.
#   -DryRun       Only say what would be done.
#   -Elevate      Ask for administrator rights (UAC) when a change needs them.
#
# Exit codes: 0 done, 1 error, 2 and 3 see start, 5 administrator rights needed.
param(
  [ValidateSet("sync", "remove", "start")]
  [string]$Mode = "sync",
  [string]$ExePath = "",
  [string]$WorkingDirectory = "",
  [int]$Enabled = -1,
  [int]$Admin = -1,
  [string]$SettingsPath = "",
  [string]$TaskName = "Handy Voice Admin Startup",
  [switch]$DryRun,
  [switch]$Elevate
)

$ErrorActionPreference = "Stop"

function Test-Elevated {
  $Identity = [Security.Principal.WindowsIdentity]::GetCurrent()
  ([Security.Principal.WindowsPrincipal]$Identity).IsInRole(
    [Security.Principal.WindowsBuiltInRole]::Administrator)
}

function Get-TaskExe($Task) {
  $Action = @($Task.Actions)[0]
  if ($null -eq $Action -or $null -eq $Action.Execute) { return "" }
  $Action.Execute.Trim('"')
}

# Runs this script again with administrator rights (UAC) and returns its exit code.
function Invoke-Elevated {
  $Arguments = @("-NoProfile", "-ExecutionPolicy", "Bypass", "-File", "`"$PSCommandPath`"",
    "-Mode", $Mode, "-Enabled", $Enabled, "-Admin", $Admin, "-TaskName", "`"$TaskName`"")
  if ($ExePath) { $Arguments += @("-ExePath", "`"$ExePath`"") }
  if ($WorkingDirectory) { $Arguments += @("-WorkingDirectory", "`"$WorkingDirectory`"") }
  $Process = Start-Process -FilePath "powershell.exe" -ArgumentList $Arguments `
    -Verb RunAs -WindowStyle Hidden -Wait -PassThru
  $Process.ExitCode
}

$Task = Get-ScheduledTask -TaskName $TaskName -ErrorAction SilentlyContinue | Select-Object -First 1

if ($Mode -eq "start") {
  if ($null -eq $Task -or [string]$Task.State -eq "Disabled") { exit 2 }
  if ($ExePath -and (Get-TaskExe $Task) -ne $ExePath) { exit 3 }
  if ($DryRun) { Write-Output "Would start the startup task"; exit 0 }
  Start-ScheduledTask -TaskName $Task.TaskName -TaskPath $Task.TaskPath
  exit 0
}

if ($Mode -eq "sync" -and $SettingsPath -and (Test-Path -LiteralPath $SettingsPath)) {
  try {
    $Stored = (Get-Content -LiteralPath $SettingsPath -Raw -Encoding UTF8 | ConvertFrom-Json).settings
    if ($Enabled -lt 0 -and $null -ne $Stored.windows_task_startup_enabled) {
      $Enabled = [int][bool]$Stored.windows_task_startup_enabled
    }
    if ($Admin -lt 0 -and $null -ne $Stored.windows_task_startup_admin) {
      $Admin = [int][bool]$Stored.windows_task_startup_admin
    }
  } catch {
    Write-Output "Could not read the settings, using the defaults: $($_.Exception.Message)"
  }
}
if ($Enabled -lt 0) { $Enabled = 1 }
if ($Admin -lt 0) { $Admin = 1 }

if ($Mode -eq "remove" -or $Enabled -eq 0) {
  if ($null -eq $Task) { Write-Output "No startup task"; exit 0 }
  if ($DryRun) { Write-Output "Would remove the startup task"; exit 0 }
  $Change = {
    Unregister-ScheduledTask -TaskName $Task.TaskName -TaskPath $Task.TaskPath -Confirm:$false
  }
  $Done = "Startup task removed"
} else {
  if (-not $ExePath) { throw "ExePath is required" }
  if (-not $WorkingDirectory) { $WorkingDirectory = Split-Path -Parent $ExePath }
  $RunLevel = if ($Admin -ne 0) { "Highest" } else { "Limited" }

  if ($null -ne $Task -and
      (Get-TaskExe $Task) -eq $ExePath -and
      @($Task.Actions)[0].WorkingDirectory -eq $WorkingDirectory -and
      [string]$Task.Principal.RunLevel -eq $RunLevel -and
      $Task.Settings.ExecutionTimeLimit -eq "PT0S" -and
      $Task.Settings.Priority -eq 5) {
    Write-Output "Startup task is up to date"
    exit 0
  }
  if ($DryRun) {
    $What = if ($null -eq $Task) { "create" } else { "update" }
    Write-Output "Would $What the startup task ($RunLevel)"
    exit 0
  }

  $Change = {
    $User = [Security.Principal.WindowsIdentity]::GetCurrent().Name
    $Description = if ($Admin -ne 0) {
      "Start Handy Voice with admin rights when the user logs in."
    } else {
      "Start Handy Voice when the user logs in."
    }
    $Action = New-ScheduledTaskAction -Execute $ExePath -WorkingDirectory $WorkingDirectory
    $Trigger = New-ScheduledTaskTrigger -AtLogOn -User $User
    # No run time limit (the default stops the app after 3 days) and normal
    # priority (the default runs it below normal).
    $Settings = New-ScheduledTaskSettingsSet -AllowStartIfOnBatteries -DontStopIfGoingOnBatteries `
      -ExecutionTimeLimit ([TimeSpan]::Zero) -Priority 5
    if ($null -ne $Task -and -not $Task.Settings.Enabled) { $Settings.Enabled = $false }
    $Principal = New-ScheduledTaskPrincipal -UserId $User -LogonType Interactive -RunLevel $RunLevel
    Register-ScheduledTask -TaskName $TaskName -Action $Action -Trigger $Trigger -Settings $Settings `
      -Principal $Principal -Description $Description -Force | Out-Null
  }
  $Done = "Startup task set ($RunLevel)"
}

try {
  & $Change
} catch {
  if (Test-Elevated) { throw }
  if ($Elevate) { exit (Invoke-Elevated) }
  [Console]::Error.WriteLine("Administrator rights are needed: $($_.Exception.Message)")
  exit 5
}
Write-Output $Done
exit 0
