pub mod utils;

use crate::utils::{generate_ascii_string, par_render_frame, play_frames};
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Arguments {
    /// Input files to convert to ascii
    #[clap(short, short, num_args = 1.., required=true)]
    input: Vec<String>,
    /// Width of output
    #[clap(short, short, value_parser)]
    width: u32,

    /// Framerate for showing images
    #[clap(short, long)]
    framerate: Option<f64>,
    /// Render, and then show
    #[clap(short, long, action)]
    pre_render: bool,

    /// Makes frames colorful
    #[clap(short, long, action)]
    colored: bool,
    /// String to represent lightness of pixels
    #[clap(short, long, default_value_t = String::from(" .,:;+*?%S#@"))]
    ascii_string: String,
    /// Reverse the ascii string
    #[clap(short, long, action)]
    reverse: bool,
}

fn main() {
    let args = Arguments::parse();

    // String for pixel lightness
    let ascii_string = generate_ascii_string(args.ascii_string, args.reverse);

    play_frames(
        args.input,
        args.width,
        ascii_string,
        args.colored,
        args.framerate,
        args.pre_render,
    )
}
