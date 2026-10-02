use crate::Language;

/// Output from a language-specific generator.
#[derive(Debug, Clone, PartialEq)]
pub struct GeneratedOutput {
    pub text: String,
    /// Fraction of content tokens converted (0.0–1.0).
    pub conversion_ratio: f32,
    /// Whether a high-confidence template matched.
    pub template_matched: bool,
}

/// Language-specific conversational generator.
pub trait LanguageGenerator: Send + Sync {
    fn language(&self) -> Language;
    fn lexicon(&self) -> &crate::Lexicon;

    /// Generate conversational Roman-script output from analyzed slots.
    fn generate(&self, ctx: &GenerationContext) -> GeneratedOutput;

    fn should_preserve_english(&self, word: &str, ctx: &GenerationContext) -> bool {
        let lower = word.to_ascii_lowercase();
        if ctx.protected.contains(&lower) {
            return true;
        }
        self.lexicon().code_mix_words().iter().any(|w| w == &lower)
    }

    fn translate_word(&self, word: &str, ctx: &GenerationContext) -> String {
        if self.should_preserve_english(word, ctx) {
            return word.to_string();
        }
        self.lexicon().lookup_word(word).unwrap_or(word).to_string()
    }
}

/// Context passed from the core analyzer to language generators.
#[derive(Debug, Clone, Default)]
pub struct GenerationContext {
    pub normalized: String,
    pub intent: IntentKind,
    pub tense: TenseKind,
    pub negated: bool,
    pub subject: Option<String>,
    pub verb: Option<String>,
    pub object: Option<String>,
    pub time_expr: Option<String>,
    pub tokens: Vec<String>,
    pub protected: Vec<String>,
    pub trailing_punct: String,
    pub leading_expressive: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum IntentKind {
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
    Apology,
    Thanks,
    Agreement,
    Disagreement,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TenseKind {
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
