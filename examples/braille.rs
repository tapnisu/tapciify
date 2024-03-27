use image::imageops::FilterType;
use std::error::Error;
use tapciify::braille::BrailleArtConverter;

fn main() -> Result<(), Box<dyn Error>> {
    let img = image::open("./assets/examples/rin-shima.webp")?;

    let result = img
        .resize(64 * 2, u32::max_value(), FilterType::Triangle)
        .to_luma8()
        .braille_art(false)?;

    println!("{}", result);

    Ok(())
}
