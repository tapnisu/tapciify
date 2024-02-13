use image::imageops::FilterType;
use std::error::Error;
use tapciify::{AsciiConverter, AsciiConverterOptions, CustomRatioResize, DEFAULT_FONT_RATIO};

fn main() -> Result<(), Box<dyn Error>> {
    let orig_img = image::open("./assets/examples/original.webp")?;

    let img =
        orig_img.resize_custom_ratio(Some(64), None, DEFAULT_FONT_RATIO, FilterType::Triangle);

    let options = AsciiConverterOptions {
        // Put your other options here
        ..Default::default()
    };

    let result = AsciiConverter::convert(&img, &options)?;

    println!("{}", result.text);

    Ok(())
}
