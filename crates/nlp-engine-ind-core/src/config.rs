use crate::error::{EngineError, Result};
use nlp_engine_ind_languages::Language;
use nlp_engine_ind_romanize::RomanizationStyle;

/// Conversational style for generation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Style {
    #[default]
    Casual,
    Neutral,
    Formal,
}

/// Code-mixing behavior.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CodeMixMode {
    #[default]
    Natural,
    Minimal,
    Aggressive,
}

/// Optional keyboard/context hints for conversion.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ConversionContext {
    pub previous_text: Option<String>,
    pub selected_text: Option<String>,
    pub current_sentence: Option<String>,
}

impl ConversionContext {
    pub fn with_selected(selected: impl Into<String>) -> Self {
        Self {
            selected_text: Some(selected.into()),
            ..Default::default()
        }
    }
}

/// Engine configuration applied to every conversion.
#[derive(Debug, Clone)]
pub struct EngineConfig {
    pub language: Language,
    pub style: Style,
    pub code_mixing: CodeMixMode,
    pub romanization: RomanizationStyle,
    /// Languages enabled for this engine instance. Empty means all compiled languages.
    pub enabled_languages: Vec<Language>,
    /// Minimum confidence threshold for downstream consumers (0.0–1.0).
    pub confidence_threshold: f32,
    /// When true, unknown words are preserved instead of guessed.
    pub strictness: bool,
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            language: Language::Kannada,
            style: Style::Casual,
            code_mixing: CodeMixMode::Natural,
            romanization: RomanizationStyle::CasualChat,
            enabled_languages: Language::all_enabled(),
            confidence_threshold: 0.5,
            strictness: true,
        }
    }
}

impl EngineConfig {
    pub fn for_language(language: Language) -> Result<Self> {
        let config = Self {
            language,
            enabled_languages: vec![language],
            ..Default::default()
        };
        config.validate()?;
        Ok(config)
    }

    pub fn validate(&self) -> Result<()> {
        if !self.language.is_enabled() {
            return Err(EngineError::UnsupportedLanguage {
                language: self.language.display_name().to_string(),
            });
        }
        if !self.enabled_languages.is_empty() && !self.enabled_languages.contains(&self.language) {
            return Err(EngineError::InvalidConfiguration {
                reason: format!(
                    "active language {} is not in enabled_languages",
                    self.language.display_name()
                ),
            });
        }
        for lang in &self.enabled_languages {
            if !lang.is_enabled() {
                return Err(EngineError::UnsupportedLanguage {
                    language: lang.display_name().to_string(),
                });
            }
        }
        if !(0.0..=1.0).contains(&self.confidence_threshold) {
            return Err(EngineError::InvalidConfiguration {
                reason: "confidence_threshold must be between 0.0 and 1.0".into(),
            });
        }
        Ok(())
    }

    pub fn is_language_available(&self, language: Language) -> bool {
        language.is_enabled()
            && (self.enabled_languages.is_empty() || self.enabled_languages.contains(&language))
    }
}

/// Request to convert English chat text.
#[derive(Debug, Clone)]
pub struct ConversionRequest {
    pub text: String,
    pub language: Language,
    pub style: Style,
    pub code_mixing: CodeMixMode,
    pub romanization: RomanizationStyle,
    pub context: Option<ConversionContext>,
    /// Deprecated alias kept for backward compatibility.
    pub code_mixed: bool,
}

impl ConversionRequest {
    pub fn new(text: impl Into<String>, language: Language) -> Self {
        Self {
            text: text.into(),
            language,
            style: Style::Casual,
            code_mixing: CodeMixMode::Natural,
            romanization: RomanizationStyle::CasualChat,
            context: None,
            code_mixed: true,
        }
    }

    pub fn with_code_mix(mut self, mode: CodeMixMode) -> Self {
        self.code_mixing = mode;
        self.code_mixed = !matches!(mode, CodeMixMode::Minimal);
        self
    }

    /// Text that should be converted, respecting keyboard context when present.
    pub fn effective_text(&self) -> &str {
        if let Some(ctx) = &self.context {
            if let Some(selected) = ctx.selected_text.as_deref() {
                let trimmed = selected.trim();
                if !trimmed.is_empty() {
                    return trimmed;
                }
            }
            if let Some(sentence) = ctx.current_sentence.as_deref() {
                let trimmed = sentence.trim();
                if !trimmed.is_empty() {
                    return trimmed;
                }
            }
        }
        self.text.trim()
    }
}

impl From<ConversionRequest> for EngineConfig {
    fn from(req: ConversionRequest) -> Self {
        EngineConfig {
            language: req.language,
            style: req.style,
            code_mixing: req.code_mixing,
            romanization: req.romanization,
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn effective_text_prefers_selected() {
        let req = ConversionRequest {
            text: "full message".into(),
            language: Language::Kannada,
            style: Style::Casual,
            code_mixing: CodeMixMode::Natural,
            romanization: RomanizationStyle::CasualChat,
            context: Some(ConversionContext {
                selected_text: Some("selected part".into()),
                ..Default::default()
            }),
            code_mixed: true,
        };
        assert_eq!(req.effective_text(), "selected part");
    }
}
