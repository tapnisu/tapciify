use tapciify::{AsciiConverter, AsciiConverterOptions};

fn main() {
    let img = image::open("./assets/examples/original.webp").unwrap();

    let options = AsciiConverterOptions {
        width: 64,
        colored: true,
        ..Default::default()
    };

    let result = AsciiConverter::convert(&img, &options).unwrap();

    println!("{}", result.text);
}
