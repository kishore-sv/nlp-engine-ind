use nlp_engine_ind_core::{ConversionRequest, Converter, Language};
use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
struct GoldenCase {
    input: String,
    language: String,
    expected_output: String,
    acceptable_alternatives: Vec<String>,
    #[allow(dead_code)]
    intent: String,
}

#[test]
fn golden_conversations() {
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../tests/fixtures/golden.json"
    );
    let raw = fs::read_to_string(path).expect("golden fixtures");
    let cases: Vec<GoldenCase> = serde_json::from_str(&raw).expect("valid golden json");
    let converter = Converter::new();

    for case in cases {
        let language = Language::from_code(&case.language).expect("language code");
        let result = converter
            .convert(ConversionRequest::new(&case.input, language))
            .unwrap_or_else(|e| panic!("failed for '{}': {e}", case.input));

        let actual = result.output.to_ascii_lowercase();
        let expected = case.expected_output.to_ascii_lowercase();
        let alternatives: Vec<String> = case
            .acceptable_alternatives
            .iter()
            .map(|s| s.to_ascii_lowercase())
            .collect();

        let ok = actual == expected || alternatives.contains(&actual);
        assert!(
            ok,
            "input: {}\nexpected: {}\nalternatives: {:?}\nactual: {}",
            case.input, case.expected_output, case.acceptable_alternatives, result.output
        );
    }
}

#[test]
fn all_languages_produce_output() {
    let converter = Converter::new();
    let msg = "I'll call you later";
    for lang in Language::all_enabled() {
        let result = converter
            .convert(ConversionRequest::new(msg, lang))
            .unwrap();
        assert!(!result.output.is_empty(), "empty for {lang}");
    }
}
