use colored::Colorize;
use image::DynamicImage;
use std::{
    cmp::{max, min},
    fmt,
};

#[cfg(feature = "rayon")]
use rayon::prelude::*;

pub const DEFAULT_ASCII_STRING: &str = " .,:;+*?%S#@";
pub const DEFAULT_FONT_RATIO: f64 = 11.0 / 24.0;

/// Calculate lightness (from 0.0 to 1.0)
///
/// # Examples
///
/// ```rust
/// use tapciify::get_lightness;
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

#[derive(Debug, Clone)]
pub struct AsciiStringError;

impl fmt::Display for AsciiStringError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "lightness is out of ASCII string")
    }
}

/// Convert lightness of pixel to ASCII character
///
/// # Examples
///
/// ```rust
/// use tapciify::{ascii_character, AsciiStringError};
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
/// # Ok::<(), AsciiStringError>(())
/// `````
pub fn ascii_character(lightness: f32, ascii_string: &str) -> Result<char, AsciiStringError> {
    ascii_string
        .chars()
        .nth(((ascii_string.chars().count() - 1) as f32 * lightness) as usize)
        .ok_or(AsciiStringError)
}

/// Calculate new width from aspect ratio and new height
pub fn calc_new_width(new_height: u32, width: u32, height: u32, font_ratio: f64) -> u32 {
    (new_height as f64 / (height as f64) * width as f64 / font_ratio) as u32
}

/// Calculate new height from aspect ratio and new width
pub fn calc_new_height(new_width: u32, width: u32, height: u32, font_ratio: f64) -> u32 {
    (new_width as f64 * (height as f64) / width as f64 * font_ratio) as u32
}

/// Ascii character of RawAsciiImage
#[derive(Debug, Clone)]
pub struct AsciiCharacter {
    pub character: char,
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl AsciiCharacter {
    /// Convert RGBA and ASCII string into [`AsciiCharacter`]
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tapciify::{AsciiCharacter, AsciiStringError};
    ///
    /// let ascii_string = " *#";
    ///
    /// let result = AsciiCharacter::new(255, 255, 255, 255, ascii_string)?;
    /// assert_eq!(result.character, '#');
    ///
    /// let result = AsciiCharacter::new(255, 255, 255, 0, ascii_string)?;
    /// assert_eq!(result.character, ' ');
    ///
    /// let result = AsciiCharacter::new(0, 0, 0, 255, ascii_string)?;
    /// assert_eq!(result.character, ' ');
    /// # Ok::<(), AsciiStringError>(())
    /// `````
    pub fn new(
        r: u8,
        g: u8,
        b: u8,
        a: u8,
        ascii_string: &str,
    ) -> Result<AsciiCharacter, AsciiStringError> {
        let lightness = get_lightness(r, g, b, a);

        let character = ascii_character(lightness, ascii_string)?;

        Ok(AsciiCharacter {
            character,
            r,
            g,
            b,
            a,
        })
    }
}

/// Raw Ascii art conversion result
#[derive(Debug, Clone)]
pub struct RawAsciiArt {
    pub characters: Vec<AsciiCharacter>,
    pub width: u32,
    pub height: u32,
    pub colored: bool,
}

impl RawAsciiArt {
    pub fn new(
        characters: Vec<AsciiCharacter>,
        width: u32,
        height: u32,
        colored: bool,
    ) -> RawAsciiArt {
        RawAsciiArt {
            characters,
            width,
            height,
            colored,
        }
    }
}

/// Ascii art conversion result
#[derive(Debug, Clone)]
pub struct AsciiArt {
    pub text: String,
    pub width: u32,
    pub height: u32,
    pub colored: bool,
}

impl AsciiArt {
    pub fn new(text: String, width: u32, height: u32, colored: bool) -> AsciiArt {
        AsciiArt {
            text,
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

#[derive(Debug, Clone)]
pub enum AsciiConverterError {
    AsciiStringError(AsciiStringError),
    SizeError(SizeError),
}

impl From<AsciiStringError> for AsciiConverterError {
    fn from(e: AsciiStringError) -> AsciiConverterError {
        AsciiConverterError::AsciiStringError(e)
    }
}

impl From<SizeError> for AsciiConverterError {
    fn from(e: SizeError) -> AsciiConverterError {
        AsciiConverterError::SizeError(e)
    }
}

impl fmt::Display for AsciiConverterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AsciiConverterError::AsciiStringError(err) => err.fmt(f),
            AsciiConverterError::SizeError(err) => err.fmt(f),
        }
    }
}

/// Converter of images to ASCII art
#[derive(Debug, Clone)]
pub struct AsciiConverter {}

/// Options for converter of images to ASCII art
#[derive(Debug, Clone)]
pub struct AsciiConverterOptions {
    pub width: u32,
    pub height: u32,
    pub ascii_string: String,
    pub colored: bool,
    pub font_ratio: f64,
}

impl AsciiConverter {
    /// Convert image to raw ASCII art
    #[cfg(feature = "rayon")]
    pub fn convert_raw(
        img: &DynamicImage,
        options: &AsciiConverterOptions,
    ) -> Result<RawAsciiArt, AsciiConverterError> {
        if options.width == 0 && options.height == 0 {
            return Err(AsciiConverterError::SizeError(SizeError));
        }

        let width = if options.width == 0 {
            calc_new_width(
                options.height,
                img.width(),
                img.height(),
                options.font_ratio,
            )
        } else {
            options.width
        };

        let height = if options.height == 0 {
            calc_new_height(options.width, img.width(), img.height(), options.font_ratio)
        } else {
            options.height
        };

        let img_buffer = img
            .resize_exact(width, height, image::imageops::FilterType::Triangle)
            .to_rgba8();
        let chunks = img_buffer.as_raw().par_chunks(4);

        let characters = chunks
            .map(|raw| AsciiCharacter::new(raw[0], raw[1], raw[2], raw[3], &options.ascii_string))
            .collect::<Result<Vec<AsciiCharacter>, AsciiStringError>>()?;

        Ok(RawAsciiArt::new(characters, width, height, options.colored))
    }

    /// Convert image to raw ASCII art
    #[cfg(not(feature = "rayon"))]
    pub fn convert_raw(
        img: &DynamicImage,
        options: &AsciiConverterOptions,
    ) -> Result<RawAsciiArt, AsciiConverterError> {
        if options.width == 0 && options.height == 0 {
            return Err(AsciiConverterError::SizeError(SizeError));
        }

        let width = if options.width == 0 {
            calc_new_width(
                options.height,
                img.width(),
                img.height(),
                options.font_ratio,
            )
        } else {
            options.width
        };

        let height = if options.height == 0 {
            calc_new_height(options.width, img.width(), img.height(), options.font_ratio)
        } else {
            options.height
        };

        let img_buffer = img
            .resize_exact(width, height, image::imageops::FilterType::Triangle)
            .to_rgba8();
        let chunks = img_buffer.as_raw().chunks(4);

        let characters = chunks
            .map(|raw| AsciiCharacter::new(raw[0], raw[1], raw[2], raw[3], &options.ascii_string))
            .collect::<Result<Vec<AsciiCharacter>, AsciiStringError>>()?;

        Ok(RawAsciiArt::new(characters, width, height, options.colored))
    }

    /// Convert image to ASCII art
    #[cfg(feature = "rayon")]
    pub fn convert(
        img: &DynamicImage,
        options: &AsciiConverterOptions,
    ) -> Result<AsciiArt, AsciiConverterError> {
        let raw_ascii_art = AsciiConverter::convert_raw(img, options)?;

        let characters = raw_ascii_art
            .characters
            .into_par_iter()
            .map(|ascii_character| {
                if options.colored {
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

        let text = characters
            .par_chunks(raw_ascii_art.width.try_into().unwrap())
            .map(|line| line.join(""))
            .collect::<Vec<String>>()
            .join("\n");

        Ok(AsciiArt::new(
            text,
            raw_ascii_art.width,
            raw_ascii_art.height,
            raw_ascii_art.colored,
        ))
    }

    /// Convert image to ASCII art
    #[cfg(not(feature = "rayon"))]
    pub fn convert(
        img: &DynamicImage,
        options: &AsciiConverterOptions,
    ) -> Result<AsciiArt, AsciiConverterError> {
        let raw_ascii_art = AsciiConverter::convert_raw(img, options)?;

        let characters = raw_ascii_art
            .characters
            .into_iter()
            .map(|ascii_character| {
                if options.colored {
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

        let text = characters
            .chunks(raw_ascii_art.width.try_into().unwrap())
            .map(|line| line.join(""))
            .collect::<Vec<String>>()
            .join("\n");

        Ok(AsciiArt::new(
            text,
            raw_ascii_art.width,
            raw_ascii_art.height,
            raw_ascii_art.colored,
        ))
    }
}

impl Default for AsciiConverterOptions {
    fn default() -> AsciiConverterOptions {
        AsciiConverterOptions {
            width: 0,
            height: 0,
            ascii_string: DEFAULT_ASCII_STRING.to_owned(),
            colored: false,
            font_ratio: DEFAULT_FONT_RATIO,
        }
    }
}

#[test]
fn renders_frame() {
    let path = "./assets/examples/original.webp";
    let img = image::open(path).unwrap();

    let options = AsciiConverterOptions {
        width: 128,
        ..Default::default()
    };

    assert!(
        AsciiConverter::convert_raw(&img, &options).is_ok(),
        "Converting image \"{}\" failed",
        path
    )
}

#[test]
fn renders_colored_frame() {
    let path = "./assets/examples/original.webp";
    let img = image::open(path).unwrap();

    let options = AsciiConverterOptions {
        width: 128,
        colored: true,
        ..Default::default()
    };

    assert!(
        AsciiConverter::convert_raw(&img, &options).is_ok(),
        "Converting image \"{}\" failed",
        path
    )
}
