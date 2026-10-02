use nlp_engine_ind_core::{ConversionRequest, Converter, Language};

fn main() -> nlp_engine_ind_core::Result<()> {
    let converter = Converter::new();
    let inputs = [
        "I'll call you later",
        "Are you coming?",
        "What are you doing?",
    ];

    for language in Language::all_enabled() {
        println!("=== {} ===", language);
        for input in inputs {
            let result = converter.convert(ConversionRequest::new(input, language))?;
            println!("  {input} -> {}", result.output);
        }
        println!();
    }
    Ok(())
}
