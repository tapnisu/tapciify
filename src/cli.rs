//! Utils used in tapciify CLI

use crate::{DEFAULT_ASCII_STRING, DEFAULT_FONT_RATIO};
use clap::Parser;

#[cfg(target_family = "windows")]
use err_derive::Error;
#[cfg(target_family = "windows")]
use glob::{glob, GlobError, PatternError};

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
    #[clap(short, long, value_parser)]
    pub width: Option<u32>,
    /// Height of output
    #[clap(short = 'H', long, value_parser)]
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
}

/// Error caused by [`glob_to_paths`]
#[cfg(target_family = "windows")]
#[derive(Debug, Error)]
pub enum GlobToPathsError {
    #[error(display = "{}", _0)]
    PatternError(#[source] PatternError),
    #[error(display = "{}", _0)]
    GlobError(#[source] GlobError),
}

/// Add glob support for paths parsing on Windows
///
/// # Examples
///
/// ```
/// use tapciify::cli::glob_to_paths;
///
/// let paths = vec!["assets\\examples\\*.webp".to_owned()];
/// let result = glob_to_paths(&paths)?;
///
/// assert_eq!(
///     result,
///     vec![
///         "assets\\examples\\ascii-colored.webp",
///         "assets\\examples\\ascii-pixels.webp",
///         "assets\\examples\\ascii.webp",
///         "assets\\examples\\original.webp"
///     ]
/// );
/// # Ok::<(), tapciify::cli::GlobToPathsError>(())
/// `````
#[cfg(target_family = "windows")]
pub fn glob_to_paths(patterns: &[String]) -> Result<Vec<String>, GlobToPathsError> {
    #[cfg(feature = "rayon")]
    let iter = patterns.into_par_iter();
    #[cfg(not(feature = "rayon"))]
    let iter = patterns.into_iter();

    iter.map(|glob_p| -> Result<Vec<String>, GlobToPathsError> {
        glob(glob_p)?
            .map(|path| Ok(path?.display().to_string()))
            .collect()
    })
    .flat_map(|result| match result {
        Ok(vec) => vec.into_iter().map(Ok).collect(),
        Err(er) => vec![Err(er)],
    })
    .collect()
}
