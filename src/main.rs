pub mod ascii;
pub mod cli;
pub mod player;

use clap::Parser;
use cli::Cli;
use player::{calculate_frame_time, generate_ascii_string, Player};

fn main() {
    let cli = Cli::parse();

    let ascii_string = generate_ascii_string(cli.ascii_string, cli.reverse);
    let frame_time = calculate_frame_time(cli.frame_rate);

    Player {
        images_paths: cli.input,
        width: Some(cli.width),
        ascii_string,
        colored: cli.colored,
        frame_time,
        pre_render: cli.pre_render,
        font_ratio: cli.font_ratio,
    }
    .play();
}
