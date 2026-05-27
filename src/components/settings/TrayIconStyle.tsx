import React from "react";
import { Brain, Check, Ear, FileText, Hand, Mic } from "lucide-react";
import { useTranslation } from "react-i18next";
import type { TrayIconStyle as TrayIconStyleValue } from "@/bindings";
import { useSettings } from "../../hooks/useSettings";
import { SettingContainer } from "../ui/SettingContainer";

interface TrayIconStyleProps {
  descriptionMode?: "inline" | "tooltip";
  grouped?: boolean;
}

interface StyleView {
  value: TrayIconStyleValue;
  name: string;
  icons: React.ReactNode[];
}

const IconBox: React.FC<{ children: React.ReactNode; active: boolean }> = ({
  children,
  active,
}) => (
  <span
    className={`flex h-8 w-8 items-center justify-center rounded-full border transition-colors ${
      active
        ? "border-[#3e7288] bg-[#e8f1f4] text-[#2f5f73]"
        : "border-mid-gray/25 bg-mid-gray/5 text-text/70"
    }`}
  >
    {children}
  </span>
);

const StyleCard: React.FC<{
  view: StyleView;
  selected: boolean;
  disabled: boolean;
  onSelect: () => void;
}> = ({ view, selected, disabled, onSelect }) => (
  <button
    type="button"
    aria-pressed={selected}
    disabled={disabled}
    onClick={onSelect}
    className={`rounded-lg border p-3 text-start transition-all ${
      selected
        ? "border-[#3e7288] bg-[#e8f1f4] shadow-sm"
        : "border-mid-gray/25 bg-mid-gray/5 hover:border-[#3e7288]/70 hover:bg-[#e8f1f4]/55"
    } ${disabled ? "cursor-not-allowed opacity-60" : "cursor-pointer"}`}
  >
    <div className="mb-3 flex min-w-0 items-center justify-between gap-3">
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
    <div className="flex items-center justify-center gap-2">
      {view.icons.map((icon, index) => (
        <IconBox key={index} active={selected}>
          {icon}
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

    const views: StyleView[] = [
      {
        value: "states",
        name: t("settings.advanced.trayIconStyle.options.states"),
        icons: [
          <Mic key="idle" size={17} strokeWidth={2.2} />,
          <Mic key="recording" size={17} strokeWidth={2.8} />,
          <FileText key="transcribing" size={17} strokeWidth={2.2} />,
        ],
      },
      {
        value: "original",
        name: t("settings.advanced.trayIconStyle.options.original"),
        icons: [
          <Hand key="idle" size={17} strokeWidth={2.2} />,
          <Ear key="recording" size={17} strokeWidth={2.2} />,
          <Brain key="transcribing" size={17} strokeWidth={2.2} />,
        ],
      },
      {
        value: "logo",
        name: t("settings.advanced.trayIconStyle.options.logo"),
        icons: [
          <Mic key="idle" size={17} strokeWidth={2.2} />,
          <Mic key="recording" size={17} strokeWidth={2.2} />,
          <Mic key="transcribing" size={17} strokeWidth={2.2} />,
        ],
      },
    ];

    return (
      <SettingContainer
        title={t("settings.advanced.trayIconStyle.title")}
        description={t("settings.advanced.trayIconStyle.description")}
        descriptionMode={descriptionMode}
        grouped={grouped}
        layout="stacked"
      >
        <div className="grid gap-3 md:grid-cols-3">
          {views.map((view) => (
            <StyleCard
              key={view.value}
              view={view}
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
