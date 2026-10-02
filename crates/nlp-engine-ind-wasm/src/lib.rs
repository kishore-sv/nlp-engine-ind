use nlp_engine_ind_core::{
    CodeMixMode, ConversionRequest, Converter, EngineConfig, Language, RomanizationStyle, Style,
};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn init() {
    console_error_panic_hook::set_once();
}

#[derive(Debug, Deserialize)]
pub struct ConvertOptions {
    pub text: String,
    pub language: String,
    #[serde(default = "default_style")]
    pub style: String,
    #[serde(default = "default_code_mix")]
    pub code_mix: String,
}

fn default_style() -> String {
    "casual".into()
}

fn default_code_mix() -> String {
    "natural".into()
}

#[derive(Debug, Serialize)]
pub struct WasmConversionResult {
    pub input: String,
    pub output: String,
    pub language: String,
    pub confidence: f32,
    pub processing_time_us: u64,
}

fn parse_language(code: &str) -> std::result::Result<Language, String> {
    Language::from_code(code).ok_or_else(|| format!("unsupported language: {code}"))
}

fn parse_style(style: &str) -> Style {
    match style.to_ascii_lowercase().as_str() {
        "neutral" => Style::Neutral,
        "formal" => Style::Formal,
        _ => Style::Casual,
    }
}

fn parse_code_mix(mode: &str) -> CodeMixMode {
    match mode.to_ascii_lowercase().as_str() {
        "minimal" => CodeMixMode::Minimal,
        "aggressive" => CodeMixMode::Aggressive,
        _ => CodeMixMode::Natural,
    }
}

fn to_js_value(result: WasmConversionResult) -> std::result::Result<JsValue, String> {
    serde_wasm_bindgen::to_value(&result).map_err(|e| e.to_string())
}

fn convert_internal(options: ConvertOptions) -> std::result::Result<WasmConversionResult, String> {
    let language = parse_language(&options.language)?;
    let mut config = EngineConfig::for_language(language).map_err(|e| e.to_string())?;
    config.style = parse_style(&options.style);
    config.code_mixing = parse_code_mix(&options.code_mix);
    config.romanization = RomanizationStyle::CasualChat;

    let converter = Converter::new();
    let request = ConversionRequest::new(options.text.clone(), language)
        .with_code_mix(parse_code_mix(&options.code_mix));
    let result = converter.convert(request).map_err(|e| e.to_string())?;
    let processing_time_us = result.processing_time_us();
    Ok(WasmConversionResult {
        input: result.input,
        output: result.output,
        language: result.language.code().to_string(),
        confidence: result.confidence,
        processing_time_us,
    })
}

#[wasm_bindgen]
pub fn convert(text: &str, language: &str) -> std::result::Result<JsValue, JsValue> {
    convert_internal(ConvertOptions {
        text: text.to_string(),
        language: language.to_string(),
        style: default_style(),
        code_mix: default_code_mix(),
    })
    .and_then(to_js_value)
    .map_err(|e| JsValue::from_str(&e))
}

#[wasm_bindgen]
pub fn convert_json(options_json: &str) -> std::result::Result<JsValue, JsValue> {
    let options: ConvertOptions = serde_json::from_str(options_json)
        .map_err(|e| JsValue::from_str(&format!("invalid options json: {e}")))?;
    convert_internal(options)
        .and_then(to_js_value)
        .map_err(|e| JsValue::from_str(&e))
}

#[wasm_bindgen]
pub fn supported_languages() -> JsValue {
    serde_wasm_bindgen::to_value(
        &Language::all_enabled()
            .iter()
            .map(|l| l.code())
            .collect::<Vec<_>>(),
    )
    .unwrap_or(JsValue::NULL)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wasm_internal_convert() {
        let result = convert_internal(ConvertOptions {
            text: "I'll call you later".into(),
            language: "kn".into(),
            style: "casual".into(),
            code_mix: "natural".into(),
        })
        .unwrap();
        assert!(!result.output.is_empty());
    }
}
