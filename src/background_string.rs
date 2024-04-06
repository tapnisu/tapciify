use crate::{
    threshold_utils::{ThresholdPixel, DEFAULT_THRESHOLD},
    AsciiArt, AsciiArtPixel, SizeError,
};

#[cfg(feature = "rayon")]
use rayon::prelude::*;

pub trait BackgroundStringArtConverter {
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
