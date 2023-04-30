pub mod utils;

use crate::utils::{generate_ascii_string, par_render_frame, play_frames};
use clap::Parser;

/// CLI tool that can let you view images/videos in terminal as ASCII
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Arguments {
    /// Input file or directory
    #[clap(short, short, value_parser)]
    input: String,
    /// Width of output
    #[clap(short, short, value_parser)]
    width: u32,

    /// Fps of showing images from directory (video)
    #[clap(short, long)]
    fps: Option<f64>,
    /// Renders before showing (works only for video)
    #[clap(short, long, action)]
    pre_render: bool,

    /// Makes frames colorful
    #[clap(short, long, action)]
    colored: bool,
    /// String to represent lightness of pixels
    #[clap(short, long)]
    ascii_string: Option<String>,
    /// Reverse the ascii string
    #[clap(short, long, action)]
    reverse: bool,
}

fn main() {
    let args = Arguments::parse();

    // String for pixel lightness
    let ascii_string = generate_ascii_string(
        args.ascii_string
            .unwrap_or_else(|| " .,:;+*?%S#@".to_owned()),
        args.reverse,
    );

    play_frames(
        args.input,
        args.width,
        ascii_string,
        args.colored,
        args.fps,
        args.pre_render,
    )
}
