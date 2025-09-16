use std::error::Error;

use tapciify::utils::player::{AsciiPlayer, AsciiPlayerOptions};

fn main() -> Result<(), Box<dyn Error>> {
    let paths = ["./assets/examples/ferris.webp".into()];

    AsciiPlayer::play(
        &paths,
        &AsciiPlayerOptions {
            width: Some(64),
            // Put your other options here
            ..Default::default()
        },
    )?;

    Ok(())
}
