use criterion::{black_box, criterion_group, criterion_main, Criterion};
use nlp_engine_ind_core::{ConversionRequest, Converter, Language};

fn bench_pipeline(c: &mut Criterion) {
    let converter = Converter::new();
    let messages = [
        "I'll call you later",
        "Are you coming?",
        "Can you send me the file?",
        "I'll finish this work and call you once I'm free",
    ];

    let mut group = c.benchmark_group("conversion");
    for msg in messages {
        group.bench_with_input(format!("kn: {msg}"), msg, |b, text| {
            b.iter(|| {
                converter
                    .convert(black_box(ConversionRequest::new(text, Language::Kannada)))
                    .unwrap()
            });
        });
    }
    group.finish();
}

criterion_group!(benches, bench_pipeline);
criterion_main!(benches);
