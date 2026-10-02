use crate::analyzer::{to_language_intent, to_language_tense, Analyzer, RuleBasedAnalyzer};
use crate::confidence::{build_diagnostics, compute_confidence};
use crate::config::{ConversionRequest, EngineConfig};
use crate::error::{EngineError, Result};
use crate::normalizer::{extract_leading_expressive, extract_trailing_punct, normalize};
use crate::protected_tokens::{mark_protected, protected_strings};
use crate::tokenizer::{content_tokens, tokenize};
use crate::types::ConversionResult;
use nlp_engine_ind_languages::generator::GenerationContext;
use nlp_engine_ind_languages::generator_for;
use nlp_engine_ind_romanize::romanize;
use crate::timing::TimerStart;

/// Main offline conversion pipeline.
#[derive(Debug, Default, Clone)]
pub struct Pipeline {
    analyzer: RuleBasedAnalyzer,
}

impl Pipeline {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn convert_with_config(
        &self,
        input: &str,
        config: &EngineConfig,
    ) -> Result<ConversionResult> {
        config.validate()?;

        let trimmed = input.trim();
        if trimmed.is_empty() {
            return Err(EngineError::EmptyInput);
        }

        let lang_gen =
            generator_for(config.language).ok_or_else(|| EngineError::UnsupportedLanguage {
                language: config.language.display_name().to_string(),
            })?;

        let start = TimerStart::now();
        let normalized = normalize(trimmed);
        let mut tokens = tokenize(&normalized);
        mark_protected(&mut tokens);
        let analysis = self.analyzer.analyze(&normalized, &tokens);

        let gen_ctx = build_gen_ctx(trimmed, &normalized, &analysis, &tokens);
        let gen_out = lang_gen.generate(&gen_ctx);
        if gen_out.text.is_empty() {
            return Err(EngineError::GenerationFailure {
                reason: "empty output from language generator".into(),
            });
        }

        let output = romanize(&gen_out.text, config.language, config.romanization);
        let protected_count = protected_strings(&tokens).len();
        let unknown_count = count_unknown(&tokens, &gen_ctx, lang_gen);
        let confidence = compute_confidence(
            &analysis,
            gen_out.template_matched,
            gen_out.conversion_ratio,
            unknown_count,
            protected_count,
        );

        Ok(ConversionResult {
            input: trimmed.to_string(),
            output,
            language: config.language,
            confidence,
            intent: analysis.intent,
            latency_us: start.elapsed_us(),
            diagnostics: build_diagnostics(
                gen_out.template_matched,
                gen_out.conversion_ratio,
                unknown_count,
                protected_count,
            ),
        })
    }

    pub fn convert(&self, request: ConversionRequest) -> Result<ConversionResult> {
        let config: EngineConfig = request.clone().into();
        self.convert_with_config(request.effective_text(), &config)
    }
}

fn count_unknown(
    tokens: &[crate::types::Token],
    ctx: &GenerationContext,
    lang_gen: &dyn nlp_engine_ind_languages::LanguageGenerator,
) -> usize {
    content_tokens(tokens)
        .iter()
        .filter(|t| {
            !t.protected
                && lang_gen.lexicon().lookup_word(&t.normalized).is_none()
                && !lang_gen.should_preserve_english(&t.text, ctx)
        })
        .count()
}

fn build_gen_ctx(
    original: &str,
    normalized: &str,
    analysis: &crate::types::Analysis,
    tokens: &[crate::types::Token],
) -> GenerationContext {
    GenerationContext {
        normalized: normalized.to_string(),
        intent: to_language_intent(analysis.intent),
        tense: to_language_tense(analysis.tense),
        negated: analysis.negated,
        subject: analysis.subject.clone(),
        verb: analysis.verb.clone(),
        object: analysis.object.clone(),
        time_expr: analysis.time_expression.clone(),
        tokens: content_tokens(tokens)
            .iter()
            .map(|t| t.text.clone())
            .collect(),
        protected: protected_strings(tokens),
        trailing_punct: extract_trailing_punct(original),
        leading_expressive: extract_leading_expressive(original),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nlp_engine_ind_languages::Language;

    #[test]
    fn converts_call_later() {
        let pipeline = Pipeline::new();
        let req = ConversionRequest::new("I'll call you later", Language::Kannada);
        let result = pipeline.convert(req).unwrap();
        assert!(!result.output.is_empty());
        assert!(result.confidence > 0.0);
    }

    #[test]
    fn converts_work_sentence_kannada() {
        let pipeline = Pipeline::new();
        let req = ConversionRequest::new(
            "I'll finish this work and call you once I'm free",
            Language::Kannada,
        );
        let result = pipeline.convert(req).unwrap();
        assert!(result.output.to_ascii_lowercase().contains("work"));
        assert!(result.output.to_ascii_lowercase().contains("call"));
    }
}
