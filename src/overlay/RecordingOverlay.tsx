import { listen } from "@tauri-apps/api/event";
import { Check, FileText, Mic, X } from "lucide-react";
import React, { useEffect, useRef, useState } from "react";
import { useTranslation } from "react-i18next";
import "./RecordingOverlay.css";
import {
  CancelIcon,
  InsertIcon,
  MicrophoneIcon,
  TranscriptionIcon,
} from "@/components/icons";
import {
  commands,
  type OverlayButtonStyle,
  type OverlayIconSet,
  type OverlayOpacity,
  type OverlayTheme,
} from "@/bindings";
import i18n, { syncLanguageFromSettings } from "@/i18n";
import { getLanguageDirection } from "@/lib/utils/rtl";

type OverlayState = "recording" | "transcribing" | "processing";

const themeIconColor: Record<OverlayTheme, string> = {
  calm: "#2f5f73",
  classic: "#faa2ca",
  dark: "#8bb9c9",
  gray: "#59636b",
};

const RecordingOverlay: React.FC = () => {
  const { t } = useTranslation();
  const [isVisible, setIsVisible] = useState(false);
  const [state, setState] = useState<OverlayState>("recording");
  const [theme, setTheme] = useState<OverlayTheme>("calm");
  const [iconSet, setIconSet] = useState<OverlayIconSet>("original");
  const [buttonStyle, setButtonStyle] = useState<OverlayButtonStyle>("circle");
  const [opacity, setOpacity] = useState<OverlayOpacity>("medium");
  const [levels, setLevels] = useState<number[]>(Array(16).fill(0));
  const smoothedLevelsRef = useRef<number[]>(Array(16).fill(0));
  const direction = getLanguageDirection(i18n.language);

  useEffect(() => {
    const loadTheme = async () => {
      const result = await commands.getAppSettings();
      if (result.status === "ok") {
        setTheme(result.data.overlay_theme ?? "calm");
        setIconSet(result.data.overlay_icon_set ?? "original");
        setButtonStyle(result.data.overlay_button_style ?? "circle");
        setOpacity(result.data.overlay_opacity ?? "medium");
      }
    };

    const setupEventListeners = async () => {
      // Listen for show-overlay event from Rust
      const unlistenShow = await listen("show-overlay", async (event) => {
        // Sync language from settings each time overlay is shown
        await syncLanguageFromSettings();
        await loadTheme();
        const overlayState = event.payload as OverlayState;
        setState(overlayState);
        setIsVisible(true);
      });

      // Listen for hide-overlay event from Rust
      const unlistenHide = await listen("hide-overlay", () => {
        setIsVisible(false);
      });

      // Listen for mic-level updates
      const unlistenLevel = await listen<number[]>("mic-level", (event) => {
        const newLevels = event.payload as number[];

        // Apply smoothing to reduce jitter
        const smoothed = smoothedLevelsRef.current.map((prev, i) => {
          const target = newLevels[i] || 0;
          return prev * 0.7 + target * 0.3; // Smooth transition
        });

        smoothedLevelsRef.current = smoothed;
        setLevels(smoothed.slice(0, 5));
      });

      // Cleanup function
      return () => {
        unlistenShow();
        unlistenHide();
        unlistenLevel();
      };
    };

    setupEventListeners();
  }, []);

  const getStatusIcon = () => {
    const iconColor = themeIconColor[theme];

    if (iconSet === "original") {
      if (state === "recording") {
        return <MicrophoneIcon width={18} height={18} color={iconColor} />;
      }

      return <TranscriptionIcon width={18} height={18} color={iconColor} />;
    }

    if (state === "recording") {
      return <Mic size={18} strokeWidth={2.2} color={iconColor} />;
    }

    return <FileText size={18} strokeWidth={2.2} color={iconColor} />;
  };

  const getFinishIcon = () => {
    if (iconSet === "original") {
      return <InsertIcon width={16} height={16} color="currentColor" />;
    }

    return <Check size={15} strokeWidth={2.4} />;
  };

  const getCancelIcon = () => {
    if (iconSet === "original") {
      return <CancelIcon width={16} height={16} color="currentColor" />;
    }

    return <X size={15} strokeWidth={2.4} />;
  };

  return (
    <div
      dir={direction}
      className={`recording-overlay recording-overlay-${theme} recording-overlay-buttons-${buttonStyle} recording-overlay-opacity-${opacity} ${
        isVisible ? "fade-in" : ""
      }`}
    >
      <div className="overlay-left">{getStatusIcon()}</div>

      <div className="overlay-middle">
        {state === "recording" && (
          <div className="bars-container">
            {levels.map((v, i) => (
              <div
                key={i}
                className="bar"
                style={{
                  height: `${Math.min(16, 3 + Math.pow(v, 0.7) * 13)}px`,
                  transition: "height 60ms ease-out, opacity 120ms ease-out",
                  opacity: Math.max(0.2, v * 1.7),
                }}
              />
            ))}
          </div>
        )}
        {state === "transcribing" && (
          <div className="transcribing-text">{t("overlay.transcribing")}</div>
        )}
        {state === "processing" && (
          <div className="transcribing-text">{t("overlay.processing")}</div>
        )}
      </div>

      <div className="overlay-right">
        {state === "recording" && (
          <div className="overlay-actions">
            <button
              type="button"
              className="overlay-action finish-button"
              aria-label={t("overlay.insertWithoutSubmit")}
              title={t("overlay.insertWithoutSubmit")}
              onClick={() => {
                commands.finishWithoutSubmit();
              }}
            >
              {getFinishIcon()}
            </button>
            <button
              type="button"
              className="overlay-action cancel-button"
              aria-label={t("overlay.cancel")}
              title={t("overlay.cancel")}
              onClick={() => {
                commands.cancelOperation();
              }}
            >
              {getCancelIcon()}
            </button>
          </div>
        )}
      </div>
    </div>
  );
};

export default RecordingOverlay;
