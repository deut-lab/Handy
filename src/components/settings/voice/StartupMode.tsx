// Handy Voice: one choice for how the app starts at sign-in, in place of
// upstream's autostart switch and the fork's two Windows task switches
// (`change_startup_mode`, src-tauri/src/voice/windows_startup.rs).
import React, { useState } from "react";
import { useTranslation } from "react-i18next";
import { type } from "@tauri-apps/plugin-os";
import { commands, type StartupMode as StartupModeValue } from "@/bindings";
import { useSettings } from "../../../hooks/useSettings";
import { Dropdown } from "../../ui/Dropdown";
import { SettingContainer } from "../../ui/SettingContainer";

interface StartupModeProps {
  descriptionMode?: "inline" | "tooltip";
  grouped?: boolean;
}

export const StartupMode: React.FC<StartupModeProps> = React.memo(
  ({ descriptionMode = "tooltip", grouped = false }) => {
    const { t } = useTranslation();
    const { getSetting, refreshSettings } = useSettings();
    const [busy, setBusy] = useState(false);
    const isWindows = type() === "windows";

    const taskOn =
      isWindows && (getSetting("windows_task_startup_enabled") ?? true);
    const taskAdmin = getSetting("windows_task_startup_admin") ?? true;
    const autostartOn = getSetting("autostart_enabled") ?? false;
    const selected: StartupModeValue =
      taskOn && taskAdmin ? "admin" : taskOn || autostartOn ? "regular" : "off";

    const options = [
      { value: "off", label: t("voice.startupMode.options.off") },
      { value: "regular", label: t("voice.startupMode.options.regular") },
      ...(isWindows
        ? [{ value: "admin", label: t("voice.startupMode.options.admin") }]
        : []),
    ];

    const choose = async (value: string) => {
      setBusy(true);
      try {
        await commands.changeStartupMode(value as StartupModeValue);
        await refreshSettings();
      } finally {
        setBusy(false);
      }
    };

    return (
      <SettingContainer
        title={t("voice.startupMode.title")}
        description={t("voice.startupMode.description")}
        descriptionMode={descriptionMode}
        grouped={grouped}
      >
        <Dropdown
          options={options}
          selectedValue={selected}
          onSelect={choose}
          disabled={busy}
        />
      </SettingContainer>
    );
  },
);
