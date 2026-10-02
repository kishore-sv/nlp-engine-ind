use nlp_engine_ind_core::{ConversionRequest, Converter, Language};

fn main() -> nlp_engine_ind_core::Result<()> {
    let converter = Converter::new();
    let samples = [
        "I'll call you after work",
        "Send the GitHub link to @rahul",
        "Bro, can you send me the project?",
    ];

    for input in samples {
        let result = converter.convert(ConversionRequest::new(input, Language::Kannada))?;
        println!("Input:  {input}");
        println!("Output: {}", result.output);
        println!(
            "Protected tokens preserved: {}",
            result.diagnostics.protected_token_count
        );
        println!();
    }
    Ok(())
}
