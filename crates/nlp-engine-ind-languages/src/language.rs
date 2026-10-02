use std::fmt;

/// Target Indian language for conversational generation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Language {
    Kannada,
    Hindi,
    Telugu,
    Tamil,
    Malayalam,
}

impl Language {
    pub fn all_enabled() -> Vec<Self> {
        [
            cfg!(feature = "kannada").then_some(Self::Kannada),
            cfg!(feature = "hindi").then_some(Self::Hindi),
            cfg!(feature = "telugu").then_some(Self::Telugu),
            cfg!(feature = "tamil").then_some(Self::Tamil),
            cfg!(feature = "malayalam").then_some(Self::Malayalam),
        ]
        .into_iter()
        .flatten()
        .collect()
    }

    pub fn code(self) -> &'static str {
        match self {
            Self::Kannada => "kn",
            Self::Hindi => "hi",
            Self::Telugu => "te",
            Self::Tamil => "ta",
            Self::Malayalam => "ml",
        }
    }

    pub fn display_name(self) -> &'static str {
        match self {
            Self::Kannada => "Kannada",
            Self::Hindi => "Hindi",
            Self::Telugu => "Telugu",
            Self::Tamil => "Tamil",
            Self::Malayalam => "Malayalam",
        }
    }

    pub fn from_code(code: &str) -> Option<Self> {
        match code.to_ascii_lowercase().as_str() {
            "kn" | "kannada" => Some(Self::Kannada),
            "hi" | "hindi" => Some(Self::Hindi),
            "te" | "telugu" => Some(Self::Telugu),
            "ta" | "tamil" => Some(Self::Tamil),
            "ml" | "malayalam" => Some(Self::Malayalam),
            _ => None,
        }
    }

    pub fn is_enabled(self) -> bool {
        Self::all_enabled().contains(&self)
    }
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.display_name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_codes() {
        assert_eq!(Language::from_code("kn"), Some(Language::Kannada));
        assert_eq!(Language::from_code("hindi"), Some(Language::Hindi));
    }
}
