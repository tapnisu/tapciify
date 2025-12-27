use criterion::{criterion_group, criterion_main, Criterion};
use image::imageops::FilterType;
use tapciify::{prelude::*, utils::resize::DEFAULT_FONT_RATIO};

fn criterion_benchmark(c: &mut Criterion) {
    let img = image::open("./assets/examples/ferris.webp")
        .unwrap()
        .resize_custom_ratio(None, None, DEFAULT_FONT_RATIO, FilterType::Triangle);

    c.bench_function("demo", |b| {
        b.iter(|| img.ascii_art(&AsciiArtConverterOptions::default()).unwrap());
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
