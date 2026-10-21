#[cfg(not(target_family = "windows"))]
use std::path::PathBuf;

use clap::{CommandFactory, Parser, error::ErrorKind};

use tapciify::{
    cli::{Cli, FitMode},
    renderers::braille::DEFAULT_BRAILLE_FONT_RATIO,
    utils::{
        player::{AsciiPlayer, AsciiPlayerOptions, calculate_frame_time},
        resize::DEFAULT_FONT_RATIO,
    },
};

fn main() {
    // https://github.com/colored-rs/colored/issues/180
    #[cfg(target_family = "windows")]
    if std::env::var("COLORTERM").is_err() && std::env::var("WT_SESSION").is_ok() {
        unsafe {
            std::env::set_var("COLORTERM", "truecolor");
        }
    }

    let cli = Cli::parse();
    let mut cmd = Cli::command();

    #[cfg(target_family = "windows")]
    let images_paths = tapciify::cli::glob_to_paths(&cli.input)
        .unwrap_or_else(|err| cmd.error(ErrorKind::InvalidValue, err).exit());
    #[cfg(not(target_family = "windows"))]
    let images_paths: Vec<PathBuf> = cli.input.into_iter().map(PathBuf::from).collect();

    let font_ratio = match (cli.font_ratio, cli.braille) {
        (Some(ratio), _) => ratio,
        (None, true) => DEFAULT_BRAILLE_FONT_RATIO,
        (None, false) => DEFAULT_FONT_RATIO,
    };

    let (ascii_string, colored) = match (cli.reverse, cli.pixels) {
        (true, false) => (cli.ascii_string.chars().rev().collect(), cli.colored),
        (false, false) => (cli.ascii_string, cli.colored),
        (_, true) => ("█".to_owned(), true),
    };

    let (term_width, term_height) = crossterm::terminal::size()
        .map(|(w, h)| (Some(w as u32), Some(h as u32)))
        .unwrap_or((None, None));

    let width = match cli.fit {
        FitMode::Width | FitMode::Stretch => cli.width.or(term_width),
        _ => cli.width,
    };
    let height = match cli.fit {
        FitMode::Height | FitMode::Stretch => cli.height.or(term_height),
        _ => cli.height,
    };

    let frame_time = calculate_frame_time(cli.framerate);
    let options = AsciiPlayerOptions {
        width,
        height,
        ascii_string,
        colored,
        frame_time,
        pre_render: cli.pre_render,
        font_ratio,
        looped: cli.looped,
        threshold: cli.threshold,
        braille: cli.braille,
        background_string: cli.background_string,
        center: cli.center.into(),
        ..Default::default()
    };

    if let Err(err) = AsciiPlayer::play(&images_paths, &options) {
        cmd.error(ErrorKind::Io, err).exit()
    }
}
