use crate::types::{Token, TokenKind};

/// Tokenize normalized conversational English text.
pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut i = 0;
    let chars: Vec<char> = input.chars().collect();
    let len = chars.len();

    while i < len {
        let start = i;
        if chars[i].is_whitespace() {
            let text = chars[i].to_string();
            tokens.push(Token {
                kind: TokenKind::Whitespace,
                text: text.clone(),
                normalized: text,
                protected: false,
                start,
                end: i + 1,
            });
            i += 1;
            continue;
        }

        if let Some((tok, consumed)) = try_url(&chars[i..]) {
            let end = i + consumed;
            tokens.push(Token {
                kind: TokenKind::Url,
                text: tok.clone(),
                normalized: tok.to_ascii_lowercase(),
                protected: true,
                start,
                end,
            });
            i = end;
            continue;
        }

        if let Some((tok, consumed)) = try_email(&chars[i..]) {
            let end = i + consumed;
            tokens.push(Token {
                kind: TokenKind::Email,
                text: tok.clone(),
                normalized: tok.to_ascii_lowercase(),
                protected: true,
                start,
                end,
            });
            i = end;
            continue;
        }

        if chars[i] == '@' {
            let (tok, consumed) = read_alnum_underscore(&chars[i..]);
            let end = i + consumed;
            tokens.push(Token {
                kind: TokenKind::Mention,
                text: tok.clone(),
                normalized: tok.to_ascii_lowercase(),
                protected: true,
                start,
                end,
            });
            i = end;
            continue;
        }

        if chars[i] == '#' {
            let (tok, consumed) = read_alnum_underscore(&chars[i..]);
            let end = i + consumed;
            tokens.push(Token {
                kind: TokenKind::Hashtag,
                text: tok.clone(),
                normalized: tok.to_ascii_lowercase(),
                protected: true,
                start,
                end,
            });
            i = end;
            continue;
        }

        if is_emoji(chars[i]) {
            let (tok, consumed) = read_emoji_run(&chars[i..]);
            let end = i + consumed;
            tokens.push(Token {
                kind: TokenKind::Emoji,
                text: tok.clone(),
                normalized: tok.clone(),
                protected: true,
                start,
                end,
            });
            i = end;
            continue;
        }

        if chars[i].is_ascii_digit() {
            let (tok, consumed) = read_number(&chars[i..]);
            let end = i + consumed;
            tokens.push(Token {
                kind: TokenKind::Number,
                text: tok.clone(),
                normalized: tok.clone(),
                protected: true,
                start,
                end,
            });
            i = end;
            continue;
        }

        if is_punctuation(chars[i]) {
            let (tok, consumed) = read_punct_run(&chars[i..]);
            let end = i + consumed;
            tokens.push(Token {
                kind: TokenKind::Punctuation,
                text: tok.clone(),
                normalized: tok.clone(),
                protected: false,
                start,
                end,
            });
            i = end;
            continue;
        }

        let (tok, consumed) = read_word(&chars[i..]);
        let end = i + consumed;
        let lower = tok.to_ascii_lowercase();
        let kind = classify_word(&tok, &lower);
        let protected = matches!(
            kind,
            TokenKind::Url | TokenKind::Email | TokenKind::Mention | TokenKind::Hashtag
        ) || is_brand(&lower);
        tokens.push(Token {
            kind,
            text: tok.clone(),
            normalized: lower,
            protected,
            start,
            end,
        });
        i = end;
    }

    tokens
}

fn try_url(chars: &[char]) -> Option<(String, usize)> {
    let s: String = chars.iter().collect();
    for prefix in ["http://", "https://", "www."] {
        if s.starts_with(prefix) {
            let consumed = read_until_space(chars);
            return Some((chars[..consumed].iter().collect(), consumed));
        }
    }
    None
}

fn try_email(chars: &[char]) -> Option<(String, usize)> {
    let s: String = chars.iter().collect();
    if s.contains('@') {
        let consumed = read_until_space(chars);
        let token: String = chars[..consumed].iter().collect();
        if token.contains('@') && token.contains('.') {
            return Some((token, consumed));
        }
    }
    None
}

fn read_until_space(chars: &[char]) -> usize {
    let mut i = 0;
    while i < chars.len() && !chars[i].is_whitespace() {
        i += 1;
    }
    i
}

fn read_alnum_underscore(chars: &[char]) -> (String, usize) {
    let mut i = 0;
    while i < chars.len()
        && (chars[i].is_alphanumeric() || chars[i] == '_' || chars[i] == '@' || chars[i] == '#')
    {
        i += 1;
    }
    (chars[..i].iter().collect(), i)
}

fn read_word(chars: &[char]) -> (String, usize) {
    let mut i = 0;
    while i < chars.len()
        && !chars[i].is_whitespace()
        && !is_punctuation(chars[i])
        && chars[i] != '@'
        && chars[i] != '#'
        && !is_emoji(chars[i])
    {
        i += 1;
    }
    if i == 0 {
        i = 1;
    }
    (chars[..i].iter().collect(), i)
}

fn read_number(chars: &[char]) -> (String, usize) {
    let mut i = 0;
    while i < chars.len()
        && (chars[i].is_ascii_digit() || chars[i] == '.' || chars[i] == ',' || chars[i] == '-')
    {
        i += 1;
    }
    (chars[..i].iter().collect(), i)
}

fn read_punct_run(chars: &[char]) -> (String, usize) {
    let mut i = 0;
    while i < chars.len() && is_punctuation(chars[i]) {
        i += 1;
    }
    (chars[..i].iter().collect(), i)
}

fn read_emoji_run(chars: &[char]) -> (String, usize) {
    let mut i = 0;
    while i < chars.len() && is_emoji(chars[i]) {
        i += 1;
    }
    (chars[..i].iter().collect(), i)
}

fn is_punctuation(c: char) -> bool {
    matches!(
        c,
        '.' | ',' | '!' | '?' | ';' | ':' | '\'' | '"' | '(' | ')' | '[' | ']' | '{' | '}' | '-'
    )
}

fn is_emoji(c: char) -> bool {
    let cp = c as u32;
    (0x1F300..=0x1FAFF).contains(&cp)
        || (0x2600..=0x27BF).contains(&cp)
        || matches!(c, '😂' | '😭' | '❤' | '🔥' | '👍' | '🙏')
}

fn is_brand(lower: &str) -> bool {
    matches!(
        lower,
        "github"
            | "youtube"
            | "iphone"
            | "android"
            | "whatsapp"
            | "instagram"
            | "telegram"
            | "discord"
            | "google"
            | "gmail"
    )
}

fn classify_word(raw: &str, lower: &str) -> TokenKind {
    if is_brand(lower) {
        return TokenKind::Word;
    }
    if raw.chars().any(|c| c.is_ascii_digit()) && raw.chars().any(|c| c.is_ascii_alphabetic()) {
        return TokenKind::Mixed;
    }
    if lower.contains('\'') {
        return TokenKind::Contraction;
    }
    TokenKind::Word
}

/// Content tokens excluding whitespace and pure punctuation.
pub fn content_tokens(tokens: &[Token]) -> Vec<&Token> {
    tokens
        .iter()
        .filter(|t| !matches!(t.kind, TokenKind::Whitespace | TokenKind::Punctuation))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenizes_url() {
        let tokens = tokenize("check https://example.com now");
        assert!(tokens.iter().any(|t| t.kind == TokenKind::Url));
    }

    #[test]
    fn tokenizes_mention() {
        let tokens = tokenize("hi @kishore");
        assert!(tokens.iter().any(|t| t.kind == TokenKind::Mention));
    }

    #[test]
    fn tokenizes_hashtag() {
        let tokens = tokenize("#project update");
        assert!(tokens.iter().any(|t| t.kind == TokenKind::Hashtag));
    }

    #[test]
    fn protects_github() {
        let tokens = tokenize("open GitHub");
        let gh = tokens.iter().find(|t| t.normalized == "github").unwrap();
        assert!(gh.protected);
    }
}
