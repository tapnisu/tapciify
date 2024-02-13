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

#[derive(Clone, Debug)]
pub struct ResizingOptions {
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub font_ratio: f64,
    pub filter: imageops::FilterType,
}

impl Default for ResizingOptions {
    fn default() -> ResizingOptions {
        ResizingOptions {
            width: None,
            height: None,
            font_ratio: DEFAULT_FONT_RATIO,
            filter: imageops::FilterType::Triangle,
        }
    }
}

/// Resize [`DynamicImage`] to your sizes
/// When both `width` and `height` are None, will return the original [`DynamicImage`]
///
/// # Examples
///
/// ```rust
/// use tapciify::resizing::{resize, ResizingOptions};
///
/// let img = image::open("./assets/examples/original.webp").unwrap();
///
/// let result = resize(
///     &img,
///     &ResizingOptions {
///         width: Some(64),
///         ..Default::default()
///     },
/// );
/// ```
pub fn resize(img: &DynamicImage, options: &ResizingOptions) -> DynamicImage {
    if options.width.is_none() && options.height.is_none() {
        return img.clone();
    }

    let nwidth = options.width.unwrap_or_else(|| {
        calc_new_width(
            options.height.unwrap(),
            img.width(),
            img.height(),
            options.font_ratio,
        )
    });

    let nheight = options.height.unwrap_or_else(|| {
        calc_new_height(
            options.width.unwrap(),
            img.width(),
            img.height(),
            options.font_ratio,
        )
    });

    img.resize_exact(nwidth, nheight, options.filter)
}
