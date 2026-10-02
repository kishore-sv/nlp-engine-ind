use crate::tokenizer::content_tokens;
use crate::types::{Analysis, Token};

/// Extract grammatical slots from tokens and normalized text.
pub fn extract_grammar(normalized: &str, tokens: &[Token]) -> Analysis {
    let content = content_tokens(tokens);
    let words: Vec<&str> = content.iter().map(|t| t.normalized.as_str()).collect();

    let subject = extract_subject(&words);
    let verb = extract_verb(normalized, &words);
    let object = extract_object(&words);
    let time_expression = extract_time(normalized);

    Analysis {
        subject,
        verb,
        object,
        time_expression,
        ..Default::default()
    }
}

fn extract_subject(words: &[&str]) -> Option<String> {
    words
        .first()
        .filter(|w| matches!(**w, "i" | "you" | "we" | "they" | "he" | "she"))
        .map(|s| (*s).to_string())
}

fn extract_verb(normalized: &str, words: &[&str]) -> Option<String> {
    const VERBS: &[&str] = &[
        "call", "send", "go", "come", "do", "make", "finish", "handle", "work", "meet", "check",
        "confirm", "wait", "help", "try", "find", "leave", "reach", "sleep", "eat", "drink",
    ];
    for v in VERBS {
        if normalized.contains(v) {
            return Some((*v).to_string());
        }
    }
    words
        .iter()
        .find(|w| VERBS.contains(w))
        .map(|s| (*s).to_string())
}

fn extract_object(words: &[&str]) -> Option<String> {
    const OBJECTS: &[&str] = &[
        "file",
        "project",
        "link",
        "message",
        "work",
        "assignment",
        "update",
        "plan",
    ];
    words
        .iter()
        .find(|w| OBJECTS.contains(w))
        .map(|s| (*s).to_string())
}

fn extract_time(normalized: &str) -> Option<String> {
    const TIMES: &[&str] = &[
        "later",
        "now",
        "today",
        "tomorrow",
        "yesterday",
        "right now",
        "once i am free",
        "once im free",
    ];
    for t in TIMES {
        if normalized.contains(t) {
            return Some((*t).to_string());
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokenizer::tokenize;

    #[test]
    fn extracts_verb_call() {
        let tokens = tokenize("i will call you later");
        let g = extract_grammar("i will call you later", &tokens);
        assert_eq!(g.verb.as_deref(), Some("call"));
    }
}
