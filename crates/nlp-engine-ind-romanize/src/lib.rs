use nlp_engine_ind_languages::Language;

/// Casual chat Roman spelling normalization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RomanizationStyle {
    #[default]
    CasualChat,
    Standard,
    Academic,
}

/// Apply language-aware casual Roman normalization to generated output.
pub fn romanize(text: &str, _language: Language, style: RomanizationStyle) -> String {
    match style {
        RomanizationStyle::CasualChat => normalize_casual_chat(text),
        RomanizationStyle::Standard | RomanizationStyle::Academic => text.to_string(),
    }
}

fn normalize_casual_chat(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut prev_space = false;
    for ch in text.chars() {
        if ch.is_whitespace() {
            if !prev_space && !out.is_empty() {
                out.push(' ');
            }
            prev_space = true;
            continue;
        }
        prev_space = false;
        out.push(ch);
    }
    trim_punctuation_spacing(&out)
}

fn trim_punctuation_spacing(s: &str) -> String {
    let mut result = s.to_string();
    for (pat, rep) in [(" !", "!"), (" ?", "?"), (" .", "."), (" ,", ",")] {
        result = result.replace(pat, rep);
    }
    result.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collapses_spaces() {
        let out = romanize(
            "hello   world",
            Language::Kannada,
            RomanizationStyle::CasualChat,
        );
        assert_eq!(out, "hello world");
    }
}
