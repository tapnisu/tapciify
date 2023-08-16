use crate::ascii::{DEFAULT_ASCII_STRING, DEFAULT_FONT_RATIO};
use clap::{ArgGroup, Parser};

#[cfg(feature = "glob")]
#[cfg(feature = "parallelism")]
use rayon::prelude::*;

#[cfg(feature = "glob")]
use glob::glob;

#[cfg(feature = "glob")]
use clap::{error::ErrorKind, CommandFactory};

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

    /// Makes frames colorful
    #[clap(short, long, action)]
    pub colored: bool,
    /// String to represent lightness of pixels
    #[clap(short, long, default_value_t = DEFAULT_ASCII_STRING.to_string())]
    pub ascii_string: String,
    /// Reverse the ASCII string
    #[clap(short, long, action)]
    pub reverse: bool,
    /// Font ratio: width / height
    #[clap(long="ratio", default_value_t = DEFAULT_FONT_RATIO)]
    pub font_ratio: f64,
}

/// Add glob support for paths parsing on non unix
#[cfg(feature = "glob")]
#[cfg(feature = "parallelism")]
pub fn glob_to_paths(patterns: Vec<String>) -> Vec<String> {
    patterns
        .par_iter()
        .flat_map(|glob_p| {
            let paths = glob(glob_p);

            if let Err(err) = paths {
                Cli::command().error(ErrorKind::InvalidValue, err).exit()
            }

            paths
                .unwrap()
                .map(|path| {
                    if let Err(err) = path {
                        Cli::command().error(ErrorKind::InvalidValue, err).exit()
                    }

                    path.unwrap().display().to_string()
                })
                .collect::<Vec<String>>()
        })
        .collect::<Vec<String>>()
}

/// Add glob support for paths parsing on non unix
#[cfg(feature = "glob")]
#[cfg(feature = "parallelism")]
pub fn glob_to_paths(patterns: Vec<String>) -> Vec<String> {
    patterns
        .iter()
        .flat_map(|glob_p| {
            let paths = glob(glob_p);

            if let Err(err) = paths {
                Cli::command().error(ErrorKind::InvalidValue, err).exit()
            }

            paths
                .unwrap()
                .map(|path| {
                    if let Err(err) = path {
                        Cli::command().error(ErrorKind::InvalidValue, err).exit()
                    }

                    path.unwrap().display().to_string()
                })
                .collect::<Vec<String>>()
        })
        .collect::<Vec<String>>()
}

/// Add glob support for paths parsing on non unix
#[cfg(not(feature = "glob"))]
pub fn glob_to_paths(patterns: Vec<String>) -> Vec<String> {
    patterns
}
