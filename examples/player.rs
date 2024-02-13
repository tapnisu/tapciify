use std::error::Error;
use tapciify::{AsciiPlayer, AsciiPlayerOptions};

fn main() -> Result<(), Box<dyn Error>> {
    let paths = ["./assets/examples/original.webp".to_owned()];

    AsciiPlayer::play(
        &paths,
        &AsciiPlayerOptions {
            width: Some(64),
            ..Default::default()
        },
    )?;

    Ok(())
}
