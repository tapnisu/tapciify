use colored::Colorize;
use image::{DynamicImage, RgbaImage};
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
pub fn render_frame(img: RgbaImage, width: u32, ascii_string: &str) -> String {
    // Resize the image, so the terminal can show it
    let rgb: Vec<u8> = img.into_raw();

    let mut frame = String::new();

    // Counter for the end of the pixels row
    let mut x = 0;

    for i in (0..(rgb.len() - 1)).step_by(4) {
        frame.push(ascii_symbol(
            get_lightness(rgb[i], rgb[i + 1], rgb[i + 2], rgb[i + 3]),
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
pub fn render_colored_frame(img: RgbaImage, width: u32, ascii_string: &str) -> String {
    let rgb: Vec<u8> = img.into_raw();

    let mut x = 0;
    let mut result: String = "".to_string();

    for i in (0..(rgb.len() - 1)).step_by(4) {
        result = format!(
            "{}{}",
            result,
            ascii_symbol(
                get_lightness(rgb[i], rgb[i + 1], rgb[i + 2], rgb[i + 3]),
                ascii_string,
            )
            .to_string()
            .truecolor(rgb[i], rgb[i + 1], rgb[i + 2])
        );

        x += 1;

        if x == width {
            result = format!("{}\n", result);

            x = 0;
        }
    }

    result
}

/// Run one of 2 functions depending on arguments
pub fn render_frame_case(
    image: RgbaImage,
    width: u32,
    ascii_string: String,
    colored: bool,
) -> String {
    if colored {
        render_colored_frame(image.clone(), width, &ascii_string)
    } else {
        render_frame(image.clone(), width, &ascii_string)
    }
}

/// Cut image into parts by amount
pub fn cut_image_by_amount(image: DynamicImage, amount: u32) -> Vec<DynamicImage> {
    let mut parts: Vec<DynamicImage> = vec![];

    for i in 0..amount {
        let part_height = image.height() / amount;

        parts.push(
            image
                .clone()
                .crop(0, part_height * i, image.width(), part_height),
        );
    }

    parts
}

/// Render image by parts, and return String
pub fn render_full_frame(
    img: DynamicImage,
    width: u32,
    ascii_string: String,
    colored: bool,
) -> (String, u32) {
    // Resize image
    let height = calc_new_height(width, img.width(), img.height());
    let img = resize_image(img, width, height);

    // Split into parts, and render them
    let image_parts = cut_image_by_amount(img.clone(), img.height());

    let outputs: Vec<String> = image_parts
        .par_iter()
        .map(|part| render_frame_case(part.to_rgba8(), width, ascii_string.clone(), colored))
        .collect();

    (outputs.join(""), height)
}

/// Resize image
pub fn resize_image(img: DynamicImage, new_width: u32, new_height: u32) -> DynamicImage {
    image::DynamicImage::ImageRgba8(image::imageops::resize(
        &img,
        new_width,
        new_height,
        image::imageops::FilterType::Lanczos3,
    ))
}
