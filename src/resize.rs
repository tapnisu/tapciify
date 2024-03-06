//! Utils for resizing your images, but including your font ratio

use image::{imageops, DynamicImage};

/// Consolas font ratio
pub const DEFAULT_FONT_RATIO: f64 = 11.0 / 24.0;

/// Trait for resizing images and counting in font ratio
pub trait CustomRatioResize {
    /// Resize [`DynamicImage`] to your sizes
    /// When both `width` and `height` are [`None`], will return the original [`DynamicImage`]
    ///
    /// # Examples
    ///
    /// ```
    /// use image::imageops::FilterType;
    /// use tapciify::{CustomRatioResize, DEFAULT_FONT_RATIO};
    ///
    /// let img = image::open("./assets/examples/original.webp")?;
    ///
    /// let result = img.resize_custom_ratio(Some(64), None, DEFAULT_FONT_RATIO, FilterType::Triangle);
    /// # Ok::<(), image::ImageError>(())
    /// ```
    fn resize_custom_ratio(
        &self,
        width: Option<u32>,
        height: Option<u32>,
        font_ratio: f64,
        filter: imageops::FilterType,
    ) -> DynamicImage;
}

impl CustomRatioResize for DynamicImage {
    fn resize_custom_ratio(
        &self,
        width: Option<u32>,
        height: Option<u32>,
        font_ratio: f64,
        filter: imageops::FilterType,
    ) -> DynamicImage {
        let (new_width, new_height) = match (width, height) {
            (None, None) => return self.to_owned(),
            (None, Some(height)) => (
                calc_new_width(height, self.width(), self.height(), font_ratio),
                height,
            ),
            (Some(width), None) => (
                width,
                calc_new_height(width, self.width(), self.height(), font_ratio),
            ),
            (Some(width), Some(height)) => (width, height),
        };

        self.resize_exact(new_width, new_height, filter)
    }
}

/// Calculate new width from aspect ratio and new height
pub fn calc_new_width(new_height: u32, width: u32, height: u32, font_ratio: f64) -> u32 {
    ((new_height * width) as f64 / (height as f64 * font_ratio)) as u32
}

/// Calculate new height from aspect ratio and new width
pub fn calc_new_height(new_width: u32, width: u32, height: u32, font_ratio: f64) -> u32 {
    (new_width as f64 * font_ratio * height as f64 / width as f64) as u32
}
