pub mod utils;

use clap::Parser;
use crossterm::cursor::MoveUp;
use crossterm::execute;
use std::fs;
use std::io::stdout;
use std::time::Instant;
use tapciify::{render_full_frame, string_to_static_str};
use utils::calc_new_height;

/// CLI tool that can let you view images in terminal
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Arguments {
    /// Input file or dir
    #[clap(short, short, value_parser)]
    input: String,
    /// Width of output
    #[clap(short, short, value_parser)]
    width: u32,
    /// Slideshow from folder
    #[clap(short, long, action)]
    dir: bool,
    /// Renders before showing (works only for video)
    #[clap(short, long, action)]
    prerender: bool,
    /// Speed of slideshow (video)
    #[clap(short, long)]
    fps: Option<f64>,
    /// String to represent lightness of pixels
    #[clap(short, long)]
    ascii_string: Option<String>,
    /// Reverse the ascii string
    #[clap(short, long, action)]
    reverse: bool,
    /// Makes frames colorful
    #[clap(short, long, action)]
    colored: bool,
}

#[tokio::main]
async fn main() {
    let args = Arguments::parse();

    // String for pixel lightness
    let mut ascii_string = args
        .ascii_string
        .unwrap_or_else(|| " .,:;+*?%S#@".to_owned());

    if args.reverse {
        ascii_string = ascii_string.chars().rev().collect::<String>().to_owned();
    }

    let static_ascii_string = string_to_static_str(ascii_string);

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
            let mut height: Option<u32> = None;
            let mut frames: Vec<String> = Vec::new();

            for image_path in image_paths {
                let image = image::open(image_path.clone()).unwrap();
                frames.push(
                    render_full_frame(image.clone(), args.width, static_ascii_string, args.colored)
                        .await,
                );

                height = Some(calc_new_height(args.width, image.width(), image.height()));

                println!("Rendered {}", image_path);
            }

            for frame in frames {
                let start = Instant::now();

                println!("{}", frame);

                execute!(stdout(), MoveUp(height.unwrap() as u16 + 1)).expect("");

                while frametime > start.elapsed().as_millis() as u64 {}
            }
        } else {
            for image_path in image_paths {
                let start = Instant::now();

                let image = image::open(image_path).unwrap();
                let height = calc_new_height(args.width, image.width(), image.height());

                println!(
                    "{}",
                    render_full_frame(image.clone(), args.width, static_ascii_string, args.colored)
                        .await
                );

                execute!(stdout(), MoveUp(height as u16 + 1)).expect("");

                while frametime > start.elapsed().as_millis() as u64 {}
            }
        }
    } else {
        let image = image::open(args.input).unwrap();

        println!(
            "{}",
            render_full_frame(image.clone(), args.width, static_ascii_string, args.colored).await
        )
    }
}
