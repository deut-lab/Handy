// Handy Voice: rows in About for this version (as in the fork): its source on
// deut-lab and its latest installer. Shown after Handy's own source code row.
import React from "react";
import { useTranslation } from "react-i18next";
import { openUrl } from "@tauri-apps/plugin-opener";
import { SettingContainer } from "../../ui/SettingContainer";
import { Button } from "../../ui/Button";

const REPO = "https://github.com/deut-lab/Handy";

const open = (url: string) => {
  openUrl(url).catch((error) =>
    console.error("Failed to open Handy Voice link:", error),
  );
};

export const AboutHandyVoice: React.FC = () => {
  const { t } = useTranslation();

  return (
    <>
      <SettingContainer
        title={t("voice.about.updatedVersion.title")}
        description={t("voice.about.updatedVersion.description")}
        grouped={true}
      >
        <Button variant="secondary" size="md" onClick={() => open(REPO)}>
          {t("voice.about.updatedVersion.button")}
        </Button>
      </SettingContainer>

      <SettingContainer
        title={t("voice.about.readyInstaller.title")}
        description={t("voice.about.readyInstaller.description")}
        grouped={true}
      >
        <Button
          variant="secondary"
          size="md"
          onClick={() => open(`${REPO}/releases/latest`)}
        >
          {t("voice.about.readyInstaller.button")}
        </Button>
      </SettingContainer>
    </>
  );
};
