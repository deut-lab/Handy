; Handy Voice installer hooks (tauri.conf.json: bundle > windows > nsis >
; installerHooks), included at the top of installer.nsi.
;
; Start at sign-in through the Windows task "Handy Voice Admin Startup", made
; by resources\windows_startup_task.ps1 (shared with the app, see
; src-tauri/src/voice/windows_startup.rs):
; - after installing, the task is set as the app's settings say; without a
;   settings file (first install) it is on, with administrator rights. The
;   fork's task has the same name and exe path, so it is updated in place.
;   The installer runs with administrator rights (installer.nsi), which a
;   task with administrator rights needs;
; - "Run Handy Voice" on the last page and the restart after an update go
;   through the task when it is on, so the app keeps the task's rights;
; - uninstalling removes the task, but not when an update reinstalls.

Var VoiceStartedByTask

!macro VOICE_STARTUP_TASK ARGS
  nsExec::ExecToLog 'powershell.exe -NoProfile -NonInteractive -ExecutionPolicy Bypass -File "$INSTDIR\resources\windows_startup_task.ps1" ${ARGS}'
!macroend

!macro NSIS_HOOK_POSTINSTALL
  ${If} $PortableMode <> 1
    Push $0
    !insertmacro VOICE_STARTUP_TASK '-Mode sync -ExePath "$INSTDIR\${MAINBINARYNAME}.exe" -WorkingDirectory "$INSTDIR" -SettingsPath "$APPDATA\${BUNDLEID}\settings_store.json"'
    Pop $0
    DetailPrint "Windows startup task: exit code $0"
    Pop $0
  ${EndIf}
!macroend

!macro NSIS_HOOK_PREUNINSTALL
  ${If} $UpdateMode <> 1
    Push $0
    !insertmacro VOICE_STARTUP_TASK "-Mode remove"
    Pop $0
    DetailPrint "Windows startup task removal: exit code $0"
    Pop $0
  ${EndIf}
!macroend

; Starts the app through the task when there are no app arguments to pass on
; and the task is on and starts this install. $VoiceStartedByTask = 1 if so.
!macro VOICE_START_BY_TASK APP_ARGS
  StrCpy $VoiceStartedByTask 0
  ${If} $PortableMode <> 1
  ${AndIf} "${APP_ARGS}" == ""
    Push $0
    !insertmacro VOICE_STARTUP_TASK '-Mode start -ExePath "$INSTDIR\${MAINBINARYNAME}.exe"'
    Pop $0
    ${If} $0 == 0
      StrCpy $VoiceStartedByTask 1
    ${EndIf}
    Pop $0
  ${EndIf}
!macroend
