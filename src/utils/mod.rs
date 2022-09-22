pub fn get_brightness(r: u8, g: u8, b: u8) -> f32 {
    ((0.21 * r as f32) + (0.72 * g as f32) + (0.07 * b as f32)) / 255f32
}

pub fn ascii_symbol(brightness: f32) -> char {
    let ascii_string = " .,:;+*?%S#@";
    let index = (ascii_string.chars().count() as f32 * brightness) as usize;

    ascii_string.chars().nth(index).unwrap()
}
