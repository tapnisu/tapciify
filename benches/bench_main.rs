use criterion::{Criterion, criterion_group, criterion_main};
use image::imageops::FilterType;
use tapciify::{
    prelude::*,
    renderers::{
        ascii::DEFAULT_ASCII_STRING, background_string::BackgroundStringArtConverter,
        braille::BrailleArtConverter,
    },
    utils::resize::DEFAULT_FONT_RATIO,
};

fn bench_main(c: &mut Criterion) {
    let img = image::open("./assets/examples/ferris.webp")
        .unwrap()
        .resize_custom_ratio(None, None, DEFAULT_FONT_RATIO, FilterType::Triangle);

    c.bench_function("ascii", |b| {
        let options = AsciiArtConverterOptions::default();
        b.iter(|| img.ascii_art(&options).unwrap());
    });

    c.bench_function("ascii display", |b| {
        let options = AsciiArtConverterOptions::default();
        b.iter(|| img.ascii_art(&options).unwrap().to_string());
    });

    c.bench_function("ascii colored display", |b| {
        let options = AsciiArtConverterOptions {
            colored: true,
            ..Default::default()
        };
        b.iter(|| img.ascii_art(&options).unwrap().to_string());
    });

    c.bench_function("background string", |b| {
        b.iter(|| {
            img.background_string_art(DEFAULT_ASCII_STRING, false)
                .unwrap()
        });
    });

    c.bench_function("background string display", |b| {
        b.iter(|| {
            img.background_string_art(DEFAULT_ASCII_STRING, false)
                .unwrap()
                .to_string()
        });
    });

    c.bench_function("background string colored display", |b| {
        b.iter(|| {
            img.background_string_art(DEFAULT_ASCII_STRING, true)
                .unwrap()
                .to_string()
        });
    });

    c.bench_function("braille", |b| {
        b.iter(|| img.braille_art(false).unwrap());
    });

    c.bench_function("braille display", |b| {
        b.iter(|| img.braille_art(false).unwrap().to_string());
    });

    c.bench_function("braille colored display", |b| {
        b.iter(|| img.braille_art(true).unwrap().to_string());
    });
}

criterion_group!(benches, bench_main);
criterion_main!(benches);
