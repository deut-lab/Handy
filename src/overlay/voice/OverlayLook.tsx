// Handy Voice: the recording overlay's look (src-tauri/src/voice/overlay_look.rs):
// color theme, icon set, transcribing icon, button style and transparency.
// Colors live in OverlayLook.css; this file gives the overlay its classes and
// icons.
import { listen } from "@tauri-apps/api/event";
import {
  Check,
  LetterText,
  MessageSquareText,
  Mic,
  ScanText,
  ScrollText,
  TextCursorInput,
  TextSelect,
  X,
  type LucideIcon,
} from "lucide-react";
import React, { useEffect, useState } from "react";
import {
  commands,
  type AppSettings,
  type OverlayButtonStyle,
  type OverlayIconSet,
  type OverlayTheme,
  type OverlayTranscribingIcon,
} from "@/bindings";
import "./OverlayLook.css";

/**
 * The line icons for "transcribing", by the fork's store names (Lucide icon
 * names). lucide-react 0.542 still calls square-dashed-text `TextSelect` and
 * text-initial `LetterText`: the same drawings.
 */
export const TRANSCRIBING_ICONS: Record<OverlayTranscribingIcon, LucideIcon> = {
  scan_text: ScanText,
  square_dashed_text: TextSelect,
  scroll_text: ScrollText,
  text_cursor_input: TextCursorInput,
  message_square_text: MessageSquareText,
  text_initial: LetterText,
};

export interface OverlayLook {
  theme: OverlayTheme;
  iconSet: OverlayIconSet;
  transcribingIcon: OverlayTranscribingIcon;
  buttonStyle: OverlayButtonStyle;
  /** Transparency in percent, 0-100 in steps of 10. */
  opacity: number;
}

export interface OverlayLookView extends OverlayLook {
  className: string;
  style: React.CSSProperties;
}

// The fork's defaults (same as get_default_settings in Rust).
export const DEFAULT_LOOK: OverlayLook = {
  theme: "calm",
  iconSet: "line",
  transcribingIcon: "scan_text",
  buttonStyle: "circle",
  opacity: 30,
};

const clampOpacity = (value: unknown): number => {
  const percent = typeof value === "number" ? value : Number(value);
  if (!Number.isFinite(percent)) return DEFAULT_LOOK.opacity;
  return Math.min(100, Math.max(0, Math.round(percent / 10) * 10));
};

export const lookFromSettings = (
  settings: Partial<AppSettings> | null | undefined,
): OverlayLook => ({
  theme: settings?.overlay_theme ?? DEFAULT_LOOK.theme,
  iconSet: settings?.overlay_icon_set ?? DEFAULT_LOOK.iconSet,
  transcribingIcon:
    settings?.overlay_transcribing_icon ?? DEFAULT_LOOK.transcribingIcon,
  buttonStyle: settings?.overlay_button_style ?? DEFAULT_LOOK.buttonStyle,
  opacity: clampOpacity(settings?.overlay_opacity ?? DEFAULT_LOOK.opacity),
});

/** Classes and variables that apply a look to an element and its children. */
export const viewLook = (look: OverlayLook): OverlayLookView => ({
  ...look,
  className: `vlook vlook-${look.theme} vlook-icons-${look.iconSet} vlook-buttons-${look.buttonStyle}`,
  style: { "--vl-alpha": `${100 - look.opacity}%` } as React.CSSProperties,
});

/** The overlay's look, read again each time the overlay is shown. */
export const useOverlayLook = (): OverlayLookView => {
  const [look, setLook] = useState<OverlayLook>(DEFAULT_LOOK);

  useEffect(() => {
    let active = true;
    const load = async () => {
      try {
        const result = await commands.getAppSettings();
        if (active && result.status === "ok") {
          setLook(lookFromSettings(result.data));
        }
      } catch {
        // Keep the previous look if settings can't be read.
      }
    };
    load();
    const unlisten = listen("show-overlay", load);
    return () => {
      active = false;
      unlisten.then((stop) => stop());
    };
  }, []);

  return viewLook(look);
};

type LookIconKind = "recording" | "working" | "finish" | "cancel";

/**
 * A line icon, placed next to Handy's own one (the dot, the spinner, a thin
 * button glyph). With the `line` set OverlayLook.css hides Handy's icon; with
 * the `original` set this renders nothing.
 */
export const LookIcon: React.FC<{
  look: OverlayLook;
  kind: LookIconKind;
  ready?: boolean;
}> = ({ look, kind, ready = true }) => {
  if (look.iconSet !== "line") return null;

  switch (kind) {
    case "recording":
      return (
        <span className={`vicon vstatus ${ready ? "ready" : "arming"}`}>
          <Mic size={18} strokeWidth={2.2} aria-hidden="true" />
        </span>
      );
    case "working": {
      const Icon = TRANSCRIBING_ICONS[look.transcribingIcon] ?? ScanText;
      return (
        <span className="vicon vstatus working">
          <Icon size={18} strokeWidth={2.2} aria-hidden="true" />
        </span>
      );
    }
    case "finish":
      return (
        <span className="vicon">
          <Check size={14} strokeWidth={2.4} aria-hidden="true" />
        </span>
      );
    case "cancel":
      return (
        <span className="vicon">
          <X size={14} strokeWidth={2.4} aria-hidden="true" />
        </span>
      );
  }
};
