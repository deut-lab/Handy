// Handy Voice: auto-stop on silence (src-tauri/src/voice/silence_stop.rs).
import React from "react";
import { useTranslation } from "react-i18next";
import { useSettings } from "../../../hooks/useSettings";
import { Input } from "../../ui/Input";
import { SettingContainer } from "../../ui/SettingContainer";
import { Switch } from "../../ui/Switch";

interface AutoStopSilenceProps {
  descriptionMode?: "inline" | "tooltip";
  grouped?: boolean;
}

const MIN_SECONDS = 1;
const MAX_SECONDS = 30;

export const AutoStopSilence: React.FC<AutoStopSilenceProps> = React.memo(
  ({ descriptionMode = "tooltip", grouped = false }) => {
    const { t } = useTranslation();
    const { getSetting, updateSetting, isUpdating } = useSettings();

    const enabled = getSetting("auto_stop_silence_enabled") ?? true;
    const seconds = getSetting("auto_stop_silence_seconds") ?? 7;

    const setSeconds = (event: React.ChangeEvent<HTMLInputElement>) => {
      const value = Number.parseInt(event.target.value, 10);
      if (Number.isNaN(value)) {
        return;
      }
      updateSetting(
        "auto_stop_silence_seconds",
        Math.min(MAX_SECONDS, Math.max(MIN_SECONDS, value)),
      );
    };

    return (
      <SettingContainer
        title={t("voice.autoStopSilence.title")}
        description={t("voice.autoStopSilence.description")}
        descriptionMode={descriptionMode}
        grouped={grouped}
      >
        <div className="flex items-center gap-2">
          <Switch
            checked={enabled}
            disabled={isUpdating("auto_stop_silence_enabled")}
            ariaLabel={t("voice.autoStopSilence.title")}
            onChange={(checked) =>
              updateSetting("auto_stop_silence_enabled", checked)
            }
          />
          <span className="text-sm whitespace-nowrap">
            {t("voice.autoStopSilence.after")}
          </span>
          <Input
            type="number"
            min={MIN_SECONDS}
            max={MAX_SECONDS}
            value={seconds}
            onChange={setSeconds}
            disabled={!enabled || isUpdating("auto_stop_silence_seconds")}
            variant="compact"
            className="w-16 text-center"
            aria-label={t("voice.autoStopSilence.seconds")}
          />
          <span className="text-sm whitespace-nowrap">
            {t("voice.autoStopSilence.seconds")}
          </span>
        </div>
      </SettingContainer>
    );
  },
);
