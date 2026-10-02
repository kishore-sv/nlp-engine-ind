use crate::tokenizer::content_tokens;
use crate::types::Token;

static BRAND_NAMES: &[&str] = &[
    "github",
    "youtube",
    "iphone",
    "android",
    "whatsapp",
    "instagram",
    "telegram",
    "discord",
    "google",
    "gmail",
    "netflix",
    "spotify",
    "amazon",
    "flipkart",
    "swiggy",
    "zomato",
    "uber",
    "ola",
    "zoom",
    "meet",
];

/// Mark tokens that must be preserved in output.
pub fn mark_protected(tokens: &mut [Token]) {
    for token in tokens.iter_mut() {
        if token.protected {
            continue;
        }
        if is_protected_word(&token.normalized) {
            token.protected = true;
        }
    }
}

fn is_protected_word(word: &str) -> bool {
    BRAND_NAMES.contains(&word)
        || word.starts_with('@')
        || word.starts_with('#')
        || word.contains('@')
        || word.starts_with("http")
        || word.starts_with("www.")
}

pub fn protected_strings(tokens: &[Token]) -> Vec<String> {
    content_tokens(tokens)
        .into_iter()
        .filter(|t| t.protected)
        .map(|t| t.text.clone())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokenizer::tokenize;

    #[test]
    fn marks_brands() {
        let mut tokens = tokenize("send GitHub link");
        mark_protected(&mut tokens);
        let gh = tokens.iter().find(|t| t.normalized == "github").unwrap();
        assert!(gh.protected);
    }
}
