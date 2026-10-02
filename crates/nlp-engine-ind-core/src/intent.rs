/// Conversational intent categories.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Intent {
    #[default]
    Statement,
    Greeting,
    Farewell,
    Question,
    Request,
    Command,
    Suggestion,
    Invitation,
    Confirmation,
    Negation,
    Apology,
    Thanks,
    Agreement,
    Disagreement,
}

pub fn detect_intent(normalized: &str) -> Intent {
    let s = normalized.trim();
    if s.is_empty() {
        return Intent::Statement;
    }

    if is_greeting(s) {
        return Intent::Greeting;
    }
    if is_farewell(s) {
        return Intent::Farewell;
    }
    if is_thanks(s) {
        return Intent::Thanks;
    }
    if is_apology(s) {
        return Intent::Apology;
    }
    if is_agreement(s) {
        return Intent::Agreement;
    }
    if is_disagreement(s) {
        return Intent::Disagreement;
    }
    if is_negation(s) {
        return Intent::Negation;
    }
    if is_invitation(s) {
        return Intent::Invitation;
    }
    if is_suggestion(s) {
        return Intent::Suggestion;
    }
    if is_request(s) {
        return Intent::Request;
    }
    if is_command(s) {
        return Intent::Command;
    }
    if is_question(s) {
        return Intent::Question;
    }
    Intent::Statement
}

fn is_greeting(s: &str) -> bool {
    s.starts_with("hi")
        || s.starts_with("hello")
        || s.starts_with("hey")
        || s.contains("good morning")
        || s.contains("good evening")
}

fn is_farewell(s: &str) -> bool {
    s.contains("bye") || s.contains("see you") || s.contains("good night")
}

fn is_thanks(s: &str) -> bool {
    s.contains("thank you") || s.starts_with("thanks") || s.contains("thank u")
}

fn is_apology(s: &str) -> bool {
    s.starts_with("sorry") || s.contains("my bad") || s.contains("apolog")
}

fn is_agreement(s: &str) -> bool {
    matches!(s, "yes" | "yeah" | "yep" | "sure" | "ok" | "okay" | "fine")
        || s.starts_with("yes ")
        || s.starts_with("yeah ")
}

fn is_disagreement(s: &str) -> bool {
    s.starts_with("no ") || s == "no" || s.contains("not really") || s.contains("disagree")
}

fn is_negation(s: &str) -> bool {
    s.contains("do not") || s.contains("don't") || s.contains("dont") || s.contains("never")
}

fn is_invitation(s: &str) -> bool {
    s.contains("let us") || s.contains("let's") || s.contains("lets meet") || s.contains("join")
}

fn is_suggestion(s: &str) -> bool {
    s.contains("how about") || s.contains("why don't") || s.contains("maybe we")
}

fn is_request(s: &str) -> bool {
    s.contains("can you")
        || s.contains("could you")
        || s.contains("would you")
        || s.contains("please send")
        || s.contains("please share")
        || s.starts_with("please ")
}

fn is_command(s: &str) -> bool {
    s.starts_with("send ")
        || s.starts_with("call ")
        || s.starts_with("check ")
        || s.starts_with("confirm ")
}

fn is_question(s: &str) -> bool {
    s.ends_with('?')
        || s.starts_with("what ")
        || s.starts_with("where ")
        || s.starts_with("when ")
        || s.starts_with("why ")
        || s.starts_with("how ")
        || s.starts_with("who ")
        || s.starts_with("which ")
        || s.starts_with("are you")
        || s.starts_with("is it")
        || s.starts_with("can you")
        || s.starts_with("do you")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_request() {
        assert_eq!(detect_intent("can you send me the file"), Intent::Request);
    }

    #[test]
    fn detects_question() {
        assert_eq!(detect_intent("are you coming?"), Intent::Question);
    }
}
