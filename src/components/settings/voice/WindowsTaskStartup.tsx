// Handy Voice: start at sign-in through a Windows task
// (src-tauri/src/voice/windows_startup.rs). Shown on Windows only.
import React from "react";
import { useTranslation } from "react-i18next";
import { type } from "@tauri-apps/plugin-os";
import { useSettings } from "../../../hooks/useSettings";
import { ToggleSwitch } from "../../ui/ToggleSwitch";

interface WindowsTaskStartupProps {
  descriptionMode?: "inline" | "tooltip";
  grouped?: boolean;
}

export const WindowsTaskStartup: React.FC<WindowsTaskStartupProps> = React.memo(
  ({ descriptionMode = "tooltip", grouped = false }) => {
    const { t } = useTranslation();
    const { getSetting, updateSetting, isUpdating } = useSettings();

    if (type() !== "windows") {
      return null;
    }

    const taskEnabled = getSetting("windows_task_startup_enabled") ?? true;
    const adminEnabled = getSetting("windows_task_startup_admin") ?? true;

    return (
      <>
        <ToggleSwitch
          checked={taskEnabled}
          onChange={(enabled) =>
            updateSetting("windows_task_startup_enabled", enabled)
          }
          isUpdating={isUpdating("windows_task_startup_enabled")}
          label={t("voice.windowsTaskStartup.label")}
          description={t("voice.windowsTaskStartup.description")}
          descriptionMode={descriptionMode}
          grouped={grouped}
        />
        <ToggleSwitch
          checked={adminEnabled}
          onChange={(enabled) =>
            updateSetting("windows_task_startup_admin", enabled)
          }
          disabled={!taskEnabled}
          isUpdating={isUpdating("windows_task_startup_admin")}
          label={t("voice.windowsTaskStartupAdmin.label")}
          description={t("voice.windowsTaskStartupAdmin.description")}
          descriptionMode={descriptionMode}
          grouped={grouped}
        />
      </>
    );
  },
);
