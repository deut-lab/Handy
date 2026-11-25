//! How audio is decoded: the one place the model's traits and the settings
//! choose between streaming, long-form chunking and a single run.

use crate::audio_toolkit::chunking::ChunkSizes;
use crate::engine_supervisor::LoadedInfo;

/// What a model can do that decides how its audio is decoded.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ModelTraits {
    /// Decodes audio incrementally as it is recorded.
    pub streaming: bool,
    /// Keeps its accuracy on audio longer than its native window in one run
    /// (whisper today).
    pub long_form: bool,
}

impl ModelTraits {
    /// The loaded model's own traits: the authority once a model is loaded.
    pub fn of(info: &LoadedInfo) -> Self {
        Self {
            streaming: info.capabilities.supports_streaming,
            // Handy Voice: Parakeet takes long audio in one run, but loses
            // punctuation on it, so its long audio is chunked too.
            long_form: info.supports_long_form && !punctuates_per_run(info),
        }
    }
}

/// Handy Voice: Parakeet decides for a whole run whether to write
/// punctuation and capitals, and on runs of a minute or more it drops them
/// for long stretches. On recorded Russian dictations of 1.5-3.5 min the
/// longest stretch without a mark fell from 100-160 words in one run to
/// 40-60 with 30 s chunks and 9-20 with 15 s chunks.
fn punctuates_per_run(info: &LoadedInfo) -> bool {
    info.arch.to_ascii_lowercase().contains("parakeet")
}

/// How long the chunks of a chunked decode are for the loaded model.
pub fn chunk_sizes(info: &LoadedInfo) -> ChunkSizes {
    if punctuates_per_run(info) {
        ChunkSizes::SHORT
    } else {
        ChunkSizes::LONG
    }
}

/// How a recording is decoded while it is still going.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LiveDecode {
    /// The model decodes the audio as it arrives.
    Stream,
    /// Long audio is cut at pauses and each chunk decoded once its cut is
    /// certain.
    Chunked,
    /// Nothing until the recording stops; then it is decoded in batch.
    None,
}

/// How a whole clip is decoded: a recording without a live result, a
/// re-transcribed history entry, or a file from the CLI.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BatchDecode {
    /// Cut at pauses into chunks of about 30 s, each decoded in turn; a clip
    /// of 40 s or less is one chunk.
    Chunked,
    /// One run.
    Whole,
}

/// Streaming wins: a streaming model handles long audio itself. Otherwise
/// long audio is chunked unless the model keeps its accuracy on it.
pub fn live_decode(model: ModelTraits, long_form_chunking: bool) -> LiveDecode {
    if model.streaming {
        LiveDecode::Stream
    } else if long_form_chunking && !model.long_form {
        LiveDecode::Chunked
    } else {
        LiveDecode::None
    }
}

/// As live, except that a streaming model takes a whole clip in one run, as
/// it did before long-form chunking existed.
pub fn batch_decode(model: ModelTraits, long_form_chunking: bool) -> BatchDecode {
    if long_form_chunking && !model.long_form && !model.streaming {
        BatchDecode::Chunked
    } else {
        BatchDecode::Whole
    }
}
