mod ascii;
mod cli;
mod player;

use clap::{error::ErrorKind, CommandFactory, Parser};
use cli::{glob_to_paths, Cli};
use player::{calculate_frame_time, Player};

fn main() -> Result<(), clap::Error> {
    let cli = Cli::parse();

    let frame_time = calculate_frame_time(cli.frame_rate);

    let paths = glob_to_paths(cli.input);

    let mut player = Player {
        images_paths: paths,
        width: cli.width.unwrap_or(0),
        height: cli.height.unwrap_or(0),
        ascii_string: cli.ascii_string,
        colored: cli.colored,
        frame_time,
        pre_render: cli.pre_render,
        font_ratio: cli.font_ratio,
    };

    if cli.reverse {
        player.reverse_ascii_string();
    }

    let result = player.play();

    match result {
        Ok(_) => (),
        Err(err) => Cli::command().error(ErrorKind::InvalidValue, err).exit(),
    }

    Ok(())
}
