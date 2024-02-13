use tapciify::{
    resizing::{resize, ResizingOptions},
    AsciiConverter, AsciiConverterOptions,
};

fn main() {
    let orig_img = image::open("./assets/examples/original.webp").unwrap();

    let img = resize(
        &orig_img,
        &ResizingOptions {
            width: Some(64),
            ..Default::default()
        },
    );

    let options = AsciiConverterOptions::default();

    let result = AsciiConverter::convert(&img, &options).unwrap();

    println!("{}", result.text);
}
