import React from "react";
import { useTranslation } from "react-i18next";
import { Check, FileText, Mic, X } from "lucide-react";
import {
  CancelIcon,
  InsertIcon,
  MicrophoneIcon,
  TranscriptionIcon,
} from "../icons";
import type {
  OverlayButtonStyle,
  OverlayIconSet,
  OverlayOpacity,
  OverlayTheme as OverlayThemeValue,
} from "@/bindings";
import { useSettings } from "../../hooks/useSettings";
import { SettingContainer } from "../ui/SettingContainer";

interface OverlayThemeProps {
  descriptionMode?: "inline" | "tooltip";
  grouped?: boolean;
}

interface ThemeView {
  value: OverlayThemeValue;
  name: string;
  panelClass: string;
  overlayClass: string;
  iconColor: string;
  barClass: string;
  finishClass: string;
  cancelClass: string;
  textClass: string;
}

interface ChoiceView<T extends string> {
  value: T;
  name: string;
}

const bars = [5, 11, 16, 9, 14];

const opacityPreview: Record<OverlayOpacity, number> = {
  solid: 0.96,
  medium: 0.84,
  light: 0.7,
};

const MiniBars: React.FC<{ barClass: string }> = ({ barClass }) => (
  <div className="flex h-4 items-end justify-center gap-0.5 overflow-hidden">
    {bars.map((height, index) => (
      <span
        key={index}
        className={`w-1 rounded-sm ${barClass}`}
        style={{ height }}
      />
    ))}
  </div>
);

const MiniStatusIcon: React.FC<{
  iconSet: OverlayIconSet;
  state: "recording" | "transcribing";
  color: string;
}> = ({ iconSet, state, color }) => {
  if (iconSet === "original") {
    return state === "recording" ? (
      <MicrophoneIcon width={15} height={15} color={color} />
    ) : (
      <TranscriptionIcon width={15} height={15} color={color} />
    );
  }

  return state === "recording" ? (
    <Mic size={15} strokeWidth={2.2} color={color} />
  ) : (
    <FileText size={15} strokeWidth={2.2} color={color} />
  );
};

const MiniFinishIcon: React.FC<{ iconSet: OverlayIconSet }> = ({ iconSet }) =>
  iconSet === "original" ? (
    <InsertIcon width={14} height={14} color="currentColor" />
  ) : (
    <Check size={14} strokeWidth={2.4} />
  );

const MiniCancelIcon: React.FC<{ iconSet: OverlayIconSet }> = ({ iconSet }) =>
  iconSet === "original" ? (
    <CancelIcon width={14} height={14} color="currentColor" />
  ) : (
    <X size={14} strokeWidth={2.4} />
  );

const ThemeCard: React.FC<{
  view: ThemeView;
  selected: boolean;
  disabled: boolean;
  iconSet: OverlayIconSet;
  buttonStyle: OverlayButtonStyle;
  opacity: OverlayOpacity;
  transcribingText: string;
  recordingText: string;
  onSelect: () => void;
}> = ({
  view,
  selected,
  disabled,
  iconSet,
  buttonStyle,
  opacity,
  transcribingText,
  recordingText,
  onSelect,
}) => (
  <button
    type="button"
    aria-pressed={selected}
    disabled={disabled}
    onClick={onSelect}
    className={`w-full rounded-lg border p-3 text-start transition-all ${
      selected
        ? "border-[#3e7288] bg-[#e8f1f4] shadow-sm"
        : "border-mid-gray/25 bg-mid-gray/5 hover:border-[#3e7288]/70 hover:bg-[#e8f1f4]/55"
    } ${disabled ? "cursor-not-allowed opacity-60" : "cursor-pointer"}`}
  >
    <div className="flex min-w-0 items-center justify-between gap-3">
      <span className="min-w-0 truncate text-sm font-semibold text-text">
        {view.name}
      </span>
      <span
        className={`flex h-4 w-4 shrink-0 items-center justify-center rounded-full border ${
          selected
            ? "border-[#3e7288] bg-[#3e7288] text-white"
            : "border-mid-gray/40"
        }`}
      >
        {selected && <Check size={12} strokeWidth={2.6} />}
      </span>
    </div>

    <div className={`mt-3 rounded-md p-2 ${view.panelClass}`}>
      <div className="mb-1 text-[11px] font-medium text-text/60">
        {recordingText}
      </div>
      <div
        className={`grid h-8 min-w-0 grid-cols-[18px_minmax(24px,1fr)_50px] items-center gap-1 rounded-full px-2 shadow-sm ${view.overlayClass}`}
        style={{ opacity: opacityPreview[opacity] }}
      >
        <MiniStatusIcon
          iconSet={iconSet}
          state="recording"
          color={view.iconColor}
        />
        <MiniBars barClass={view.barClass} />
        <div className="flex items-center justify-end gap-1.5 overflow-hidden">
          <span
            className={`flex h-5 w-5 shrink-0 items-center justify-center rounded-full ${
              buttonStyle === "circle"
                ? `border bg-white/50 ${view.finishClass}`
                : view.finishClass
            }`}
          >
            <MiniFinishIcon iconSet={iconSet} />
          </span>
          <span
            className={`flex h-5 w-5 shrink-0 items-center justify-center rounded-full ${
              buttonStyle === "circle"
                ? `border bg-white/50 ${view.cancelClass}`
                : view.cancelClass
            }`}
          >
            <MiniCancelIcon iconSet={iconSet} />
          </span>
        </div>
      </div>

      <div className="mb-1 mt-3 text-[11px] font-medium text-text/60">
        {transcribingText}
      </div>
      <div
        className={`grid h-8 grid-cols-[20px_minmax(42px,1fr)] items-center gap-1 rounded-full px-2 shadow-sm ${view.overlayClass}`}
        style={{ opacity: opacityPreview[opacity] }}
      >
        <MiniStatusIcon
          iconSet={iconSet}
          state="transcribing"
          color={view.iconColor}
        />
        <span
          className={`truncate text-center text-[11px] font-semibold ${view.textClass}`}
        >
          {transcribingText}
        </span>
      </div>
    </div>
  </button>
);

const ChoiceButton = <T extends string>({
  view,
  selected,
  disabled,
  onSelect,
}: {
  view: ChoiceView<T>;
  selected: boolean;
  disabled: boolean;
  onSelect: () => void;
}) => (
  <button
    type="button"
    aria-pressed={selected}
    disabled={disabled}
    onClick={onSelect}
    className={`rounded-md border px-3 py-2 text-sm font-medium transition-all ${
      selected
        ? "border-[#3e7288] bg-[#e8f1f4] text-text"
        : "border-mid-gray/25 bg-mid-gray/5 text-text/80 hover:border-[#3e7288]/70"
    } ${disabled ? "cursor-not-allowed opacity-60" : "cursor-pointer"}`}
  >
    {view.name}
  </button>
);

export const OverlayTheme: React.FC<OverlayThemeProps> = React.memo(
  ({ descriptionMode = "tooltip", grouped = false }) => {
    const { t } = useTranslation();
    const { getSetting, updateSetting, isUpdating } = useSettings();

    const selectedTheme = (getSetting("overlay_theme") ||
      "calm") as OverlayThemeValue;
    const selectedIconSet = (getSetting("overlay_icon_set") ||
      "original") as OverlayIconSet;
    const selectedButtonStyle = (getSetting("overlay_button_style") ||
      "circle") as OverlayButtonStyle;
    const selectedOpacity = (getSetting("overlay_opacity") ||
      "medium") as OverlayOpacity;

    const views: ThemeView[] = [
      {
        value: "classic",
        name: t("settings.advanced.overlayTheme.options.classic"),
        panelClass: "bg-neutral-200/60",
        overlayClass: "border border-[#3a363f] bg-[#242428]",
        iconColor: "#faa2ca",
        barClass: "bg-[#ffe5ee]",
        finishClass: "text-[#faa2ca]",
        cancelClass: "text-[#faa2ca]",
        textClass: "text-white",
      },
      {
        value: "calm",
        name: t("settings.advanced.overlayTheme.options.calm"),
        panelClass: "bg-slate-100/70",
        overlayClass: "border border-[#c8d2da] bg-[#f7fafc]",
        iconColor: "#2f5f73",
        barClass: "bg-[#3e7288]",
        finishClass: "text-[#2f6f57]",
        cancelClass: "text-[#587080]",
        textClass: "text-[#243743]",
      },
      {
        value: "gray",
        name: t("settings.advanced.overlayTheme.options.gray"),
        panelClass: "bg-zinc-100",
        overlayClass: "border border-[#b8c0c6] bg-[#ebeef0]",
        iconColor: "#59636b",
        barClass: "bg-[#64727c]",
        finishClass: "text-[#3f6d57]",
        cancelClass: "text-[#697781]",
        textClass: "text-[#2f363b]",
      },
      {
        value: "dark",
        name: t("settings.advanced.overlayTheme.options.dark"),
        panelClass: "bg-slate-200/70",
        overlayClass: "border border-[#3a4a55] bg-[#1f2933]",
        iconColor: "#8bb9c9",
        barClass: "bg-[#8bb9c9]",
        finishClass: "text-[#95d1bb]",
        cancelClass: "text-[#b8c7d0]",
        textClass: "text-[#edf4f7]",
      },
    ];

    const iconSetViews: ChoiceView<OverlayIconSet>[] = [
      {
        value: "original",
        name: t("settings.advanced.overlayTheme.iconSet.options.original"),
      },
      {
        value: "line",
        name: t("settings.advanced.overlayTheme.iconSet.options.line"),
      },
    ];

    const opacityViews: ChoiceView<OverlayOpacity>[] = [
      {
        value: "solid",
        name: t("settings.advanced.overlayTheme.opacity.options.solid"),
      },
      {
        value: "medium",
        name: t("settings.advanced.overlayTheme.opacity.options.medium"),
      },
      {
        value: "light",
        name: t("settings.advanced.overlayTheme.opacity.options.light"),
      },
    ];

    const buttonStyleViews: ChoiceView<OverlayButtonStyle>[] = [
      {
        value: "circle",
        name: t("settings.advanced.overlayTheme.buttonStyle.options.circle"),
      },
      {
        value: "simple",
        name: t("settings.advanced.overlayTheme.buttonStyle.options.simple"),
      },
    ];

    const isThemeUpdating = isUpdating("overlay_theme");
    const isIconSetUpdating = isUpdating("overlay_icon_set");
    const isButtonStyleUpdating = isUpdating("overlay_button_style");
    const isOpacityUpdating = isUpdating("overlay_opacity");

    return (
      <SettingContainer
        title={t("settings.advanced.overlayTheme.title")}
        description={t("settings.advanced.overlayTheme.description")}
        descriptionMode={descriptionMode}
        grouped={grouped}
      >
        <div className="space-y-4">
          <div className="grid w-full grid-cols-1 gap-3 md:grid-cols-2 2xl:grid-cols-4">
            {views.map((view) => (
              <ThemeCard
                key={view.value}
                view={view}
                selected={selectedTheme === view.value}
                disabled={isThemeUpdating}
                iconSet={selectedIconSet}
                buttonStyle={selectedButtonStyle}
                opacity={selectedOpacity}
                recordingText={t(
                  "settings.advanced.overlayTheme.states.recording",
                )}
                transcribingText={t(
                  "settings.advanced.overlayTheme.states.transcribing",
                )}
                onSelect={() => updateSetting("overlay_theme", view.value)}
              />
            ))}
          </div>

          <div className="grid gap-3 md:grid-cols-3">
            <div>
              <div className="mb-2 text-xs font-semibold uppercase text-text/60">
                {t("settings.advanced.overlayTheme.iconSet.title")}
              </div>
              <div className="grid grid-cols-2 gap-2">
                {iconSetViews.map((view) => (
                  <ChoiceButton
                    key={view.value}
                    view={view}
                    selected={selectedIconSet === view.value}
                    disabled={isIconSetUpdating}
                    onSelect={() =>
                      updateSetting("overlay_icon_set", view.value)
                    }
                  />
                ))}
              </div>
            </div>

            <div>
              <div className="mb-2 text-xs font-semibold uppercase text-text/60">
                {t("settings.advanced.overlayTheme.buttonStyle.title")}
              </div>
              <div className="grid grid-cols-2 gap-2">
                {buttonStyleViews.map((view) => (
                  <ChoiceButton
                    key={view.value}
                    view={view}
                    selected={selectedButtonStyle === view.value}
                    disabled={isButtonStyleUpdating}
                    onSelect={() =>
                      updateSetting("overlay_button_style", view.value)
                    }
                  />
                ))}
              </div>
            </div>

            <div>
              <div className="mb-2 text-xs font-semibold uppercase text-text/60">
                {t("settings.advanced.overlayTheme.opacity.title")}
              </div>
              <div className="grid grid-cols-3 gap-2">
                {opacityViews.map((view) => (
                  <ChoiceButton
                    key={view.value}
                    view={view}
                    selected={selectedOpacity === view.value}
                    disabled={isOpacityUpdating}
                    onSelect={() =>
                      updateSetting("overlay_opacity", view.value)
                    }
                  />
                ))}
              </div>
            </div>
          </div>
        </div>
      </SettingContainer>
    );
  },
);
