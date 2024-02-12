//! > **Tool to convert your images into ASCII art**
//!
//! Useful functions, when using as lib
//! - [`AsciiConverter::convert`]
//! - [`AsciiConverter::convert_raw`]
//!
//! ## Install
//!
//! ```bash
//! cargo install tapciify
//! ```
//!
//! ## View image
//!
//! 1. Run: `tapciify -i imagePath -w imageWidth` for image.
//!
//! 2. Run: `tapciify -i imagePath -w imageWidth -r` for reversed colors.
//!
//! ## View video
//!
//! In this example I set framerate to 24 (but you can use any another)
//!
//! > Requires ffmpeg
//!
//! 1. Make frames from video into dir:
//!
//!    ```bash
//!    mkdir frames; ffmpeg -i badapple.mkv frames/%08d.jpeg
//!    ```
//!
//! 2. Run:
//!
//!    ```bash
//!    tapciify -i frames/* -w videoWidth -f 24
//!    ```

pub mod ascii;
#[cfg(feature = "player")]
pub mod player;

#[cfg(feature = "player")]
pub mod cli;

#[doc(inline)]
pub use ascii::*;
#[doc(inline)]
#[cfg(feature = "player")]
pub use player::*;
