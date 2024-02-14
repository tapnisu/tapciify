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
use image::DynamicImage;
use std::{
    cmp::{max, min},
    error::Error,
    fmt,
};

#[cfg(feature = "rayon")]
use rayon::prelude::*;

/// Default ASCII string, feel free to use your one
pub const DEFAULT_ASCII_STRING: &str = " .,:;+*?%S#@";

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
pub fn get_lightness(r: u8, g: u8, b: u8, a: u8) -> f32 {
    let max = max(max(r, g), b);
    let min = min(min(r, g), b);

    ((max as f32 + min as f32) * a as f32) / 130050.0 // 130050 - we need to divide by 512, and divide by 255 from alpha
}

/// Error caused by lightness being out of ASCII string in [`ascii_character`]
#[derive(Debug, Clone)]
pub struct AsciiStringError;

impl fmt::Display for AsciiStringError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "lightness is out of ASCII string")
    }
}

impl Error for AsciiStringError {}

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
    pub fn new(
        r: u8,
        g: u8,
        b: u8,
        a: u8,
        ascii_string: &str,
    ) -> Result<AsciiArtPixel, AsciiStringError> {
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
}

/// Raw ASCII art conversion result
#[derive(Debug, Clone)]
pub struct AsciiArt {
    pub characters: Vec<AsciiArtPixel>,
    pub width: u32,
    pub height: u32,
    pub colored: bool,
}

impl fmt::Display for AsciiArt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        #[cfg(feature = "rayon")]
        let iter = self.characters.par_iter();
        #[cfg(not(feature = "rayon"))]
        let iter = self.characters.iter();

        let characters = iter
            .map(|ascii_character| {
                if self.colored {
                    ascii_character
                        .character
                        .to_string()
                        .truecolor(ascii_character.r, ascii_character.g, ascii_character.b)
                        .to_string()
                } else {
                    ascii_character.character.to_string()
                }
            })
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

#[derive(Debug, Clone)]
pub struct SizeError;

impl fmt::Display for SizeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "width and height can't both be 0")
    }
}

/// Error caused by [`AsciiArtConverter`]
#[derive(Debug, Clone)]
pub enum AsciiArtConverterError {
    AsciiStringError(AsciiStringError),
}

impl From<AsciiStringError> for AsciiArtConverterError {
    fn from(e: AsciiStringError) -> AsciiArtConverterError {
        AsciiArtConverterError::AsciiStringError(e)
    }
}

impl Error for AsciiArtConverterError {}

impl fmt::Display for AsciiArtConverterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AsciiArtConverterError::AsciiStringError(err) => err.fmt(f),
        }
    }
}

/// Options for converter of images to ASCII art
#[derive(Debug, Clone)]
pub struct AsciiArtConverterOptions {
    pub ascii_string: String,
    pub colored: bool,
}

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
        let img_buffer = self.to_rgba8();

        #[cfg(feature = "rayon")]
        let chunks = img_buffer.as_raw().par_chunks(4);
        #[cfg(not(feature = "rayon"))]
        let chunks = img_buffer.as_raw().chunks(4);

        let characters = chunks
            .map(|rgba| {
                AsciiArtPixel::new(rgba[0], rgba[1], rgba[2], rgba[3], &options.ascii_string)
            })
            .collect::<Result<Vec<AsciiArtPixel>, AsciiStringError>>()?;

        Ok(AsciiArt::new(
            characters,
            self.width(),
            self.height(),
            options.colored,
        ))
    }
}

impl Default for AsciiArtConverterOptions {
    fn default() -> AsciiArtConverterOptions {
        AsciiArtConverterOptions {
            ascii_string: DEFAULT_ASCII_STRING.to_owned(),
            colored: false,
        }
    }
}
