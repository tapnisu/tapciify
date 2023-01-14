use colored::Colorize;
use image::RgbaImage;
use std::cmp::{max, min};

// Get brightness of pixel from 0.0 to 1.0 (calculated by HSL's lightness formula)
pub fn get_lightness(r: u8, g: u8, b: u8, a: u8) -> f32 {
    let max = max(max(r, g), b);
    let min = min(min(r, g), b);

    (max as f32 + min as f32) * a as f32 / 255f32 / 510f32
}

// Convert lightness of pixel to symbol
pub fn ascii_symbol(brightness: f32, ascii_string: &str) -> char {
    let index = ((ascii_string.chars().count() - 1) as f32 * brightness) as usize;

    ascii_string.chars().nth(index).unwrap()
}

// Calculate height by multiplying width by original aspect ratio
pub fn calc_new_height(new_width: u32, width: u32, height: u32) -> u32 {
    (new_width as f64 * (height as f64) / width as f64 * (11.0f64 / 24.0f64)) as u32
}

// Converts image to symbols
pub fn render_frame(image: RgbaImage, width: u32, height: u32, ascii_string: &str) -> String {
    // Resize the image, so the terminal can show it
    let img = image::imageops::resize(&image, width, height, image::imageops::FilterType::Triangle);
    let rgb: Vec<u8> = img.into_raw();

    let mut frame = String::new();

    // Counter for the end of the pixels row
    let mut x = 0;

    for i in (0..(rgb.len() - 1)).step_by(4) {
        frame.push(ascii_symbol(
            get_lightness(
                rgb[i as usize],
                rgb[i as usize + 1],
                rgb[i as usize + 2],
                rgb[i as usize + 3],
            ),
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

// Converts image to symbols and adds colors
pub fn render_colored_frame(
    image: RgbaImage,
    width: u32,
    height: u32,
    ascii_string: &str,
) -> String {
    let img = image::imageops::resize(&image, width, height, image::imageops::FilterType::Triangle);

    let rgb: Vec<u8> = img.into_raw();

    let mut x = 0;
    let mut result: String = "".to_string();

    for i in (0..(rgb.len() - 1)).step_by(4) {
        result = format!(
            "{}{}",
            result,
            ascii_symbol(
                get_lightness(
                    rgb[i as usize],
                    rgb[i as usize + 1],
                    rgb[i as usize + 2],
                    rgb[i as usize + 3]
                ),
                ascii_string,
            )
            .to_string()
            .truecolor(rgb[i as usize], rgb[i as usize + 1], rgb[i as usize + 2])
        );

        x += 1;

        if x == width {
            result = format!("{}{}", result, "\n");

            x = 0;
        }
    }

    result
}

// Run one of 2 functions depending on arguments
pub fn render_frame_case(
    image: RgbaImage,
    width: u32,
    ascii_string: &str,
    colored: bool,
) -> String {
    if colored {
        render_colored_frame(
            image.clone(),
            width,
            calc_new_height(width, image.width(), image.height()),
            &ascii_string,
        )
    } else {
        render_frame(
            image.clone(),
            width,
            calc_new_height(width, image.width(), image.height()),
            &ascii_string,
        )
    }
}
