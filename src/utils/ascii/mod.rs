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

/// Calculate height by multiplying width by original aspect ratio
pub fn calc_new_height(new_width: u32, width: u32, height: u32) -> u32 {
    (new_width as f64 * (height as f64) / width as f64 * (11.0f64 / 24.0f64)) as u32
}

/// Converts image to symbols
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
pub fn render_colored_frame(img_raw: Vec<u8>, width: u32, ascii_string: &str) -> String {
    let mut x = 0;
    let mut result: String = "".to_string();

    for i in (0..(img_raw.len() - 1)).step_by(4) {
        result.push_str(
            &ascii_symbol(
                get_lightness(img_raw[i], img_raw[i + 1], img_raw[i + 2], img_raw[i + 3]),
                ascii_string,
            )
            .to_string()
            .truecolor(img_raw[i], img_raw[i + 1], img_raw[i + 2])
            .to_string(),
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

/// Cut image into parts by amount
#[deprecated(since = "1.1.0", note = "please use `split_image_by_height` instead")]
pub fn cut_image_by_amount(mut img: DynamicImage, amount: u32) -> Vec<DynamicImage> {
    let mut parts: Vec<DynamicImage> = vec![];

    for i in 0..amount {
        let part_height = img.height() / amount;

        parts.push(img.crop(0, part_height * i, img.width(), part_height));
    }

    parts
}

/// Splits image by its height
pub fn split_image_by_height(mut img: DynamicImage) -> Vec<DynamicImage> {
    let mut parts: Vec<DynamicImage> = vec![];

    for i in 0..img.height() {
        parts.push(img.crop(0, i, img.width(), 1));
    }

    parts
}

/// Render image by parts, and return String
pub fn par_render_frame(
    img: DynamicImage,
    width: u32,
    ascii_string: String,
    colored: bool,
) -> (String, u32) {
    // Resize image
    let height = calc_new_height(width, img.width(), img.height());
    let img = resize_image(img, width, height);

    // Split into parts, and render them
    let outputs: Vec<String> = split_image_by_height(img.clone())
        .par_iter()
        .map(|part| {
            render_frame_case(
                part.to_rgba8().as_raw().to_vec(),
                width,
                ascii_string.clone(),
                colored,
            )
        })
        .collect();

    (outputs.join(""), height)
}

/// Resize image using triangle filter
pub fn resize_image(img: DynamicImage, new_width: u32, new_height: u32) -> DynamicImage {
    image::DynamicImage::ImageRgba8(image::imageops::resize(
        &img,
        new_width,
        new_height,
        image::imageops::FilterType::Triangle,
    ))
}
