use crate::intent::Intent;
use crate::tense::Tense;
use std::fmt;

/// Kind of token produced by the tokenizer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
    Word,
    Number,
    Punctuation,
    Emoji,
    Url,
    Email,
    Mention,
    Hashtag,
    Contraction,
    Mixed,
    Whitespace,
}

/// A single token from the input text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub text: String,
    pub normalized: String,
    pub protected: bool,
    pub start: usize,
    pub end: usize,
}

/// Result of deterministic linguistic analysis.
#[derive(Debug, Clone, Default)]
pub struct Analysis {
    pub intent: Intent,
    pub tense: Tense,
    pub negated: bool,
    pub subject: Option<String>,
    pub verb: Option<String>,
    pub object: Option<String>,
    pub time_expression: Option<String>,
    pub is_question: bool,
    pub is_imperative: bool,
    pub tokens: Vec<Token>,
}

/// Output of a full conversion.
#[derive(Debug, Clone, PartialEq)]
pub struct ConversionResult {
    pub input: String,
    pub output: String,
    pub language: nlp_engine_ind_languages::Language,
    pub confidence: f32,
    pub intent: Intent,
    pub latency_us: u64,
    pub diagnostics: Diagnostics,
}

/// Optional diagnostic details for debugging and tuning.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Diagnostics {
    pub template_matched: bool,
    pub conversion_ratio: f32,
    pub unknown_token_count: usize,
    pub protected_token_count: usize,
}

impl ConversionResult {
    /// Processing time in microseconds.
    pub fn processing_time_us(&self) -> u64 {
        self.latency_us
    }

    /// Processing time in milliseconds.
    pub fn processing_time_ms(&self) -> f64 {
        self.latency_us as f64 / 1000.0
    }
}

impl fmt::Display for ConversionResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.output)
    }
}
