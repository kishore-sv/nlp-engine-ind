mod gen_common;

pub mod generator;
pub mod language;
pub mod lexicon;

#[cfg(feature = "hindi")]
pub mod hindi;
#[cfg(feature = "kannada")]
pub mod kannada;
#[cfg(feature = "malayalam")]
pub mod malayalam;
#[cfg(feature = "tamil")]
pub mod tamil;
#[cfg(feature = "telugu")]
pub mod telugu;

pub use generator::{GeneratedOutput, LanguageGenerator};
pub use language::Language;
pub use lexicon::Lexicon;

use std::sync::OnceLock;

static GLOBAL_CODE_MIX: OnceLock<Vec<String>> = OnceLock::new();

/// Shared English words commonly preserved across Indian chat languages.
pub fn global_code_mix_words() -> &'static [String] {
    GLOBAL_CODE_MIX
        .get_or_init(|| {
            let raw = include_str!("../../../data/common/code_mix.json");
            let parsed: serde_json::Value = serde_json::from_str(raw).expect("valid code_mix.json");
            parsed["preserve_global"]
                .as_array()
                .expect("preserve_global array")
                .iter()
                .filter_map(|v| v.as_str().map(str::to_ascii_lowercase))
                .collect()
        })
        .as_slice()
}

/// Returns the generator for a language enabled in this build.
pub fn generator_for(language: Language) -> Option<&'static dyn LanguageGenerator> {
    match language {
        #[cfg(feature = "kannada")]
        Language::Kannada => Some(kannada::generator()),
        #[cfg(feature = "hindi")]
        Language::Hindi => Some(hindi::generator()),
        #[cfg(feature = "telugu")]
        Language::Telugu => Some(telugu::generator()),
        #[cfg(feature = "tamil")]
        Language::Tamil => Some(tamil::generator()),
        #[cfg(feature = "malayalam")]
        Language::Malayalam => Some(malayalam::generator()),
        #[allow(unreachable_patterns)]
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loads_global_code_mix() {
        assert!(global_code_mix_words().contains(&"call".to_string()));
    }

    #[test]
    fn generator_available_for_kannada() {
        let gen = generator_for(Language::Kannada);
        assert!(gen.is_some());
    }
}
