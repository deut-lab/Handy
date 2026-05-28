import React from "react";
import { useTranslation } from "react-i18next";
import { type } from "@tauri-apps/plugin-os";
import { useSettings } from "../../hooks/useSettings";
import { ToggleSwitch } from "../ui/ToggleSwitch";

interface WindowsTaskStartupProps {
  descriptionMode?: "inline" | "tooltip";
  grouped?: boolean;
}

export const WindowsTaskStartup: React.FC<WindowsTaskStartupProps> = React.memo(
  ({ descriptionMode = "tooltip", grouped = false }) => {
    const { t } = useTranslation();
    const { getSetting, updateSetting, isUpdating } = useSettings();
    const isWindows = type() === "windows";

    const taskEnabled = getSetting("windows_task_startup_enabled") ?? false;
    const adminEnabled = getSetting("windows_task_startup_admin") ?? false;

    if (!isWindows) {
      return null;
    }

    return (
      <>
        <ToggleSwitch
          checked={taskEnabled}
          onChange={(enabled) =>
            updateSetting("windows_task_startup_enabled", enabled)
          }
          isUpdating={isUpdating("windows_task_startup_enabled")}
          label={t("settings.advanced.windowsTaskStartup.label")}
          description={t("settings.advanced.windowsTaskStartup.description")}
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
          label={t("settings.advanced.windowsTaskStartupAdmin.label")}
          description={t(
            "settings.advanced.windowsTaskStartupAdmin.description",
          )}
          descriptionMode={descriptionMode}
          grouped={grouped}
        />
      </>
    );
  },
);
