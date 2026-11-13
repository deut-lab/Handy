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
import React, { useEffect, useLayoutEffect, useRef, useState } from "react";
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
  /** Size of the compact pill in percent, 80-110 in steps of 5. */
  size: number;
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
  size: 100,
};

const clampOpacity = (value: unknown): number => {
  const percent = typeof value === "number" ? value : Number(value);
  if (!Number.isFinite(percent)) return DEFAULT_LOOK.opacity;
  return Math.min(100, Math.max(0, Math.round(percent / 10) * 10));
};

const clampSize = (value: unknown): number => {
  const percent = typeof value === "number" ? value : Number(value);
  if (!Number.isFinite(percent)) return DEFAULT_LOOK.size;
  return Math.min(110, Math.max(80, Math.round(percent / 5) * 5));
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
  size: clampSize(settings?.overlay_size ?? DEFAULT_LOOK.size),
});

/**
 * Bar width and gap in whole screen pixels. WebView2 zooms the overlay by the
 * Windows text size (110% gives 1.1), so a 4px bar would cover 4.4 screen
 * pixels: some bars would land on 4 pixels and others on 5, all blurred.
 */
const barSizes = (dpr: number) => ({
  "--vl-bar-w": `${Math.max(1, Math.round(4 * dpr)) / dpr}px`,
  "--vl-bar-gap": `${Math.max(1, Math.round(2 * dpr)) / dpr}px`,
});

/** Classes and variables that apply a look to an element and its children. */
export const viewLook = (look: OverlayLook): OverlayLookView => ({
  ...look,
  className: `vlook vlook-${look.theme} vlook-icons-${look.iconSet} vlook-buttons-${look.buttonStyle}`,
  style: {
    "--vl-alpha": `${100 - look.opacity}%`,
    // The pill is zoomed by its size, so its pixels are dpr * size.
    "--vl-size": look.size / 100,
    ...barSizes((window.devicePixelRatio || 1) * (look.size / 100)),
  } as React.CSSProperties,
});

/** Re-renders when the screen scale changes (another monitor, text size). */
const useDevicePixelRatio = (): number => {
  const [dpr, setDpr] = useState(() => window.devicePixelRatio || 1);

  useEffect(() => {
    let query: MediaQueryList | null = null;
    const update = () => {
      query?.removeEventListener("change", update);
      const next = window.devicePixelRatio || 1;
      setDpr(next);
      query = window.matchMedia(`(resolution: ${next}dppx)`);
      query.addEventListener("change", update);
    };
    update();
    return () => query?.removeEventListener("change", update);
  }, []);

  return dpr;
};

/**
 * Puts the compact pill's bars on whole screen pixels: with whole-pixel bars
 * and gaps, only the start can still fall between two pixels, so the bars are
 * nudged by that fraction. Measured once per bar row and screen scale.
 */
const useSnappedBars = (dpr: number, zoom: number) => {
  const snapped = useRef<{ wave: Element | null; key: number }>({
    wave: null,
    key: 0,
  });

  useLayoutEffect(() => {
    const wave = document.querySelector<HTMLElement>(
      ".vlook .scard.compact .swave",
    );
    if (!wave) return;
    const key = dpr * zoom;
    if (snapped.current.wave === wave && snapped.current.key === key) return;
    wave.style.translate = "";
    // In screen pixels; the shift is set in the zoomed pill's own pixels.
    const left = wave.getBoundingClientRect().left * dpr;
    wave.style.translate = `${(Math.round(left) - left) / key}px 0`;
    snapped.current = { wave, key };
  });
};

/** The overlay's look, read again each time the overlay is shown. */
export const useOverlayLook = (): OverlayLookView => {
  const [look, setLook] = useState<OverlayLook>(DEFAULT_LOOK);
  const dpr = useDevicePixelRatio();
  useSnappedBars(dpr, look.size / 100);

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
