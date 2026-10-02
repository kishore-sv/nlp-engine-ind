/// Conversational tense and aspect.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Tense {
    #[default]
    Present,
    Past,
    Future,
    PresentContinuous,
    PastContinuous,
    PresentPerfect,
    PastPerfect,
    Intentional,
}

pub fn detect_tense(normalized: &str) -> Tense {
    let s = normalized;
    if s.contains("i will") || s.contains("i'll") || s.contains("ill ") {
        return Tense::Future;
    }
    if s.contains("going to") || s.contains("gonna") {
        return Tense::Intentional;
    }
    if s.contains("i am") && (s.contains("ing") || s.ends_with("ing")) {
        return Tense::PresentContinuous;
    }
    if s.contains("i was") && s.contains("ing") {
        return Tense::PastContinuous;
    }
    if s.contains("i have") || s.contains("i've") {
        return Tense::PresentPerfect;
    }
    if s.contains("i had") {
        return Tense::PastPerfect;
    }
    if s.contains("i went") || s.contains("i did") || s.contains("i was") {
        return Tense::Past;
    }
    Tense::Present
}

pub fn is_negated(normalized: &str) -> bool {
    normalized.contains("do not")
        || normalized.contains("not ")
        || normalized.contains("never")
        || normalized.contains("no ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn future_tense() {
        assert_eq!(detect_tense("i will call you"), Tense::Future);
    }

    #[test]
    fn present_continuous() {
        assert_eq!(detect_tense("i am going home"), Tense::PresentContinuous);
    }
}
