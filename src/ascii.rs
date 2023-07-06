use colored::Colorize;
use image::DynamicImage;
use rayon::prelude::*;
use std::cmp::{max, min};

pub const DEFAULT_ASCII_STRING: &str = " .,:;+*?%S#@";
pub const DEFAULT_FONT_RATIO: f64 = 11.0 / 24.0;

/// Calculate lightness (from 0.0 to 1.0)
pub fn get_lightness(r: u8, g: u8, b: u8, a: u8) -> f32 {
    let max = max(max(r, g), b);
    let min = min(min(r, g), b);

    ((max as f32 + min as f32) * a as f32) / 130050f32 // 130050 - we need to divide by 512, and divide by 255 from alpha
}

/// Convert lightness of pixel to symbol
pub fn ascii_symbol(brightness: f32, ascii_string: &str) -> char {
    ascii_string
        .chars()
        .nth(((ascii_string.chars().count() - 1) as f32 * brightness) as usize)
        .unwrap()
}

/// Calculate height by multiplying width by original aspect ratio
pub fn calc_new_height(new_width: u32, width: u32, height: u32, font_ratio: f64) -> u32 {
    (new_width as f64 * (height as f64) / width as f64 * font_ratio) as u32
}

/// Converts image to symbols
pub fn render_frame(img_raw: Vec<u8>, width: u32, ascii_string: &str) -> String {
    let mut result = String::new();
    let mut x = 0;

    for i in (0..(img_raw.len() - 1)).step_by(4) {
        result.push(ascii_symbol(
            get_lightness(img_raw[i], img_raw[i + 1], img_raw[i + 2], img_raw[i + 3]),
            ascii_string,
        ));

        x += 1;

        if x == width {
            result.push('\n');

            x = 0;
        }
    }

    result
}

/// Converts image to symbols and adds colors
pub fn render_colored_frame(img_raw: Vec<u8>, width: u32, ascii_string: &str) -> String {
    let mut result = String::new();
    let mut x = 0;

    for i in (0..(img_raw.len() - 1)).step_by(4) {
        result.push_str(
            &ascii_symbol(
                get_lightness(img_raw[i], img_raw[i + 1], img_raw[i + 2], img_raw[i + 3]),
                ascii_string,
            )
            .to_string()
            .truecolor(img_raw[i], img_raw[i + 1], img_raw[i + 2]),
        );

        x += 1;

        if x == width {
            result.push('\n');

            x = 0;
        }
    }

    result
}

/// Run one of 2 functions depending on arguments
#[cfg(not(feature = "parallelism"))]
pub fn image_to_ascii(
    img: DynamicImage,
    width: u32,
    ascii_string: &str,
    colored: bool,
    font_ratio: f64,
) -> (String, u32) {
    let height = calc_new_height(width, img.width(), img.height(), font_ratio);
    let img_raw = img
        .resize_exact(width, height, image::imageops::FilterType::Triangle)
        .to_rgba8()
        .as_raw()
        .to_owned();

    if colored {
        (render_colored_frame(img_raw, width, ascii_string), height)
    } else {
        (render_frame(img_raw, width, ascii_string), height)
    }
}

/// Converts image to text
#[cfg(feature = "parallelism")]
pub fn image_to_ascii(
    img: DynamicImage,
    width: u32,
    ascii_string: &str,
    colored: bool,
    font_ratio: f64,
) -> (String, u32) {
    let height = calc_new_height(width, img.width(), img.height(), font_ratio);
    let img_buffer = img
        .resize_exact(width, height, image::imageops::FilterType::Triangle)
        .to_rgba8();
    let chunks = img_buffer.as_raw().par_chunks(4);

    let ascii = chunks
        .map(|raw| {
            if colored {
                ascii_symbol(get_lightness(raw[0], raw[1], raw[2], raw[3]), ascii_string)
                    .to_string()
                    .truecolor(raw[0], raw[1], raw[2])
                    .to_string()
            } else {
                ascii_symbol(get_lightness(raw[0], raw[1], raw[2], raw[3]), ascii_string)
                    .to_string()
            }
        })
        .collect::<Vec<String>>()
        .par_chunks(width.try_into().unwrap())
        .map(|line| line.join(""))
        .collect::<Vec<String>>()
        .join("\n");

    (ascii, height)
}

#[test]
fn renders_frame() {
    let img = image::open("./assets/logo.png").unwrap();

    image_to_ascii(img, 128, DEFAULT_ASCII_STRING, false, DEFAULT_FONT_RATIO);
}

#[test]
fn renders_colored_frame() {
    let img = image::open("./assets/logo.png").unwrap();

    image_to_ascii(img, 128, DEFAULT_ASCII_STRING, true, DEFAULT_FONT_RATIO);
}
