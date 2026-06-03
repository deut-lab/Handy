import React from "react";
import { useTranslation } from "react-i18next";
import { Check, Mic, X } from "lucide-react";
import {
  CancelIcon,
  InsertIcon,
  LineTranscribingIcon,
  lineTranscribingIcons,
  MicrophoneIcon,
  TranscriptionIcon,
} from "../icons";
import type {
  OverlayButtonStyle,
  OverlayIconSet,
  OverlayTheme as OverlayThemeValue,
  OverlayTranscribingIcon,
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
  overlayRgb: [number, number, number];
  iconColor: string;
  barClass: string;
  finishClass: string;
  cancelClass: string;
  textClass: string;
}

interface ChoiceView<T extends string> {
  value: T;
  name: string;
  icon?: React.ReactNode;
}

const bars = [5, 11, 16, 9, 14];

const opacityStops = Array.from({ length: 11 }, (_, index) => index * 10);

const getOpacityPercent = (value: unknown): number => {
  if (typeof value === "number" && Number.isFinite(value)) {
    return Math.min(100, Math.max(0, Math.round(value / 10) * 10));
  }

  if (typeof value === "string") {
    if (value === "solid") {
      return 0;
    }
    if (value === "medium") {
      return 20;
    }
    if (value === "light") {
      return 30;
    }

    const parsed = Number(value);
    if (Number.isFinite(parsed)) {
      return Math.min(100, Math.max(0, Math.round(parsed / 10) * 10));
    }
  }

  return 30;
};

const getOverlayAlpha = (opacityPercent: number): number =>
  Math.min(1, Math.max(0, (100 - opacityPercent) / 100));

const getRgb = (rgb: [number, number, number], alpha: number): string =>
  `rgba(${rgb[0]}, ${rgb[1]}, ${rgb[2]}, ${alpha})`;

const MiniBars: React.FC<{ barClass: string }> = ({ barClass }) => (
  <div className="flex h-4 items-end justify-center gap-px overflow-hidden">
    {bars.map((height, index) => (
      <span
        key={index}
        className={`w-[3px] rounded-sm ${barClass}`}
        style={{ height }}
      />
    ))}
  </div>
);

const MiniStatusIcon: React.FC<{
  iconSet: OverlayIconSet;
  transcribingIcon: OverlayTranscribingIcon;
  state: "recording" | "transcribing";
  color: string;
}> = ({ iconSet, transcribingIcon, state, color }) => {
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
    <LineTranscribingIcon
      icon={transcribingIcon}
      size={15}
      strokeWidth={2.2}
      color={color}
    />
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
  transcribingIcon: OverlayTranscribingIcon;
  buttonStyle: OverlayButtonStyle;
  opacity: number;
  transcribingText: string;
  recordingText: string;
  onSelect: () => void;
}> = ({
  view,
  selected,
  disabled,
  iconSet,
  transcribingIcon,
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
        ? "border-[#168a6f] bg-[#e5f6ee] shadow-sm"
        : "border-mid-gray/25 bg-mid-gray/5 hover:border-[#168a6f]/70 hover:bg-[#e5f6ee]/55"
    } ${disabled ? "cursor-not-allowed opacity-60" : "cursor-pointer"}`}
  >
    <div className="flex min-w-0 items-center justify-between gap-3">
      <span className="min-w-0 truncate text-sm font-semibold text-text">
        {view.name}
      </span>
      <span
        className={`flex h-4 w-4 shrink-0 items-center justify-center rounded-full border ${
          selected
            ? "border-[#168a6f] bg-[#168a6f] text-white"
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
        className={`mx-auto grid h-8 w-[118px] max-w-full grid-cols-[16px_32px_46px] items-center gap-1 rounded-full px-1.5 shadow-sm ${view.overlayClass}`}
        style={{
          backgroundColor: getRgb(view.overlayRgb, getOverlayAlpha(opacity)),
        }}
      >
        <MiniStatusIcon
          iconSet={iconSet}
          transcribingIcon={transcribingIcon}
          state="recording"
          color={view.iconColor}
        />
        <MiniBars barClass={view.barClass} />
        <div className="flex items-center justify-end gap-1 overflow-hidden">
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
        className={`mx-auto grid h-8 w-[118px] max-w-full grid-cols-[16px_minmax(0,1fr)] items-center gap-1 rounded-full px-1.5 shadow-sm ${view.overlayClass}`}
        style={{
          backgroundColor: getRgb(view.overlayRgb, getOverlayAlpha(opacity)),
        }}
      >
        <MiniStatusIcon
          iconSet={iconSet}
          transcribingIcon={transcribingIcon}
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
    className={`flex min-w-0 items-center justify-center gap-2 rounded-md border px-2 py-2 text-sm font-medium transition-all ${
      selected
        ? "border-[#168a6f] bg-[#e5f6ee] text-text"
        : "border-mid-gray/25 bg-mid-gray/5 text-text/80 hover:border-[#168a6f]/70"
    } ${disabled ? "cursor-not-allowed opacity-60" : "cursor-pointer"}`}
  >
    {view.icon && <span className="shrink-0">{view.icon}</span>}
    <span className="min-w-0 text-center leading-snug">{view.name}</span>
  </button>
);

const ChoiceSection: React.FC<{
  title: string;
  children: React.ReactNode;
}> = ({ title, children }) => (
  <fieldset className="rounded-md border border-mid-gray/25 bg-mid-gray/5 p-3">
    <legend className="px-1 text-xs font-semibold uppercase text-text/60">
      {title}
    </legend>
    {children}
  </fieldset>
);

const OpacitySlider: React.FC<{
  title: string;
  value: number;
  disabled: boolean;
  onChange: (value: number) => void;
}> = ({ title, value, disabled, onChange }) => (
  <ChoiceSection title={title}>
    <div className="flex items-center gap-4">
      <input
        type="range"
        min={0}
        max={100}
        step={10}
        value={value}
        disabled={disabled}
        aria-label={title}
        list="overlay-opacity-stops"
        onChange={(event) => onChange(Number(event.currentTarget.value))}
        className="h-2 flex-1 cursor-pointer accent-[#168a6f] disabled:cursor-not-allowed disabled:opacity-60"
      />
      <span className="w-12 shrink-0 text-right text-sm font-semibold tabular-nums text-text">
        {value}%
      </span>
    </div>
    <datalist id="overlay-opacity-stops">
      {opacityStops.map((stop) => (
        <option key={stop} value={stop} />
      ))}
    </datalist>
    <div className="mt-2 grid grid-cols-11 text-center text-[10px] tabular-nums text-text/55">
      {opacityStops.map((stop) => (
        <span key={stop}>{stop}</span>
      ))}
    </div>
  </ChoiceSection>
);

export const OverlayTheme: React.FC<OverlayThemeProps> = React.memo(
  ({ descriptionMode = "tooltip", grouped = false }) => {
    const { t } = useTranslation();
    const { getSetting, updateSetting, isUpdating } = useSettings();

    const selectedTheme = (getSetting("overlay_theme") ||
      "calm") as OverlayThemeValue;
    const selectedIconSet = (getSetting("overlay_icon_set") ||
      "original") as OverlayIconSet;
    const selectedTranscribingIcon = (getSetting("overlay_transcribing_icon") ||
      "scan_text") as OverlayTranscribingIcon;
    const selectedButtonStyle = (getSetting("overlay_button_style") ||
      "circle") as OverlayButtonStyle;
    const selectedOpacity = getOpacityPercent(getSetting("overlay_opacity"));

    const views: ThemeView[] = [
      {
        value: "classic",
        name: t("settings.advanced.overlayTheme.options.classic"),
        panelClass: "bg-neutral-200/60",
        overlayClass: "border border-[#3a363f] bg-[#242428]",
        overlayRgb: [36, 36, 40],
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
        overlayRgb: [247, 250, 252],
        iconColor: "#127f78",
        barClass: "bg-[#18a572]",
        finishClass: "text-[#13914d]",
        cancelClass: "text-[#5f7078]",
        textClass: "text-[#243743]",
      },
      {
        value: "gray",
        name: t("settings.advanced.overlayTheme.options.gray"),
        panelClass: "bg-zinc-100",
        overlayClass: "border border-[#b8c0c6] bg-[#ebeef0]",
        overlayRgb: [235, 238, 240],
        iconColor: "#2f7669",
        barClass: "bg-[#34816d]",
        finishClass: "text-[#208853]",
        cancelClass: "text-[#66737a]",
        textClass: "text-[#2f363b]",
      },
      {
        value: "dark",
        name: t("settings.advanced.overlayTheme.options.dark"),
        panelClass: "bg-slate-200/70",
        overlayClass: "border border-[#3a4a55] bg-[#1f2933]",
        overlayRgb: [31, 41, 51],
        iconColor: "#74d7b4",
        barClass: "bg-[#74d7b4]",
        finishClass: "text-[#78e39b]",
        cancelClass: "text-[#c3d0d6]",
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

    const transcribingIconViews: ChoiceView<OverlayTranscribingIcon>[] =
      lineTranscribingIcons.map((icon) => ({
        value: icon,
        name: t(
          `settings.advanced.overlayTheme.transcribingIcon.options.${icon}`,
        ),
        icon: (
          <LineTranscribingIcon
            icon={icon}
            size={16}
            strokeWidth={2.2}
            color="currentColor"
          />
        ),
      }));

    const isThemeUpdating = isUpdating("overlay_theme");
    const isIconSetUpdating = isUpdating("overlay_icon_set");
    const isTranscribingIconUpdating = isUpdating("overlay_transcribing_icon");
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
                transcribingIcon={selectedTranscribingIcon}
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

          <div className="grid gap-3 md:grid-cols-2">
            <ChoiceSection
              title={t("settings.advanced.overlayTheme.iconSet.title")}
            >
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
            </ChoiceSection>

            <ChoiceSection
              title={t("settings.advanced.overlayTheme.buttonStyle.title")}
            >
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
            </ChoiceSection>
          </div>

          <OpacitySlider
            title={t("settings.advanced.overlayTheme.opacity.title")}
            value={selectedOpacity}
            disabled={isOpacityUpdating}
            onChange={(value) => updateSetting("overlay_opacity", value)}
          />

          <ChoiceSection
            title={t("settings.advanced.overlayTheme.transcribingIcon.title")}
          >
            <div className="grid grid-cols-2 gap-2 md:grid-cols-3">
              {transcribingIconViews.map((view) => (
                <ChoiceButton
                  key={view.value}
                  view={view}
                  selected={selectedTranscribingIcon === view.value}
                  disabled={isTranscribingIconUpdating}
                  onSelect={() =>
                    updateSetting("overlay_transcribing_icon", view.value)
                  }
                />
              ))}
            </div>
          </ChoiceSection>
        </div>
      </SettingContainer>
    );
  },
);
