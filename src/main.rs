pub mod utils;

use clap::Parser;
use crossterm::cursor::MoveUp;
use crossterm::execute;
use std::io::stdout;
use std::{fs, thread, time};
use utils::{calc_new_height, render_frame_case};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Arguments {
    #[clap(short, short, value_parser)]
    input: String,
    #[clap(short, short, value_parser)]
    width: u32,
    #[clap(short, long, action)]
    dir: bool,
    #[clap(short, long, action)]
    prerender: bool,
    #[clap(short, long)]
    fps: Option<f64>,
    #[clap(short, long, action)]
    reverse: bool,
    #[clap(short, long, action)]
    colored: bool,
    #[clap(short, long)]
    ascii_string: Option<String>,
}

fn main() {
    let args = Arguments::parse();

    // String for pixel lightness
    let mut ascii_string = args
        .ascii_string
        .unwrap_or_else(|| " .,:;+*?%S#@".to_string());

    if args.reverse {
        ascii_string = ascii_string.chars().rev().collect::<String>().to_owned();
    }

    // Play frames from folder
    if args.dir {
        let mut image_paths: Vec<String> = Vec::new();

        let images_paths = fs::read_dir(args.input).unwrap();
        for image_path in images_paths {
            if let Some(path) = image_path.unwrap().path().to_str() {
                image_paths.push(path.to_string());
            }
        }

        let frametime: u64 = (1f64 / args.fps.unwrap_or_else(|| 1f64) * 1000f64) as u64;

        if args.prerender {
            let mut image;
            let mut height: Option<u32> = None;

            let mut frames: Vec<String> = Vec::new();

            for image_path in image_paths {
                image = image::open(image_path.clone()).unwrap().to_rgb8();
                frames.push(render_frame_case(
                    image.clone(),
                    args.width,
                    &ascii_string,
                    args.colored,
                ));

                height = Some(calc_new_height(args.width, image.width(), image.height()));

                println!("Rendered {}", image_path);
            }

            for frame in frames {
                println!("{}", frame);

                execute!(stdout(), MoveUp(height.unwrap() as u16 + 1)).expect("");

                thread::sleep(time::Duration::from_millis(frametime));
            }
        } else {
            let mut image;
            let mut height;

            for image_path in image_paths {
                image = image::open(image_path).unwrap().to_rgb8();
                height = calc_new_height(args.width, image.width(), image.height());
                println!(
                    "{}",
                    render_frame_case(image.clone(), args.width, &ascii_string, args.colored,)
                );
                execute!(stdout(), MoveUp(height as u16 + 1)).expect("");

                thread::sleep(time::Duration::from_millis(frametime));
            }
        }
    } else {
        let image = image::open(args.input).unwrap().to_rgb8();

        println!(
            "{}",
            render_frame_case(image.clone(), args.width, &ascii_string, args.colored,)
        )
    }
}
