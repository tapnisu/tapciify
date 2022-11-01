use clap::Parser;
pub mod utils;
use std::io::{stdout, Write};

use crossterm::{
    cursor::{DisableBlinking, EnableBlinking, MoveTo, RestorePosition, SavePosition},
    execute, ExecutableCommand, Result,
};
use std::{fs, thread, time};

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
}

fn main() {
    let args = Arguments::parse();

    if args.dir {
        let mut image_paths: Vec<String> = Vec::new();

        let images_paths = fs::read_dir(args.input).unwrap();
        for image_path in images_paths {
            match image_path.unwrap().path().to_str() {
                Some(x) => image_paths.push(x.to_string()),
                None => println!(),
            }
        }

        let frametime: u64;

        if let Some(fps) = args.fps {
            frametime = (1f64 / fps * 1000f64) as u64;
        } else {
            panic!("Frametime is None");
        }

        match args.prerender {
            true => {
                let mut frames: Vec<String> = Vec::new();

                for image_path in image_paths {
                    frames.push(utils::render_frame(
                        image_path.clone(),
                        args.width,
                        args.reverse,
                    ));
                    println!("Rendered {}", image_path);
                }

                for frame in frames {
                    println!("{}", utils::render_frame(frame, args.width, args.reverse));

                    execute!(stdout(), MoveTo(0, 0));

                    thread::sleep(time::Duration::from_millis(frametime));
                }
            }
            _ => {
                for image_path in image_paths {
                    println!(
                        "{}",
                        utils::render_frame(image_path, args.width, args.reverse)
                    );

                    execute!(stdout(), MoveTo(0, 0));

                    thread::sleep(time::Duration::from_millis(frametime));
                }
            }
        }
    } else {
        println!(
            "{}",
            utils::render_frame(args.input, args.width, args.reverse)
        )
    }
}
