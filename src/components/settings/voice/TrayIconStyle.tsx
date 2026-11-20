// Handy Voice: tray icon style picker (src-tauri/src/voice/tray_style.rs).
import React from "react";
import { Brain, Check, Ear, FileText, Hand, Mic } from "lucide-react";
import { useTranslation } from "react-i18next";
import type { TrayIconStyle as TrayIconStyleValue } from "@/bindings";
import { useSettings } from "../../../hooks/useSettings";
import { SettingContainer } from "../../ui/SettingContainer";

interface TrayIconStyleProps {
  descriptionMode?: "inline" | "tooltip";
  grouped?: boolean;
}

// The four tray states, in the order the icons are shown.
const STATES = ["idle", "recording", "transcribing", "attention"] as const;
type TrayState = (typeof STATES)[number];

interface StyleView {
  value: TrayIconStyleValue;
  name: string;
  icons: Record<TrayState, React.ReactNode>;
}

// Microphone with level bars, as the tray shows it while listening.
const MicWaves: React.FC = () => (
  <span className="flex items-center">
    <Mic size={13} strokeWidth={2.2} />
    <svg
      width="10"
      height="16"
      viewBox="0 0 10 16"
      fill="none"
      stroke="currentColor"
      strokeWidth="2.2"
      strokeLinecap="round"
      aria-hidden="true"
    >
      <path d="M1.5 5.5v5" />
      <path d="M5 2v12" />
      <path d="M8.5 4v8" />
    </svg>
  </span>
);

// Mark drawn over the icon as in the tray: an orange "!" while something
// needs attention (upstream icons only; the others turn the microphone orange).
const Mark: React.FC<{ state: TrayState; badge: boolean }> = ({
  state,
  badge,
}) => {
  if (badge && state === "attention") {
    return (
      <span className="absolute -bottom-0.5 -end-0.5 flex h-3.5 w-3.5 items-center justify-center rounded-full bg-amber-500 text-[9px] font-bold leading-none text-white ring-2 ring-background">
        !
      </span>
    );
  }
  return null;
};

const IconBox: React.FC<{
  children: React.ReactNode;
  active: boolean;
  state: TrayState;
  label: string;
  badge: boolean;
}> = ({ children, active, state, label, badge }) => (
  <span className="flex w-16 flex-col items-center gap-1" title={label}>
    <span
      className={`relative flex h-8 w-8 items-center justify-center rounded-full border transition-colors ${
        active
          ? "border-logo-primary bg-logo-primary/15 text-text"
          : "border-line-strong bg-wash text-ink-2"
      }`}
    >
      {children}
      <Mark state={state} badge={badge} />
    </span>
    <span className="text-[11px] leading-tight text-ink-2">{label}</span>
  </span>
);

const StyleCard: React.FC<{
  view: StyleView;
  labels: Record<TrayState, string>;
  selected: boolean;
  disabled: boolean;
  onSelect: () => void;
}> = ({ view, labels, selected, disabled, onSelect }) => (
  <button
    type="button"
    aria-pressed={selected}
    disabled={disabled}
    onClick={onSelect}
    className={`rounded-lg border p-3 text-start transition-all ${
      selected
        ? "border-logo-primary bg-logo-primary/10 shadow-sm"
        : "border-line-strong bg-wash hover:border-logo-primary/70 hover:bg-logo-primary/5"
    } ${disabled ? "cursor-not-allowed opacity-60" : "cursor-pointer"}`}
  >
    <div className="mb-3 flex min-w-0 items-center justify-between gap-3">
      <span className="min-w-0 truncate text-sm font-semibold text-text">
        {view.name}
      </span>
      <span
        className={`flex h-4 w-4 shrink-0 items-center justify-center rounded-full border ${
          selected
            ? "border-logo-primary bg-logo-primary text-white"
            : "border-line-strong"
        }`}
      >
        {selected && <Check size={12} strokeWidth={2.6} />}
      </span>
    </div>
    <div className="flex items-center justify-center gap-2">
      {STATES.map((state) => (
        <IconBox
          key={state}
          active={selected}
          state={state}
          label={labels[state]}
          badge={view.value === "original"}
        >
          {view.icons[state]}
        </IconBox>
      ))}
    </div>
  </button>
);

export const TrayIconStyle: React.FC<TrayIconStyleProps> = React.memo(
  ({ descriptionMode = "tooltip", grouped = false }) => {
    const { t } = useTranslation();
    const { getSetting, updateSetting, isUpdating } = useSettings();

    const selectedStyle = (getSetting("tray_icon_style") ||
      "states") as TrayIconStyleValue;
    const disabled = isUpdating("tray_icon_style");

    const labels: Record<TrayState, string> = {
      idle: t("voice.trayIconStyle.states.idle"),
      recording: t("voice.trayIconStyle.states.recording"),
      transcribing: t("voice.trayIconStyle.states.transcribing"),
      attention: t("voice.trayIconStyle.states.attention"),
    };
    const mic = <Mic size={17} strokeWidth={2.2} />;
    const hand = <Hand size={17} strokeWidth={2.2} />;
    const orangeMic = (
      <Mic size={17} strokeWidth={2.2} className="text-amber-500" />
    );

    const views: StyleView[] = [
      {
        value: "states",
        name: t("voice.trayIconStyle.options.states"),
        icons: {
          idle: mic,
          recording: <MicWaves />,
          transcribing: <FileText size={17} strokeWidth={2.2} />,
          attention: orangeMic,
        },
      },
      {
        value: "original",
        name: t("voice.trayIconStyle.options.original"),
        icons: {
          idle: hand,
          recording: <Ear size={17} strokeWidth={2.2} />,
          transcribing: <Brain size={17} strokeWidth={2.2} />,
          attention: hand,
        },
      },
      {
        value: "logo",
        name: t("voice.trayIconStyle.options.logo"),
        icons: {
          idle: mic,
          recording: mic,
          transcribing: mic,
          attention: orangeMic,
        },
      },
    ];

    return (
      <SettingContainer
        title={t("voice.trayIconStyle.title")}
        description={t("voice.trayIconStyle.description")}
        descriptionMode={descriptionMode}
        grouped={grouped}
        layout="stacked"
      >
        <div className="grid gap-3">
          {views.map((view) => (
            <StyleCard
              key={view.value}
              view={view}
              labels={labels}
              selected={selectedStyle === view.value}
              disabled={disabled}
              onSelect={() => updateSetting("tray_icon_style", view.value)}
            />
          ))}
        </div>
      </SettingContainer>
    );
  },
);
