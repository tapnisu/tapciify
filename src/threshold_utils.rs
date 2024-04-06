use image::Pixel;

pub const DEFAULT_THRESHOLD: f32 = 0.5;

pub trait ThresholdPixel {
    fn threshold_pixel(&self, threshold: f32) -> bool;
}

impl ThresholdPixel for image::Luma<u8> {
    fn threshold_pixel(&self, threshold: f32) -> bool {
        self[0] as f32 > 255.0 * 255.0 * threshold
    }
}

impl ThresholdPixel for image::LumaA<u8> {
    fn threshold_pixel(&self, threshold: f32) -> bool {
        (self[0] as u16 * self[1] as u16) as f32 > 255.0 * 255.0 * threshold
    }
}

impl ThresholdPixel for image::Rgb<u8> {
    fn threshold_pixel(&self, threshold: f32) -> bool {
        self.to_luma().threshold_pixel(threshold)
    }
}

impl ThresholdPixel for image::Rgba<u8> {
    fn threshold_pixel(&self, threshold: f32) -> bool {
        self.to_luma_alpha().threshold_pixel(threshold)
    }
}
