mod ascii;
mod cli;
mod player;

use clap::{error::ErrorKind, CommandFactory, Parser};
use cli::{glob_to_paths, Cli};
use player::{calculate_frame_time, Player};

fn main() -> Result<(), clap::Error> {
    let cli = Cli::parse();

    let images_paths = glob_to_paths(cli.input);

    let width = cli.width.unwrap_or(0);
    let height = cli.height.unwrap_or(0);

    let (ascii_string, colored) = if cli.pixels {
        ("█".to_string(), true)
    } else {
        (cli.ascii_string, cli.colored)
    };

    let frame_time = calculate_frame_time(cli.frame_rate);

    let mut player = Player {
        images_paths,
        width,
        height,
        ascii_string,
        colored,
        frame_time,
        pre_render: cli.pre_render,
        font_ratio: cli.font_ratio,
        looped: cli.looped
    };

    if cli.reverse {
        player.reverse_ascii_string();
    }

    let result = player.play();

    if let Err(err) = result {
        Cli::command().error(ErrorKind::InvalidValue, err).exit()
    }

    Ok(())
}
