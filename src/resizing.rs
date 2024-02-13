use image::{imageops, DynamicImage};

pub const DEFAULT_FONT_RATIO: f64 = 11.0 / 24.0;

/// Calculate new width from aspect ratio and new height
pub fn calc_new_width(new_height: u32, width: u32, height: u32, font_ratio: f64) -> u32 {
    (new_height as f64 / (height as f64) * width as f64 / font_ratio) as u32
}

/// Calculate new height from aspect ratio and new width
pub fn calc_new_height(new_width: u32, width: u32, height: u32, font_ratio: f64) -> u32 {
    (new_width as f64 * (height as f64) / width as f64 * font_ratio) as u32
}

pub trait CustomRatioResize {
    fn resize_custom_ratio(
        &self,
        width: Option<u32>,
        height: Option<u32>,
        font_ratio: f64,
        filter: imageops::FilterType,
    ) -> DynamicImage;
}

impl CustomRatioResize for DynamicImage {
    /// Resize [`DynamicImage`] to your sizes
    /// When both `width` and `height` are [`None`], will return the original [`DynamicImage`]
    ///
    /// # Examples
    ///
    /// ```rust
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
    ) -> DynamicImage {
        if width.is_none() && height.is_none() {
            return self.clone();
        }

        let nwidth = width.unwrap_or_else(|| {
            calc_new_width(height.unwrap(), self.width(), self.height(), font_ratio)
        });

        let nheight = height.unwrap_or_else(|| {
            calc_new_height(width.unwrap(), self.width(), self.height(), font_ratio)
        });

        self.resize_exact(nwidth, nheight, filter)
    }
}
