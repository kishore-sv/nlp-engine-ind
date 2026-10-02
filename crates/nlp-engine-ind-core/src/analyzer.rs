use crate::grammar::extract_grammar;
use crate::intent::{detect_intent, Intent};
use crate::tense::{detect_tense, is_negated};
use crate::tokenizer::{content_tokens, tokenize};
use crate::types::{Analysis, Token};

/// Trait for analyzers — enables future ML/hybrid implementations.
pub trait Analyzer: Send + Sync {
    fn analyze(&self, normalized: &str, tokens: &[Token]) -> Analysis;
}

/// Deterministic rule-based analyzer for conversational English.
#[derive(Debug, Default, Clone, Copy)]
pub struct RuleBasedAnalyzer;

impl Analyzer for RuleBasedAnalyzer {
    fn analyze(&self, normalized: &str, tokens: &[Token]) -> Analysis {
        let mut analysis = extract_grammar(normalized, tokens);
        analysis.intent = detect_intent(normalized);
        analysis.tense = detect_tense(normalized);
        analysis.negated = is_negated(normalized);
        analysis.is_question = matches!(analysis.intent, Intent::Question | Intent::Request)
            || normalized.contains('?')
            || normalized.starts_with("what ")
            || normalized.starts_with("are you");
        analysis.is_imperative = matches!(analysis.intent, Intent::Command);
        analysis.tokens = tokens.to_vec();
        analysis
    }
}

/// Convenience function using the default rule-based analyzer.
pub fn analyze(normalized: &str) -> Analysis {
    let tokens = tokenize(normalized);
    RuleBasedAnalyzer.analyze(normalized, &tokens)
}

/// Convert core intent to language crate intent.
pub fn to_language_intent(intent: Intent) -> nlp_engine_ind_languages::generator::IntentKind {
    use nlp_engine_ind_languages::generator::IntentKind;
    match intent {
        Intent::Greeting => IntentKind::Greeting,
        Intent::Farewell => IntentKind::Farewell,
        Intent::Question => IntentKind::Question,
        Intent::Request => IntentKind::Request,
        Intent::Command => IntentKind::Command,
        Intent::Suggestion => IntentKind::Suggestion,
        Intent::Invitation => IntentKind::Invitation,
        Intent::Confirmation => IntentKind::Confirmation,
        Intent::Apology => IntentKind::Apology,
        Intent::Thanks => IntentKind::Thanks,
        Intent::Agreement => IntentKind::Agreement,
        Intent::Disagreement => IntentKind::Disagreement,
        Intent::Negation | Intent::Statement => IntentKind::Statement,
    }
}

pub fn to_language_tense(
    tense: crate::tense::Tense,
) -> nlp_engine_ind_languages::generator::TenseKind {
    use nlp_engine_ind_languages::generator::TenseKind;
    match tense {
        crate::tense::Tense::Past => TenseKind::Past,
        crate::tense::Tense::Future => TenseKind::Future,
        crate::tense::Tense::PresentContinuous => TenseKind::PresentContinuous,
        crate::tense::Tense::PastContinuous => TenseKind::PastContinuous,
        crate::tense::Tense::PresentPerfect => TenseKind::PresentPerfect,
        crate::tense::Tense::PastPerfect => TenseKind::PastPerfect,
        crate::tense::Tense::Intentional => TenseKind::Intentional,
        crate::tense::Tense::Present => TenseKind::Present,
    }
}

pub fn content_words(tokens: &[Token]) -> Vec<String> {
    content_tokens(tokens)
        .iter()
        .filter(|t| !t.protected)
        .map(|t| t.text.clone())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn analyzes_future_statement() {
        let a = analyze("i will call you later");
        assert_eq!(a.tense, crate::tense::Tense::Future);
    }

    #[test]
    fn analyzes_request() {
        let a = analyze("can you send me the file");
        assert_eq!(a.intent, Intent::Request);
    }
}
