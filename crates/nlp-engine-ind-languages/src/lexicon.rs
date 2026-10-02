use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize)]
pub struct TemplateRule {
    pub contains: Vec<String>,
    pub output: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Lexicon {
    pub pronouns: HashMap<String, String>,
    pub verbs: HashMap<String, String>,
    pub auxiliaries: HashMap<String, String>,
    pub particles: HashMap<String, String>,
    pub phrases: HashMap<String, String>,
    pub verb_forms: HashMap<String, String>,
    #[serde(default)]
    pub code_mix_extra: Vec<String>,
    #[serde(default)]
    pub templates: Vec<TemplateRule>,
}

impl Lexicon {
    pub fn from_embedded(json: &str) -> Self {
        serde_json::from_str(json).expect("valid lexicon json")
    }

    pub fn lookup_phrase(&self, phrase: &str) -> Option<&str> {
        self.phrases
            .get(&phrase.to_ascii_lowercase())
            .map(String::as_str)
    }

    pub fn lookup_word(&self, word: &str) -> Option<&str> {
        let w = word.to_ascii_lowercase();
        self.pronouns
            .get(&w)
            .or_else(|| self.verbs.get(&w))
            .or_else(|| self.particles.get(&w))
            .or_else(|| self.auxiliaries.get(&w))
            .map(String::as_str)
    }

    pub fn verb_form(&self, key: &str) -> Option<&str> {
        self.verb_forms.get(key).map(String::as_str)
    }

    pub fn code_mix_words(&self) -> Vec<String> {
        let mut words = crate::global_code_mix_words().to_vec();
        words.extend(self.code_mix_extra.iter().cloned());
        words.sort();
        words.dedup();
        words
    }

    pub fn match_template(&self, normalized: &str) -> Option<&str> {
        let norm = normalized.to_ascii_lowercase();
        for rule in &self.templates {
            if rule
                .contains
                .iter()
                .all(|needle| norm.contains(&needle.to_ascii_lowercase()))
            {
                return Some(rule.output.as_str());
            }
        }
        None
    }
}
