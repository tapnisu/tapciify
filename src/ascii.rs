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
pub fn ascii_symbol(brightness: f32, ascii_string: &str) -> char {
    ascii_string
        .chars()
        .nth(((ascii_string.chars().count() - 1) as f32 * brightness) as usize)
        .unwrap()
}

#[test]
fn converts_to_ascii() {
    assert_eq!(ascii_symbol(1.0, " *#"), '#');
    assert_eq!(ascii_symbol(0.5, " *#"), '*');
    assert_eq!(ascii_symbol(0.0, " *#"), ' ');
}

/// Calculate height by multiplying width by original aspect ratio
pub fn calc_new_height(new_width: u32, width: u32, height: u32, font_ratio: f64) -> u32 {
    (new_width as f64 * (height as f64) / width as f64 * font_ratio) as u32
}

/// Ascii image conversion result
pub struct AsciiImage {
    pub result: String,
    pub width: u32,
    pub height: u32,
    pub colored: bool,
}

impl AsciiImage {
    pub fn new(result: String, width: u32, height: u32, colored: bool) -> Self {
        Self {
            result,
            width,
            height,
            colored,
        }
    }
}

pub struct AsciiConverter {
    pub img: DynamicImage,
    pub width: u32,
    pub ascii_string: String,
    pub colored: bool,
    pub font_ratio: f64,
}

impl AsciiConverter {
    /// Convert image to text
    #[cfg(not(feature = "parallelism"))]
    pub fn convert(&self) -> AsciiImage {
        let height = calc_new_height(
            self.width,
            self.img.width(),
            self.img.height(),
            self.font_ratio,
        );
        let img_buffer = self
            .img
            .resize_exact(self.width, height, image::imageops::FilterType::Triangle)
            .to_rgba8();
        let chunks = img_buffer.as_raw().chunks(4);

        let ascii = chunks
            .map(|raw| {
                if self.colored {
                    ascii_symbol(
                        get_lightness(raw[0], raw[1], raw[2], raw[3]),
                        &self.ascii_string,
                    )
                    .to_string()
                    .truecolor(raw[0], raw[1], raw[2])
                    .to_string()
                } else {
                    ascii_symbol(
                        get_lightness(raw[0], raw[1], raw[2], raw[3]),
                        &self.ascii_string,
                    )
                    .to_string()
                }
            })
            .collect::<Vec<String>>()
            .chunks(self.width.try_into().unwrap())
            .map(|line| line.join(""))
            .collect::<Vec<String>>()
            .join("\n");

        AsciiImage::new(ascii, self.width, height, self.colored)
    }

    /// Convert image to text
    #[cfg(feature = "parallelism")]
    pub fn convert(&self) -> AsciiImage {
        let height = calc_new_height(
            self.width,
            self.img.width(),
            self.img.height(),
            self.font_ratio,
        );
        let img_buffer = self
            .img
            .resize_exact(self.width, height, image::imageops::FilterType::Triangle)
            .to_rgba8();
        let chunks = img_buffer.as_raw().par_chunks(4);

        let result = chunks
            .map(|raw| {
                if self.colored {
                    ascii_symbol(
                        get_lightness(raw[0], raw[1], raw[2], raw[3]),
                        &self.ascii_string,
                    )
                    .to_string()
                    .truecolor(raw[0], raw[1], raw[2])
                    .to_string()
                } else {
                    ascii_symbol(
                        get_lightness(raw[0], raw[1], raw[2], raw[3]),
                        &self.ascii_string,
                    )
                    .to_string()
                }
            })
            .collect::<Vec<String>>()
            .par_chunks(self.width.try_into().unwrap())
            .map(|line| line.join(""))
            .collect::<Vec<String>>()
            .join("\n");

        AsciiImage::new(result, self.width, height, self.colored)
    }
}

impl Default for AsciiConverter {
    fn default() -> AsciiConverter {
        AsciiConverter {
            img: DynamicImage::new_rgba16(0, 0),
            width: 32,
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

    ascii_converter.convert();
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

    ascii_converter.convert();
}
