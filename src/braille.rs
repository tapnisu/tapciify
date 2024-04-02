//! Converting images to ASCII art using Braille characters

use crate::{AsciiArt, AsciiArtPixel, SizeError};
use image::Pixel;

#[cfg(feature = "rayon")]
use rayon::prelude::*;

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
/// use tapciify::braille::boolean_array_to_braille;
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
    /// use image::imageops::FilterType;
    /// # use std::error::Error;
    /// use tapciify::{
    ///     braille::{BrailleArtConverter, DEFAULT_BRAILLE_FONT_RATIO},
    ///     CustomRatioResize,
    /// };
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

impl BrailleArtConverter for image::RgbImage {
    fn braille_art(&self, colored: bool) -> Result<AsciiArt, SizeError> {
        let width = self.width();
        let height = self.height();

        if width < 4 || height < 8 {
            return Err(SizeError);
        }

        let x_range: Vec<u32> = (0..(width - width % 2)).step_by(2).collect();
        let y_range: Vec<u32> = (0..(height - height % 4)).step_by(4).collect();

        let width = x_range.clone().len() as u32;
        let height = y_range.clone().len() as u32;

        #[cfg(feature = "rayon")]
        let x_iter = x_range.into_par_iter();
        #[cfg(not(feature = "rayon"))]
        let x_iter = x_range.into_iter();

        #[cfg(feature = "rayon")]
        let y_iter = y_range.into_par_iter();
        #[cfg(not(feature = "rayon"))]
        let y_iter = y_range.into_iter();

        let characters = y_iter
            .flat_map(|y| {
                x_iter.clone().map(move |x| {
                    // Top left pixel
                    let tlpx = self.get_pixel(x, y);

                    let braille_array = &[
                        tlpx,
                        self.get_pixel(x, y + 1),
                        self.get_pixel(x, y + 2),
                        self.get_pixel(x + 1, y),
                        self.get_pixel(x, y + 1),
                        self.get_pixel(x, y + 2),
                        self.get_pixel(x, y + 3),
                        self.get_pixel(x + 1, y + 3),
                    ]
                    .map(|p| p.to_luma()[0] > 255 / 2);

                    AsciiArtPixel {
                        character: boolean_array_to_braille(braille_array),
                        r: tlpx.0[0],
                        g: tlpx.0[1],
                        b: tlpx.0[2],
                        a: 255,
                    }
                })
            })
            .collect();

        Ok(AsciiArt::new(characters, width, height, colored))
    }
}

impl BrailleArtConverter for image::RgbaImage {
    fn braille_art(&self, colored: bool) -> Result<AsciiArt, SizeError> {
        let width = self.width();
        let height = self.height();

        if width < 4 || height < 8 {
            return Err(SizeError);
        }

        let x_range: Vec<u32> = (0..(width - width % 2)).step_by(2).collect();
        let y_range: Vec<u32> = (0..(height - height % 4)).step_by(4).collect();

        let width = x_range.clone().len() as u32;
        let height = y_range.clone().len() as u32;

        #[cfg(feature = "rayon")]
        let x_iter = x_range.into_par_iter();
        #[cfg(not(feature = "rayon"))]
        let x_iter = x_range.into_iter();

        #[cfg(feature = "rayon")]
        let y_iter = y_range.into_par_iter();
        #[cfg(not(feature = "rayon"))]
        let y_iter = y_range.into_iter();

        let characters = y_iter
            .flat_map(|y| {
                x_iter.clone().map(move |x| {
                    // Top left pixel
                    let tlpx = self.get_pixel(x, y);

                    let braille_array = &[
                        tlpx,
                        self.get_pixel(x, y + 1),
                        self.get_pixel(x, y + 2),
                        self.get_pixel(x + 1, y),
                        self.get_pixel(x, y + 1),
                        self.get_pixel(x, y + 2),
                        self.get_pixel(x, y + 3),
                        self.get_pixel(x + 1, y + 3),
                    ]
                    .map(|p| {
                        let la_px = p.to_luma_alpha();
                        la_px[0] as u16 * la_px[1] as u16 > ((255 / 2) * (255 / 2))
                    });

                    AsciiArtPixel {
                        character: boolean_array_to_braille(braille_array),
                        r: tlpx.0[0],
                        g: tlpx.0[1],
                        b: tlpx.0[2],
                        a: tlpx.0[3],
                    }
                })
            })
            .collect();

        Ok(AsciiArt::new(characters, width, height, colored))
    }
}

impl BrailleArtConverter for image::GrayImage {
    fn braille_art(&self, colored: bool) -> Result<AsciiArt, SizeError> {
        let width = self.width();
        let height = self.height();

        if width < 4 || height < 8 {
            return Err(SizeError);
        }

        let x_range: Vec<u32> = (0..(width - width % 2)).step_by(2).collect();
        let y_range: Vec<u32> = (0..(height - height % 4)).step_by(4).collect();

        let width = x_range.clone().len() as u32;
        let height = y_range.clone().len() as u32;

        #[cfg(feature = "rayon")]
        let x_iter = x_range.into_par_iter();
        #[cfg(not(feature = "rayon"))]
        let x_iter = x_range.into_iter();

        #[cfg(feature = "rayon")]
        let y_iter = y_range.into_par_iter();
        #[cfg(not(feature = "rayon"))]
        let y_iter = y_range.into_iter();

        let characters = y_iter
            .flat_map(|y| {
                x_iter.clone().map(move |x| {
                    // Top left pixel
                    let tlpx = self.get_pixel(x, y);

                    let braille_array = &[
                        tlpx,
                        self.get_pixel(x, y + 1),
                        self.get_pixel(x, y + 2),
                        self.get_pixel(x + 1, y),
                        self.get_pixel(x, y + 1),
                        self.get_pixel(x, y + 2),
                        self.get_pixel(x, y + 3),
                        self.get_pixel(x + 1, y + 3),
                    ]
                    .map(|p| p[0] > 255 / 2);

                    AsciiArtPixel {
                        character: boolean_array_to_braille(braille_array),
                        r: tlpx.0[0],
                        g: tlpx.0[0],
                        b: tlpx.0[0],
                        a: 255,
                    }
                })
            })
            .collect();

        Ok(AsciiArt::new(characters, width, height, colored))
    }
}

impl BrailleArtConverter for image::GrayAlphaImage {
    fn braille_art(&self, colored: bool) -> Result<AsciiArt, SizeError> {
        let width = self.width();
        let height = self.height();

        if width < 2 || height < 4 {
            return Err(SizeError);
        }

        let x_range: Vec<u32> = (0..(width - width % 2)).step_by(2).collect();
        let y_range: Vec<u32> = (0..(height - height % 4)).step_by(4).collect();

        let width = x_range.clone().len() as u32;
        let height = y_range.clone().len() as u32;

        #[cfg(feature = "rayon")]
        let x_iter = x_range.into_par_iter();
        #[cfg(not(feature = "rayon"))]
        let x_iter = x_range.into_iter();

        #[cfg(feature = "rayon")]
        let y_iter = y_range.into_par_iter();
        #[cfg(not(feature = "rayon"))]
        let y_iter = y_range.into_iter();

        let characters = y_iter
            .flat_map(|y| {
                x_iter.clone().map(move |x| {
                    // Top left pixel
                    let tlpx = self.get_pixel(x, y);

                    let braille_array = &[
                        tlpx,
                        self.get_pixel(x, y + 1),
                        self.get_pixel(x, y + 2),
                        self.get_pixel(x + 1, y),
                        self.get_pixel(x, y + 1),
                        self.get_pixel(x, y + 2),
                        self.get_pixel(x, y + 3),
                        self.get_pixel(x + 1, y + 3),
                    ]
                    .map(|p| p[0] as u16 * p[1] as u16 > ((255 / 2) * (255 / 2)));

                    AsciiArtPixel {
                        character: boolean_array_to_braille(braille_array),
                        r: tlpx.0[0],
                        g: tlpx.0[0],
                        b: tlpx.0[0],
                        a: tlpx.0[1],
                    }
                })
            })
            .collect();

        Ok(AsciiArt::new(characters, width, height, colored))
    }
}
