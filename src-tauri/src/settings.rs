use log::{debug, warn};
use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use specta::Type;
use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};
use tauri::AppHandle;
use tauri_plugin_store::StoreExt;

pub const APPLE_INTELLIGENCE_PROVIDER_ID: &str = "apple_intelligence";
pub const APPLE_INTELLIGENCE_DEFAULT_MODEL_ID: &str = "Apple Intelligence";

#[derive(Serialize, Debug, Clone, Copy, PartialEq, Eq, Type)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

// Custom deserializer to handle both old numeric format (1-5) and new string format ("trace", "debug", etc.)
impl<'de> Deserialize<'de> for LogLevel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LogLevelVisitor;

        impl<'de> Visitor<'de> for LogLevelVisitor {
            type Value = LogLevel;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a string or integer representing log level")
            }

            fn visit_str<E: de::Error>(self, value: &str) -> Result<LogLevel, E> {
                match value.to_lowercase().as_str() {
                    "trace" => Ok(LogLevel::Trace),
                    "debug" => Ok(LogLevel::Debug),
                    "info" => Ok(LogLevel::Info),
                    "warn" => Ok(LogLevel::Warn),
                    "error" => Ok(LogLevel::Error),
                    _ => Err(E::unknown_variant(
                        value,
                        &["trace", "debug", "info", "warn", "error"],
                    )),
                }
            }

            fn visit_u64<E: de::Error>(self, value: u64) -> Result<LogLevel, E> {
                match value {
                    1 => Ok(LogLevel::Trace),
                    2 => Ok(LogLevel::Debug),
                    3 => Ok(LogLevel::Info),
                    4 => Ok(LogLevel::Warn),
                    5 => Ok(LogLevel::Error),
                    _ => Err(E::invalid_value(de::Unexpected::Unsigned(value), &"1-5")),
                }
            }
        }

        deserializer.deserialize_any(LogLevelVisitor)
    }
}

impl From<LogLevel> for tauri_plugin_log::LogLevel {
    fn from(level: LogLevel) -> Self {
        match level {
            LogLevel::Trace => tauri_plugin_log::LogLevel::Trace,
            LogLevel::Debug => tauri_plugin_log::LogLevel::Debug,
            LogLevel::Info => tauri_plugin_log::LogLevel::Info,
            LogLevel::Warn => tauri_plugin_log::LogLevel::Warn,
            LogLevel::Error => tauri_plugin_log::LogLevel::Error,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Type)]
pub struct ShortcutBinding {
    pub id: String,
    pub name: String,
    pub description: String,
    pub default_binding: String,
    pub current_binding: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Type)]
pub struct LLMPrompt {
    pub id: String,
    pub name: String,
    pub prompt: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Type)]
pub struct PostProcessProvider {
    pub id: String,
    pub label: String,
    pub base_url: String,
    #[serde(default)]
    pub allow_base_url_edit: bool,
    #[serde(default)]
    pub models_endpoint: Option<String>,
    #[serde(default)]
    pub supports_structured_output: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Type)]
#[serde(rename_all = "lowercase")]
pub enum OverlayPosition {
    None,
    Top,
    Bottom,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Type)]
#[serde(rename_all = "snake_case")]
pub enum OverlayTheme {
    Calm,
    Classic,
    Dark,
    Gray,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Type)]
#[serde(rename_all = "snake_case")]
pub enum OverlayIconSet {
    Original,
    Line,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Type, Default)]
#[serde(rename_all = "snake_case")]
pub enum OverlayTranscribingIcon {
    #[default]
    ScanText,
    SquareDashedText,
    ScrollText,
    TextCursorInput,
    MessageSquareText,
    TextInitial,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Type)]
#[serde(rename_all = "snake_case")]
pub enum OverlayButtonStyle {
    Simple,
    Circle,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Type, Default)]
#[serde(rename_all = "snake_case")]
pub enum TrayIconStyle {
    Original,
    #[default]
    States,
    Logo,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Type)]
#[serde(rename_all = "snake_case")]
pub enum ModelUnloadTimeout {
    Never,
    Immediately,
    Min2,
    Min5,
    Min10,
    Min15,
    Hour1,
    Sec15, // Debug mode only
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Type)]
#[serde(rename_all = "snake_case")]
pub enum PasteMethod {
    CtrlV,
    Direct,
    None,
    ShiftInsert,
    CtrlShiftV,
    ExternalScript,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Type)]
#[serde(rename_all = "snake_case")]
pub enum ClipboardHandling {
    DontModify,
    CopyToClipboard,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Type)]
#[serde(rename_all = "snake_case")]
pub enum AutoSubmitKey {
    Enter,
    CtrlEnter,
    CmdEnter,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Type)]
#[serde(rename_all = "snake_case")]
pub enum LongDictationMode {
    Off,
    PauseChunks,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Type)]
#[serde(rename_all = "snake_case")]
pub enum RecordingRetentionPeriod {
    Never,
    PreserveLimit,
    Days3,
    Weeks2,
    Months3,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Type)]
#[serde(rename_all = "snake_case")]
pub enum KeyboardImplementation {
    Tauri,
    HandyKeys,
}

impl Default for KeyboardImplementation {
    fn default() -> Self {
        #[cfg(target_os = "linux")]
        return KeyboardImplementation::Tauri;
        #[cfg(not(target_os = "linux"))]
        return KeyboardImplementation::HandyKeys;
    }
}

impl Default for OverlayTheme {
    fn default() -> Self {
        OverlayTheme::Calm
    }
}

impl Default for OverlayIconSet {
    fn default() -> Self {
        OverlayIconSet::Line
    }
}

impl Default for OverlayButtonStyle {
    fn default() -> Self {
        OverlayButtonStyle::Circle
    }
}

impl Default for ModelUnloadTimeout {
    fn default() -> Self {
        ModelUnloadTimeout::Never
    }
}

impl Default for PasteMethod {
    fn default() -> Self {
        // Default to CtrlV for macOS and Windows, Direct for Linux
        #[cfg(target_os = "linux")]
        return PasteMethod::Direct;
        #[cfg(not(target_os = "linux"))]
        return PasteMethod::CtrlV;
    }
}

impl Default for ClipboardHandling {
    fn default() -> Self {
        ClipboardHandling::CopyToClipboard
    }
}

impl Default for AutoSubmitKey {
    fn default() -> Self {
        AutoSubmitKey::Enter
    }
}

impl Default for LongDictationMode {
    fn default() -> Self {
        LongDictationMode::PauseChunks
    }
}

impl ModelUnloadTimeout {
    pub fn to_minutes(self) -> Option<u64> {
        match self {
            ModelUnloadTimeout::Never => None,
            ModelUnloadTimeout::Immediately => Some(0), // Special case for immediate unloading
            ModelUnloadTimeout::Min2 => Some(2),
            ModelUnloadTimeout::Min5 => Some(5),
            ModelUnloadTimeout::Min10 => Some(10),
            ModelUnloadTimeout::Min15 => Some(15),
            ModelUnloadTimeout::Hour1 => Some(60),
            ModelUnloadTimeout::Sec15 => Some(0), // Special case for debug - handled separately
        }
    }

    pub fn to_seconds(self) -> Option<u64> {
        match self {
            ModelUnloadTimeout::Never => None,
            ModelUnloadTimeout::Immediately => Some(0), // Special case for immediate unloading
            ModelUnloadTimeout::Sec15 => Some(15),
            _ => self.to_minutes().map(|m| m * 60),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Type)]
#[serde(rename_all = "snake_case")]
pub enum SoundTheme {
    Marimba,
    Pop,
    Bell,
    Chime,
    Pluck,
    Switcher,
    Click,
    Mouse,
    SoftSwitch,
    BrightSwitch,
    Minimal,
    Custom,
}

impl SoundTheme {
    fn as_str(&self) -> &'static str {
        match self {
            SoundTheme::Marimba => "marimba",
            SoundTheme::Pop => "pop",
            SoundTheme::Bell => "bell",
            SoundTheme::Chime => "chime",
            SoundTheme::Pluck => "pluck",
            SoundTheme::Switcher => "switcher",
            SoundTheme::Click => "click",
            SoundTheme::Mouse => "mouse",
            SoundTheme::SoftSwitch => "soft_switch",
            SoundTheme::BrightSwitch => "bright_switch",
            SoundTheme::Minimal => "minimal",
            SoundTheme::Custom => "custom",
        }
    }

    pub fn to_start_path(&self) -> String {
        format!("resources/{}_start.wav", self.as_str())
    }

    pub fn to_stop_path(&self) -> String {
        format!("resources/{}_stop.wav", self.as_str())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Type)]
#[serde(rename_all = "snake_case")]
pub enum TypingTool {
    Auto,
    Wtype,
    Kwtype,
    Dotool,
    Ydotool,
    Xdotool,
}

impl Default for TypingTool {
    fn default() -> Self {
        TypingTool::Auto
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Type)]
#[serde(rename_all = "snake_case")]
pub enum WhisperAcceleratorSetting {
    Auto,
    Cpu,
    Gpu,
}

impl Default for WhisperAcceleratorSetting {
    fn default() -> Self {
        WhisperAcceleratorSetting::Gpu
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Type)]
#[serde(rename_all = "snake_case")]
pub enum OrtAcceleratorSetting {
    Auto,
    Cpu,
    Cuda,
    #[serde(rename = "directml")]
    DirectMl,
    Rocm,
}

impl Default for OrtAcceleratorSetting {
    fn default() -> Self {
        OrtAcceleratorSetting::Auto
    }
}

#[derive(Clone, Serialize, Deserialize, Type)]
#[serde(transparent)]
pub(crate) struct SecretMap(HashMap<String, String>);

impl fmt::Debug for SecretMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let redacted: HashMap<&String, &str> = self
            .0
            .iter()
            .map(|(k, v)| (k, if v.is_empty() { "" } else { "[REDACTED]" }))
            .collect();
        redacted.fmt(f)
    }
}

impl std::ops::Deref for SecretMap {
    type Target = HashMap<String, String>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for SecretMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/* still handy for composing the initial JSON in the store ------------- */
#[derive(Serialize, Deserialize, Debug, Clone, Type)]
pub struct AppSettings {
    pub bindings: HashMap<String, ShortcutBinding>,
    pub push_to_talk: bool,
    pub audio_feedback: bool,
    #[serde(default = "default_audio_feedback_volume")]
    pub audio_feedback_volume: f32,
    #[serde(default = "default_sound_theme")]
    pub sound_theme: SoundTheme,
    #[serde(default = "default_start_hidden")]
    pub start_hidden: bool,
    #[serde(default = "default_autostart_enabled")]
    pub autostart_enabled: bool,
    #[serde(default = "default_windows_task_startup_enabled")]
    pub windows_task_startup_enabled: bool,
    #[serde(default = "default_windows_task_startup_admin")]
    pub windows_task_startup_admin: bool,
    #[serde(default = "default_update_checks_enabled")]
    pub update_checks_enabled: bool,
    #[serde(default = "default_model")]
    pub selected_model: String,
    #[serde(default = "default_always_on_microphone")]
    pub always_on_microphone: bool,
    #[serde(default)]
    pub selected_microphone: Option<String>,
    #[serde(default)]
    pub clamshell_microphone: Option<String>,
    #[serde(default)]
    pub selected_output_device: Option<String>,
    #[serde(default = "default_translate_to_english")]
    pub translate_to_english: bool,
    #[serde(default = "default_selected_language")]
    pub selected_language: String,
    #[serde(default = "default_overlay_position")]
    pub overlay_position: OverlayPosition,
    #[serde(default)]
    pub overlay_theme: OverlayTheme,
    #[serde(default)]
    pub overlay_icon_set: OverlayIconSet,
    #[serde(default)]
    pub overlay_transcribing_icon: OverlayTranscribingIcon,
    #[serde(default)]
    pub overlay_button_style: OverlayButtonStyle,
    #[serde(
        default = "default_overlay_opacity",
        deserialize_with = "deserialize_overlay_opacity"
    )]
    pub overlay_opacity: u8,
    #[serde(default)]
    pub tray_icon_style: TrayIconStyle,
    #[serde(default = "default_debug_mode")]
    pub debug_mode: bool,
    #[serde(default = "default_log_level")]
    pub log_level: LogLevel,
    #[serde(default)]
    pub custom_words: Vec<String>,
    #[serde(default)]
    pub model_unload_timeout: ModelUnloadTimeout,
    #[serde(default = "default_word_correction_threshold")]
    pub word_correction_threshold: f64,
    #[serde(default = "default_history_limit")]
    pub history_limit: usize,
    #[serde(default = "default_recording_retention_period")]
    pub recording_retention_period: RecordingRetentionPeriod,
    #[serde(default)]
    pub paste_method: PasteMethod,
    #[serde(default)]
    pub clipboard_handling: ClipboardHandling,
    #[serde(default = "default_auto_submit")]
    pub auto_submit: bool,
    #[serde(default)]
    pub auto_submit_key: AutoSubmitKey,
    #[serde(default)]
    pub auto_stop_silence_enabled: bool,
    #[serde(default = "default_auto_stop_silence_seconds")]
    pub auto_stop_silence_seconds: u64,
    #[serde(default)]
    pub long_dictation_mode: LongDictationMode,
    #[serde(default = "default_long_dictation_silence_seconds")]
    pub long_dictation_silence_seconds: u64,
    #[serde(default = "default_long_dictation_min_chunk_seconds")]
    pub long_dictation_min_chunk_seconds: u64,
    #[serde(default = "default_post_process_enabled")]
    pub post_process_enabled: bool,
    #[serde(default = "default_post_process_provider_id")]
    pub post_process_provider_id: String,
    #[serde(default = "default_post_process_providers")]
    pub post_process_providers: Vec<PostProcessProvider>,
    #[serde(default = "default_post_process_api_keys")]
    pub post_process_api_keys: SecretMap,
    #[serde(default = "default_post_process_models")]
    pub post_process_models: HashMap<String, String>,
    #[serde(default = "default_post_process_prompts")]
    pub post_process_prompts: Vec<LLMPrompt>,
    #[serde(default)]
    pub post_process_selected_prompt_id: Option<String>,
    #[serde(default)]
    pub mute_while_recording: bool,
    #[serde(default)]
    pub append_trailing_space: bool,
    #[serde(default = "default_app_language")]
    pub app_language: String,
    #[serde(default)]
    pub experimental_enabled: bool,
    #[serde(default)]
    pub lazy_stream_close: bool,
    #[serde(default)]
    pub keyboard_implementation: KeyboardImplementation,
    #[serde(default = "default_show_tray_icon")]
    pub show_tray_icon: bool,
    #[serde(default = "default_paste_delay_ms")]
    pub paste_delay_ms: u64,
    #[serde(default = "default_typing_tool")]
    pub typing_tool: TypingTool,
    pub external_script_path: Option<String>,
    #[serde(default)]
    pub custom_filler_words: Option<Vec<String>>,
    #[serde(default)]
    pub whisper_accelerator: WhisperAcceleratorSetting,
    #[serde(default)]
    pub ort_accelerator: OrtAcceleratorSetting,
    #[serde(default = "default_whisper_gpu_device")]
    pub whisper_gpu_device: i32,
    #[serde(default)]
    pub extra_recording_buffer_ms: u64,
}

fn default_model() -> String {
    "parakeet-tdt-0.6b-v3".to_string()
}

fn default_always_on_microphone() -> bool {
    false
}

fn default_translate_to_english() -> bool {
    false
}

fn default_start_hidden() -> bool {
    true
}

fn default_autostart_enabled() -> bool {
    false
}

fn default_windows_task_startup_enabled() -> bool {
    cfg!(target_os = "windows")
}

fn default_windows_task_startup_admin() -> bool {
    cfg!(target_os = "windows")
}

fn default_update_checks_enabled() -> bool {
    true
}

fn default_selected_language() -> String {
    "auto".to_string()
}

fn default_overlay_position() -> OverlayPosition {
    #[cfg(target_os = "linux")]
    return OverlayPosition::None;
    #[cfg(not(target_os = "linux"))]
    return OverlayPosition::Bottom;
}

fn default_debug_mode() -> bool {
    true
}

fn default_log_level() -> LogLevel {
    LogLevel::Error
}

fn default_word_correction_threshold() -> f64 {
    0.18
}

fn default_paste_delay_ms() -> u64 {
    60
}

fn default_auto_submit() -> bool {
    true
}

fn default_auto_stop_silence_seconds() -> u64 {
    7
}

fn default_overlay_opacity() -> u8 {
    30
}

fn normalize_overlay_opacity(value: u64) -> u8 {
    (((value.min(100) + 5) / 10) * 10) as u8
}

fn deserialize_overlay_opacity<'de, D>(deserializer: D) -> Result<u8, D::Error>
where
    D: Deserializer<'de>,
{
    let value = serde_json::Value::deserialize(deserializer)?;

    match value {
        serde_json::Value::Number(number) => number
            .as_u64()
            .map(normalize_overlay_opacity)
            .ok_or_else(|| de::Error::custom("overlay opacity must be 0-100")),
        serde_json::Value::String(text) => {
            let percent = match text.as_str() {
                "solid" => 0,
                "medium" => 20,
                "light" => 30,
                other => other.parse::<u64>().map_err(de::Error::custom)?,
            };
            Ok(normalize_overlay_opacity(percent))
        }
        _ => Err(de::Error::custom(
            "overlay opacity must be a number or string",
        )),
    }
}

fn default_long_dictation_silence_seconds() -> u64 {
    1
}

fn default_long_dictation_min_chunk_seconds() -> u64 {
    8
}

fn default_history_limit() -> usize {
    999
}

fn default_recording_retention_period() -> RecordingRetentionPeriod {
    RecordingRetentionPeriod::Never
}

fn default_audio_feedback_volume() -> f32 {
    1.0
}

fn default_sound_theme() -> SoundTheme {
    SoundTheme::Pop
}

fn default_post_process_enabled() -> bool {
    false
}

fn default_app_language() -> String {
    tauri_plugin_os::locale()
        .map(|l| l.replace('_', "-"))
        .unwrap_or_else(|| "en".to_string())
}

fn default_show_tray_icon() -> bool {
    true
}

fn default_post_process_provider_id() -> String {
    "openai".to_string()
}

fn default_post_process_providers() -> Vec<PostProcessProvider> {
    let mut providers = vec![
        PostProcessProvider {
            id: "openai".to_string(),
            label: "OpenAI".to_string(),
            base_url: "https://api.openai.com/v1".to_string(),
            allow_base_url_edit: false,
            models_endpoint: Some("/models".to_string()),
            supports_structured_output: true,
        },
        PostProcessProvider {
            id: "zai".to_string(),
            label: "Z.AI".to_string(),
            base_url: "https://api.z.ai/api/paas/v4".to_string(),
            allow_base_url_edit: false,
            models_endpoint: Some("/models".to_string()),
            supports_structured_output: true,
        },
        PostProcessProvider {
            id: "openrouter".to_string(),
            label: "OpenRouter".to_string(),
            base_url: "https://openrouter.ai/api/v1".to_string(),
            allow_base_url_edit: false,
            models_endpoint: Some("/models".to_string()),
            supports_structured_output: true,
        },
        PostProcessProvider {
            id: "anthropic".to_string(),
            label: "Anthropic".to_string(),
            base_url: "https://api.anthropic.com/v1".to_string(),
            allow_base_url_edit: false,
            models_endpoint: Some("/models".to_string()),
            supports_structured_output: false,
        },
        PostProcessProvider {
            id: "groq".to_string(),
            label: "Groq".to_string(),
            base_url: "https://api.groq.com/openai/v1".to_string(),
            allow_base_url_edit: false,
            models_endpoint: Some("/models".to_string()),
            supports_structured_output: false,
        },
        PostProcessProvider {
            id: "cerebras".to_string(),
            label: "Cerebras".to_string(),
            base_url: "https://api.cerebras.ai/v1".to_string(),
            allow_base_url_edit: false,
            models_endpoint: Some("/models".to_string()),
            supports_structured_output: true,
        },
    ];

    // Note: We always include Apple Intelligence on macOS ARM64 without checking availability
    // at startup. The availability check is deferred to when the user actually tries to use it
    // (in actions.rs). This prevents crashes on macOS 26.x beta where accessing
    // SystemLanguageModel.default during early app initialization causes SIGABRT.
    #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
    {
        providers.push(PostProcessProvider {
            id: APPLE_INTELLIGENCE_PROVIDER_ID.to_string(),
            label: "Apple Intelligence".to_string(),
            base_url: "apple-intelligence://local".to_string(),
            allow_base_url_edit: false,
            models_endpoint: None,
            supports_structured_output: true,
        });
    }

    // AWS Bedrock via Mantle (OpenAI-compatible endpoint)
    providers.push(PostProcessProvider {
        id: "bedrock_mantle".to_string(),
        label: "AWS Bedrock (Mantle)".to_string(),
        base_url: "https://bedrock-mantle.us-east-1.api.aws/v1".to_string(),
        allow_base_url_edit: false,
        models_endpoint: Some("/models".to_string()),
        supports_structured_output: true,
    });

    // Custom provider always comes last
    providers.push(PostProcessProvider {
        id: "custom".to_string(),
        label: "Custom".to_string(),
        base_url: "http://localhost:11434/v1".to_string(),
        allow_base_url_edit: true,
        models_endpoint: Some("/models".to_string()),
        supports_structured_output: false,
    });

    providers
}

fn default_post_process_api_keys() -> SecretMap {
    let mut map = HashMap::new();
    for provider in default_post_process_providers() {
        map.insert(provider.id, String::new());
    }
    SecretMap(map)
}

fn default_model_for_provider(provider_id: &str) -> String {
    if provider_id == APPLE_INTELLIGENCE_PROVIDER_ID {
        return APPLE_INTELLIGENCE_DEFAULT_MODEL_ID.to_string();
    }
    String::new()
}

fn default_post_process_models() -> HashMap<String, String> {
    let mut map = HashMap::new();
    for provider in default_post_process_providers() {
        map.insert(
            provider.id.clone(),
            default_model_for_provider(&provider.id),
        );
    }
    map
}

fn default_post_process_prompts() -> Vec<LLMPrompt> {
    vec![LLMPrompt {
        id: "default_improve_transcriptions".to_string(),
        name: "Improve Transcriptions".to_string(),
        prompt: "Clean this transcript:\n1. Fix spelling, capitalization, and punctuation errors\n2. Convert number words to digits (twenty-five → 25, ten percent → 10%, five dollars → $5)\n3. Replace spoken punctuation with symbols (period → ., comma → ,, question mark → ?)\n4. Remove filler words (um, uh, like as filler)\n5. Keep the language in the original version (if it was french, keep it in french for example)\n\nPreserve exact meaning and word order. Do not paraphrase or reorder content.\n\nReturn only the cleaned transcript.\n\nTranscript:\n${output}".to_string(),
    }]
}

fn default_whisper_gpu_device() -> i32 {
    0
}

fn default_typing_tool() -> TypingTool {
    TypingTool::Auto
}

fn ensure_post_process_defaults(settings: &mut AppSettings) -> bool {
    let mut changed = false;
    for provider in default_post_process_providers() {
        // Use match to do a single lookup - either sync existing or add new
        match settings
            .post_process_providers
            .iter_mut()
            .find(|p| p.id == provider.id)
        {
            Some(existing) => {
                // Sync supports_structured_output field for existing providers (migration)
                if existing.supports_structured_output != provider.supports_structured_output {
                    debug!(
                        "Updating supports_structured_output for provider '{}' from {} to {}",
                        provider.id,
                        existing.supports_structured_output,
                        provider.supports_structured_output
                    );
                    existing.supports_structured_output = provider.supports_structured_output;
                    changed = true;
                }
            }
            None => {
                // Provider doesn't exist, add it
                settings.post_process_providers.push(provider.clone());
                changed = true;
            }
        }

        if !settings.post_process_api_keys.contains_key(&provider.id) {
            settings
                .post_process_api_keys
                .insert(provider.id.clone(), String::new());
            changed = true;
        }

        let default_model = default_model_for_provider(&provider.id);
        match settings.post_process_models.get_mut(&provider.id) {
            Some(existing) => {
                if existing.is_empty() && !default_model.is_empty() {
                    *existing = default_model.clone();
                    changed = true;
                }
            }
            None => {
                settings
                    .post_process_models
                    .insert(provider.id.clone(), default_model);
                changed = true;
            }
        }
    }

    changed
}

pub const SETTINGS_STORE_PATH: &str = "settings_store.json";

fn import_settings_if_missing(target: &Path, old: &Path) -> std::io::Result<bool> {
    if target.exists() || !old.is_file() {
        return Ok(false);
    }

    if let Some(parent) = target.parent() {
        fs::create_dir_all(parent)?;
    }

    fs::copy(old, target)?;
    reset_imported_auto_fields(target)?;
    Ok(true)
}

fn reset_imported_auto_fields(path: &Path) -> std::io::Result<()> {
    let text = fs::read_to_string(path)?;
    let mut value: serde_json::Value =
        serde_json::from_str(&text).map_err(std::io::Error::other)?;

    if let Some(settings) = value
        .get_mut("settings")
        .and_then(|item| item.as_object_mut())
    {
        settings.insert(
            "app_language".to_string(),
            serde_json::Value::String(default_app_language()),
        );
        settings.insert(
            "selected_language".to_string(),
            serde_json::Value::String(default_selected_language()),
        );
    }

    let text = serde_json::to_string_pretty(&value).map_err(std::io::Error::other)?;
    fs::write(path, text)
}

#[cfg(target_os = "windows")]
fn old_handy_settings_path() -> Option<PathBuf> {
    std::env::var_os("APPDATA")
        .map(PathBuf::from)
        .map(|path| path.join("com.pais.handy").join(SETTINGS_STORE_PATH))
}

#[cfg(not(target_os = "windows"))]
fn old_handy_settings_path() -> Option<PathBuf> {
    None
}

fn import_old_handy_settings(app: &AppHandle) {
    if crate::portable::is_portable() {
        return;
    }

    let Ok(target) = crate::portable::resolve_app_data(app, SETTINGS_STORE_PATH) else {
        return;
    };

    let Some(old) = old_handy_settings_path() else {
        return;
    };

    if target == old {
        return;
    }

    match import_settings_if_missing(&target, &old) {
        Ok(true) => debug!("Imported settings from old Handy profile"),
        Ok(false) => {}
        Err(error) => warn!("Failed to import old Handy settings: {}", error),
    }
}

pub fn get_default_settings() -> AppSettings {
    #[cfg(target_os = "windows")]
    let default_shortcut = "`";
    #[cfg(target_os = "macos")]
    let default_shortcut = "option+space";
    #[cfg(target_os = "linux")]
    let default_shortcut = "ctrl+space";
    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    let default_shortcut = "alt+space";

    let mut bindings = HashMap::new();
    bindings.insert(
        "transcribe".to_string(),
        ShortcutBinding {
            id: "transcribe".to_string(),
            name: "Transcribe".to_string(),
            description: "Converts your speech into text.".to_string(),
            default_binding: default_shortcut.to_string(),
            current_binding: default_shortcut.to_string(),
        },
    );
    #[cfg(target_os = "windows")]
    let default_post_process_shortcut = "ctrl+shift+space";
    #[cfg(target_os = "macos")]
    let default_post_process_shortcut = "option+shift+space";
    #[cfg(target_os = "linux")]
    let default_post_process_shortcut = "ctrl+shift+space";
    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    let default_post_process_shortcut = "alt+shift+space";

    bindings.insert(
        "transcribe_with_post_process".to_string(),
        ShortcutBinding {
            id: "transcribe_with_post_process".to_string(),
            name: "Transcribe with Post-Processing".to_string(),
            description: "Converts your speech into text and applies AI post-processing."
                .to_string(),
            default_binding: default_post_process_shortcut.to_string(),
            current_binding: default_post_process_shortcut.to_string(),
        },
    );
    bindings.insert(
        "cancel".to_string(),
        ShortcutBinding {
            id: "cancel".to_string(),
            name: "Cancel".to_string(),
            description: "Cancels the current recording.".to_string(),
            default_binding: "escape".to_string(),
            current_binding: "escape".to_string(),
        },
    );

    AppSettings {
        bindings,
        push_to_talk: false,
        audio_feedback: true,
        audio_feedback_volume: default_audio_feedback_volume(),
        sound_theme: default_sound_theme(),
        start_hidden: default_start_hidden(),
        autostart_enabled: default_autostart_enabled(),
        windows_task_startup_enabled: default_windows_task_startup_enabled(),
        windows_task_startup_admin: default_windows_task_startup_admin(),
        update_checks_enabled: default_update_checks_enabled(),
        selected_model: default_model(),
        always_on_microphone: false,
        selected_microphone: None,
        clamshell_microphone: None,
        selected_output_device: None,
        translate_to_english: false,
        selected_language: default_selected_language(),
        overlay_position: default_overlay_position(),
        overlay_theme: OverlayTheme::default(),
        overlay_icon_set: OverlayIconSet::default(),
        overlay_transcribing_icon: OverlayTranscribingIcon::default(),
        overlay_button_style: OverlayButtonStyle::default(),
        overlay_opacity: default_overlay_opacity(),
        tray_icon_style: TrayIconStyle::default(),
        debug_mode: default_debug_mode(),
        log_level: default_log_level(),
        custom_words: Vec::new(),
        model_unload_timeout: ModelUnloadTimeout::default(),
        word_correction_threshold: default_word_correction_threshold(),
        history_limit: default_history_limit(),
        recording_retention_period: default_recording_retention_period(),
        paste_method: PasteMethod::default(),
        clipboard_handling: ClipboardHandling::default(),
        auto_submit: default_auto_submit(),
        auto_submit_key: AutoSubmitKey::default(),
        auto_stop_silence_enabled: true,
        auto_stop_silence_seconds: default_auto_stop_silence_seconds(),
        long_dictation_mode: LongDictationMode::default(),
        long_dictation_silence_seconds: default_long_dictation_silence_seconds(),
        long_dictation_min_chunk_seconds: default_long_dictation_min_chunk_seconds(),
        post_process_enabled: default_post_process_enabled(),
        post_process_provider_id: default_post_process_provider_id(),
        post_process_providers: default_post_process_providers(),
        post_process_api_keys: default_post_process_api_keys(),
        post_process_models: default_post_process_models(),
        post_process_prompts: default_post_process_prompts(),
        post_process_selected_prompt_id: None,
        mute_while_recording: false,
        append_trailing_space: false,
        app_language: default_app_language(),
        experimental_enabled: true,
        lazy_stream_close: false,
        keyboard_implementation: KeyboardImplementation::default(),
        show_tray_icon: default_show_tray_icon(),
        paste_delay_ms: default_paste_delay_ms(),
        typing_tool: default_typing_tool(),
        external_script_path: None,
        custom_filler_words: None,
        whisper_accelerator: WhisperAcceleratorSetting::default(),
        ort_accelerator: OrtAcceleratorSetting::default(),
        whisper_gpu_device: default_whisper_gpu_device(),
        extra_recording_buffer_ms: 250,
    }
}

impl AppSettings {
    pub fn active_post_process_provider(&self) -> Option<&PostProcessProvider> {
        self.post_process_providers
            .iter()
            .find(|provider| provider.id == self.post_process_provider_id)
    }

    pub fn post_process_provider(&self, provider_id: &str) -> Option<&PostProcessProvider> {
        self.post_process_providers
            .iter()
            .find(|provider| provider.id == provider_id)
    }

    pub fn post_process_provider_mut(
        &mut self,
        provider_id: &str,
    ) -> Option<&mut PostProcessProvider> {
        self.post_process_providers
            .iter_mut()
            .find(|provider| provider.id == provider_id)
    }
}

pub fn load_or_create_app_settings(app: &AppHandle) -> AppSettings {
    import_old_handy_settings(app);

    // Initialize store
    let store = app
        .store(crate::portable::store_path(SETTINGS_STORE_PATH))
        .expect("Failed to initialize store");

    let mut settings = if let Some(settings_value) = store.get("settings") {
        // Parse the entire settings object
        match serde_json::from_value::<AppSettings>(settings_value) {
            Ok(mut settings) => {
                debug!("Found existing settings: {:?}", settings);
                let default_settings = get_default_settings();
                let mut updated = false;

                // Merge default bindings into existing settings
                for (key, value) in default_settings.bindings {
                    if !settings.bindings.contains_key(&key) {
                        debug!("Adding missing binding: {}", key);
                        settings.bindings.insert(key, value);
                        updated = true;
                    }
                }

                if updated {
                    debug!("Settings updated with new bindings");
                    store.set("settings", serde_json::to_value(&settings).unwrap());
                }

                settings
            }
            Err(e) => {
                warn!("Failed to parse settings: {}", e);
                // Fall back to default settings if parsing fails
                let default_settings = get_default_settings();
                store.set("settings", serde_json::to_value(&default_settings).unwrap());
                default_settings
            }
        }
    } else {
        let default_settings = get_default_settings();
        store.set("settings", serde_json::to_value(&default_settings).unwrap());
        default_settings
    };

    if ensure_post_process_defaults(&mut settings) {
        store.set("settings", serde_json::to_value(&settings).unwrap());
    }

    settings
}

pub fn get_settings(app: &AppHandle) -> AppSettings {
    import_old_handy_settings(app);

    let store = app
        .store(crate::portable::store_path(SETTINGS_STORE_PATH))
        .expect("Failed to initialize store");

    let mut settings = if let Some(settings_value) = store.get("settings") {
        serde_json::from_value::<AppSettings>(settings_value).unwrap_or_else(|_| {
            let default_settings = get_default_settings();
            store.set("settings", serde_json::to_value(&default_settings).unwrap());
            default_settings
        })
    } else {
        let default_settings = get_default_settings();
        store.set("settings", serde_json::to_value(&default_settings).unwrap());
        default_settings
    };

    if ensure_post_process_defaults(&mut settings) {
        store.set("settings", serde_json::to_value(&settings).unwrap());
    }

    settings
}

pub fn write_settings(app: &AppHandle, settings: AppSettings) {
    let store = app
        .store(crate::portable::store_path(SETTINGS_STORE_PATH))
        .expect("Failed to initialize store");

    store.set("settings", serde_json::to_value(&settings).unwrap());
}

pub fn get_bindings(app: &AppHandle) -> HashMap<String, ShortcutBinding> {
    let settings = get_settings(app);

    settings.bindings
}

pub fn get_stored_binding(app: &AppHandle, id: &str) -> ShortcutBinding {
    let bindings = get_bindings(app);

    let binding = bindings.get(id).unwrap().clone();

    binding
}

pub fn get_history_limit(app: &AppHandle) -> usize {
    let settings = get_settings(app);
    settings.history_limit
}

pub fn get_recording_retention_period(app: &AppHandle) -> RecordingRetentionPeriod {
    let settings = get_settings(app);
    settings.recording_retention_period
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_settings_enable_auto_submit() {
        let settings = get_default_settings();
        assert!(settings.auto_submit);
        assert_eq!(settings.auto_submit_key, AutoSubmitKey::Enter);
    }

    #[test]
    fn default_settings_enable_silence_stop() {
        let settings = get_default_settings();
        assert!(settings.auto_stop_silence_enabled);
        assert_eq!(settings.auto_stop_silence_seconds, 7);
    }

    #[test]
    fn default_language_settings_stay_automatic() {
        let settings = get_default_settings();
        assert_eq!(settings.selected_language, "auto");
    }

    #[test]
    fn default_personal_settings_match_profile() {
        let settings = get_default_settings();
        assert!(!settings.push_to_talk);
        assert!(settings.audio_feedback);
        assert_eq!(settings.audio_feedback_volume, 1.0);
        assert_eq!(settings.sound_theme, SoundTheme::Pop);
        assert!(settings.start_hidden);
        assert!(!settings.autostart_enabled);
        assert_eq!(settings.selected_model, "parakeet-tdt-0.6b-v3");
        assert!(!settings.always_on_microphone);
        assert!(!settings.translate_to_english);
        #[cfg(target_os = "linux")]
        assert_eq!(settings.overlay_position, OverlayPosition::None);
        #[cfg(not(target_os = "linux"))]
        assert_eq!(settings.overlay_position, OverlayPosition::Bottom);
        assert_eq!(settings.overlay_theme, OverlayTheme::Calm);
        assert_eq!(settings.overlay_icon_set, OverlayIconSet::Line);
        assert_eq!(
            settings.overlay_transcribing_icon,
            OverlayTranscribingIcon::ScanText
        );
        assert_eq!(settings.overlay_button_style, OverlayButtonStyle::Circle);
        assert_eq!(settings.overlay_opacity, 30);
        assert_eq!(settings.tray_icon_style, TrayIconStyle::States);
        assert!(settings.debug_mode);
        assert_eq!(settings.log_level, LogLevel::Error);
        assert_eq!(settings.model_unload_timeout, ModelUnloadTimeout::Never);
        assert_eq!(settings.word_correction_threshold, 0.18);
        assert_eq!(settings.history_limit, 999);
        assert_eq!(
            settings.recording_retention_period,
            RecordingRetentionPeriod::Never
        );
        #[cfg(target_os = "linux")]
        assert_eq!(settings.paste_method, PasteMethod::Direct);
        #[cfg(not(target_os = "linux"))]
        assert_eq!(settings.paste_method, PasteMethod::CtrlV);
        assert_eq!(
            settings.clipboard_handling,
            ClipboardHandling::CopyToClipboard
        );
        assert!(settings.auto_submit);
        assert_eq!(settings.auto_submit_key, AutoSubmitKey::Enter);
        assert!(settings.auto_stop_silence_enabled);
        assert_eq!(settings.auto_stop_silence_seconds, 7);
        assert_eq!(settings.long_dictation_mode, LongDictationMode::PauseChunks);
        assert_eq!(settings.long_dictation_silence_seconds, 1);
        assert_eq!(settings.long_dictation_min_chunk_seconds, 8);
        assert!(!settings.post_process_enabled);
        assert_eq!(settings.post_process_provider_id, "openai");
        assert!(!settings.mute_while_recording);
        assert!(!settings.append_trailing_space);
        assert!(settings.experimental_enabled);
        assert!(!settings.lazy_stream_close);
        #[cfg(target_os = "linux")]
        assert_eq!(
            settings.keyboard_implementation,
            KeyboardImplementation::Tauri
        );
        #[cfg(not(target_os = "linux"))]
        assert_eq!(
            settings.keyboard_implementation,
            KeyboardImplementation::HandyKeys
        );
        assert!(settings.show_tray_icon);
        assert_eq!(settings.paste_delay_ms, 60);
        assert_eq!(settings.typing_tool, TypingTool::Auto);
        assert_eq!(settings.whisper_accelerator, WhisperAcceleratorSetting::Gpu);
        assert_eq!(settings.ort_accelerator, OrtAcceleratorSetting::Auto);
        assert_eq!(settings.whisper_gpu_device, 0);
        assert_eq!(settings.extra_recording_buffer_ms, 250);
    }

    #[test]
    fn default_long_dictation_uses_pause_chunks() {
        let settings = get_default_settings();
        assert_eq!(settings.long_dictation_mode, LongDictationMode::PauseChunks);
        assert_eq!(settings.long_dictation_silence_seconds, 1);
        assert_eq!(settings.long_dictation_min_chunk_seconds, 8);
    }

    #[test]
    fn long_dictation_mode_uses_stable_store_names() {
        assert_eq!(
            serde_json::to_string(&LongDictationMode::Off).unwrap(),
            "\"off\""
        );
        assert_eq!(
            serde_json::to_string(&LongDictationMode::PauseChunks).unwrap(),
            "\"pause_chunks\""
        );
    }

    #[test]
    fn imported_settings_reset_language_fields() {
        let dir = tempfile::tempdir().unwrap();
        let old = dir.path().join("old.json");
        let target = dir.path().join("new").join(SETTINGS_STORE_PATH);

        fs::write(
            &old,
            r#"{
              "settings": {
                "app_language": "fixed-old",
                "selected_language": "ru",
                "debug_mode": true
              }
            }"#,
        )
        .unwrap();

        assert!(import_settings_if_missing(&target, &old).unwrap());

        let value: serde_json::Value =
            serde_json::from_str(&fs::read_to_string(&target).unwrap()).unwrap();
        let settings = value.get("settings").unwrap();
        assert_ne!(settings.get("app_language").unwrap(), "fixed-old");
        assert_eq!(settings.get("selected_language").unwrap(), "auto");
        assert_eq!(settings.get("debug_mode").unwrap(), true);
    }

    #[test]
    fn default_windows_task_startup_settings_follow_platform() {
        let settings = get_default_settings();

        #[cfg(target_os = "windows")]
        {
            assert!(settings.windows_task_startup_enabled);
            assert!(settings.windows_task_startup_admin);
        }

        #[cfg(not(target_os = "windows"))]
        {
            assert!(!settings.windows_task_startup_enabled);
            assert!(!settings.windows_task_startup_admin);
        }
    }

    #[test]
    fn default_overlay_visual_settings_keep_author_style() {
        let settings = get_default_settings();
        assert_eq!(settings.overlay_theme, OverlayTheme::Calm);
        assert_eq!(settings.overlay_icon_set, OverlayIconSet::Line);
        assert_eq!(
            settings.overlay_transcribing_icon,
            OverlayTranscribingIcon::ScanText
        );
        assert_eq!(settings.overlay_button_style, OverlayButtonStyle::Circle);
        assert_eq!(settings.overlay_opacity, 30);
    }

    #[test]
    fn default_tray_icon_style_uses_state_icons() {
        let settings = get_default_settings();
        assert_eq!(settings.tray_icon_style, TrayIconStyle::States);
    }

    #[test]
    fn overlay_visual_settings_use_stable_store_names() {
        assert_eq!(
            serde_json::to_string(&OverlayTheme::Gray).unwrap(),
            "\"gray\""
        );
        assert_eq!(
            serde_json::to_string(&OverlayIconSet::Original).unwrap(),
            "\"original\""
        );
        assert_eq!(
            serde_json::to_string(&OverlayIconSet::Line).unwrap(),
            "\"line\""
        );
        assert_eq!(
            serde_json::to_string(&OverlayTranscribingIcon::ScanText).unwrap(),
            "\"scan_text\""
        );
        assert_eq!(
            serde_json::to_string(&OverlayTranscribingIcon::SquareDashedText).unwrap(),
            "\"square_dashed_text\""
        );
        assert_eq!(
            serde_json::to_string(&OverlayTranscribingIcon::ScrollText).unwrap(),
            "\"scroll_text\""
        );
        assert_eq!(
            serde_json::to_string(&OverlayTranscribingIcon::TextCursorInput).unwrap(),
            "\"text_cursor_input\""
        );
        assert_eq!(
            serde_json::to_string(&OverlayTranscribingIcon::MessageSquareText).unwrap(),
            "\"message_square_text\""
        );
        assert_eq!(
            serde_json::to_string(&OverlayTranscribingIcon::TextInitial).unwrap(),
            "\"text_initial\""
        );
        assert_eq!(
            serde_json::to_string(&OverlayButtonStyle::Simple).unwrap(),
            "\"simple\""
        );
        assert_eq!(
            serde_json::to_string(&OverlayButtonStyle::Circle).unwrap(),
            "\"circle\""
        );
        assert_eq!(
            serde_json::to_string(&TrayIconStyle::Original).unwrap(),
            "\"original\""
        );
        assert_eq!(
            serde_json::to_string(&TrayIconStyle::States).unwrap(),
            "\"states\""
        );
        assert_eq!(
            serde_json::to_string(&TrayIconStyle::Logo).unwrap(),
            "\"logo\""
        );
    }

    #[test]
    fn overlay_opacity_accepts_old_names_and_rounds_numbers() {
        assert_eq!(normalize_overlay_opacity(0), 0);
        assert_eq!(normalize_overlay_opacity(14), 10);
        assert_eq!(normalize_overlay_opacity(15), 20);
        assert_eq!(normalize_overlay_opacity(101), 100);

        let value: AppSettings = serde_json::from_value(serde_json::json!({
            "bindings": {},
            "push_to_talk": false,
            "audio_feedback": false,
            "overlay_opacity": "light",
            "external_script_path": null
        }))
        .unwrap();

        assert_eq!(value.overlay_opacity, 30);
    }

    #[test]
    fn built_in_sound_themes_have_start_and_stop_paths() {
        let themes = [
            (SoundTheme::Marimba, "marimba"),
            (SoundTheme::Pop, "pop"),
            (SoundTheme::Bell, "bell"),
            (SoundTheme::Chime, "chime"),
            (SoundTheme::Pluck, "pluck"),
            (SoundTheme::Switcher, "switcher"),
            (SoundTheme::Click, "click"),
            (SoundTheme::Mouse, "mouse"),
            (SoundTheme::SoftSwitch, "soft_switch"),
            (SoundTheme::BrightSwitch, "bright_switch"),
            (SoundTheme::Minimal, "minimal"),
        ];

        for (theme, name) in themes {
            assert_eq!(
                theme.to_start_path(),
                format!("resources/{}_start.wav", name)
            );
            assert_eq!(theme.to_stop_path(), format!("resources/{}_stop.wav", name));
        }
    }

    #[test]
    fn debug_output_redacts_api_keys() {
        let mut settings = get_default_settings();
        settings
            .post_process_api_keys
            .insert("openai".to_string(), "sk-proj-secret-key-12345".to_string());
        settings.post_process_api_keys.insert(
            "anthropic".to_string(),
            "sk-ant-secret-key-67890".to_string(),
        );
        settings
            .post_process_api_keys
            .insert("empty_provider".to_string(), "".to_string());

        let debug_output = format!("{:?}", settings);

        assert!(!debug_output.contains("sk-proj-secret-key-12345"));
        assert!(!debug_output.contains("sk-ant-secret-key-67890"));
        assert!(debug_output.contains("[REDACTED]"));
    }

    #[test]
    fn secret_map_debug_redacts_values() {
        let map = SecretMap(HashMap::from([("key".into(), "secret".into())]));
        let out = format!("{:?}", map);
        assert!(!out.contains("secret"));
        assert!(out.contains("[REDACTED]"));
    }
}
