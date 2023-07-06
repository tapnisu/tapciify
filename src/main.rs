pub mod ascii;
pub mod cli;
pub mod player;

use clap::Parser;
use cli::Arguments;
use player::{generate_ascii_string, play_frames};

fn main() {
    let args = Arguments::parse();

    // String for pixel lightness
    let ascii_string = generate_ascii_string(args.ascii_string, args.reverse);

    play_frames(
        args.input,
        args.width,
        ascii_string.as_str(),
        args.colored,
        args.framerate,
        args.pre_render,
        args.font_ratio
    )
}
