use crate::{
    threshold_utils::{ThresholdPixel, DEFAULT_THRESHOLD},
    AsciiArt, AsciiArtPixel, SizeError,
};

#[cfg(feature = "rayon")]
use rayon::prelude::*;

/// Convert image into ASCII art with text on the background
pub trait BackgroundStringArtConverter {
    /// Convert image into ASCII art with text on the background
    ///
    /// # Examples
    ///
    /// ```
    /// use image::imageops::FilterType;
    /// # use std::error::Error;
    /// use tapciify::background_string::BackgroundStringArtConverter;
    /// use tapciify::{CustomRatioResize, DEFAULT_FONT_RATIO};
    ///
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// let img = image::open("./assets/examples/original.webp")?;
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

impl BackgroundStringArtConverter for image::RgbImage {
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
            .map(|(index, pixel)| AsciiArtPixel {
                character: if pixel.threshold_pixel(DEFAULT_THRESHOLD) {
                    string.chars().nth(index % string.len()).unwrap()
                } else {
                    ' '
                },
                r: pixel.0[0],
                g: pixel.0[1],
                b: pixel.0[2],
                a: 255,
            })
            .collect::<Vec<AsciiArtPixel>>();

        Ok(AsciiArt::new(
            characters,
            self.width(),
            self.height(),
            colored,
        ))
    }
}

impl BackgroundStringArtConverter for image::RgbaImage {
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
            .map(|(index, pixel)| AsciiArtPixel {
                character: if pixel.threshold_pixel(DEFAULT_THRESHOLD) {
                    string.chars().nth(index % string.len()).unwrap()
                } else {
                    ' '
                },
                r: pixel.0[0],
                g: pixel.0[1],
                b: pixel.0[2],
                a: pixel.0[3],
            })
            .collect::<Vec<AsciiArtPixel>>();

        Ok(AsciiArt::new(
            characters,
            self.width(),
            self.height(),
            colored,
        ))
    }
}

impl BackgroundStringArtConverter for image::GrayImage {
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
            .map(|(index, pixel)| AsciiArtPixel {
                character: if pixel.threshold_pixel(DEFAULT_THRESHOLD) {
                    string.chars().nth(index % string.len()).unwrap()
                } else {
                    ' '
                },
                r: pixel.0[0],
                g: pixel.0[0],
                b: pixel.0[0],
                a: 255,
            })
            .collect::<Vec<AsciiArtPixel>>();

        Ok(AsciiArt::new(
            characters,
            self.width(),
            self.height(),
            colored,
        ))
    }
}

impl BackgroundStringArtConverter for image::GrayAlphaImage {
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
            .map(|(index, pixel)| AsciiArtPixel {
                character: if pixel.threshold_pixel(DEFAULT_THRESHOLD) {
                    string.chars().nth(index % string.len()).unwrap()
                } else {
                    ' '
                },
                r: pixel.0[0],
                g: pixel.0[0],
                b: pixel.0[0],
                a: pixel.0[1],
            })
            .collect::<Vec<AsciiArtPixel>>();

        Ok(AsciiArt::new(
            characters,
            self.width(),
            self.height(),
            colored,
        ))
    }
}
