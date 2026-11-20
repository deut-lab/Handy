// Handy Voice: "Dictionary" section, words with their forms by language
// (src-tauri/src/voice/word_forms.rs). Below it, the plain word list that
// correction works on, with words added by hand or by other programs.
import React, { useState } from "react";
import { useTranslation } from "react-i18next";
import type { WordEntry } from "@/bindings";
import { useSettings } from "../../../hooks/useSettings";
import { CustomWords } from "../CustomWords";
import { Button } from "../../ui/Button";
import { Dropdown } from "../../ui/Dropdown";
import { Input } from "../../ui/Input";
import { SettingsGroup } from "../../ui/SettingsGroup";

const LANGUAGES = ["ru", "uk", "de", "en"] as const;

const splitForms = (text: string) =>
  text
    .split(/[,;\n]/)
    .map((f) => f.trim())
    .filter(Boolean);

export const DictionarySettings: React.FC = () => {
  const { t } = useTranslation();
  const { getSetting, updateSetting, isUpdating } = useSettings();
  const entries: WordEntry[] = getSetting("word_entries") || [];
  const busy = isUpdating("word_entries");

  const [language, setLanguage] = useState<string>("ru");
  const [word, setWord] = useState("");
  const [forms, setForms] = useState("");
  const [editing, setEditing] = useState<number | null>(null);

  const reset = () => {
    setWord("");
    setForms("");
    setEditing(null);
  };

  const save = () => {
    const entry: WordEntry = {
      language,
      word: word.trim(),
      forms: splitForms(forms),
    };
    if (!entry.word) return;
    const next =
      editing === null
        ? [...entries, entry]
        : entries.map((e, i) => (i === editing ? entry : e));
    updateSetting("word_entries", next);
    reset();
  };

  const edit = (index: number) => {
    const e = entries[index];
    setLanguage(e.language);
    setWord(e.word);
    setForms((e.forms || []).join(", "));
    setEditing(index);
  };

  const remove = (index: number) => {
    updateSetting(
      "word_entries",
      entries.filter((_, i) => i !== index),
    );
    if (editing === index) reset();
  };

  const languageName = (code: string) =>
    t(`voice.dictionary.languages.${code}`, { defaultValue: code });

  const languageOptions = LANGUAGES.map((code) => ({
    value: code,
    label: languageName(code),
  }));

  const groups = Array.from(new Set(entries.map((e) => e.language)));

  return (
    <div className="max-w-3xl w-full mx-auto space-y-6">
      <SettingsGroup title={t("voice.dictionary.title")}>
        <div className="space-y-3 p-4">
          <p className="text-sm text-ink-2">
            {t("voice.dictionary.description")}
          </p>
          <div className="grid grid-cols-[8rem_1fr] items-center gap-2">
            <span className="text-sm">{t("voice.dictionary.language")}</span>
            <Dropdown
              options={languageOptions}
              selectedValue={language}
              onSelect={setLanguage}
              disabled={busy}
            />
            <span className="text-sm">{t("voice.dictionary.word")}</span>
            <Input
              type="text"
              value={word}
              onChange={(e) => setWord(e.target.value)}
              placeholder={t("voice.dictionary.wordPlaceholder")}
              variant="compact"
              disabled={busy}
            />
            <span className="text-sm">{t("voice.dictionary.forms")}</span>
            <Input
              type="text"
              value={forms}
              onChange={(e) => setForms(e.target.value)}
              onKeyDown={(e) => {
                if (e.key === "Enter") {
                  e.preventDefault();
                  save();
                }
              }}
              placeholder={t("voice.dictionary.formsPlaceholder")}
              variant="compact"
              disabled={busy}
            />
          </div>
          <div className="flex gap-2">
            <Button
              onClick={save}
              disabled={!word.trim() || busy}
              variant="primary"
              size="md"
            >
              {editing === null
                ? t("voice.dictionary.add")
                : t("voice.dictionary.save")}
            </Button>
            {editing !== null && (
              <Button onClick={reset} variant="ghost" size="md">
                {t("voice.dictionary.cancel")}
              </Button>
            )}
          </div>
        </div>

        {groups.length === 0 && (
          <p className="px-4 pb-4 text-sm text-ink-2">
            {t("voice.dictionary.empty")}
          </p>
        )}
        {groups.map((code) => (
          <div key={code} className="px-4 pb-4">
            <h3 className="mb-1 text-xs font-semibold uppercase text-ink-2">
              {languageName(code)}
            </h3>
            <ul className="divide-y divide-line rounded-lg border border-line">
              {entries.map((e, index) =>
                e.language !== code ? null : (
                  <li
                    key={`${e.word}-${index}`}
                    className={`flex items-center gap-3 px-3 py-2 ${
                      editing === index ? "bg-logo-primary/10" : ""
                    }`}
                  >
                    <div className="min-w-0 flex-1">
                      <div className="text-sm font-medium">{e.word}</div>
                      {(e.forms || []).length > 0 && (
                        <div className="truncate text-xs text-ink-2">
                          {(e.forms || []).join(", ")}
                        </div>
                      )}
                    </div>
                    <Button
                      onClick={() => edit(index)}
                      disabled={busy}
                      variant="secondary"
                      size="sm"
                    >
                      {t("voice.dictionary.edit")}
                    </Button>
                    <Button
                      onClick={() => remove(index)}
                      disabled={busy}
                      variant="danger-ghost"
                      size="sm"
                    >
                      {t("voice.dictionary.remove")}
                    </Button>
                  </li>
                ),
              )}
            </ul>
          </div>
        ))}
      </SettingsGroup>

      <SettingsGroup title={t("voice.dictionary.allWordsTitle")}>
        <CustomWords descriptionMode="tooltip" grouped />
      </SettingsGroup>
    </div>
  );
};
