//! # nlp-engine-ind-core
//!
//! Offline Indian conversational NLP engine core.
//!
//! Converts English chat messages into casual, code-mixed Roman-script
//! Indian-language conversational text.

pub mod analyzer;
pub mod code_mix;
pub mod confidence;
pub mod config;
pub mod engine;
pub mod error;
pub mod generator;
pub mod grammar;
pub mod intent;
pub mod normalizer;
pub mod pipeline;
pub mod protected_tokens;
pub mod tense;
pub mod timing;
pub mod tokenizer;
pub mod types;

pub use analyzer::{Analyzer, RuleBasedAnalyzer};
pub use config::{CodeMixMode, ConversionContext, ConversionRequest, EngineConfig, Style};
pub use engine::{Engine, KeyboardConverter, KeyboardResult};
pub use error::{EngineError, Result};
pub use pipeline::Pipeline;
pub use types::{ConversionResult, Diagnostics};

pub use nlp_engine_ind_languages::Language;
pub use nlp_engine_ind_romanize::RomanizationStyle;

/// Primary public API for converting English chat to Indian conversational text.
#[derive(Debug, Clone, Default)]
pub struct Converter {
    pub(crate) pipeline: Pipeline,
}

impl Converter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_config(&self, config: EngineConfig) -> ConfiguredConverter {
        ConfiguredConverter {
            inner: self.clone(),
            config,
        }
    }

    pub fn convert(&self, request: ConversionRequest) -> Result<ConversionResult> {
        self.pipeline.convert(request)
    }

    pub fn convert_text(&self, text: &str, language: Language) -> Result<ConversionResult> {
        self.convert(ConversionRequest::new(text, language))
    }
}

/// Converter bound to a specific configuration.
#[derive(Debug, Clone)]
pub struct ConfiguredConverter {
    inner: Converter,
    config: EngineConfig,
}

impl ConfiguredConverter {
    pub fn convert(&self, text: &str) -> Result<ConversionResult> {
        self.inner.pipeline.convert_with_config(text, &self.config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn public_api_works() {
        let converter = Converter::new();
        let result = converter
            .convert_text("I'll call you later", Language::Kannada)
            .unwrap();
        assert!(!result.output.is_empty());
    }

    #[test]
    fn concurrent_conversions() {
        use std::sync::Arc;
        use std::thread;

        let engine = Arc::new(Engine::with_language(Language::Kannada).unwrap());
        let handles: Vec<_> = (0..100)
            .map(|_| {
                let engine = Arc::clone(&engine);
                thread::spawn(move || engine.convert_text("I'll call you later").unwrap())
            })
            .collect();
        for handle in handles {
            assert!(!handle.join().unwrap().output.is_empty());
        }
    }
}
