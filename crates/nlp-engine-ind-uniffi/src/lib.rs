uniffi::setup_scaffolding!();

use nlp_engine_ind_core::{
    CodeMixMode as CoreCodeMix, ConversionContext as CoreContext, ConversionRequest as CoreRequest,
    ConversionResult as CoreResult, Engine as CoreEngine, EngineConfig as CoreConfig,
    EngineError as CoreError, KeyboardConverter, Result as CoreResultType,
    RomanizationStyle as CoreRoman, Style as CoreStyle,
};
use std::sync::Mutex;

#[derive(uniffi::Enum, Clone, Copy)]
pub enum Language {
    Kannada,
    Hindi,
    Telugu,
    Tamil,
    Malayalam,
}

#[derive(uniffi::Enum, Clone, Copy)]
pub enum Style {
    Casual,
    Neutral,
    Formal,
}

#[derive(uniffi::Enum, Clone, Copy)]
pub enum CodeMixMode {
    Natural,
    Minimal,
    Aggressive,
}

#[derive(uniffi::Enum, Clone, Copy)]
pub enum RomanizationStyle {
    CasualChat,
    Standard,
    Academic,
}

#[derive(uniffi::Record)]
pub struct ConversionContext {
    pub previous_text: Option<String>,
    pub selected_text: Option<String>,
    pub current_sentence: Option<String>,
}

#[derive(uniffi::Record)]
pub struct ConversionRequest {
    pub text: String,
    pub language: Language,
    pub style: Style,
    pub code_mix: CodeMixMode,
    pub romanization: RomanizationStyle,
    pub context: Option<ConversionContext>,
}

#[derive(uniffi::Record)]
pub struct ConversionResult {
    pub input: String,
    pub output: String,
    pub language: Language,
    pub confidence: f32,
    pub processing_time_us: u64,
}

#[derive(uniffi::Record)]
pub struct EngineConfig {
    pub language: Language,
    pub style: Style,
    pub code_mix: CodeMixMode,
    pub romanization: RomanizationStyle,
    pub enabled_languages: Vec<Language>,
}

#[derive(uniffi::Record)]
pub struct KeyboardResult {
    pub output: String,
    pub confidence: f32,
    pub processing_time_us: u64,
}

#[derive(Debug, thiserror::Error, uniffi::Error)]
pub enum EngineError {
    #[error("unsupported language: {language}")]
    UnsupportedLanguage { language: String },
    #[error("invalid input: {reason}")]
    InvalidInput { reason: String },
    #[error("empty input")]
    EmptyInput,
    #[error("generation failed: {reason}")]
    GenerationFailure { reason: String },
    #[error("invalid configuration: {reason}")]
    InvalidConfiguration { reason: String },
}

fn map_language(lang: Language) -> nlp_engine_ind_core::Language {
    match lang {
        Language::Kannada => nlp_engine_ind_core::Language::Kannada,
        Language::Hindi => nlp_engine_ind_core::Language::Hindi,
        Language::Telugu => nlp_engine_ind_core::Language::Telugu,
        Language::Tamil => nlp_engine_ind_core::Language::Tamil,
        Language::Malayalam => nlp_engine_ind_core::Language::Malayalam,
    }
}

fn map_language_back(lang: nlp_engine_ind_core::Language) -> Language {
    match lang {
        nlp_engine_ind_core::Language::Kannada => Language::Kannada,
        nlp_engine_ind_core::Language::Hindi => Language::Hindi,
        nlp_engine_ind_core::Language::Telugu => Language::Telugu,
        nlp_engine_ind_core::Language::Tamil => Language::Tamil,
        nlp_engine_ind_core::Language::Malayalam => Language::Malayalam,
    }
}

fn map_style(style: Style) -> CoreStyle {
    match style {
        Style::Casual => CoreStyle::Casual,
        Style::Neutral => CoreStyle::Neutral,
        Style::Formal => CoreStyle::Formal,
    }
}

fn map_code_mix(mode: CodeMixMode) -> CoreCodeMix {
    match mode {
        CodeMixMode::Natural => CoreCodeMix::Natural,
        CodeMixMode::Minimal => CoreCodeMix::Minimal,
        CodeMixMode::Aggressive => CoreCodeMix::Aggressive,
    }
}

fn map_roman(style: RomanizationStyle) -> CoreRoman {
    match style {
        RomanizationStyle::CasualChat => CoreRoman::CasualChat,
        RomanizationStyle::Standard => CoreRoman::Standard,
        RomanizationStyle::Academic => CoreRoman::Academic,
    }
}

fn map_error(err: CoreError) -> EngineError {
    match err {
        CoreError::UnsupportedLanguage { language } => {
            EngineError::UnsupportedLanguage { language }
        }
        CoreError::InvalidInput { reason } => EngineError::InvalidInput { reason },
        CoreError::EmptyInput => EngineError::EmptyInput,
        CoreError::GenerationFailure { reason } => EngineError::GenerationFailure { reason },
        CoreError::InvalidConfiguration { reason } => EngineError::InvalidConfiguration { reason },
        CoreError::ResourceMissing { resource } => {
            EngineError::GenerationFailure { reason: resource }
        }
    }
}

fn to_core_config(config: EngineConfig) -> CoreResultType<CoreConfig> {
    let core = CoreConfig {
        language: map_language(config.language),
        style: map_style(config.style),
        code_mixing: map_code_mix(config.code_mix),
        romanization: map_roman(config.romanization),
        enabled_languages: config
            .enabled_languages
            .into_iter()
            .map(map_language)
            .collect(),
        ..Default::default()
    };
    core.validate()?;
    Ok(core)
}

fn to_result(result: CoreResult) -> ConversionResult {
    let processing_time_us = result.processing_time_us();
    ConversionResult {
        input: result.input,
        output: result.output,
        language: map_language_back(result.language),
        confidence: result.confidence,
        processing_time_us,
    }
}

#[derive(uniffi::Object)]
pub struct NlpEngine {
    engine: Mutex<CoreEngine>,
    keyboard: Mutex<KeyboardConverter>,
}

#[uniffi::export]
impl NlpEngine {
    #[uniffi::constructor]
    pub fn new(config: EngineConfig) -> Result<Self, EngineError> {
        let core_config = to_core_config(config).map_err(map_error)?;
        let engine = CoreEngine::new(core_config.clone()).map_err(map_error)?;
        let keyboard = KeyboardConverter::new(core_config.language).map_err(map_error)?;
        Ok(Self {
            engine: Mutex::new(engine),
            keyboard: Mutex::new(keyboard),
        })
    }

    pub fn convert(&self, request: ConversionRequest) -> Result<ConversionResult, EngineError> {
        let core_request = CoreRequest {
            text: request.text,
            language: map_language(request.language),
            style: map_style(request.style),
            code_mixing: map_code_mix(request.code_mix),
            romanization: map_roman(request.romanization),
            context: request.context.map(|ctx| CoreContext {
                previous_text: ctx.previous_text,
                selected_text: ctx.selected_text,
                current_sentence: ctx.current_sentence,
            }),
            code_mixed: !matches!(request.code_mix, CodeMixMode::Minimal),
        };
        self.engine
            .lock()
            .map_err(|_| EngineError::GenerationFailure {
                reason: "engine lock poisoned".into(),
            })?
            .convert(core_request)
            .map(to_result)
            .map_err(map_error)
    }

    pub fn convert_text(&self, text: String) -> Result<ConversionResult, EngineError> {
        self.engine
            .lock()
            .map_err(|_| EngineError::GenerationFailure {
                reason: "engine lock poisoned".into(),
            })?
            .convert_text(&text)
            .map(to_result)
            .map_err(map_error)
    }

    pub fn convert_for_keyboard(&self, text: String) -> Result<KeyboardResult, EngineError> {
        let result = self
            .keyboard
            .lock()
            .map_err(|_| EngineError::GenerationFailure {
                reason: "keyboard lock poisoned".into(),
            })?
            .convert(&text)
            .map_err(map_error)?;
        Ok(KeyboardResult {
            output: result.output,
            confidence: result.confidence,
            processing_time_us: result.processing_time_us,
        })
    }

    pub fn set_language(&self, language: Language) -> Result<(), EngineError> {
        let mapped = map_language(language);
        self.engine
            .lock()
            .map_err(|_| EngineError::GenerationFailure {
                reason: "engine lock poisoned".into(),
            })?
            .set_language(mapped)
            .map_err(map_error)?;
        self.keyboard
            .lock()
            .map_err(|_| EngineError::GenerationFailure {
                reason: "keyboard lock poisoned".into(),
            })?
            .set_language(mapped)
            .map_err(map_error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uniffi_engine_converts() {
        let engine = NlpEngine::new(EngineConfig {
            language: Language::Kannada,
            style: Style::Casual,
            code_mix: CodeMixMode::Natural,
            romanization: RomanizationStyle::CasualChat,
            enabled_languages: vec![Language::Kannada],
        })
        .unwrap();
        let result = engine.convert_text("I'll call you later".into()).unwrap();
        assert!(!result.output.is_empty());
    }
}
