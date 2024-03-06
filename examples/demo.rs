use image::imageops::FilterType;
use std::error::Error;
use tapciify::{
    AsciiArtConverter, AsciiArtConverterOptions, CustomRatioResize, DEFAULT_FONT_RATIO,
};

fn main() -> Result<(), Box<dyn Error>> {
    let img = image::open("./assets/examples/original.webp")?;

    let result = img
        .resize_custom_ratio(Some(64), None, DEFAULT_FONT_RATIO, FilterType::Triangle)
        .to_luma_alpha8()
        .ascii_art(&AsciiArtConverterOptions {
            // Put your other options here
            ..Default::default()
        })?;

    println!("{}", result);

    Ok(())
}
