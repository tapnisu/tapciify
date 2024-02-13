use tapciify::{
    image_resizing::{resize_img, ImageResizingOptions},
    AsciiConverter, AsciiConverterOptions,
};

fn main() {
    let orig_img = image::open("./assets/examples/original.webp").unwrap();

    let img = resize_img(
        &orig_img,
        ImageResizingOptions {
            width: Some(64),
            ..Default::default()
        },
    );

    let options = AsciiConverterOptions {
        colored: true,
        ..Default::default()
    };

    let result = AsciiConverter::convert(&img, &options).unwrap();

    println!("{}", result.text);
}
