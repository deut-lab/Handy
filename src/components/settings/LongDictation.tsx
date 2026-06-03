import React from "react";
import { useTranslation } from "react-i18next";
import { Input } from "../ui/Input";
import { SettingContainer } from "../ui/SettingContainer";
import { useSettings } from "../../hooks/useSettings";

interface LongDictationProps {
  descriptionMode?: "inline" | "tooltip";
  grouped?: boolean;
}

export const LongDictation: React.FC<LongDictationProps> = React.memo(
  ({ descriptionMode = "tooltip", grouped = false }) => {
    const { t } = useTranslation();
    const { getSetting, updateSetting, isUpdating } = useSettings();

    const mode = getSetting("long_dictation_mode") ?? "pause_chunks";
    const enabled = mode === "pause_chunks";
    const pauseSeconds = getSetting("long_dictation_silence_seconds") ?? 1;
    const minSeconds = getSetting("long_dictation_min_chunk_seconds") ?? 8;

    const setPauseSeconds = (event: React.ChangeEvent<HTMLInputElement>) => {
      const value = Number.parseInt(event.target.value, 10);
      if (Number.isNaN(value)) {
        return;
      }
      updateSetting(
        "long_dictation_silence_seconds",
        Math.min(10, Math.max(1, value)),
      );
    };

    const setMinSeconds = (event: React.ChangeEvent<HTMLInputElement>) => {
      const value = Number.parseInt(event.target.value, 10);
      if (Number.isNaN(value)) {
        return;
      }
      updateSetting(
        "long_dictation_min_chunk_seconds",
        Math.min(60, Math.max(2, value)),
      );
    };

    return (
      <SettingContainer
        title={t("settings.advanced.longDictation.title")}
        description={t("settings.advanced.longDictation.description")}
        descriptionMode={descriptionMode}
        grouped={grouped}
        layout="stacked"
      >
        <div className="flex flex-wrap items-center gap-2">
          <label className="flex items-center gap-2 text-sm whitespace-nowrap">
            <input
              type="checkbox"
              className="h-4 w-4 accent-logo-primary"
              checked={enabled}
              disabled={isUpdating("long_dictation_mode")}
              onChange={(event) =>
                updateSetting(
                  "long_dictation_mode",
                  event.target.checked ? "pause_chunks" : "off",
                )
              }
            />
            {t("settings.advanced.longDictation.enabled")}
          </label>
          <span className="text-sm whitespace-nowrap">
            {t("settings.advanced.longDictation.pause")}
          </span>
          <Input
            type="number"
            min={1}
            max={10}
            value={pauseSeconds}
            onChange={setPauseSeconds}
            disabled={!enabled || isUpdating("long_dictation_silence_seconds")}
            variant="compact"
            className="w-16 text-center"
          />
          <span className="text-sm whitespace-nowrap">
            {t("settings.advanced.longDictation.min")}
          </span>
          <Input
            type="number"
            min={2}
            max={60}
            value={minSeconds}
            onChange={setMinSeconds}
            disabled={
              !enabled || isUpdating("long_dictation_min_chunk_seconds")
            }
            variant="compact"
            className="w-16 text-center"
          />
        </div>
      </SettingContainer>
    );
  },
);
