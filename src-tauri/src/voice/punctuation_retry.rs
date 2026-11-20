//! Second try for a decode that came back without punctuation.
//!
//! Parakeet-style models decide for a whole clip whether to write
//! punctuation and capitals. On roughly one Russian dictation chunk in four
//! Parakeet Ultra returns plain lowercase words with no punctuation at all,
//! and the language hint or the model's PNC switch does not change that.
//! The decision does change with a little silence in front of the audio: on
//! recorded dictations, retrying with 1 s, then 0.5 s, 0.25 s and 2 s of
//! leading silence brought punctuation back in 10 of 12 such chunks. A retry
//! of a 30 s chunk costs about a quarter of a second on the CPU.
//!
//! Timestamps of a retried transcript are shifted by the padding; Handy does
//! not use them.

/// Leading silence to try, in order, in seconds (trailing silence is the same).
const PADS_SECS: [f32; 4] = [1.0, 0.5, 0.25, 2.0];

/// Only longer texts are checked: a short phrase may rightly have no
/// punctuation, and scripts without spaces never reach this many words.
const MIN_WORDS: usize = 8;

fn word_count(text: &str) -> usize {
    text.split_whitespace().count()
}

fn has_punctuation(text: &str) -> bool {
    text.chars()
        .any(|c| matches!(c, '.' | ',' | '!' | '?' | ';' | ':' | '…'))
}

/// True when a transcript should be decoded again.
pub fn lacks_punctuation(text: &str) -> bool {
    word_count(text) >= MIN_WORDS && !has_punctuation(text)
}

/// A retry is used only when it has punctuation and about the same words,
/// so the padding can never trade a correct text for an invented one.
pub fn retry_is_better(original: &str, retry: &str) -> bool {
    if !has_punctuation(retry) {
        return false;
    }
    let (a, b) = (word_count(original), word_count(retry));
    a.abs_diff(b) * 5 <= a.max(1)
}

/// The audio with `secs` of silence before and after it.
pub fn padded(pcm: &[f32], secs: f32, sample_rate: usize) -> Vec<f32> {
    let pad = (secs * sample_rate as f32) as usize;
    let mut out = Vec::with_capacity(pcm.len() + 2 * pad);
    out.resize(pad, 0.0);
    out.extend_from_slice(pcm);
    out.resize(out.len() + pad, 0.0);
    out
}

/// Runs `decode` again on padded audio until a retry has punctuation.
/// Returns the better text's result, or `None` to keep the original.
pub fn retry<T, E>(
    pcm: &[f32],
    sample_rate: usize,
    original: &str,
    text_of: impl Fn(&T) -> &str,
    mut decode: impl FnMut(&[f32]) -> Result<T, E>,
) -> Option<T> {
    if !lacks_punctuation(original) {
        return None;
    }
    for secs in PADS_SECS {
        let Ok(result) = decode(&padded(pcm, secs, sample_rate)) else {
            return None;
        };
        if retry_is_better(original, text_of(&result)) {
            log::info!("Punctuation restored by a retry with {secs} s of leading silence");
            return Some(result);
        }
    }
    log::info!("Punctuation missing; retries did not restore it");
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    const PLAIN: &str = "сначала живая проверка чтобы ты увидел как устроен этот сайт";

    #[test]
    fn plain_long_text_lacks_punctuation() {
        assert!(lacks_punctuation(PLAIN));
        assert!(!lacks_punctuation("Сначала живая проверка, чтобы ты увидел сайт."));
        assert!(!lacks_punctuation("да давай так"));
    }

    #[test]
    fn retry_must_keep_the_words() {
        let good = "Сначала живая проверка, чтобы ты увидел, как устроен этот сайт.";
        assert!(retry_is_better(PLAIN, good));
        assert!(!retry_is_better(PLAIN, PLAIN));
        assert!(!retry_is_better(PLAIN, "Спасибо."));
    }

    #[test]
    fn padding_adds_silence_on_both_sides() {
        let out = padded(&[1.0, 1.0], 0.5, 4);
        assert_eq!(out, vec![0.0, 0.0, 1.0, 1.0, 0.0, 0.0]);
    }

    #[test]
    fn retry_stops_at_the_first_punctuated_result() {
        let mut calls = Vec::new();
        let got = retry(&[1.0; 4], 4, PLAIN, |t: &String| t.as_str(), |pcm| {
            calls.push(pcm.len());
            Ok::<_, ()>(if calls.len() == 2 {
                "Сначала живая проверка, чтобы ты увидел, как устроен этот сайт.".to_string()
            } else {
                PLAIN.to_string()
            })
        });
        assert!(got.is_some());
        assert_eq!(calls, vec![12, 8]);
    }

    #[test]
    fn punctuated_text_is_not_retried() {
        let got = retry(&[0.0; 4], 4, "Всё хорошо, спасибо, до свидания, увидимся завтра утром.", |t: &String| t.as_str(), |_| -> Result<String, ()> {
            panic!("no retry expected")
        });
        assert!(got.is_none());
    }
}
