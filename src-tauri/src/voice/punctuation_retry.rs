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
//! Audio of 8 s or more gets only the first padding; when it does not help,
//! the audio is cut in two at its
//! quietest moment and each half is decoded (and retried) on its own: a
//! shorter run makes its own decision. On recorded dictations this fixed the
//! 15 s chunks that padding left without punctuation.
//!
//! Timestamps of a retried transcript are shifted by the padding, and a
//! halved one keeps the first half's; Handy does not use them.

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

/// Audio at least this long is halved when padding does not help.
const MIN_HALVED_SECS: usize = 8;

/// Where to cut `pcm` in two: the middle of its quietest 0.3 s in the middle
/// third, so the cut falls in a pause between words rather than in one.
pub fn quiet_cut(pcm: &[f32], sample_rate: usize) -> usize {
    let window = (sample_rate * 3 / 10).max(1);
    let step = (sample_rate / 20).max(1);
    let from = pcm.len() / 3;
    let to = (pcm.len() * 2 / 3).max(from);
    let energy = |at: usize| pcm[at..at + window].iter().map(|x| x * x).sum::<f32>();
    (from..=to)
        .step_by(step)
        .filter(|&at| at + window <= pcm.len())
        .min_by(|&a, &b| energy(a).total_cmp(&energy(b)))
        .map_or(pcm.len() / 2, |at| at + window / 2)
}

/// Runs `decode` again on padded audio until a retry has punctuation; then,
/// for long enough audio, on its two halves. Returns the better result, or
/// `None` to keep the original. `join` puts two halves' results together.
pub fn retry<T, E>(
    pcm: &[f32],
    sample_rate: usize,
    original: &str,
    text_of: impl Fn(&T) -> &str + Copy,
    join: impl Fn(T, T) -> T + Copy,
    mut decode: impl FnMut(&[f32]) -> Result<T, E>,
) -> Option<T> {
    retry_with(pcm, sample_rate, original, text_of, join, &mut decode)
}

fn retry_with<T, E, F: FnMut(&[f32]) -> Result<T, E>>(
    pcm: &[f32],
    sample_rate: usize,
    original: &str,
    text_of: impl Fn(&T) -> &str + Copy,
    join: impl Fn(T, T) -> T + Copy,
    decode: &mut F,
) -> Option<T> {
    if !lacks_punctuation(original) {
        return None;
    }
    // Audio long enough to halve gets one padding first: halving helps more
    // and costs less than the other paddings on the whole.
    let halvable = pcm.len() >= MIN_HALVED_SECS * sample_rate;
    let pads = if halvable { &PADS_SECS[..1] } else { &PADS_SECS[..] };
    for &secs in pads {
        let Ok(result) = decode(&padded(pcm, secs, sample_rate)) else {
            return None;
        };
        if retry_is_better(original, text_of(&result)) {
            log::info!("Punctuation restored by a retry with {secs} s of leading silence");
            return Some(result);
        }
    }
    if halvable {
        let cut = quiet_cut(pcm, sample_rate);
        let first = decode_half(&pcm[..cut], sample_rate, text_of, join, decode)?;
        let second = decode_half(&pcm[cut..], sample_rate, text_of, join, decode)?;
        let joined = join(first, second);
        if retry_is_better(original, text_of(&joined)) {
            log::info!("Punctuation restored by decoding the audio in two halves");
            return Some(joined);
        }
    }
    log::info!("Punctuation missing; retries did not restore it");
    None
}

/// One half: decoded, and retried in turn when it has no punctuation.
fn decode_half<T, E, F: FnMut(&[f32]) -> Result<T, E>>(
    pcm: &[f32],
    sample_rate: usize,
    text_of: impl Fn(&T) -> &str + Copy,
    join: impl Fn(T, T) -> T + Copy,
    decode: &mut F,
) -> Option<T> {
    let result = decode(pcm).ok()?;
    let text = text_of(&result).to_string();
    Some(retry_with(pcm, sample_rate, &text, text_of, join, decode).unwrap_or(result))
}

#[cfg(test)]
mod tests {
    use super::*;

    const PLAIN: &str = "сначала живая проверка чтобы ты увидел как устроен этот сайт";

    fn join_strings(a: String, b: String) -> String {
        format!("{a} {b}")
    }

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
        let got = retry(&[1.0; 4], 4, PLAIN, |t: &String| t.as_str(), join_strings, |pcm| {
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
        let got = retry(&[0.0; 4], 4, "Всё хорошо, спасибо, до свидания, увидимся завтра утром.", |t: &String| t.as_str(), join_strings, |_| -> Result<String, ()> {
            panic!("no retry expected")
        });
        assert!(got.is_none());
    }

    #[test]
    fn long_audio_without_punctuation_is_halved() {
        // 10 "seconds" at 4 samples a second, a pause in the middle third.
        let mut pcm = vec![1.0; 40];
        pcm[18..22].fill(0.0);
        let mut calls = Vec::new();
        let got = retry(&pcm, 4, PLAIN, |t: &String| t.as_str(), join_strings, |part| {
            calls.push(part.len());
            Ok::<_, ()>(match calls.len() {
                2 => "Сначала живая проверка, чтобы ты".to_string(),
                3 => "увидел, как устроен этот сайт.".to_string(),
                _ => PLAIN.to_string(),
            })
        });
        assert_eq!(
            got.as_deref(),
            Some("Сначала живая проверка, чтобы ты увидел, как устроен этот сайт.")
        );
        // one padding, then the two halves, cut in the pause
        assert_eq!(calls.len(), 3);
        assert!((18..=22).contains(&calls[1]), "first half {}", calls[1]);
    }

    #[test]
    fn short_audio_is_not_halved() {
        let mut calls = 0;
        let got = retry(&[1.0; 28], 4, PLAIN, |t: &String| t.as_str(), join_strings, |_| {
            calls += 1;
            Ok::<_, ()>(PLAIN.to_string())
        });
        assert!(got.is_none());
        assert_eq!(calls, 4);
    }

    #[test]
    fn the_cut_falls_in_the_pause() {
        let mut pcm = vec![1.0; 400];
        pcm[200..240].fill(0.0);
        let cut = quiet_cut(&pcm, 40);
        assert!((200..240).contains(&cut), "cut at {cut}");
    }
}
