param(
  [ValidateSet("install", "remove", "start")]
  [string]$Mode = "install",
  [string]$ExePath = "",
  [string]$WorkingDirectory = "",
  [int]$Admin = 1
)

$ErrorActionPreference = "Stop"
$TaskName = "Handy Voice Admin Startup"

if ($Mode -eq "remove") {
  Unregister-ScheduledTask -TaskName $TaskName -Confirm:$false -ErrorAction SilentlyContinue
  exit 0
}

if ($Mode -eq "start") {
  Start-ScheduledTask -TaskName $TaskName -ErrorAction SilentlyContinue
  exit 0
}

if ([string]::IsNullOrWhiteSpace($ExePath)) {
  throw "ExePath is required"
}

if ([string]::IsNullOrWhiteSpace($WorkingDirectory)) {
  $WorkingDirectory = Split-Path -Parent $ExePath
}

$User = [System.Security.Principal.WindowsIdentity]::GetCurrent().Name
$UseAdmin = $Admin -ne 0
$RunLevel = if ($UseAdmin) { "Highest" } else { "Limited" }
$Description = if ($UseAdmin) {
  "Start Handy Voice with admin rights when the user logs in."
} else {
  "Start Handy Voice when the user logs in."
}

$Action = New-ScheduledTaskAction -Execute $ExePath -WorkingDirectory $WorkingDirectory
$Trigger = New-ScheduledTaskTrigger -AtLogOn -User $User
$Settings = New-ScheduledTaskSettingsSet -AllowStartIfOnBatteries -DontStopIfGoingOnBatteries -ExecutionTimeLimit (New-TimeSpan -Hours 72)
$Principal = New-ScheduledTaskPrincipal -UserId $User -LogonType Interactive -RunLevel $RunLevel

Register-ScheduledTask -TaskName $TaskName -Action $Action -Trigger $Trigger -Settings $Settings -Principal $Principal -Description $Description -Force | Out-Null
