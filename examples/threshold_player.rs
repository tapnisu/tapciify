use std::error::Error;
use tapciify::{AsciiPlayer, AsciiPlayerOptions};

fn main() -> Result<(), Box<dyn Error>> {
    let paths = ["./assets/examples/rin-shima.webp".into()];

    AsciiPlayer::play(
        &paths,
        &AsciiPlayerOptions {
            width: Some(64),
            threshold: Some(20),
            // Put your other options here
            ..Default::default()
        },
    )?;

    Ok(())
}
