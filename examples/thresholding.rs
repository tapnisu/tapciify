use image::{imageops::FilterType, DynamicImage};
use imageproc::contrast::adaptive_threshold;
use std::error::Error;
use tapciify::{
    AsciiArtConverter, AsciiArtConverterOptions, CustomRatioResize, DEFAULT_FONT_RATIO,
};

fn main() -> Result<(), Box<dyn Error>> {
    let img = image::open("./assets/examples/rin-shima.webp")?;
    let thresholded_img: DynamicImage = adaptive_threshold(&img.to_luma8(), 20).into();

    let result = thresholded_img
        .resize_custom_ratio(Some(64), None, DEFAULT_FONT_RATIO, FilterType::Triangle)
        .ascii_art(&AsciiArtConverterOptions {
            // Put your other options here
            ..Default::default()
        })?;

    println!("{}", result);

    Ok(())
}
