//! Utils used in tapciify CLI

use crate::{DEFAULT_ASCII_STRING, DEFAULT_FONT_RATIO};
use clap::Parser;

#[cfg(target_family = "windows")]
use glob::glob;
#[cfg(target_family = "windows")]
use std::{error, fmt, path::PathBuf};

#[cfg(target_family = "windows")]
#[cfg(feature = "rayon")]
use rayon::prelude::*;

/// Parse command arguments for tapciify CLI
#[derive(Parser, Debug, Clone)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    /// Input files to convert to ASCII art
    #[clap(short, long, num_args = 1.., required=true)]
    pub input: Vec<String>,
    /// Width of output
    #[clap(short, long)]
    pub width: Option<u32>,
    /// Height of output
    #[clap(short = 'H', long)]
    pub height: Option<u32>,

    /// Framerate for showing images
    #[clap(short, long)]
    pub framerate: Option<f64>,
    /// Render, and then show
    #[clap(short, long, action)]
    pub pre_render: bool,
    /// Loop frames
    #[clap(short, long = "loop", action)]
    pub looped: bool,

    /// Makes frames colorful
    #[clap(short, long, action)]
    pub colored: bool,
    /// String to represent lightness of pixels
    #[clap(short, long, default_value_t = DEFAULT_ASCII_STRING.to_owned())]
    pub ascii_string: String,
    /// Use █ symbol for ASCII string
    #[clap(long, action)]
    pub pixels: bool,
    /// Reverse the ASCII string
    #[clap(short, long, action)]
    pub reverse: bool,
    /// Font ratio: width / height
    #[clap(long="ratio", default_value_t = DEFAULT_FONT_RATIO)]
    pub font_ratio: f64,
    /// Use threshold for images
    #[clap(short, long)]
    pub threshold: Option<u32>,
    /// Use braille patter for rendering images
    #[clap(short, long, action)]
    pub braille: bool,
}

/// Add glob support for paths parsing on Windows
///
/// # Examples
///
/// ```
/// use tapciify::cli::glob_to_paths;
/// use std::path::PathBuf;
///
/// # fn main() -> Result<(), tapciify::cli::GlobToPathsError> {
/// let paths = vec!["assets\\examples\\*.webp".to_owned()];
/// let result = glob_to_paths(&paths)?;
///
/// assert_eq!(
///     result,
///     vec![
///         "assets\\examples\\ascii-colored.webp",
///         "assets\\examples\\ascii-pixels.webp",
///         "assets\\examples\\ascii.webp",
///         "assets\\examples\\braille-colored.webp",
///         "assets\\examples\\braille.webp",
///         "assets\\examples\\original.webp",
///         "assets\\examples\\rin-shima.webp",
///     ]
///     .iter()
///     .map(PathBuf::from)
///     .collect::<Vec<PathBuf>>()
/// );
/// # Ok(())
/// # }
/// `````
#[cfg(target_family = "windows")]
pub fn glob_to_paths(patterns: &[String]) -> Result<Vec<PathBuf>, GlobToPathsError> {
    #[cfg(feature = "rayon")]
    let iter = patterns.into_par_iter();
    #[cfg(not(feature = "rayon"))]
    let iter = patterns.iter();

    iter.map(|glob_p| {
        glob(glob_p)?
            .map(|path| Ok(path?))
            .collect::<Result<Vec<PathBuf>, GlobToPathsError>>()
    })
    .flat_map(|result| match result {
        Ok(vec) => vec.into_iter().map(Ok).collect(),
        Err(e) => vec![Err(e)],
    })
    .collect()
}

/// Error caused by [`glob_to_paths`]
#[cfg(target_family = "windows")]
#[derive(Debug)]
pub enum GlobToPathsError {
    PatternError(glob::PatternError),
    GlobError(glob::GlobError),
}

impl fmt::Display for GlobToPathsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GlobToPathsError::PatternError(err) => write!(f, "Pattern error: {}", err),
            GlobToPathsError::GlobError(err) => write!(f, "Glob error: {}", err),
        }
    }
}

impl From<glob::PatternError> for GlobToPathsError {
    fn from(err: glob::PatternError) -> GlobToPathsError {
        GlobToPathsError::PatternError(err)
    }
}

impl From<glob::GlobError> for GlobToPathsError {
    fn from(err: glob::GlobError) -> GlobToPathsError {
        GlobToPathsError::GlobError(err)
    }
}

impl error::Error for GlobToPathsError {}
