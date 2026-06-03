use crate::managers::transcription::TranscriptionManager;
use anyhow::Result;
use log::{debug, error, warn};
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;

struct ChunkJob {
    sequence: u64,
    audio: Vec<f32>,
}

#[derive(Clone, Debug)]
struct ChunkResult {
    sequence: u64,
    text: String,
}

struct LiveSession {
    tx: mpsc::Sender<ChunkJob>,
    handle: Option<thread::JoinHandle<()>>,
    results: Arc<Mutex<Vec<ChunkResult>>>,
    next_sequence: u64,
}

#[derive(Default)]
pub struct LiveTranscriptionManager {
    session: Mutex<Option<LiveSession>>,
}

impl LiveTranscriptionManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn start(&self, transcription_manager: Arc<TranscriptionManager>) {
        self.cancel();

        let (tx, rx) = mpsc::channel::<ChunkJob>();
        let results = Arc::new(Mutex::new(Vec::new()));
        let worker_results = Arc::clone(&results);

        let handle = thread::spawn(move || {
            debug!("Live transcription worker started");
            while let Ok(job) = rx.recv() {
                if job.audio.is_empty() {
                    continue;
                }

                match transcription_manager.transcribe_live_chunk(job.audio) {
                    Ok(text) => {
                        let text = text.trim().to_string();
                        if !text.is_empty() {
                            worker_results.lock().unwrap().push(ChunkResult {
                                sequence: job.sequence,
                                text,
                            });
                        }
                    }
                    Err(err) => {
                        error!("Live transcription chunk {} failed: {}", job.sequence, err);
                    }
                }
            }
            debug!("Live transcription worker stopped");
        });

        *self.session.lock().unwrap() = Some(LiveSession {
            tx,
            handle: Some(handle),
            results,
            next_sequence: 0,
        });
    }

    pub fn submit_chunk(&self, audio: Vec<f32>) {
        if audio.is_empty() {
            return;
        }

        let mut guard = self.session.lock().unwrap();
        let Some(session) = guard.as_mut() else {
            return;
        };

        let sequence = session.next_sequence;
        session.next_sequence += 1;

        if let Err(err) = session.tx.send(ChunkJob { sequence, audio }) {
            warn!("Failed to queue live transcription chunk: {}", err);
        }
    }

    pub fn finish(&self, final_audio: Vec<f32>) -> Option<Result<String>> {
        let mut session = self.session.lock().unwrap().take()?;

        if !final_audio.is_empty() {
            let sequence = session.next_sequence;
            session.next_sequence += 1;
            if let Err(err) = session.tx.send(ChunkJob {
                sequence,
                audio: final_audio,
            }) {
                warn!("Failed to queue final live transcription chunk: {}", err);
            }
        }

        drop(session.tx);

        if let Some(handle) = session.handle.take() {
            if let Err(err) = handle.join() {
                return Some(Err(anyhow::anyhow!(
                    "Live transcription worker panicked: {:?}",
                    err
                )));
            }
        }

        let mut results = session.results.lock().unwrap().clone();
        results.sort_by_key(|item| item.sequence);
        Some(Ok(join_chunk_texts(
            results.into_iter().map(|item| item.text),
        )))
    }

    pub fn cancel(&self) {
        let _ = self.session.lock().unwrap().take();
    }
}

fn join_chunk_texts<I>(texts: I) -> String
where
    I: IntoIterator<Item = String>,
{
    let mut out = String::new();

    for text in texts {
        let text = text.trim();
        if text.is_empty() {
            continue;
        }

        if out.is_empty() {
            out.push_str(text);
            continue;
        }

        if starts_with_close_punctuation(text) {
            out.push_str(text);
        } else {
            out.push(' ');
            out.push_str(text);
        }
    }

    out
}

fn starts_with_close_punctuation(text: &str) -> bool {
    text.chars()
        .next()
        .map(|ch| {
            matches!(
                ch,
                '.' | ',' | '!' | '?' | ';' | ':' | ')' | ']' | '}' | '»'
            )
        })
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::join_chunk_texts;

    #[test]
    fn joins_chunk_texts_with_single_spaces() {
        let text = join_chunk_texts([
            "  hello world ".to_string(),
            "".to_string(),
            "next phrase".to_string(),
        ]);

        assert_eq!(text, "hello world next phrase");
    }

    #[test]
    fn joins_close_punctuation_without_extra_space() {
        let text = join_chunk_texts(["hello".to_string(), ", world".to_string()]);

        assert_eq!(text, "hello, world");
    }
}
