pub mod ascii;
pub mod cli;
pub mod player;

use clap::Parser;
use cli::Arguments;
use player::{calculate_frame_time, generate_ascii_string, Player};

fn main() {
    let args = Arguments::parse();

    let ascii_string = generate_ascii_string(args.ascii_string, args.reverse);
    let frame_time = calculate_frame_time(args.framerate);

    Player {
        images_paths: args.input,
        width: Some(args.width),
        ascii_string,
        colored: args.colored,
        frame_time,
        pre_render: args.pre_render,
        font_ratio: args.font_ratio,
    }
    .play();
}
