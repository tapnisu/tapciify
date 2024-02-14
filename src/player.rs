//! Utils for playing multiple frames.
//! Probably you only need to use that if you are creating a CLI
//!
//! # Examples
//!
//! ```
#![doc = include_str!("../examples/player.rs")]
//! ```

use crate::{
    ascii::{
        AsciiArt, AsciiArtConverter, AsciiArtConverterError, AsciiArtConverterOptions,
        AsciiStringError, DEFAULT_ASCII_STRING,
    },
    CustomRatioResize, DEFAULT_FONT_RATIO,
};
use crossterm::{cursor::MoveUp, execute};
use image::{imageops::FilterType, ImageError};
use indicatif::ProgressBar;
use std::{error::Error, fmt, io::stdout, time::Instant};

#[cfg(feature = "rayon")]
use rayon::prelude::*;

/// Calculate frame time in millis (1 / framerate)
///
/// # Examples
///
/// ```
/// use tapciify::player::calculate_frame_time;
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
    pub filter: FilterType,
}

impl Default for AsciiPlayerOptions {
    fn default() -> AsciiPlayerOptions {
        AsciiPlayerOptions {
            width: None,
            height: None,
            ascii_string: DEFAULT_ASCII_STRING.to_owned(),
            colored: false,
            frame_time: 0,
            pre_render: false,
            font_ratio: DEFAULT_FONT_RATIO,
            looped: false,
            filter: FilterType::Triangle,
        }
    }
}

impl From<AsciiPlayerOptions> for AsciiArtConverterOptions {
    fn from(o: AsciiPlayerOptions) -> AsciiArtConverterOptions {
        AsciiArtConverterOptions {
            ascii_string: o.ascii_string,
            colored: o.colored,
        }
    }
}

/// Error caused by [`AsciiPlayer`]
#[derive(Debug)]
pub enum AsciiPlayerError {
    Image(ImageError),
    AsciiConverter(AsciiArtConverterError),
    AsciiString(AsciiStringError),
}

impl From<ImageError> for AsciiPlayerError {
    fn from(e: ImageError) -> AsciiPlayerError {
        AsciiPlayerError::Image(e)
    }
}

impl From<AsciiArtConverterError> for AsciiPlayerError {
    fn from(e: AsciiArtConverterError) -> AsciiPlayerError {
        AsciiPlayerError::AsciiConverter(e)
    }
}

impl From<AsciiStringError> for AsciiPlayerError {
    fn from(e: AsciiStringError) -> AsciiPlayerError {
        AsciiPlayerError::AsciiString(e)
    }
}

impl fmt::Display for AsciiPlayerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AsciiPlayerError::Image(err) => err.fmt(f),
            AsciiPlayerError::AsciiConverter(err) => err.fmt(f),
            AsciiPlayerError::AsciiString(err) => err.fmt(f),
        }
    }
}

impl Error for AsciiPlayerError {}

impl AsciiPlayer {
    /// Reverse ASCII string
    pub fn reverse_ascii_string(ascii_string: String) -> String {
        ascii_string.chars().rev().collect()
    }

    /// Play paths as ASCII arts
    pub fn play_frames(
        paths: &[String],
        options: &AsciiPlayerOptions,
    ) -> Result<(), AsciiPlayerError> {
        let mut first_frame = true;

        let converter_options = AsciiArtConverterOptions::from(options.to_owned());

        loop {
            for path in paths.iter() {
                let start = Instant::now();

                let img = image::open(path)?.resize_custom_ratio(
                    options.width,
                    options.height,
                    options.font_ratio,
                    options.filter,
                );
                let ascii_art = img.ascii_art(&converter_options)?;

                if !first_frame {
                    execute!(stdout(), MoveUp(ascii_art.height.try_into().unwrap()))
                        .unwrap_or_default();
                } else {
                    first_frame = false;
                }

                println!("{}", ascii_art);

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
        paths: &[String],
        options: &AsciiPlayerOptions,
    ) -> Result<Vec<AsciiArt>, AsciiPlayerError> {
        let pb = ProgressBar::new(paths.len().try_into().unwrap());

        let converter_options = AsciiArtConverterOptions::from(options.to_owned());

        #[cfg(feature = "rayon")]
        let iter = paths.into_par_iter();
        #[cfg(not(feature = "rayon"))]
        let iter = paths.into_iter();

        let frames = iter
            .map(|path| {
                let ascii_art = image::open(path)?
                    .resize_custom_ratio(
                        options.width,
                        options.height,
                        options.font_ratio,
                        options.filter,
                    )
                    .ascii_art(&converter_options)?;

                pb.inc(1);

                Ok(ascii_art)
            })
            .collect::<Result<Vec<AsciiArt>, AsciiPlayerError>>()?;

        Ok(frames)
    }

    /// Convert paths to of ASCII arts and play them
    pub fn play_pre_rendered_frames(
        paths: &[String],
        options: &AsciiPlayerOptions,
    ) -> Result<(), AsciiPlayerError> {
        let mut first_frame = true;

        let frames = AsciiPlayer::pre_render(paths, options)?;

        loop {
            frames.iter().for_each(|ascii_art| {
                let start = Instant::now();

                if !first_frame {
                    execute!(stdout(), MoveUp(ascii_art.height.try_into().unwrap()))
                        .unwrap_or_default();
                } else {
                    first_frame = false;
                }

                println!("{}", ascii_art);

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
    /// ```
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
    pub fn play(paths: &[String], options: &AsciiPlayerOptions) -> Result<(), AsciiPlayerError> {
        if options.pre_render {
            return AsciiPlayer::play_pre_rendered_frames(paths, options);
        }

        AsciiPlayer::play_frames(paths, options)
    }
}
