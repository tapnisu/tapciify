use crate::{
    ascii::{
        AsciiArt, AsciiConverter, AsciiConverterError, AsciiConverterOptions, AsciiStringError,
        SizeError, DEFAULT_ASCII_STRING,
    },
    image_resizing::{resize, ImageResizingOptions, DEFAULT_FONT_RATIO},
};
use crossterm::{cursor::MoveUp, execute};
use image::ImageError;
use indicatif::ProgressBar;
use std::{fmt, io::stdout, time::Instant};

#[cfg(feature = "rayon")]
use rayon::prelude::*;

/// Calculate frame time in millis (1 / framerate)
///
/// # Examples
///
/// ```rust
/// use tapciify::calculate_frame_time;
///
/// let result = calculate_frame_time(Some(20.0));
///
/// assert_eq!(result, 50)
/// ```
pub fn calculate_frame_time(framerate: Option<f64>) -> u64 {
    framerate.map_or(0, |framerate| (1000f64 / framerate) as u64)
}

/// Player to convert and play frames
#[derive(Debug, Clone)]
pub struct AsciiPlayer {}

/// Options of player to convert and play frames
#[derive(Debug, Clone)]
pub struct AsciiPlayerOptions {
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub ascii_string: String,
    pub colored: bool,
    pub frame_time: u64,
    pub pre_render: bool,
    pub font_ratio: f64,
    pub looped: bool,
}

impl Default for AsciiPlayerOptions {
    fn default() -> AsciiPlayerOptions {
        AsciiPlayerOptions {
            width: Some(0),
            height: Some(0),
            ascii_string: DEFAULT_ASCII_STRING.to_owned(),
            colored: false,
            frame_time: 0,
            pre_render: false,
            font_ratio: DEFAULT_FONT_RATIO,
            looped: false,
        }
    }
}

impl From<AsciiPlayerOptions> for AsciiConverterOptions {
    fn from(o: AsciiPlayerOptions) -> AsciiConverterOptions {
        AsciiConverterOptions {
            ascii_string: o.ascii_string,
            colored: o.colored,
        }
    }
}

impl From<AsciiPlayerOptions> for ImageResizingOptions {
    fn from(o: AsciiPlayerOptions) -> ImageResizingOptions {
        ImageResizingOptions {
            width: o.width,
            height: o.height,
            font_ratio: o.font_ratio,
            ..Default::default()
        }
    }
}

#[derive(Debug)]
pub enum AsciiPlayerError {
    Image(ImageError),

    AsciiConverter(AsciiConverterError),

    AsciiString(AsciiStringError),
    Size(SizeError),
}

impl From<ImageError> for AsciiPlayerError {
    fn from(e: ImageError) -> AsciiPlayerError {
        AsciiPlayerError::Image(e)
    }
}

impl From<AsciiConverterError> for AsciiPlayerError {
    fn from(e: AsciiConverterError) -> AsciiPlayerError {
        AsciiPlayerError::AsciiConverter(e)
    }
}

impl From<AsciiStringError> for AsciiPlayerError {
    fn from(e: AsciiStringError) -> AsciiPlayerError {
        AsciiPlayerError::AsciiString(e)
    }
}

impl From<SizeError> for AsciiPlayerError {
    fn from(e: SizeError) -> AsciiPlayerError {
        AsciiPlayerError::Size(e)
    }
}

impl fmt::Display for AsciiPlayerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AsciiPlayerError::Image(err) => err.fmt(f),

            AsciiPlayerError::AsciiConverter(err) => err.fmt(f),

            AsciiPlayerError::AsciiString(err) => err.fmt(f),
            AsciiPlayerError::Size(err) => err.fmt(f),
        }
    }
}

impl AsciiPlayer {
    /// Reverse ASCII string
    pub fn reverse_ascii_string(ascii_string: String) -> String {
        ascii_string.chars().rev().collect()
    }

    /// Play paths as ASCII arts
    pub fn play_frames(
        images_paths: &[String],
        options: &AsciiPlayerOptions,
    ) -> Result<(), AsciiPlayerError> {
        let mut first_frame = true;

        let converter_options = AsciiConverterOptions::from(options.clone());

        loop {
            for image_path in images_paths.iter() {
                let start = Instant::now();

                let img = resize(
                    &image::open(image_path)?,
                    &ImageResizingOptions {
                        width: options.width,
                        height: options.height,
                        font_ratio: options.font_ratio,
                        ..Default::default()
                    },
                );
                let ascii_image = AsciiConverter::convert(&img, &converter_options)?;

                if !first_frame {
                    execute!(stdout(), MoveUp((ascii_image.height).try_into().unwrap()))
                        .unwrap_or_default();
                } else {
                    first_frame = false;
                }

                println!("{}", ascii_image.text);

                while options.frame_time > start.elapsed().as_millis().try_into().unwrap() {}
            }

            if !options.looped {
                break;
            }
        }

        Ok(())
    }

    /// Convert paths to of ASCII arts
    fn pre_render(
        images_paths: &[String],
        options: &AsciiPlayerOptions,
    ) -> Result<Vec<AsciiArt>, AsciiPlayerError> {
        let pb = ProgressBar::new(images_paths.len().try_into().unwrap());

        let converter_options = AsciiConverterOptions::from(options.clone());

        #[cfg(feature = "rayon")]
        let iter = images_paths.into_par_iter();
        #[cfg(not(feature = "rayon"))]
        let iter = images_paths.into_iter();

        let frames = iter
            .map(|path| {
                let img = resize(
                    &image::open(path)?,
                    &ImageResizingOptions {
                        width: options.width,
                        height: options.height,
                        font_ratio: options.font_ratio,
                        ..Default::default()
                    },
                );
                let ascii_image = AsciiConverter::convert(&img, &converter_options)?;

                pb.inc(1);

                Ok(ascii_image)
            })
            .collect::<Result<Vec<AsciiArt>, AsciiPlayerError>>()?;

        Ok(frames)
    }

    /// Convert paths to of ASCII arts and play them
    pub fn play_pre_rendered_frames(
        images_paths: &[String],
        options: &AsciiPlayerOptions,
    ) -> Result<(), AsciiPlayerError> {
        let mut first_frame = true;

        let frames = AsciiPlayer::pre_render(images_paths, options)?;

        loop {
            frames.iter().for_each(|ascii_image| {
                let start = Instant::now();

                if !first_frame {
                    execute!(stdout(), MoveUp((ascii_image.height).try_into().unwrap()))
                        .unwrap_or_default();
                } else {
                    first_frame = false;
                }

                println!("{}", ascii_image.text);

                while options.frame_time > start.elapsed().as_millis().try_into().unwrap() {}
            });

            if !options.looped {
                break;
            }
        }

        Ok(())
    }

    /// Play frames
    ///
    /// Calls [`AsciiPlayer::play_frames`] or [`AsciiPlayer::play_pre_rendered_frames`], depending on [`AsciiPlayerOptions`]
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tapciify::{AsciiPlayer, AsciiPlayerOptions};
    ///
    /// let paths = vec!["./assets/examples/original.webp".to_owned()];
    ///
    /// let options = AsciiPlayerOptions {
    ///     width: Some(128),
    ///     ..Default::default()
    /// };
    ///
    /// assert!(AsciiPlayer::play(&paths, &options).is_ok())
    /// ```
    pub fn play(
        images_paths: &[String],
        options: &AsciiPlayerOptions,
    ) -> Result<(), AsciiPlayerError> {
        if options.pre_render {
            return AsciiPlayer::play_pre_rendered_frames(images_paths, options);
        }

        AsciiPlayer::play_frames(images_paths, options)
    }
}
