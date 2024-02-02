use crate::ascii::{DEFAULT_ASCII_STRING, DEFAULT_FONT_RATIO};
use clap::{ArgGroup, Parser};
use std::fmt;

#[cfg(target_family = "windows")]
#[cfg(feature = "rayon")]
use rayon::prelude::*;

#[cfg(target_family = "windows")]
use glob::{glob, GlobError, PatternError};

#[derive(Parser, Debug, Clone)]
#[clap(author, version, about, long_about = None)]
#[clap(group(
    ArgGroup::new("size")
    .required(true)
    .args(&["width", "height"]),
))]
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
    #[clap(short, long = "fps", long = "framerate", value_name = "framerate")]
    pub frame_rate: Option<f64>,
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

#[derive(Debug)]
pub enum GlobToPathsError {
    PatternError(PatternError),
    GlobError(GlobError),
}

impl From<PatternError> for GlobToPathsError {
    fn from(e: PatternError) -> GlobToPathsError {
        GlobToPathsError::PatternError(e)
    }
}

impl From<GlobError> for GlobToPathsError {
    fn from(e: GlobError) -> GlobToPathsError {
        GlobToPathsError::GlobError(e)
    }
}

impl fmt::Display for GlobToPathsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GlobToPathsError::PatternError(err) => err.fmt(f),
            GlobToPathsError::GlobError(err) => err.fmt(f),
        }
    }
}

/// Add glob support for paths parsing on non unix
#[cfg(target_family = "windows")]
#[cfg(feature = "rayon")]
pub fn glob_to_paths(patterns: Vec<String>) -> Result<Vec<String>, GlobToPathsError> {
    patterns
        .into_par_iter()
        .map(|glob_p| -> Result<Vec<String>, GlobToPathsError> {
            glob(&glob_p)?
                .map(|path| Ok(path?.display().to_string()))
                .collect()
        })
        .flat_map(|result| match result {
            Ok(vec) => vec.into_iter().map(Ok).collect(),
            Err(er) => vec![Err(er)],
        })
        .collect()
}

/// Add glob support for paths parsing on non unix
#[cfg(not(target_family = "unix"))]
#[cfg(not(feature = "rayon"))]
pub fn glob_to_paths(patterns: Vec<String>) -> Result<Vec<String>, GlobToPathsError> {
    patterns
        .into_iter()
        .map(|glob_p| -> Result<Vec<String>, GlobToPathsError> {
            glob(&glob_p)?
                .map(|path| Ok(path?.display().to_string()))
                .collect()
        })
        .flat_map(|result| match result {
            Ok(vec) => vec.into_iter().map(Ok).collect(),
            Err(er) => vec![Err(er)],
        })
        .collect()
}

/// Add glob support for paths parsing on non unix
#[cfg(not(target_family = "windows"))]
pub fn glob_to_paths(patterns: Vec<String>) -> Result<Vec<String>, GlobToPathsError> {
    Ok(patterns)
}
