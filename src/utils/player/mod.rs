use crate::render_full_frame;
use crossterm::{cursor::MoveUp, execute};
use indicatif::ProgressBar;
use std::{fs, io::stdout, time::Instant};

/// Reverse ascii string if true
pub fn generate_ascii_string(ascii_string: String, reversed: bool) -> String {
    if reversed {
        return ascii_string.chars().rev().collect::<String>().to_owned();
    }

    ascii_string
}

/// Play frames from directory in real time
pub fn play_normal_dir(
    image_paths: Vec<String>,
    ascii_string: String,
    width: u32,
    colored: bool,
    frame_time: u64,
) {
    let mut first_frame = false;
    // Calculate time to show frame

    for image_path in image_paths {
        let start = Instant::now();
        let image = image::open(image_path).unwrap();

        let frame = render_full_frame(image.clone(), width, ascii_string.clone(), colored);

        if first_frame {
            execute!(stdout(), MoveUp((frame.1 + 1).try_into().unwrap())).unwrap_or_default();
        } else {
            first_frame = true;
        }

        println!("{}", frame.0);

        while frame_time > start.elapsed().as_millis().try_into().unwrap() {}
    }
}

/// Render frames from directory, and then play them
pub fn play_pre_rendered_dir(
    image_paths: Vec<String>,
    ascii_string: String,
    width: u32,
    colored: bool,
    frame_time: u64,
) {
    let mut frames: Vec<(String, u32)> = Vec::new();
    let mut first_frame = false;

    let pb = ProgressBar::new(image_paths.len().try_into().unwrap());

    for image_path in image_paths {
        let image = image::open(image_path.clone()).unwrap();

        frames.push(render_full_frame(
            image.clone(),
            width,
            ascii_string.clone(),
            colored,
        ));

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

        while frame_time > start.elapsed().as_millis().try_into().unwrap() {}
    }
}

/// Play frames from directory (switch between pre_render and real time)
pub fn play_from_directory(
    input: String,
    width: u32,
    ascii_string: String,
    colored: bool,
    fps: Option<f64>,
    pre_render: bool,
) {
    let mut image_paths: Vec<String> = Vec::new();
    let images_paths = fs::read_dir(input).unwrap();

    for image_path in images_paths {
        image_paths.push(image_path.unwrap().path().to_str().unwrap().to_string());
    }

    // Calculate time to show frame
    let frame_time: u64;

    if let Some(fps) = fps {
        frame_time = (1000f64 / fps) as u64;
    } else {
        frame_time = 0;
    }

    if pre_render {
        return play_pre_rendered_dir(image_paths, ascii_string, width, colored, frame_time);
    }

    play_normal_dir(image_paths, ascii_string, width, colored, frame_time)
}
