pub mod utils;

use clap::Parser;
use tapciify::{play_dir, render_full_frame};

/// CLI tool that can let you view images in terminal
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Arguments {
    /// String to represent lightness of pixels
    #[clap(short, long)]
    ascii_string: Option<String>,
    /// Makes frames colorful
    #[clap(short, long, action)]
    colored: bool,
    /// Slideshow from folder
    #[clap(short, long, action)]
    dir: bool,
    /// Speed of slideshow (video)
    #[clap(short, long)]
    fps: Option<f64>,
    /// Input file or dir
    #[clap(short, short, value_parser)]
    input: String,
    /// Renders before showing (works only for video)
    #[clap(short, long, action)]
    prerender: bool,
    /// Reverse the ascii string
    #[clap(short, long, action)]
    reverse: bool,
    /// Width of output
    #[clap(short, short, value_parser)]
    width: u32,
}

#[tokio::main]
async fn main() {
    let args = Arguments::parse();

    // String for pixel lightness
    let ascii_string = Box::leak(
        args.ascii_string
            .unwrap_or_else(|| " .,:;+*?%S#@".to_owned())
            .into_boxed_str(),
    );

    // Play frames from folder
    if args.dir {
        play_dir(
            args.input,
            args.width,
            ascii_string,
            args.colored,
            args.fps,
            args.prerender,
        )
        .await
    } else {
        let image = image::open(args.input).unwrap();

        println!(
            "{}",
            render_full_frame(image.clone(), args.width, ascii_string, args.colored)
                .await
                .0
        )
    }
}
