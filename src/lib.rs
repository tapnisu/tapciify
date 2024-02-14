//! > **Tool to convert your images into ASCII art**
//!
//! Useful functions, when using as lib
//! - [`AsciiArtConverter::ascii_art`]
//! - [`CustomRatioResize::resize_custom_ratio`]
//!
//! ## Installation
//!
//! ```bash
//! cargo install tapciify
//! ```
//!
//! ## Converting image
//!
//! 1. Run: `tapciify -i imagePath -w imageWidth` for image.
//!
//! 2. Run: `tapciify -i imagePath -w imageWidth -r` for reversed colors.
//!
//! ## Converting video
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
//!
//! # Examples
//!
//! ## Demo
//!
//! ```
#![doc = include_str!("../examples/demo.rs")]
//! ```
//!
//! ## Colored
//!
//! ```
#![doc = include_str!("../examples/colored.rs")]
//! ```

pub mod ascii;
pub mod resize;

#[cfg(feature = "player")]
pub mod player;

#[cfg(feature = "player")]
pub mod cli;

#[doc(inline)]
pub use ascii::{
    AsciiArt, AsciiArtConverter, AsciiArtConverterError, AsciiArtConverterOptions, AsciiArtPixel,
    AsciiStringError, SizeError, DEFAULT_ASCII_STRING,
};
#[doc(inline)]
pub use resize::{CustomRatioResize, DEFAULT_FONT_RATIO};

#[doc(inline)]
#[cfg(feature = "player")]
pub use player::{AsciiPlayer, AsciiPlayerError, AsciiPlayerOptions};
