use std::error::Error;

use image::imageops::FilterType;

use tapciify::{CustomRatioResize, DEFAULT_FONT_RATIO};
use tapciify::background_string::BackgroundStringArtConverter;

fn main() -> Result<(), Box<dyn Error>> {
    let img = image::open("../assets/examples/ferris.webp")?;

    let result = img
        .resize_custom_ratio(Some(64), None, DEFAULT_FONT_RATIO, FilterType::Triangle)
        .background_string_art("hello world! ", false)?;

    println!("{}", result);

    Ok(())
}
