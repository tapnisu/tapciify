use crate::ascii::{
    AsciiArt, AsciiConverter, AsciiConverterError, AsciiConverterOptions, AsciiStringError,
    SizeError, DEFAULT_ASCII_STRING, DEFAULT_FONT_RATIO,
};
use crossterm::{cursor::MoveUp, execute};
use image::ImageError;
use indicatif::ProgressBar;
use std::{fmt, io::stdout, time::Instant};

#[cfg(feature = "rayon")]
use rayon::prelude::*;

// Calculate frame time (1 / frame rate)
pub fn calculate_frame_time(frame_rate: Option<f64>) -> u64 {
    frame_rate.map_or(0, |frame_rate| (1000f64 / frame_rate) as u64)
}

/// Player to convert and play frames
#[derive(Debug, Clone)]
pub struct AsciiPlayer {}

/// Options of player to convert and play frames
#[derive(Debug, Clone)]
pub struct AsciiPlayerOptions {
    pub width: u32,
    pub height: u32,
    pub ascii_string: String,
    pub colored: bool,
    pub frame_time: u64,
    pub pre_render: bool,
    pub font_ratio: f64,
    pub looped: bool,
}

impl From<AsciiPlayerOptions> for AsciiConverterOptions {
    fn from(o: AsciiPlayerOptions) -> AsciiConverterOptions {
        AsciiConverterOptions {
            width: o.width,
            height: o.height,
            ascii_string: o.ascii_string,
            colored: o.colored,
            font_ratio: o.font_ratio,
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
        images_paths: Vec<String>,
        options: AsciiPlayerOptions,
    ) -> Result<(), AsciiPlayerError> {
        let mut first_frame = false;

        let converter_options = AsciiConverterOptions::from(options.to_owned());

        loop {
            for image_path in images_paths.iter() {
                let start = Instant::now();

                let img = image::open(image_path)?;
                let ascii_image = AsciiConverter::convert(&img, &converter_options)?;

                if first_frame {
                    execute!(stdout(), MoveUp((ascii_image.height).try_into().unwrap()))
                        .unwrap_or_default();
                } else {
                    first_frame = true;
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
    #[cfg(feature = "rayon")]
    fn pre_render(
        images_paths: Vec<String>,
        options: AsciiPlayerOptions,
    ) -> Result<Vec<AsciiArt>, AsciiPlayerError> {
        let pb = ProgressBar::new(images_paths.len().try_into().unwrap());

        let converter_options = AsciiConverterOptions::from(options);

        let frames = images_paths
            .into_par_iter()
            .map(|path| {
                let img = image::open(path)?;
                let ascii_image = AsciiConverter::convert(&img, &converter_options)?;

                pb.inc(1);

                Ok(ascii_image)
            })
            .collect::<Result<Vec<AsciiArt>, AsciiPlayerError>>()?;

        Ok(frames)
    }

    /// Convert paths to of ASCII arts
    #[cfg(not(feature = "rayon"))]
    fn pre_render(
        images_paths: Vec<String>,
        options: AsciiPlayerOptions,
    ) -> Result<Vec<AsciiArt>, AsciiPlayerError> {
        let pb = ProgressBar::new(images_paths.len().try_into().unwrap());

        let converter_options = AsciiConverterOptions::from(options);

        let frames = images_paths
            .into_iter()
            .map(|path| {
                let img = image::open(path)?;
                let ascii_image = AsciiConverter::convert(&img, &converter_options)?;

                pb.inc(1);

                Ok(ascii_image)
            })
            .collect::<Result<Vec<AsciiArt>, AsciiPlayerError>>()?;

        Ok(frames)
    }

    /// Convert paths to of ASCII arts and play them
    pub fn play_pre_rendered_frames(
        images_paths: Vec<String>,
        options: AsciiPlayerOptions,
    ) -> Result<(), AsciiPlayerError> {
        let mut first_frame = false;

        let frames = AsciiPlayer::pre_render(images_paths, options.to_owned())?;

        loop {
            frames.iter().for_each(|ascii_image| {
                let start = Instant::now();

                if first_frame {
                    execute!(stdout(), MoveUp((ascii_image.height).try_into().unwrap()))
                        .unwrap_or_default();
                } else {
                    first_frame = true;
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
    /// let path = "./assets/examples/original.webp";
    ///
    /// let options = AsciiPlayerOptions {
    ///     width: 128,
    ///     ..Default::default()
    /// };
    ///
    /// assert!(AsciiPlayer::play(vec![path.to_owned()], options).is_ok())
    /// ```
    pub fn play(
        images_paths: Vec<String>,
        options: AsciiPlayerOptions,
    ) -> Result<(), AsciiPlayerError> {
        if options.pre_render {
            return AsciiPlayer::play_pre_rendered_frames(images_paths, options);
        }

        AsciiPlayer::play_frames(images_paths, options)
    }
}

impl Default for AsciiPlayerOptions {
    fn default() -> AsciiPlayerOptions {
        AsciiPlayerOptions {
            width: 0,
            height: 0,
            ascii_string: DEFAULT_ASCII_STRING.to_owned(),
            colored: false,
            frame_time: 0,
            pre_render: false,
            font_ratio: DEFAULT_FONT_RATIO,
            looped: false,
        }
    }
}
