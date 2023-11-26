use crate::ascii::{
    AsciiArt, AsciiConverter, AsciiConverterError, AsciiStringError, SizeError,
    DEFAULT_ASCII_STRING, DEFAULT_FONT_RATIO,
};
use crossterm::{cursor::MoveUp, execute};
use image::ImageError;
use indicatif::ProgressBar;
use std::{fmt, io::stdout, time::Instant};

#[cfg(feature = "parallelism")]
use rayon::prelude::*;

// Calculate frame time (1 / frame rate)
pub fn calculate_frame_time(frame_rate: Option<f64>) -> u64 {
    frame_rate
        .map(|frame_rate| (1000f64 / frame_rate) as u64)
        .unwrap_or(0)
}

/// Player to convert and play frames
#[derive(Debug, Clone)]
pub struct Player {
    pub images_paths: Vec<String>,
    pub width: u32,
    pub height: u32,
    pub ascii_string: String,
    pub colored: bool,
    pub frame_time: u64,
    pub pre_render: bool,
    pub font_ratio: f64,
    pub looped: bool,
}

#[derive(Debug)]
pub enum PlayerError {
    Image(ImageError),

    AsciiConverter(AsciiConverterError),

    AsciiString(AsciiStringError),
    Size(SizeError),
}

impl From<ImageError> for PlayerError {
    fn from(e: ImageError) -> Self {
        Self::Image(e)
    }
}

impl From<AsciiConverterError> for PlayerError {
    fn from(e: AsciiConverterError) -> Self {
        Self::AsciiConverter(e)
    }
}

impl From<AsciiStringError> for PlayerError {
    fn from(e: AsciiStringError) -> Self {
        Self::AsciiString(e)
    }
}

impl From<SizeError> for PlayerError {
    fn from(e: SizeError) -> Self {
        Self::Size(e)
    }
}

impl fmt::Display for PlayerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Image(err) => err.fmt(f),

            Self::AsciiConverter(err) => err.fmt(f),

            Self::AsciiString(err) => err.fmt(f),
            Self::Size(err) => err.fmt(f),
        }
    }
}

impl Player {
    /// Reverse ASCII string
    pub fn reverse_ascii_string(&mut self) -> &mut Self {
        self.ascii_string = self.ascii_string.chars().rev().collect();

        self
    }

    /// Play paths as ASCII arts
    pub fn play_frames(&self) -> Result<(), PlayerError> {
        let mut first_frame = false;

        loop {
            for image_path in &self.images_paths {
                let start = Instant::now();
                let img = image::open(image_path)?;

                let ascii_image = AsciiConverter {
                    img,
                    width: self.width,
                    height: self.height,
                    ascii_string: self.ascii_string.to_owned(),
                    colored: self.colored,
                    font_ratio: self.font_ratio,
                }
                .convert()?;

                if first_frame {
                    execute!(stdout(), MoveUp((ascii_image.height).try_into().unwrap()))
                        .unwrap_or_default();
                } else {
                    first_frame = true;
                }

                println!("{}", ascii_image.text);

                while self.frame_time > start.elapsed().as_millis().try_into().unwrap() {}
            }

            if !self.looped {
                break;
            }
        }

        Ok(())
    }

    /// Convert paths to of ASCII arts
    #[cfg(feature = "parallelism")]
    fn pre_render(&self) -> Result<Vec<AsciiArt>, PlayerError> {
        let pb = ProgressBar::new(self.images_paths.len().try_into().unwrap());

        let frames = self
            .images_paths
            .par_iter()
            .map(|path| -> Result<AsciiArt, PlayerError> {
                let img = image::open(path)?;

                let ascii_image = AsciiConverter {
                    img,
                    width: self.width,
                    height: self.height,
                    ascii_string: self.ascii_string.to_owned(),
                    colored: self.colored,
                    font_ratio: self.font_ratio,
                }
                .convert()?;

                pb.inc(1);

                Ok(ascii_image)
            })
            .collect::<Result<Vec<AsciiArt>, PlayerError>>()?;

        Ok(frames)
    }

    /// Convert paths to of ASCII arts
    #[cfg(not(feature = "parallelism"))]
    fn pre_render(&self) -> Result<Vec<AsciiArt>, PlayerError> {
        let pb = ProgressBar::new(self.images_paths.len().try_into().unwrap());

        let frames = self
            .images_paths
            .iter()
            .map(|path| -> Result<AsciiArt, PlayerError> {
                let img = image::open(path)?;

                let ascii_image = AsciiConverter {
                    img,
                    width: self.width,
                    height: self.height,
                    ascii_string: self.ascii_string.to_owned(),
                    colored: self.colored,
                    font_ratio: self.font_ratio,
                }
                .convert()?;

                pb.inc(1);

                Ok(ascii_image)
            })
            .collect::<Result<Vec<AsciiArt>, PlayerError>>()?;

        Ok(frames)
    }

    /// Convert paths to of ASCII arts and play them
    pub fn play_pre_rendered_frames(&self) -> Result<(), PlayerError> {
        let mut first_frame = false;

        let frames = Self::pre_render(self)?;

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

                while self.frame_time > start.elapsed().as_millis().try_into().unwrap() {}
            });

            if !self.looped {
                break;
            }
        }

        Ok(())
    }

    /// Play frames
    pub fn play(self) -> Result<(), PlayerError> {
        if self.pre_render {
            return Self::play_pre_rendered_frames(&self);
        }

        Self::play_frames(&self)
    }
}

impl Default for Player {
    fn default() -> Player {
        Self {
            images_paths: vec![],
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

#[test]
fn plays_frames() {
    Player {
        images_paths: vec!["./assets/examples/original.webp".to_owned()],
        width: 128,
        ..Default::default()
    }
    .play()
    .unwrap()
}
