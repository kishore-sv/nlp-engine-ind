use crate::analyzer::{to_language_intent, to_language_tense, Analyzer, RuleBasedAnalyzer};
use crate::config::EngineConfig;
use crate::error::{EngineError, Result};
use crate::normalizer::{extract_leading_expressive, extract_trailing_punct};
use crate::protected_tokens::mark_protected;
use crate::tokenizer::{content_tokens, tokenize};
use crate::types::Analysis;
use nlp_engine_ind_languages::generator::{GeneratedOutput, LanguageGenerator};

/// Generate conversational output using language-specific rules.
#[derive(Debug, Default, Clone)]
pub struct Generator {
    analyzer: RuleBasedAnalyzer,
}

impl Generator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn analyze(&self, normalized: &str) -> Analysis {
        let mut tokens = tokenize(normalized);
        mark_protected(&mut tokens);
        self.analyzer.analyze(normalized, &tokens)
    }

    pub fn generate(
        &self,
        original: &str,
        normalized: &str,
        analysis: &Analysis,
        _config: &EngineConfig,
        lang_gen: &dyn LanguageGenerator,
    ) -> Result<GeneratedOutput> {
        let ctx = build_context(original, normalized, analysis);
        let output = lang_gen.generate(&ctx);
        if output.text.is_empty() {
            return Err(EngineError::GenerationFailure {
                reason: "empty output from language generator".into(),
            });
        }
        Ok(output)
    }
}

fn build_context(
    original: &str,
    normalized: &str,
    analysis: &Analysis,
) -> nlp_engine_ind_languages::generator::GenerationContext {
    let mut tokens = tokenize(normalized);
    mark_protected(&mut tokens);
    nlp_engine_ind_languages::generator::GenerationContext {
        normalized: normalized.to_string(),
        intent: to_language_intent(analysis.intent),
        tense: to_language_tense(analysis.tense),
        negated: analysis.negated,
        subject: analysis.subject.clone(),
        verb: analysis.verb.clone(),
        object: analysis.object.clone(),
        time_expr: analysis.time_expression.clone(),
        tokens: content_tokens(&tokens)
            .iter()
            .map(|t| t.text.clone())
            .collect(),
        protected: content_tokens(&tokens)
            .iter()
            .filter(|t| t.protected)
            .map(|t| t.text.clone())
            .collect(),
        trailing_punct: extract_trailing_punct(original),
        leading_expressive: extract_leading_expressive(original),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::EngineConfig;
    use nlp_engine_ind_languages::{generator_for, Language};

    #[test]
    fn generates_kannada_output() {
        let gen = Generator::new();
        let config = EngineConfig::default();
        let lang = generator_for(Language::Kannada).unwrap();
        let analysis = gen.analyze("i will call you later");
        let out = gen
            .generate(
                "I'll call you later",
                "i will call you later",
                &analysis,
                &config,
                lang,
            )
            .unwrap();
        assert!(!out.text.is_empty());
    }
}
