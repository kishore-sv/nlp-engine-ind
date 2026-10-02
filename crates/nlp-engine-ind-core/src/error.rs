use thiserror::Error;

pub type Result<T> = std::result::Result<T, EngineError>;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum EngineError {
    #[error("unsupported language: {language}")]
    UnsupportedLanguage { language: String },

    #[error("invalid input: {reason}")]
    InvalidInput { reason: String },

    #[error("empty input")]
    EmptyInput,

    #[error("generation failed: {reason}")]
    GenerationFailure { reason: String },

    #[error("invalid configuration: {reason}")]
    InvalidConfiguration { reason: String },

    #[error("missing offline resource: {resource}")]
    ResourceMissing { resource: String },
}
