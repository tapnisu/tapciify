use std::cmp;

pub fn get_brightness(r: u8, g: u8, b: u8) -> f32 {
    let max = cmp::max(cmp::max(r, g), b);
    let min = cmp::min(cmp::min(r, g), b);

    return (max as f32 + min as f32) / 510f32;
}

pub fn ascii_symbol(brightness: f32, ascii_string: &str) -> char {
    let index = ((ascii_string.chars().count() - 1) as f32 * brightness) as usize;

    ascii_string.chars().nth(index).unwrap()
}

pub fn render_frame(dir: String, width: u32, ascii_string: &str) -> String {
    let original_image = image::open(dir).unwrap().to_rgb8();

    let new_width = width;
    let new_height = (new_width as f64 * (original_image.height() as f64)
        / original_image.width() as f64
        * (11.0f64 / 24.0f64)) as u32;

    let img = image::imageops::resize(
        &original_image,
        new_width,
        new_height,
        image::imageops::FilterType::Triangle,
    );

    let rgb: Vec<u8> = img.into_raw();

    let mut frame = String::new();
    let mut x = 0;

    for i in (0..(rgb.len() - 1)).step_by(3) {
        frame.push(ascii_symbol(
            get_brightness(rgb[i as usize], rgb[i as usize + 1], rgb[i as usize + 2]),
            ascii_string,
        ));

        x += 1;

        if x == new_width {
            frame.push('\n');

            x = 0;
        }
    }

    frame
}
