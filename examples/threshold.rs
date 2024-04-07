use std::error::Error;

use image::imageops::FilterType;
use imageproc::contrast::adaptive_threshold;

use tapciify::{
    AsciiArtConverter, AsciiArtConverterOptions, CustomRatioResize, DEFAULT_FONT_RATIO,
};

fn main() -> Result<(), Box<dyn Error>> {
    let img = image::open("./assets/examples/rin-shima.webp")?;
    let threshold_img: image::DynamicImage = adaptive_threshold(&img.to_luma8(), 20).into();

    let result = threshold_img
        .resize_custom_ratio(Some(64), None, DEFAULT_FONT_RATIO, FilterType::Triangle)
        .ascii_art(&AsciiArtConverterOptions {
            // Put your other options here
            ..Default::default()
        })?;

    println!("{}", result);

    Ok(())
}
