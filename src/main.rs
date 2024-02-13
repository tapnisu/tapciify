mod ascii;
mod cli;
mod player;

use clap::{
    error::{ContextKind, ContextValue, ErrorKind},
    CommandFactory, Parser,
};
use cli::Cli;
use player::{calculate_frame_time, AsciiPlayer, AsciiPlayerOptions};

fn main() -> Result<(), clap::Error> {
    let cli = Cli::parse();
    let mut cmd = Cli::command();

    #[cfg(target_family = "windows")]
    let images_paths = cli::glob_to_paths(cli.input)
        .unwrap_or_else(|err| command.error(ErrorKind::InvalidValue, err).exit());
    #[cfg(not(target_family = "windows"))]
    let images_paths = cli.input;

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

    let frame_time = calculate_frame_time(cli.framerate);

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
        cmd.error(ErrorKind::InvalidValue, err).exit()
    }

    Ok(())
}
