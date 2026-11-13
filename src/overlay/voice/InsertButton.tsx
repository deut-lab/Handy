// Handy Voice: ✓ next to ✕ while recording. Stops and pastes the text, without
// auto-submit this time (src-tauri/src/voice/insert_without_enter.rs).
import React from "react";
import { useTranslation } from "react-i18next";
import { commands } from "@/bindings";
import "./InsertButton.css";

export const InsertButton: React.FC = () => {
  const { t } = useTranslation();
  const label = t("voice.overlay.insertWithoutSubmit");

  return (
    <button
      className="sx sinsert"
      aria-label={label}
      title={label}
      onClick={() => commands.finishWithoutSubmit()}
    >
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
    </button>
  );
};
