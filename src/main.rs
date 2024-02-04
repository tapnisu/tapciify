mod ascii;
mod cli;
mod player;

use clap::{error::ErrorKind, CommandFactory, Parser};
use cli::{glob_to_paths, Cli};
use player::{calculate_frame_time, AsciiPlayer, AsciiPlayerOptions};

fn main() -> Result<(), clap::Error> {
    let cli = Cli::parse();

    let images_paths = glob_to_paths(cli.input)
        .unwrap_or_else(|err| Cli::command().error(ErrorKind::InvalidValue, err).exit());

    let width = cli.width.unwrap_or(0);
    let height = cli.height.unwrap_or(0);

    let (ascii_string, colored) = if cli.pixels {
        ("█".to_owned(), true)
    } else {
        (
            if cli.reverse {
                AsciiPlayer::reverse_ascii_string(cli.ascii_string)
            } else {
                cli.ascii_string
            },
            cli.colored,
        )
    };

    let frame_time = calculate_frame_time(cli.frame_rate);

    let options = AsciiPlayerOptions {
        width,
        height,
        ascii_string,
        colored,
        frame_time,
        pre_render: cli.pre_render,
        font_ratio: cli.font_ratio,
        looped: cli.looped,
    };

    let result = AsciiPlayer::play(images_paths, options);

    if let Err(err) = result {
        Cli::command().error(ErrorKind::InvalidValue, err).exit()
    }

    Ok(())
}
