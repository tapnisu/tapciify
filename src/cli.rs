use clap::Parser;

use crate::ascii::DEFAULT_ASCII_STRING;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Arguments {
    /// Input files to convert to ascii
    #[clap(short, short, num_args = 1.., required=true)]
    pub input: Vec<String>,
    /// Width of output
    #[clap(short, short, value_parser)]
    pub width: u32,

    /// Framerate for showing images
    #[clap(short, long)]
    pub framerate: Option<f64>,
    /// Render, and then show
    #[clap(short, long, action)]
    pub pre_render: bool,

    /// Makes frames colorful
    #[clap(short, long, action)]
    pub colored: bool,
    /// String to represent lightness of pixels
    #[clap(short, long, default_value_t = DEFAULT_ASCII_STRING.to_string())]
    pub ascii_string: String,
    /// Reverse the ascii string
    #[clap(short, long, action)]
    pub reverse: bool,
}
