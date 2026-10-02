use crate::config::{ConversionRequest, EngineConfig};
use crate::error::Result;
use crate::types::ConversionResult;
use crate::{Converter, Language};

/// Thread-safe, reusable SDK engine instance.
#[derive(Debug, Clone)]
pub struct Engine {
    converter: Converter,
    config: EngineConfig,
}

impl Engine {
    pub fn new(config: EngineConfig) -> Result<Self> {
        config.validate()?;
        Ok(Self {
            converter: Converter::new(),
            config,
        })
    }

    pub fn with_language(language: Language) -> Result<Self> {
        Self::new(EngineConfig::for_language(language)?)
    }

    pub fn config(&self) -> &EngineConfig {
        &self.config
    }

    pub fn set_language(&mut self, language: Language) -> Result<()> {
        if !self.config.is_language_available(language) {
            return Err(crate::error::EngineError::UnsupportedLanguage {
                language: language.display_name().to_string(),
            });
        }
        self.config.language = language;
        Ok(())
    }

    pub fn convert(&self, request: ConversionRequest) -> Result<ConversionResult> {
        if !self.config.is_language_available(request.language) {
            return Err(crate::error::EngineError::UnsupportedLanguage {
                language: request.language.display_name().to_string(),
            });
        }
        let mut config = self.config.clone();
        config.language = request.language;
        config.style = request.style;
        config.code_mixing = request.code_mixing;
        config.romanization = request.romanization;
        self.converter
            .pipeline
            .convert_with_config(request.effective_text(), &config)
    }

    pub fn convert_text(&self, text: &str) -> Result<ConversionResult> {
        self.convert(ConversionRequest::new(text, self.config.language))
    }
}

/// High-level keyboard-oriented API.
#[derive(Debug, Clone)]
pub struct KeyboardConverter {
    engine: Engine,
}

/// Minimal result for keyboard integrations.
#[derive(Debug, Clone, PartialEq)]
pub struct KeyboardResult {
    pub output: String,
    pub confidence: f32,
    pub processing_time_us: u64,
}

impl KeyboardConverter {
    pub fn new(language: Language) -> Result<Self> {
        Ok(Self {
            engine: Engine::with_language(language)?,
        })
    }

    pub fn set_language(&mut self, language: Language) -> Result<()> {
        self.engine.set_language(language)
    }

    pub fn convert_text(
        &self,
        text: &str,
        language: Language,
        style: crate::Style,
        code_mix: crate::CodeMixMode,
    ) -> Result<KeyboardResult> {
        let result = self.engine.convert(
            ConversionRequest::new(text, language)
                .with_code_mix(code_mix)
                .with_style(style),
        )?;
        Ok(KeyboardResult::from(result))
    }

    pub fn convert(&self, text: &str) -> Result<KeyboardResult> {
        Ok(KeyboardResult::from(self.engine.convert_text(text)?))
    }
}

impl KeyboardResult {
    pub fn from(result: ConversionResult) -> Self {
        let processing_time_us = result.processing_time_us();
        Self {
            output: result.output,
            confidence: result.confidence,
            processing_time_us,
        }
    }
}

trait RequestStyleExt {
    fn with_style(self, style: crate::Style) -> ConversionRequest;
}

impl RequestStyleExt for ConversionRequest {
    fn with_style(mut self, style: crate::Style) -> ConversionRequest {
        self.style = style;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::CodeMixMode;

    #[test]
    fn keyboard_converter_works() {
        let kb = KeyboardConverter::new(Language::Kannada).unwrap();
        let result = kb
            .convert_text(
                "I'll call you later",
                Language::Kannada,
                crate::Style::Casual,
                CodeMixMode::Natural,
            )
            .unwrap();
        assert!(!result.output.is_empty());
    }
}
