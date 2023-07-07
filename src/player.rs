use crate::ascii::{image_to_ascii, AsciiImage};
use crossterm::{cursor::MoveUp, execute};
use indicatif::ProgressBar;
use std::{io::stdout, time::Instant};

#[cfg(feature = "parallelism")]
use rayon::prelude::*;

#[cfg(target_family = "windows")]
use glob::glob;

/// Reverse ascii string if true
pub fn generate_ascii_string(ascii_string: String, reversed: bool) -> String {
    if reversed {
        return ascii_string.chars().rev().collect::<String>();
    }

    ascii_string
}

/// Play frames from directory in real time
pub fn render_frames(
    image_paths: Vec<String>,
    ascii_string: &str,
    width: u32,
    colored: bool,
    frame_time: u64,
    font_ratio: f64,
) {
    let mut first_frame = false;

    for image_path in image_paths {
        let start = Instant::now();
        let img = image::open(&image_path)
            .unwrap_or_else(|_| panic!("Failed to read file: {}", image_path));

        let ascii_image = image_to_ascii(img, width, ascii_string, colored, font_ratio);

        if first_frame {
            execute!(stdout(), MoveUp((ascii_image.height).try_into().unwrap()))
                .unwrap_or_default();
        } else {
            first_frame = true;
        }

        println!("{}", ascii_image.result);

        while frame_time > start.elapsed().as_millis().try_into().unwrap() {}
    }
}

/// Convert paths to of ascii images
#[cfg(feature = "parallelism")]
fn pre_render(
    image_paths: Vec<String>,
    ascii_string: &str,
    width: u32,
    colored: bool,
    font_ratio: f64,
) -> Vec<AsciiImage> {
    let pb = ProgressBar::new(image_paths.len().try_into().unwrap());

    image_paths
        .par_iter()
        .map(|path| {
            let ascii_image = image_to_ascii(
                image::open(path).unwrap(),
                width,
                ascii_string,
                colored,
                font_ratio,
            );

            pb.inc(1);

            ascii_image
        })
        .collect::<Vec<AsciiImage>>()
}

/// Convert paths to of ascii images
#[cfg(not(feature = "parallelism"))]
fn pre_render(
    image_paths: Vec<String>,
    ascii_string: &str,
    width: u32,
    colored: bool,
    font_ratio: f64,
) -> Vec<AsciiImage> {
    let pb = ProgressBar::new(image_paths.len().try_into().unwrap());

    image_paths
        .iter()
        .map(|path| {
            let ascii_image = image_to_ascii(
                image::open(path).unwrap(),
                width,
                ascii_string,
                colored,
                font_ratio,
            );

            pb.inc(1);

            ascii_image
        })
        .collect::<Vec<AsciiImage>>()
}

/// Convert paths to of ascii images and play them
pub fn play_pre_rendered_frames(
    image_paths: Vec<String>,
    ascii_string: &str,
    width: u32,
    colored: bool,
    frame_time: u64,
    font_ratio: f64,
) {
    let mut first_frame = false;

    pre_render(image_paths, ascii_string, width, colored, font_ratio)
        .iter()
        .for_each(|ascii_image| {
            let start = Instant::now();

            if first_frame {
                execute!(stdout(), MoveUp((ascii_image.height).try_into().unwrap()))
                    .unwrap_or_default();
            } else {
                first_frame = true;
            }

            println!("{}", ascii_image.result);

            while frame_time > start.elapsed().as_millis().try_into().unwrap() {}
        });
}

/// Add glob support for paths parsing on windows
#[cfg(target_family = "windows")]
#[cfg(feature = "parallelism")]
pub fn get_paths(input: Vec<String>) -> Vec<String> {
    input
        .par_iter()
        .flat_map(|glob_p| {
            glob(glob_p)
                .expect("Failed to read glob pattern")
                .map(|path| path.unwrap().display().to_string())
                .collect::<Vec<String>>()
        })
        .collect()
}

/// Add glob support for paths parsing on windows
#[cfg(target_family = "windows")]
#[cfg(not(feature = "parallelism"))]
pub fn get_paths(input: Vec<String>) -> Vec<String> {
    input
        .iter()
        .flat_map(|glob_p| {
            glob(glob_p)
                .expect("Failed to read glob pattern")
                .map(|path| path.unwrap().display().to_string())
                .collect::<Vec<String>>()
        })
        .collect()
}

/// Add glob support for paths parsing on windows
#[cfg(not(target_family = "windows"))]
pub fn get_paths(input: Vec<String>) -> Vec<String> {
    input
}

/// Play array of images as ascii images
pub fn play_frames(
    input: Vec<String>,
    width: u32,
    ascii_string: &str,
    colored: bool,
    fps: Option<f64>,
    pre_render: bool,
    font_ratio: f64,
) {
    let image_paths = get_paths(input);

    // Calculate frame time (1 / frame rate)
    let frame_time = if let Some(fps) = fps {
        (1000f64 / fps) as u64
    } else {
        0
    };

    if pre_render {
        return play_pre_rendered_frames(
            image_paths,
            ascii_string,
            width,
            colored,
            frame_time,
            font_ratio,
        );
    }

    render_frames(
        image_paths,
        ascii_string,
        width,
        colored,
        frame_time,
        font_ratio,
    )
}

#[test]
fn plays_frames() {
    use crate::ascii::DEFAULT_FONT_RATIO;

    play_frames(
        vec!["./assets/logo.png".to_string()],
        128,
        " .,:;+*?%S#@",
        true,
        None,
        false,
        DEFAULT_FONT_RATIO,
    );
}
