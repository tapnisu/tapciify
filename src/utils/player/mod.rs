use crate::render_full_frame;
use crossterm::{cursor::MoveUp, execute};
use indicatif::ProgressBar;
use std::{fs, io::stdout};
use tokio::time::Instant;

/// Reverse ascii string if true
pub fn generate_ascii_string(ascii_string: String, reversed: bool) -> String {
    if reversed {
        return ascii_string.chars().rev().collect::<String>().to_owned();
    }

    ascii_string
}

/// Play frames from directory in real time
pub async fn play_normal_dir(
    image_paths: Vec<String>,
    ascii_string: &'static str,
    width: u32,
    colored: bool,
    frametime: u64,
) {
    let mut first_frame = false;
    // Calculate time to show frame

    for image_path in image_paths {
        let start = Instant::now();
        let image = image::open(image_path).unwrap();

        let frame = render_full_frame(image.clone(), width, ascii_string, colored).await;

        if first_frame {
            execute!(stdout(), MoveUp((frame.1 + 1).try_into().unwrap())).unwrap_or_default();
        } else {
            first_frame = true;
        }

        println!("{}", frame.0);

        while frametime > start.elapsed().as_millis().try_into().unwrap() {}
    }
}

/// Render frames from directory, and then play them
pub async fn play_rerendered_dir(
    image_paths: Vec<String>,
    ascii_string: &'static str,
    width: u32,
    colored: bool,
    frametime: u64,
) {
    let mut frames: Vec<(String, u32)> = Vec::new();
    let mut first_frame = false;

    let pb = ProgressBar::new(image_paths.len().try_into().unwrap());

    for image_path in image_paths {
        let image = image::open(image_path.clone()).unwrap();

        frames.push(render_full_frame(image.clone(), width, ascii_string, colored).await);

        pb.inc(1);
    }

    pb.finish_and_clear();

    for frame in frames {
        let start = Instant::now();

        if first_frame {
            execute!(stdout(), MoveUp((frame.1 + 1).try_into().unwrap())).unwrap_or_default();
        } else {
            first_frame = true;
        }

        println!("{}", frame.0);

        while frametime > start.elapsed().as_millis().try_into().unwrap() {}
    }
}

/// Play frames from directory (switch between prerender and real time)
pub async fn play_from_directory(
    input: String,
    width: u32,
    ascii_string: &'static str,
    colored: bool,
    fps: Option<f64>,
    prerender: bool,
) {
    let mut image_paths: Vec<String> = Vec::new();
    let images_paths = fs::read_dir(input).unwrap();

    for image_path in images_paths {
        image_paths.push(image_path.unwrap().path().to_str().unwrap().to_string());
    }

    // Calculate time to show frame
    let frametime: u64;

    if let Some(fps) = fps {
        frametime = (1000f64 / fps) as u64;
    } else {
        frametime = 0;
    }

    if prerender {
        return play_rerendered_dir(image_paths, ascii_string, width, colored, frametime).await;
    }

    play_normal_dir(image_paths, ascii_string, width, colored, frametime).await
}
