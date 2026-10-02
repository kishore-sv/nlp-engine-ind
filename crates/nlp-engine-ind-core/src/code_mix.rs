use crate::config::CodeMixMode;
use crate::tokenizer::content_tokens;
use crate::types::Token;
use nlp_engine_ind_languages::global_code_mix_words;

/// Decide whether an English word should remain in output.
pub fn should_preserve_english(
    word: &str,
    token: &Token,
    mode: CodeMixMode,
    language_extra: &[String],
) -> bool {
    if token.protected {
        return true;
    }

    let lower = word.to_ascii_lowercase();

    match mode {
        CodeMixMode::Minimal => return token.protected,
        CodeMixMode::Aggressive => {
            if token.protected {
                return true;
            }
            return !language_extra.contains(&lower);
        }
        CodeMixMode::Natural => {}
    }

    if global_code_mix_words().iter().any(|w| w == &lower) {
        return true;
    }
    if language_extra.iter().any(|w| w == &lower) {
        return true;
    }
    false
}

pub fn code_mix_candidates(tokens: &[Token]) -> Vec<String> {
    content_tokens(tokens)
        .iter()
        .filter(|t| {
            matches!(
                t.kind,
                crate::types::TokenKind::Word | crate::types::TokenKind::Mixed
            )
        })
        .map(|t| t.normalized.clone())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokenizer::tokenize;
    use crate::types::TokenKind;

    #[test]
    fn preserves_call_in_natural_mode() {
        let tokens = tokenize("call me");
        let word = tokens.iter().find(|t| t.normalized == "call").unwrap();
        assert!(should_preserve_english(
            "call",
            word,
            CodeMixMode::Natural,
            &[]
        ));
    }

    #[test]
    fn preserves_protected_mention() {
        let tokens = tokenize("@rahul");
        let m = tokens
            .iter()
            .find(|t| t.kind == TokenKind::Mention)
            .unwrap();
        assert!(should_preserve_english(
            "@rahul",
            m,
            CodeMixMode::Natural,
            &[]
        ));
    }
}
