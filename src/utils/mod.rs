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
