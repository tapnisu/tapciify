use clap::Parser;
use image;
pub mod utils;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Arguments {
    #[clap(short, long, value_parser)]
    file: String,
    #[clap(short, long, value_parser)]
    width: u32,
}

fn main() {
    let args = Arguments::parse();

    let original_image = image::open(args.file).unwrap().to_rgb8();

    let nwidth = args.width;
    let nheight = (nwidth as f64 * (original_image.height() as f64) / original_image.width() as f64
        * (11.0f64 / 24.0f64)) as u32;

    let img = image::imageops::resize(
        &original_image,
        nwidth,
        nheight,
        image::imageops::FilterType::Lanczos3,
    );

    let rgb: Vec<u8> = img.into_raw();

    let mut frame = String::new();
    let mut x = 0;

    for i in (0..(rgb.len() - 1)).step_by(3) {
        frame.push(utils::ascii_symbol(utils::get_brightness(
            rgb[i as usize],
            rgb[i as usize + 1],
            rgb[i as usize + 2],
        )));

        x += 1;

        if x == nwidth {
            frame.push('\n');

            x = 0;
        }
    }

    println!("{}", frame)
}
