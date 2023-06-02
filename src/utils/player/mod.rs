use crate::par_render_frame;
use crossterm::{cursor::MoveUp, execute};
use indicatif::ProgressBar;
use std::{io::stdout, time::Instant};

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
    ascii_string: String,
    width: u32,
    colored: bool,
    frame_time: u64,
) {
    let mut first_frame = false;

    for image_path in image_paths {
        let start = Instant::now();
        let img = image::open(&image_path)
            .unwrap_or_else(|_| panic!("Failed to read file: {}", image_path));

        let frame = par_render_frame(img, width, ascii_string.clone(), colored);

        if first_frame {
            execute!(stdout(), MoveUp((frame.1).try_into().unwrap())).unwrap_or_default();
        } else {
            first_frame = true;
        }

        println!("{}", frame.0);

        while frame_time > start.elapsed().as_millis().try_into().unwrap() {}
    }
}

/// Render frames from directory, and then play them
pub fn play_pre_rendered_frames(
    image_paths: Vec<String>,
    ascii_string: String,
    width: u32,
    colored: bool,
    frame_time: u64,
) {
    let mut first_frame = false;

    let pb = ProgressBar::new(image_paths.len().try_into().unwrap());

    image_paths
        .into_iter()
        .map(|path| {
            let img = par_render_frame(
                image::open(path).unwrap(),
                width,
                ascii_string.clone(),
                colored,
            );

            pb.inc(1);

            img
        })
        .for_each(|frame| {
            let start = Instant::now();

            if first_frame {
                execute!(stdout(), MoveUp((frame.1).try_into().unwrap())).unwrap_or_default();
            } else {
                first_frame = true;
            }

            println!("{}", frame.0);

            while frame_time > start.elapsed().as_millis().try_into().unwrap() {}
        });
}

/// Use glob on windows
#[cfg(target_family = "windows")]
pub fn get_paths(input: Vec<String>) -> Vec<String> {
    input
        .into_iter()
        .flat_map(|string| {
            glob(&string)
                .expect("Failed to read glob pattern")
                .map(|path| path.unwrap().display().to_string())
        })
        .collect()
}

/// Use glob on windows
#[cfg(not(target_family = "windows"))]
pub fn get_paths(input: Vec<String>) -> Vec<String> {
    input
}

/// Play frames from directory (switch between pre_render and real time)
pub fn play_frames(
    input: Vec<String>,
    width: u32,
    ascii_string: String,
    colored: bool,
    fps: Option<f64>,
    pre_render: bool,
) {
    let image_paths = get_paths(input);

    // Calculate frame time (1 / framerate)
    let frame_time: u64;

    if let Some(fps) = fps {
        frame_time = (1000f64 / fps) as u64;
    } else {
        frame_time = 0;
    }

    if pre_render {
        return play_pre_rendered_frames(image_paths, ascii_string, width, colored, frame_time);
    }

    render_frames(image_paths, ascii_string, width, colored, frame_time)
}
