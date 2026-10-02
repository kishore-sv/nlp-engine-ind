use nlp_engine_ind_core::{ConversionRequest, Converter, Language};

fn main() -> nlp_engine_ind_core::Result<()> {
    let converter = Converter::new();
    let result = converter.convert(ConversionRequest::new(
        "I'll finish this work and call you once I'm free",
        Language::Kannada,
    ))?;
    println!("Input:  {}", result.input);
    println!("Output: {}", result.output);
    println!("Confidence: {:.2}", result.confidence);
    Ok(())
}
