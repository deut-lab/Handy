// Handy Voice: the recording overlay's look (src-tauri/src/voice/overlay_look.rs):
// theme, icons, transcribing icon, buttons and transparency, with a preview of
// the compact pill.
import React from "react";
import { Check } from "lucide-react";
import { useTranslation } from "react-i18next";
import type {
  OverlayButtonStyle,
  OverlayIconSet,
  OverlayTheme,
  OverlayTranscribingIcon,
} from "@/bindings";
import {
  LookIcon,
  lookFromSettings,
  TRANSCRIBING_ICONS,
  viewLook,
  type OverlayLook,
} from "@/overlay/voice/OverlayLook";
import { useSettings } from "../../../hooks/useSettings";
import { SettingContainer } from "../../ui/SettingContainer";
import "./OverlayLookPicker.css";

interface OverlayLookPickerProps {
  descriptionMode?: "inline" | "tooltip";
  grouped?: boolean;
}

const THEMES: OverlayTheme[] = ["classic", "calm", "gray", "dark"];
const ICON_SETS: OverlayIconSet[] = ["original", "line"];
const BUTTON_STYLES: OverlayButtonStyle[] = ["circle", "simple"];
const TRANSCRIBING_ICON_NAMES = Object.keys(
  TRANSCRIBING_ICONS,
) as OverlayTranscribingIcon[];
const BAR_HEIGHTS = [6, 12, 16, 9, 13];

// Handy's own glyphs, as in RecordingOverlay.tsx and InsertButton.tsx.
const ThinCross = () => (
  <svg viewBox="0 0 16 16" aria-hidden="true">
    <path
      d="M4 4 L12 12 M12 4 L4 12"
      stroke="currentColor"
      strokeWidth="1.6"
      strokeLinecap="round"
    />
  </svg>
);

const ThinCheck = () => (
  <svg viewBox="0 0 16 16" aria-hidden="true">
    <path
      d="M3.5 8.5 L6.5 11.5 L12.5 4.5"
      fill="none"
      stroke="currentColor"
      strokeWidth="1.6"
      strokeLinecap="round"
      strokeLinejoin="round"
    />
  </svg>
);

/** The compact pill while recording and while transcribing. */
const Preview: React.FC<{ look: OverlayLook }> = ({ look }) => {
  const { t } = useTranslation();
  const view = viewLook(look);

  return (
    <div className={`vlp-stage ${view.className}`} style={view.style}>
      <div className="vlp-row">
        <span className="vlp-caption">
          {t("voice.overlayLook.states.recording")}
        </span>
        <div className="vlp-pill" aria-hidden="true">
          <span className="vlp-left">
            <span className="vlp-dot" />
            <LookIcon look={look} kind="recording" />
          </span>
          <span className="vlp-bars">
            {BAR_HEIGHTS.map((height, index) => (
              <i key={index} style={{ height }} />
            ))}
          </span>
          <span className="vlp-right">
            <span className="vlp-button finish">
              <LookIcon look={look} kind="finish" />
              <ThinCheck />
            </span>
            <span className="vlp-button">
              <LookIcon look={look} kind="cancel" />
              <ThinCross />
            </span>
          </span>
        </div>
      </div>
      <div className="vlp-row">
        <span className="vlp-caption">
          {t("voice.overlayLook.states.transcribing")}
        </span>
        <div className="vlp-pill working" aria-hidden="true">
          <span className="vlp-left">
            <span className="vlp-spinner" />
            <LookIcon look={look} kind="working" />
          </span>
          <span className="vlp-label">{t("overlay.transcribing")}</span>
        </div>
      </div>
    </div>
  );
};

const Choice: React.FC<{
  selected: boolean;
  disabled: boolean;
  onSelect: () => void;
  children: React.ReactNode;
}> = ({ selected, disabled, onSelect, children }) => (
  <button
    type="button"
    aria-pressed={selected}
    disabled={disabled}
    onClick={onSelect}
    className={`flex min-w-0 items-center justify-between gap-2 rounded-lg border px-3 py-2 text-sm transition-all ${
      selected
        ? "border-logo-primary bg-logo-primary/10 font-semibold text-text"
        : "border-line-strong bg-wash text-ink-2 hover:border-logo-primary/70 hover:bg-logo-primary/5"
    } ${disabled ? "cursor-not-allowed opacity-60" : "cursor-pointer"}`}
  >
    <span className="flex min-w-0 items-center gap-2">{children}</span>
    <span
      className={`flex h-4 w-4 shrink-0 items-center justify-center rounded-full border ${
        selected
          ? "border-logo-primary bg-logo-primary text-white"
          : "border-line-strong"
      }`}
    >
      {selected && <Check size={12} strokeWidth={2.6} />}
    </span>
  </button>
);

const Section: React.FC<{ title: string; children: React.ReactNode }> = ({
  title,
  children,
}) => (
  <div role="group" aria-label={title} className="min-w-0">
    <div className="mb-1.5 text-xs font-medium text-ink-2">{title}</div>
    {children}
  </div>
);

export const OverlayLookPicker: React.FC<OverlayLookPickerProps> = React.memo(
  ({ descriptionMode = "tooltip", grouped = false }) => {
    const { t } = useTranslation();
    const { settings, getSetting, updateSetting, isUpdating } = useSettings();

    if (getSetting("overlay_style") === "none") return null;

    const look = lookFromSettings(settings);

    return (
      <SettingContainer
        title={t("voice.overlayLook.title")}
        description={t("voice.overlayLook.description")}
        descriptionMode={descriptionMode}
        grouped={grouped}
        layout="stacked"
      >
        <div className="space-y-3">
          <Preview look={look} />

          <Section title={t("voice.overlayLook.theme.title")}>
            <div className="grid grid-cols-2 gap-2 md:grid-cols-4">
              {THEMES.map((theme) => (
                <Choice
                  key={theme}
                  selected={look.theme === theme}
                  disabled={isUpdating("overlay_theme")}
                  onSelect={() => updateSetting("overlay_theme", theme)}
                >
                  <span className={`vlook vlook-${theme} vlp-swatch`}>
                    <i />
                  </span>
                  <span className="truncate">
                    {t(`voice.overlayLook.theme.options.${theme}`)}
                  </span>
                </Choice>
              ))}
            </div>
          </Section>

          <div className="grid gap-3 md:grid-cols-2">
            <Section title={t("voice.overlayLook.iconSet.title")}>
              <div className="grid grid-cols-2 gap-2">
                {ICON_SETS.map((iconSet) => (
                  <Choice
                    key={iconSet}
                    selected={look.iconSet === iconSet}
                    disabled={isUpdating("overlay_icon_set")}
                    onSelect={() => updateSetting("overlay_icon_set", iconSet)}
                  >
                    <span className="truncate">
                      {t(`voice.overlayLook.iconSet.options.${iconSet}`)}
                    </span>
                  </Choice>
                ))}
              </div>
            </Section>

            <Section title={t("voice.overlayLook.buttonStyle.title")}>
              <div className="grid grid-cols-2 gap-2">
                {BUTTON_STYLES.map((style) => (
                  <Choice
                    key={style}
                    selected={look.buttonStyle === style}
                    disabled={isUpdating("overlay_button_style")}
                    onSelect={() =>
                      updateSetting("overlay_button_style", style)
                    }
                  >
                    <span className="truncate">
                      {t(`voice.overlayLook.buttonStyle.options.${style}`)}
                    </span>
                  </Choice>
                ))}
              </div>
            </Section>
          </div>

          <Section title={t("voice.overlayLook.opacity.title")}>
            <div className="flex items-center gap-3">
              <input
                type="range"
                min={0}
                max={100}
                step={10}
                value={look.opacity}
                disabled={isUpdating("overlay_opacity")}
                aria-label={t("voice.overlayLook.opacity.title")}
                onChange={(event) =>
                  updateSetting(
                    "overlay_opacity",
                    Number(event.currentTarget.value),
                  )
                }
                className="h-2 flex-grow cursor-pointer appearance-none rounded-lg focus:outline-none focus:ring-2 focus:ring-logo-primary disabled:cursor-not-allowed disabled:opacity-50"
                style={{
                  background: `linear-gradient(to right, var(--color-background-ui) ${look.opacity}%, var(--color-line) ${look.opacity}%)`,
                }}
              />
              <span className="w-12 text-end text-sm font-medium tabular-nums text-text">
                {look.opacity}%
              </span>
            </div>
          </Section>

          {look.iconSet === "line" && (
            <Section title={t("voice.overlayLook.transcribingIcon.title")}>
              <div className="grid grid-cols-2 gap-2 md:grid-cols-3">
                {TRANSCRIBING_ICON_NAMES.map((icon) => {
                  const Icon = TRANSCRIBING_ICONS[icon];
                  return (
                    <Choice
                      key={icon}
                      selected={look.transcribingIcon === icon}
                      disabled={isUpdating("overlay_transcribing_icon")}
                      onSelect={() =>
                        updateSetting("overlay_transcribing_icon", icon)
                      }
                    >
                      <Icon
                        size={16}
                        strokeWidth={2.2}
                        className="shrink-0"
                        aria-hidden="true"
                      />
                      <span className="truncate">
                        {t(
                          `voice.overlayLook.transcribingIcon.options.${icon}`,
                        )}
                      </span>
                    </Choice>
                  );
                })}
              </div>
            </Section>
          )}
        </div>
      </SettingContainer>
    );
  },
);
