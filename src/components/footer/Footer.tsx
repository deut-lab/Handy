import React, { useState, useEffect } from "react";

import ModelSelector from "../model-selector";
import UpdateChecker from "../update-checker";
import { getAppLabel } from "../settings/voice/appVersion";
import NoticeChip from "./NoticeChip";
import TranscriptionStatus from "./TranscriptionStatus";

interface FooterProps {
  onShowHistory: () => void;
}

const Footer: React.FC<FooterProps> = ({ onShowHistory }) => {
  const [version, setVersion] = useState("");

  useEffect(() => {
    const fetchVersion = async () => {
      try {
        setVersion(await getAppLabel());
      } catch (error) {
        console.error("Failed to get app version:", error);
        setVersion("0.1.2");
      }
    };

    fetchVersion();
  }, []);

  return (
    <div className="w-full border-t border-line pt-3">
      <div className="flex justify-between items-center text-xs px-4 pb-3 text-ink-2">
        <div className="flex items-center gap-4">
          <ModelSelector />
          <TranscriptionStatus onShowHistory={onShowHistory} />
        </div>

        <div className="flex items-center gap-3">
          {/* Update Status */}
          <div className="flex items-center gap-1">
            <UpdateChecker />
            <span>•</span>
            <span>{version}</span>
          </div>
          <NoticeChip />
        </div>
      </div>
    </div>
  );
};

export default Footer;
