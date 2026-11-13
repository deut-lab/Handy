// Handy Voice: sound theme picker in General settings (replaces the Debug-only
// SoundPicker), with the extra themes from src-tauri/src/voice/sound_themes.rs.
import React, { useEffect } from "react";
import { useTranslation } from "react-i18next";
import { PlayIcon } from "lucide-react";
import type { SoundTheme } from "@/bindings";
import { Button } from "../../ui/Button";
import { Dropdown, DropdownOption } from "../../ui/Dropdown";
import { SettingContainer } from "../../ui/SettingContainer";
import { useSettingsStore } from "../../../stores/settingsStore";
import { useSettings } from "../../../hooks/useSettings";

const BUILT_IN_THEMES: SoundTheme[] = [
  "marimba",
  "pop",
  "bell",
  "chime",
  "pluck",
  "switcher",
  "click",
  "mouse",
  "soft_switch",
  "bright_switch",
  "minimal",
];

// Translation keys under voice.soundTheme.options
const OPTION_KEYS: Record<SoundTheme, string> = {
  marimba: "marimba",
  pop: "pop",
  bell: "bell",
  chime: "chime",
  pluck: "pluck",
  switcher: "switcher",
  click: "click",
  mouse: "mouse",
  soft_switch: "softSwitch",
  bright_switch: "brightSwitch",
  minimal: "minimal",
  custom: "custom",
};

interface SoundThemePickerProps {
  disabled?: boolean;
}

export const SoundThemePicker: React.FC<SoundThemePickerProps> = ({
  disabled = false,
}) => {
  const { t } = useTranslation();
  const { getSetting, updateSetting } = useSettings();
  const playTestSound = useSettingsStore((state) => state.playTestSound);
  const customSounds = useSettingsStore((state) => state.customSounds);
  const checkCustomSounds = useSettingsStore(
    (state) => state.checkCustomSounds,
  );

  const selectedTheme = getSetting("sound_theme") ?? "marimba";

  useEffect(() => {
    checkCustomSounds();
  }, [checkCustomSounds]);

  const label = (theme: SoundTheme) =>
    t(`voice.soundTheme.options.${OPTION_KEYS[theme]}`);

  const options: DropdownOption[] = BUILT_IN_THEMES.map((theme) => ({
    value: theme,
    label: label(theme),
  }));

  // Only add Custom option if both custom sound files exist
  if (customSounds.start && customSounds.stop) {
    options.push({ value: "custom", label: label("custom") });
  }

  const handlePlayBothSounds = async () => {
    await playTestSound("start");
    await playTestSound("stop");
  };

  return (
    <SettingContainer
      title={t("voice.soundTheme.label")}
      description={t("voice.soundTheme.description")}
      disabled={disabled}
      grouped
      layout="horizontal"
    >
      <div className="flex items-center gap-2">
        <Dropdown
          selectedValue={selectedTheme}
          onSelect={(value) =>
            updateSetting("sound_theme", value as SoundTheme)
          }
          options={options}
          onOpen={checkCustomSounds}
          disabled={disabled}
        />
        <Button
          variant="ghost"
          size="sm"
          onClick={handlePlayBothSounds}
          title={t("voice.soundTheme.preview")}
          disabled={disabled}
        >
          <PlayIcon className="h-4 w-4" />
        </Button>
      </div>
    </SettingContainer>
  );
};
