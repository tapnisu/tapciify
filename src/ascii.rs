use colored::Colorize;
use image::DynamicImage;
use std::cmp::{max, min};

#[cfg(feature = "parallelism")]
use rayon::prelude::*;

pub const DEFAULT_ASCII_STRING: &str = " .,:;+*?%S#@";
pub const DEFAULT_FONT_RATIO: f64 = 11.0 / 24.0;

/// Calculate lightness (from 0.0 to 1.0)
pub fn get_lightness(r: u8, g: u8, b: u8, a: u8) -> f32 {
    let max = max(max(r, g), b);
    let min = min(min(r, g), b);

    ((max as f32 + min as f32) * a as f32) / 130050.0 // 130050 - we need to divide by 512, and divide by 255 from alpha
}

#[test]
fn calculates_lightness() {
    assert_eq!(get_lightness(255, 255, 255, 255), 1.0);
    assert_eq!(get_lightness(0, 0, 0, 255), 0.0);
    assert_eq!(get_lightness(255, 255, 255, 0), 0.0);
    assert_eq!(get_lightness(255, 255, 255, 51), 0.2);
}

/// Convert lightness of pixel to symbol
pub fn ascii_character(lightness: f32, ascii_string: &str) -> char {
    ascii_string
        .chars()
        .nth(((ascii_string.chars().count() - 1) as f32 * lightness) as usize)
        .unwrap()
}

#[test]
fn converts_to_ascii() {
    let ascii_string = " *#";

    assert_eq!(ascii_character(1.0, ascii_string), '#');
    assert_eq!(ascii_character(0.5, ascii_string), '*');
    assert_eq!(ascii_character(0.0, ascii_string), ' ');
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
pub struct AsciiCharacter {
    pub character: char,
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl AsciiCharacter {
    pub fn new(r: u8, g: u8, b: u8, a: u8, ascii_string: &str) -> AsciiCharacter {
        let lightness = get_lightness(r, g, b, a);

        let character = ascii_character(lightness, ascii_string);

        AsciiCharacter {
            character,
            r,
            g,
            b,
            a,
        }
    }
}

#[test]
fn converts_to_ascii_character() {
    let ascii_string = " *#";

    assert_eq!(
        AsciiCharacter::new(255, 255, 255, 255, ascii_string).character,
        '#'
    );
    assert_eq!(
        AsciiCharacter::new(255, 255, 255, 0, ascii_string).character,
        ' '
    );
    assert_eq!(
        AsciiCharacter::new(0, 0, 0, 255, ascii_string).character,
        ' '
    );
}

/// Raw Ascii art conversion result
pub struct RawAsciiArt {
    pub characters: Vec<AsciiCharacter>,
    pub width: u32,
    pub height: u32,
    pub colored: bool,
}

impl RawAsciiArt {
    pub fn new(characters: Vec<AsciiCharacter>, width: u32, height: u32, colored: bool) -> Self {
        Self {
            characters,
            width,
            height,
            colored,
        }
    }
}

/// Ascii art conversion result
pub struct AsciiRaw {
    pub text: String,
    pub width: u32,
    pub height: u32,
    pub colored: bool,
}

impl AsciiRaw {
    pub fn new(text: String, width: u32, height: u32, colored: bool) -> Self {
        Self {
            text,
            width,
            height,
            colored,
        }
    }
}

/// Converter of images to ASCII art
pub struct AsciiConverter {
    pub img: DynamicImage,
    pub width: u32,
    pub height: u32,
    pub ascii_string: String,
    pub colored: bool,
    pub font_ratio: f64,
}

impl AsciiConverter {
    /// Convert image to raw ASCII art
    #[cfg(feature = "parallelism")]
    pub fn convert_raw(&self) -> RawAsciiArt {
        let width = if self.width == 0 {
            calc_new_width(
                self.height,
                self.img.width(),
                self.img.height(),
                self.font_ratio,
            )
        } else {
            self.width
        };

        let height = if self.height == 0 {
            calc_new_height(
                self.width,
                self.img.width(),
                self.img.height(),
                self.font_ratio,
            )
        } else {
            self.height
        };

        let img_buffer = self
            .img
            .resize_exact(width, height, image::imageops::FilterType::Triangle)
            .to_rgba8();
        let chunks = img_buffer.as_raw().par_chunks(4);

        let characters = chunks
            .map(|raw| AsciiCharacter::new(raw[0], raw[1], raw[2], raw[3], &self.ascii_string))
            .collect::<Vec<AsciiCharacter>>();

        RawAsciiArt::new(characters, width, height, self.colored)
    }

    /// Convert image to raw ASCII art
    #[cfg(not(feature = "parallelism"))]
    pub fn convert_raw(&self) -> RawAsciiArt {
        let width = if self.width == 0 {
            calc_new_width(
                self.height,
                self.img.width(),
                self.img.height(),
                self.font_ratio,
            )
        } else {
            self.width
        };

        let height = if self.height == 0 {
            calc_new_height(
                self.width,
                self.img.width(),
                self.img.height(),
                self.font_ratio,
            )
        } else {
            self.height
        };

        let img_buffer = self
            .img
            .resize_exact(width, height, image::imageops::FilterType::Triangle)
            .to_rgba8();
        let chunks = img_buffer.as_raw().chunks(4);

        let characters = chunks
            .map(|raw| AsciiCharacter::new(raw[0], raw[1], raw[2], raw[3], &self.ascii_string))
            .collect::<Vec<AsciiCharacter>>();

        RawAsciiArt::new(characters, width, height, self.colored)
    }

    /// Convert image to ASCII art
    #[cfg(feature = "parallelism")]
    pub fn convert(self) -> AsciiRaw {
        let raw_ascii_art = AsciiConverter::convert_raw(&self);

        let characters = raw_ascii_art
            .characters
            .par_iter()
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

        let text = characters
            .par_chunks(raw_ascii_art.width.try_into().unwrap())
            .map(|line| line.join(""))
            .collect::<Vec<String>>()
            .join("\n");

        AsciiRaw::new(
            text,
            raw_ascii_art.width,
            raw_ascii_art.height,
            raw_ascii_art.colored,
        )
    }

    /// Convert image to ASCII art
    #[cfg(not(feature = "parallelism"))]
    pub fn convert(self) -> AsciiRaw {
        let raw_ascii_art = AsciiConverter::convert_raw(&self);

        let characters = raw_ascii_art
            .characters
            .iter()
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

        let text = characters
            .chunks(raw_ascii_art.width.try_into().unwrap())
            .map(|line| line.join(""))
            .collect::<Vec<String>>()
            .join("\n");

        AsciiRaw::new(
            text,
            raw_ascii_art.width,
            raw_ascii_art.height,
            raw_ascii_art.colored,
        )
    }
}

impl Default for AsciiConverter {
    fn default() -> AsciiConverter {
        AsciiConverter {
            img: DynamicImage::new_rgba16(0, 0),
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
    let img = image::open("./assets/logo.png").unwrap();

    let ascii_converter = AsciiConverter {
        img,
        width: 128,
        ..Default::default()
    };

    ascii_converter.convert_raw();
}

#[test]
fn renders_colored_frame() {
    let img = image::open("./assets/logo.png").unwrap();

    let ascii_converter = AsciiConverter {
        img,
        width: 128,
        colored: true,
        ..Default::default()
    };

    ascii_converter.convert_raw();
}
