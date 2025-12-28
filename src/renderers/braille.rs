//! Converting images to ASCII art using Braille characters

use image::{ImageBuffer, Pixel};
#[cfg(feature = "rayon")]
use rayon::prelude::*;

use crate::{
    prelude::*,
    renderers::ascii::SizeError,
    utils::threshold::{DEFAULT_THRESHOLD, ThresholdPixel},
};

/// Braille characters aspect ratio
pub const DEFAULT_BRAILLE_FONT_RATIO: f64 = 21.0 / 24.0;

/// Convert array of booleans into braille character
///
/// Grid of booleans placement
///
/// |---|---|
/// | 0 | 3 |
/// | 1 | 4 |
/// | 2 | 5 |
/// | 6 | 7 |
///
/// # Example
///
/// ```
/// use tapciify::renderers::braille::boolean_array_to_braille;
///
/// # fn main() {
/// let braille_char =
///     boolean_array_to_braille(&[true, false, false, false, true, false, false, true]);
///
/// assert_eq!(braille_char, '⢑');
/// # }
/// ```
pub fn boolean_array_to_braille(array: &[bool; 8]) -> char {
    let mut codepoint: u32 = 0x2800; // Base codepoint for Braille characters

    // Calculate the codepoint based on the boolean array
    for (i, &value) in array.iter().enumerate() {
        if value {
            codepoint |= 1 << i;
        }
    }

    // Convert the codepoint to a char
    std::char::from_u32(codepoint).unwrap_or(' ')
}

/// Allows to render your images using Braille characters
pub trait BrailleArtConverter {
    /// Convert image into ASCII art using Braille characters
    ///
    /// # Examples
    ///
    /// ```
    /// use std::error::Error;
    ///
    /// use tapciify::{
    ///     prelude::*,
    ///     renderers::braille::{BrailleArtConverter, DEFAULT_BRAILLE_FONT_RATIO},
    /// };
    ///
    /// # use image::imageops::FilterType;
    ///
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// let img = image::open("./assets/examples/rin-shima.webp")?;
    ///
    /// let result = img
    ///     .resize_custom_ratio(
    ///         Some(64 * 2),
    ///         None,
    ///         DEFAULT_BRAILLE_FONT_RATIO,
    ///         FilterType::Triangle,
    ///     )
    ///     .to_luma8()
    ///     .braille_art(false)?;
    ///
    /// println!("{}", result);
    /// # Ok(())
    /// # }
    /// ```
    fn braille_art(&self, colored: bool) -> Result<AsciiArt, SizeError>;
}

impl BrailleArtConverter for image::DynamicImage {
    fn braille_art(&self, colored: bool) -> Result<AsciiArt, SizeError> {
        self.to_rgba8().braille_art(colored)
    }
}

impl<P, Container> BrailleArtConverter for ImageBuffer<P, Container>
where
    P: Pixel + ToAsciiArtPixel + ThresholdPixel + Sync,
    P::Subpixel: Sync,
    Container: std::ops::Deref<Target = [P::Subpixel]> + Sync,
{
    fn braille_art(&self, colored: bool) -> Result<AsciiArt, SizeError> {
        let width = self.width();
        let height = self.height();

        if width < 4 || height < 8 {
            return Err(SizeError);
        }

        let braille_width = width / 2;
        let braille_height = height / 4;
        let total_chars = braille_width * braille_height;

        #[cfg(feature = "rayon")]
        let iter = (0..total_chars).into_par_iter();
        #[cfg(not(feature = "rayon"))]
        let iter = (0..total_chars).into_iter();

        let characters = iter
            .map(|i| {
                let x = (i % braille_width) * 2;
                let y = (i / braille_width) * 4;
                (x, y)
            })
            .map(|(x, y)| {
                let braille_array = calc_braille_pixels(x, y)
                    .map(|(x, y)| self.get_pixel(x, y).threshold_pixel(DEFAULT_THRESHOLD));

                // Top left pixel (used only for colors)
                self.get_pixel(x, y)
                    .to_raw_ascii_art_pixel(boolean_array_to_braille(&braille_array))
            })
            .collect();

        Ok(AsciiArt::new(
            characters,
            braille_width,
            braille_height,
            colored,
        ))
    }
}

/// Calculates braille pixels positions
pub fn calc_braille_pixels(x: u32, y: u32) -> [(u32, u32); 8] {
    [
        (x, y),
        (x, y + 1),
        (x, y + 2),
        (x + 1, y),
        (x, y + 1),
        (x, y + 2),
        (x, y + 3),
        (x + 1, y + 3),
    ]
}
