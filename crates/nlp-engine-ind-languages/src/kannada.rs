use crate::gen_common::language_module;
use crate::Language;

language_module!(
    generator,
    Language::Kannada,
    "../../../data/kannada/lexicon.json"
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generator::{GenerationContext, IntentKind, LanguageGenerator, TenseKind};

    #[test]
    fn call_later_kannada() {
        let ctx = GenerationContext {
            normalized: "i will call you later".into(),
            intent: IntentKind::Statement,
            tense: TenseKind::Future,
            tokens: vec!["i'll".into(), "call".into(), "you".into(), "later".into()],
            ..Default::default()
        };
        let out = generator().generate(&ctx);
        assert!(out.text.to_ascii_lowercase().contains("call"));
        assert!(out.template_matched);
    }
}
