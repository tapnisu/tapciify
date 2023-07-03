use colored::Colorize;
use image::DynamicImage;
use rayon::prelude::*;
use std::cmp::{max, min};

/// Get brightness of pixel from 0.0 to 1.0 (calculated by HSL's lightness formula)
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

pub const FONT_RATIO: f64 = 11.0 / 24.0;

/// Calculate height by multiplying width by original aspect ratio
pub fn calc_new_height(new_width: u32, width: u32, height: u32) -> u32 {
    (new_width as f64 * (height as f64) / width as f64 * FONT_RATIO) as u32
}

/// Converts image to symbols
#[deprecated(since = "1.2.0", note = "please use `par_render_frame` instead")]
pub fn render_frame(img_raw: Vec<u8>, width: u32, ascii_string: &str) -> String {
    let mut frame = String::new();

    // Counter for the end of the pixels row
    let mut x = 0;

    for i in (0..(img_raw.len() - 1)).step_by(4) {
        frame.push(ascii_symbol(
            get_lightness(img_raw[i], img_raw[i + 1], img_raw[i + 2], img_raw[i + 3]),
            ascii_string,
        ));

        x += 1;

        if x == width {
            frame.push('\n');

            x = 0;
        }
    }

    frame
}

/// Converts image to symbols and adds colors
#[deprecated(since = "1.2.0", note = "please use `par_render_frame` instead")]
pub fn render_colored_frame(raw: Vec<u8>, width: u32, ascii_string: &str) -> String {
    let mut x = 0;
    let mut result: String = "".to_string();

    for i in (0..(raw.len() - 1)).step_by(4) {
        result.push_str(
            &ascii_symbol(
                get_lightness(raw[i], raw[i + 1], raw[i + 2], raw[i + 3]),
                ascii_string,
            )
            .to_string()
            .truecolor(raw[i], raw[i + 1], raw[i + 2]),
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
#[deprecated(since = "1.2.0", note = "please use `par_render_frame` instead")]
#[allow(deprecated)]
pub fn render_frame_case(
    img_raw: Vec<u8>,
    width: u32,
    ascii_string: String,
    colored: bool,
) -> String {
    if colored {
        render_colored_frame(img_raw, width, &ascii_string)
    } else {
        render_frame(img_raw, width, &ascii_string)
    }
}

/// Render image by parts, and return String
pub fn par_render_frame(
    img: DynamicImage,
    width: u32,
    ascii_string: String,
    colored: bool,
) -> (String, u32) {
    let height = calc_new_height(width, img.width(), img.height());
    let rgba = img
        .resize(width, height, image::imageops::FilterType::Triangle)
        .to_rgba8();
    let chunks = rgba.as_raw().par_chunks(4);

    let ascii = chunks
        .map(|raw| {
            if colored {
                ascii_symbol(get_lightness(raw[0], raw[1], raw[2], raw[3]), &ascii_string)
                    .to_string()
                    .truecolor(raw[0], raw[1], raw[2])
                    .to_string()
            } else {
                ascii_symbol(get_lightness(raw[0], raw[1], raw[2], raw[3]), &ascii_string)
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

/// Resize image using triangle filter
#[deprecated(since = "1.2.0")]
pub fn resize_image(img: DynamicImage, new_width: u32, new_height: u32) -> DynamicImage {
    image::DynamicImage::ImageRgba8(image::imageops::resize(
        &img,
        new_width,
        new_height,
        image::imageops::FilterType::Triangle,
    ))
}
