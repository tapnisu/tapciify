//! Use text for background on light pixels

use image::{ImageBuffer, Pixel};
#[cfg(feature = "rayon")]
use rayon::prelude::*;

use crate::prelude::*;

use crate::{
    renderers::ascii::SizeError,
    utils::threshold::{DEFAULT_THRESHOLD, ThresholdPixel},
};

/// Convert image into ASCII art with text on the background
pub trait BackgroundStringArtConverter {
    /// Convert image into ASCII art with text on the background
    ///
    /// # Examples
    ///
    /// ```
    /// use std::error::Error;
    ///
    /// use tapciify::prelude::*;
    /// use tapciify::utils::resize::DEFAULT_FONT_RATIO;
    /// use tapciify::renderers::background_string::BackgroundStringArtConverter;
    ///
    /// # use image::imageops::FilterType;
    ///
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// let img = image::open("./assets/examples/ferris.webp")?;
    ///
    /// let result = img
    ///     .resize_custom_ratio(Some(64), None, DEFAULT_FONT_RATIO, FilterType::Triangle)
    ///     .background_string_art("hello world! ", false)?;
    ///
    /// println!("{}", result);
    /// # Ok(())
    /// # }
    /// ```
    fn background_string_art(&self, string: &str, colored: bool) -> Result<AsciiArt, SizeError>;
}

impl BackgroundStringArtConverter for image::DynamicImage {
    fn background_string_art(&self, string: &str, colored: bool) -> Result<AsciiArt, SizeError> {
        self.clone()
            .into_rgba8()
            .background_string_art(string, colored)
    }
}

impl<P, Container> BackgroundStringArtConverter for ImageBuffer<P, Container>
where
    P: Pixel + ToAsciiArtPixel + ThresholdPixel + Sync,
    P::Subpixel: Sync,
    Container: std::ops::Deref<Target = [P::Subpixel]>,
{
    fn background_string_art(&self, string: &str, colored: bool) -> Result<AsciiArt, SizeError> {
        if self.width() == 0 || self.height() == 0 {
            return Err(SizeError);
        }

        #[cfg(feature = "rayon")]
        let iter = self.par_pixels();
        #[cfg(not(feature = "rayon"))]
        let iter = self.pixels();

        let characters = iter
            .enumerate()
            .map(|(index, pixel)| {
                (
                    match pixel.threshold_pixel(DEFAULT_THRESHOLD) {
                        true => string.chars().nth(index % string.chars().count()).unwrap(),
                        false => ' ',
                    },
                    pixel,
                )
            })
            .map(|(character, pixel)| pixel.to_raw_ascii_art_pixel(character))
            .collect::<Vec<AsciiArtPixel>>();

        Ok(AsciiArt::new(
            characters,
            self.width(),
            self.height(),
            colored,
        ))
    }
}
