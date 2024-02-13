use std::error::Error;
use tapciify::{
    resizing::{resize, ResizingOptions},
    AsciiConverter, AsciiConverterOptions,
};

fn main() -> Result<(), Box<dyn Error>> {
    let orig_img = image::open("./assets/examples/original.webp")?;

    let img = resize(
        &orig_img,
        &ResizingOptions {
            width: Some(64),
            ..Default::default()
        },
    );

    let options = AsciiConverterOptions {
        colored: true,
        ..Default::default()
    };

    let result = AsciiConverter::convert(&img, &options)?;

    println!("{}", result.text);

    Ok(())
}
