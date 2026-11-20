//! Dictionary with word forms (settings section "Dictionary").
//!
//! Each entry is a language, a word and its other forms (cases, plural).
//! Correction works on `custom_words`, so every word and form of every entry
//! is kept there too. Words the user or another program added to
//! `custom_words` directly stay untouched: on a change, only the words of
//! the previous entries are replaced by the words of the new ones.

use crate::settings;
use serde::{Deserialize, Serialize};
use specta::Type;
use tauri::AppHandle;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Type)]
pub struct WordEntry {
    /// Language code, as in the language list ("ru", "uk", "de", "en").
    pub language: String,
    pub word: String,
    #[serde(default)]
    pub forms: Vec<String>,
}

fn clean(word: &str) -> String {
    word.replace(['<', '>', '"', '\''], "")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

/// Entries with trimmed words, empty ones and repeated forms removed.
pub fn normalize(entries: Vec<WordEntry>) -> Vec<WordEntry> {
    entries
        .into_iter()
        .filter_map(|e| {
            let word = clean(&e.word);
            if word.is_empty() || word.chars().count() > 50 {
                return None;
            }
            let mut forms: Vec<String> = Vec::new();
            for form in e.forms.iter().map(|f| clean(f)) {
                if !form.is_empty()
                    && form.chars().count() <= 50
                    && form != word
                    && !forms.contains(&form)
                {
                    forms.push(form);
                }
            }
            Some(WordEntry {
                language: e.language.trim().to_string(),
                word,
                forms,
            })
        })
        .collect()
}

/// Every word and form of the entries, in order, without repeats.
pub fn all_words(entries: &[WordEntry]) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for e in entries {
        for w in std::iter::once(&e.word).chain(e.forms.iter()) {
            if !out.contains(w) {
                out.push(w.clone());
            }
        }
    }
    out
}

/// `custom_words` after the entries changed from `before` to `after`.
pub fn merged_custom_words(
    custom_words: &[String],
    before: &[WordEntry],
    after: &[WordEntry],
) -> Vec<String> {
    let old = all_words(before);
    let mut out: Vec<String> = custom_words
        .iter()
        .filter(|w| !old.contains(w))
        .cloned()
        .collect();
    for w in all_words(after) {
        if !out.contains(&w) {
            out.push(w);
        }
    }
    out
}

#[tauri::command]
#[specta::specta]
pub fn update_word_entries(app: AppHandle, entries: Vec<WordEntry>) -> Result<(), String> {
    let entries = normalize(entries);
    let mut s = settings::get_settings(&app);
    s.custom_words = merged_custom_words(&s.custom_words, &s.word_entries, &entries);
    s.word_entries = entries;
    settings::write_settings(&app, s);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entry(word: &str, forms: &[&str]) -> WordEntry {
        WordEntry {
            language: "ru".into(),
            word: word.into(),
            forms: forms.iter().map(|f| f.to_string()).collect(),
        }
    }

    #[test]
    fn normalize_drops_empty_and_repeated_forms() {
        let got = normalize(vec![
            entry("  Ферайн ", &["Ферайна", "", "Ферайна", "Ферайн"]),
            entry("   ", &["x"]),
        ]);
        assert_eq!(got, vec![entry("Ферайн", &["Ферайна"])]);
    }

    #[test]
    fn merge_keeps_other_words_and_replaces_old_entries() {
        let custom = vec![
            "Handy".to_string(),
            "Ферайн".to_string(),
            "Ферайна".to_string(),
        ];
        let before = vec![entry("Ферайн", &["Ферайна"])];
        let after = vec![entry("Ферайн", &["Ферайну"])];
        assert_eq!(
            merged_custom_words(&custom, &before, &after),
            vec!["Handy", "Ферайн", "Ферайну"]
        );
        assert_eq!(merged_custom_words(&custom, &before, &[]), vec!["Handy"]);
    }
}
