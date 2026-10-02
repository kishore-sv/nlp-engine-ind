use std::collections::HashMap;
use std::sync::OnceLock;

static CONTRACTIONS: OnceLock<HashMap<&'static str, &'static str>> = OnceLock::new();

fn contractions() -> &'static HashMap<&'static str, &'static str> {
    CONTRACTIONS.get_or_init(|| {
        HashMap::from([
            ("i'm", "i am"),
            ("i'll", "i will"),
            ("i've", "i have"),
            ("i'd", "i would"),
            ("don't", "do not"),
            ("can't", "can not"),
            ("won't", "will not"),
            ("isn't", "is not"),
            ("aren't", "are not"),
            ("wasn't", "was not"),
            ("weren't", "were not"),
            ("didn't", "did not"),
            ("doesn't", "does not"),
            ("haven't", "have not"),
            ("hasn't", "has not"),
            ("couldn't", "could not"),
            ("shouldn't", "should not"),
            ("wouldn't", "would not"),
            ("mustn't", "must not"),
            ("let's", "let us"),
            ("that's", "that is"),
            ("what's", "what is"),
            ("who's", "who is"),
            ("it's", "it is"),
            ("you're", "you are"),
            ("you'll", "you will"),
            ("you've", "you have"),
            ("you'd", "you would"),
            ("we're", "we are"),
            ("we'll", "we will"),
            ("we've", "we have"),
            ("they're", "they are"),
            ("they'll", "they will"),
            ("they've", "they have"),
            ("he's", "he is"),
            ("she's", "she is"),
            ("there's", "there is"),
            ("here's", "here is"),
        ])
    })
}

/// Normalize conversational English input while preserving chat expressiveness.
pub fn normalize(input: &str) -> String {
    let mut s = input.to_string();
    s = normalize_unicode(&s);
    s = expand_contractions(&s);
    s = collapse_whitespace(&s);
    s.trim().to_string()
}

fn normalize_unicode(s: &str) -> String {
    s.replace(['\u{2019}', '\u{2018}'], "'")
        .replace(['\u{201C}', '\u{201D}'], "\"")
        .replace(['\u{2013}', '\u{2014}'], "-")
}

fn expand_contractions(s: &str) -> String {
    let lower = s.to_ascii_lowercase();
    let mut result = lower;
    for (from, to) in contractions() {
        result = result.replace(from, to);
    }
    result
}

fn collapse_whitespace(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut prev_ws = false;
    for ch in s.chars() {
        if ch == '\n' || ch == '\r' {
            if !prev_ws && !out.is_empty() {
                out.push(' ');
            }
            prev_ws = true;
            continue;
        }
        if ch.is_whitespace() {
            if !prev_ws && !out.is_empty() {
                out.push(' ');
            }
            prev_ws = true;
        } else {
            out.push(ch);
            prev_ws = false;
        }
    }
    out
}

/// Extract leading expressive prefix (e.g. "Brooo 😂😂") from original input.
pub fn extract_leading_expressive(original: &str) -> String {
    let mut parts = Vec::new();
    for word in original.split_whitespace() {
        if is_expressive_bro(word) || is_emoji_word(word) {
            parts.push(word.to_string());
        } else {
            break;
        }
    }
    parts.join(" ")
}

fn is_expressive_bro(word: &str) -> bool {
    let lower = word.to_ascii_lowercase();
    lower.starts_with("bro") && lower.len() > 4 && lower.chars().skip(3).all(|c| c == 'o')
}

fn is_emoji_word(word: &str) -> bool {
    word.chars()
        .all(|c| c as u32 >= 0x1F300 || matches!(c, '😂' | '😭' | '❤' | '🔥' | '👍' | '🙏'))
}

/// Extract trailing punctuation cluster from original (e.g. "!!!", "???").
pub fn extract_trailing_punct(original: &str) -> String {
    let trimmed = original.trim_end();
    let mut punct = String::new();
    for ch in trimmed.chars().rev() {
        if matches!(ch, '!' | '?' | '.' | ',' | '…') {
            punct.insert(0, ch);
        } else {
            break;
        }
    }
    punct
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expands_im() {
        assert_eq!(normalize("I'm busy"), "i am busy");
    }

    #[test]
    fn expands_dont() {
        assert_eq!(normalize("don't worry"), "do not worry");
    }

    #[test]
    fn handles_smart_quotes() {
        assert_eq!(normalize("I\u{2019}ll go"), "i will go");
    }

    #[test]
    fn collapses_whitespace() {
        assert_eq!(normalize("hello   world"), "hello world");
    }
}
