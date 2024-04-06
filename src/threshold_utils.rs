use image::Pixel;

/// 1/2 threshold
pub const DEFAULT_THRESHOLD: f32 = 0.5;

/// Checks if pixel is light enough
pub trait ThresholdPixel {
    /// Calculates special lightness and checks if it is higher than certain threshold
    ///
    /// # Examples
    ///
    /// ```
    /// use tapciify::threshold_utils::ThresholdPixel;
    ///
    /// # fn main() {
    /// let pixel = image::Luma::<u8>([0]);
    /// assert!(!pixel.threshold_pixel(0.5));
    ///
    /// let pixel = image::Luma::<u8>([255]);
    /// assert!(pixel.threshold_pixel(0.5));
    /// # }
    /// ```
    fn threshold_pixel(&self, threshold: f32) -> bool;
}

impl ThresholdPixel for image::Luma<u8> {
    fn threshold_pixel(&self, threshold: f32) -> bool {
        self[0] as f32 > 255.0 * threshold
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
