use nlp_engine_ind_core::{ConversionRequest, Converter, Language};
use std::time::Instant;

fn main() -> nlp_engine_ind_core::Result<()> {
    let converter = Converter::new();
    let msg = "I'll call you later";

    let start = Instant::now();
    for _ in 0..1000 {
        converter.convert(ConversionRequest::new(msg, Language::Kannada))?;
    }
    let elapsed = start.elapsed();
    println!("1000 conversions in {:?}", elapsed);
    println!("Average: {:.2} µs", elapsed.as_micros() as f64 / 1000.0);
    Ok(())
}
