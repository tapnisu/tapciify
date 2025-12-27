use std::error::Error;
use image::imageops::FilterType;

use tapciify::{
    prelude::*,
    renderers::braille::{BrailleArtConverter, DEFAULT_BRAILLE_FONT_RATIO},
};

fn main() -> Result<(), Box<dyn Error>> {
    let img = image::open("./assets/examples/rin-shima.webp")?;

    let result = img
        .resize_custom_ratio(
            Some(64 * 2),
            None,
            DEFAULT_BRAILLE_FONT_RATIO,
            FilterType::Triangle,
        )
        .to_luma8()
        .braille_art(false)?;

    println!("{}", result);

    Ok(())
}
