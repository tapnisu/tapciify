//! Utils for converting your images to ASCII
//!
//! # Examples
//!
//! ## Demo
//!
//! ```
#![doc = include_str!("../examples/demo.rs")]
//! ```
//!
//! ## Colored
//!
//! ```
#![doc = include_str!("../examples/demo.rs")]
//! ```

use colored::Colorize;
use image::{DynamicImage, Pixel};
use std::{
    cmp::{max, min},
    fmt,
};
use thiserror::Error;

#[cfg(feature = "rayon")]
use rayon::prelude::*;

/// Default ASCII string, feel free to use your one
pub const DEFAULT_ASCII_STRING: &str = " .,:;+*?%S#@";

/// Converter of images to ASCII art
pub trait AsciiArtConverter {
    /// Convert image to an ASCII art
    ///
    /// # Examples
    ///
    /// Demo:
    ///
    /// ```
    #[doc = include_str!("../examples/demo.rs")]
    /// ````
    ///
    /// Colored:
    ///
    /// ```
    #[doc = include_str!("../examples/colored.rs")]
    /// ````
    fn ascii_art(
        &self,
        options: &AsciiArtConverterOptions,
    ) -> Result<AsciiArt, AsciiArtConverterError>;
}

impl AsciiArtConverter for DynamicImage {
    fn ascii_art(
        &self,
        options: &AsciiArtConverterOptions,
    ) -> Result<AsciiArt, AsciiArtConverterError> {
        self.to_rgba8().ascii_art(options)
    }
}

impl AsciiArtConverter for image::RgbImage {
    fn ascii_art(
        &self,
        options: &AsciiArtConverterOptions,
    ) -> Result<AsciiArt, AsciiArtConverterError> {
        if self.width() == 0 || self.height() == 0 {
            return Err(AsciiArtConverterError::SizeError(SizeError));
        }

        #[cfg(feature = "rayon")]
        let iter = self
            .pixels()
            .map(|rgba| rgba.to_owned())
            .collect::<Vec<image::Rgb<u8>>>()
            .into_par_iter();
        #[cfg(not(feature = "rayon"))]
        let iter = self.pixels();

        let characters = iter
            .map(|pixel| pixel.to_ascii_art_pixel(&options.ascii_string))
            .collect::<Result<Vec<AsciiArtPixel>, AsciiStringError>>()?;

        Ok(AsciiArt::new(
            characters,
            self.width(),
            self.height(),
            options.colored,
        ))
    }
}

impl AsciiArtConverter for image::RgbaImage {
    fn ascii_art(
        &self,
        options: &AsciiArtConverterOptions,
    ) -> Result<AsciiArt, AsciiArtConverterError> {
        if self.width() == 0 || self.height() == 0 {
            return Err(AsciiArtConverterError::SizeError(SizeError));
        }

        #[cfg(feature = "rayon")]
        let iter = self
            .pixels()
            .map(|rgba| rgba.to_owned())
            .collect::<Vec<image::Rgba<u8>>>()
            .into_par_iter();
        #[cfg(not(feature = "rayon"))]
        let iter = self.pixels();

        let characters = iter
            .map(|pixel| pixel.to_ascii_art_pixel(&options.ascii_string))
            .collect::<Result<Vec<AsciiArtPixel>, AsciiStringError>>()?;

        Ok(AsciiArt::new(
            characters,
            self.width(),
            self.height(),
            options.colored,
        ))
    }
}

impl AsciiArtConverter for image::GrayImage {
    fn ascii_art(
        &self,
        options: &AsciiArtConverterOptions,
    ) -> Result<AsciiArt, AsciiArtConverterError> {
        if self.width() == 0 || self.height() == 0 {
            return Err(AsciiArtConverterError::SizeError(SizeError));
        }

        #[cfg(feature = "rayon")]
        let iter = self
            .pixels()
            .map(|luma| luma.to_owned())
            .collect::<Vec<image::Luma<u8>>>()
            .into_par_iter();
        #[cfg(not(feature = "rayon"))]
        let iter = self.pixels();

        let characters = iter
            .map(|pixel| pixel.to_ascii_art_pixel(&options.ascii_string))
            .collect::<Result<Vec<AsciiArtPixel>, AsciiStringError>>()?;

        Ok(AsciiArt::new(
            characters,
            self.width(),
            self.height(),
            options.colored,
        ))
    }
}

impl AsciiArtConverter for image::GrayAlphaImage {
    fn ascii_art(
        &self,
        options: &AsciiArtConverterOptions,
    ) -> Result<AsciiArt, AsciiArtConverterError> {
        if self.width() == 0 || self.height() == 0 {
            return Err(AsciiArtConverterError::SizeError(SizeError));
        }

        #[cfg(feature = "rayon")]
        let iter = self
            .pixels()
            .map(|luma| luma.to_owned())
            .collect::<Vec<image::LumaA<u8>>>()
            .into_par_iter();
        #[cfg(not(feature = "rayon"))]
        let iter = self.pixels();

        let characters = iter
            .map(|pixel| pixel.to_ascii_art_pixel(&options.ascii_string))
            .collect::<Result<Vec<AsciiArtPixel>, AsciiStringError>>()?;

        Ok(AsciiArt::new(
            characters,
            self.width(),
            self.height(),
            options.colored,
        ))
    }
}

/// Options for converter of images to ASCII art
#[derive(Debug, Clone)]
pub struct AsciiArtConverterOptions {
    pub ascii_string: String,
    pub colored: bool,
}

/// Error caused by [`AsciiArtConverter`]
#[derive(Clone, Debug, Error)]
pub enum AsciiArtConverterError {
    #[error("{0}")]
    AsciiStringError(#[from] AsciiStringError),
    #[error("{0}")]
    SizeError(#[from] SizeError),
}

#[derive(Clone, Debug, Error)]
#[error("width and height can't both be 0")]
pub struct SizeError;

impl Default for AsciiArtConverterOptions {
    fn default() -> AsciiArtConverterOptions {
        AsciiArtConverterOptions {
            ascii_string: DEFAULT_ASCII_STRING.to_owned(),
            colored: false,
        }
    }
}

/// Raw ASCII art conversion result
#[derive(Debug, Clone)]
pub struct AsciiArt {
    pub characters: Vec<AsciiArtPixel>,
    pub width: u32,
    pub height: u32,
    pub colored: bool,
}

impl AsciiArt {
    pub fn new(characters: Vec<AsciiArtPixel>, width: u32, height: u32, colored: bool) -> AsciiArt {
        AsciiArt {
            characters,
            width,
            height,
            colored,
        }
    }
}

impl fmt::Display for AsciiArt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        #[cfg(feature = "rayon")]
        let iter = self.characters.par_iter();
        #[cfg(not(feature = "rayon"))]
        let iter = self.characters.iter();

        let characters = iter
            .map(|ascii_character| ascii_character.to_string(self.colored))
            .collect::<Vec<String>>();

        #[cfg(feature = "rayon")]
        let chunks = characters.par_chunks(self.width.try_into().unwrap());
        #[cfg(not(feature = "rayon"))]
        let chunks = characters.chunks(self.width.try_into().unwrap());

        let text = chunks
            .map(|line| line.join(""))
            .collect::<Vec<String>>()
            .join("\n");

        write!(f, "{}", text)
    }
}

/// ASCII pixel of [`AsciiArt`]
#[derive(Debug, Clone)]
pub struct AsciiArtPixel {
    pub character: char,
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl AsciiArtPixel {
    /// Convert RGBA and ASCII string into [`AsciiArtPixel`]
    ///
    /// # Examples
    ///
    /// ```
    /// use tapciify::AsciiArtPixel;
    ///
    /// let ascii_string = " *#";
    ///
    /// let result = AsciiArtPixel::new(255, 255, 255, 255, ascii_string)?;
    /// assert_eq!(result.character, '#');
    ///
    /// let result = AsciiArtPixel::new(255, 255, 255, 0, ascii_string)?;
    /// assert_eq!(result.character, ' ');
    ///
    /// let result = AsciiArtPixel::new(0, 0, 0, 255, ascii_string)?;
    /// assert_eq!(result.character, ' ');
    /// # Ok::<(), tapciify::AsciiStringError>(())
    /// `````
    #[deprecated(since = "3.1.0")]
    pub fn new(
        r: u8,
        g: u8,
        b: u8,
        a: u8,
        ascii_string: &str,
    ) -> Result<AsciiArtPixel, AsciiStringError> {
        #[allow(deprecated)]
        let lightness = get_lightness(r, g, b, a);

        let character = ascii_character(lightness, ascii_string)?;

        Ok(AsciiArtPixel {
            character,
            r,
            g,
            b,
            a,
        })
    }

    /// Allows you to convert to colored [`String`] or normal [`String`]. Just read the [`AsciiArtPixel::character`] value
    pub fn to_string(&self, colored: bool) -> String {
        if colored {
            return self
                .character
                .to_string()
                .truecolor(self.r, self.g, self.b)
                .to_string();
        }

        self.character.to_string()
    }
}

pub trait ToAsciiArtPixel {
    /// Convert [`image`] crate color types to [`AsciiArtPixel`]
    fn to_ascii_art_pixel(&self, ascii_string: &str) -> Result<AsciiArtPixel, AsciiStringError>;
}

impl ToAsciiArtPixel for image::Rgb<u8> {
    fn to_ascii_art_pixel(&self, ascii_string: &str) -> Result<AsciiArtPixel, AsciiStringError> {
        let luma_pixel = self.to_luma();

        Ok(AsciiArtPixel {
            character: ascii_character(luma_pixel[0] as f32 / 255.0, ascii_string)?,
            r: self[0],
            g: self[1],
            b: self[2],
            a: 255,
        })
    }
}

impl ToAsciiArtPixel for image::Rgba<u8> {
    fn to_ascii_art_pixel(&self, ascii_string: &str) -> Result<AsciiArtPixel, AsciiStringError> {
        let luma_pixel = self.to_luma_alpha();

        Ok(AsciiArtPixel {
            character: ascii_character(
                luma_pixel[0] as f32 * luma_pixel[1] as f32 / (255.0 * 255.0),
                ascii_string,
            )?,
            r: self[0],
            g: self[1],
            b: self[2],
            a: self[3],
        })
    }
}

impl ToAsciiArtPixel for image::Luma<u8> {
    fn to_ascii_art_pixel(&self, ascii_string: &str) -> Result<AsciiArtPixel, AsciiStringError> {
        Ok(AsciiArtPixel {
            character: ascii_character(self[0] as f32 / 255.0, ascii_string)?,
            r: self[0],
            g: self[0],
            b: self[0],
            a: 255,
        })
    }
}

impl ToAsciiArtPixel for image::LumaA<u8> {
    fn to_ascii_art_pixel(&self, ascii_string: &str) -> Result<AsciiArtPixel, AsciiStringError> {
        Ok(AsciiArtPixel {
            character: ascii_character(
                self[0] as f32 * self[1] as f32 / (255.0 * 255.0),
                ascii_string,
            )?,
            r: self[0],
            g: self[0],
            b: self[0],
            a: self[1],
        })
    }
}

/// Convert lightness of pixel to [`char`]
///
/// # Examples
///
/// ```
/// use tapciify::ascii::ascii_character;
///
/// let ascii_string = " *#";
///
/// let result = ascii_character(1.0, ascii_string)?;
/// assert_eq!(result, '#');
///
/// let result = ascii_character(0.5, ascii_string)?;
/// assert_eq!(result, '*');
///
/// let result = ascii_character(0.0, ascii_string)?;
/// assert_eq!(result, ' ');
/// # Ok::<(), tapciify::AsciiStringError>(())
/// `````
pub fn ascii_character(lightness: f32, ascii_string: &str) -> Result<char, AsciiStringError> {
    ascii_string
        .chars()
        .nth(((ascii_string.chars().count() - 1) as f32 * lightness) as usize)
        .ok_or(AsciiStringError)
}

/// Error caused by lightness being out of ASCII string in [`ascii_character`]
#[derive(Clone, Debug, Error)]
#[error("lightness is out of ASCII string")]
pub struct AsciiStringError;

/// Calculate lightness (from 0.0 to 1.0)
///
/// # Examples
///
/// ```
/// use tapciify::ascii::get_lightness;
///
/// let result = get_lightness(255, 255, 255, 255);
/// assert_eq!(result, 1.0);
///
/// let result = get_lightness(0, 0, 0, 255);
/// assert_eq!(result, 0.0);
///
/// let result = get_lightness(255, 255, 255, 0);
/// assert_eq!(result, 0.0);
///
/// let result = get_lightness(255, 255, 255, 51);
/// assert_eq!(result, 0.2);
/// ````
#[deprecated(since = "3.1.0", note = "Use `image::Pixel::to_luma` instead")]
pub fn get_lightness(r: u8, g: u8, b: u8, a: u8) -> f32 {
    let max = max(max(r, g), b);
    let min = min(min(r, g), b);

    ((max as f32 + min as f32) * a as f32) / 130050.0 // 130050 - we need to divide by 512, and divide by 255 from alpha
}

pub trait ReverseString {
    /// Reverse [`Self`]
    fn reverse(&self) -> Self;
}

impl ReverseString for String {
    fn reverse(&self) -> String {
        self.chars().rev().collect()
    }
}
