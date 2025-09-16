//! > **Tool to convert your images into ASCII art**
//!
//! Useful functions, when using as lib
//! - [`AsciiArtConverter::ascii_art`]
//! - [`CustomRatioResize::resize_custom_ratio`]
//!
//! ## Installation
//!
//! ```bash
//! cargo install tapciify --locked
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
//!    mkdir frames; ffmpeg -i bad_apple.mkv frames/%08d.jpeg
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
//! Demo:
//!
//! ```
//! use image::imageops::FilterType;
//!
//! # use std::error::Error;
//!
//! use tapciify::{prelude::*, utils::resize::DEFAULT_FONT_RATIO};
//!
//! # fn main() -> Result<(), Box<dyn Error>> {
//! let img = image::open("./assets/examples/ferris.webp")?;
//!
//! let result = img
//!     .resize_custom_ratio(Some(64), None, DEFAULT_FONT_RATIO, FilterType::Triangle)
//!     .ascii_art(&AsciiArtConverterOptions {
//!         // Put your other options here
//!         ..Default::default()
//! })?;
//!
//! println!("{}", result);
//! # Ok(())
//! # }
//! ```
//!
//! Colored:
//!
//! ```
//! use std::error::Error;
//!
//! # use image::imageops::FilterType;
//!
//! use tapciify::{prelude::*, utils::resize::DEFAULT_FONT_RATIO};
//!
//! # fn main() -> Result<(), Box<dyn Error>> {
//! let img = image::open("./assets/examples/ferris.webp")?;
//!
//! let result = img
//!     .resize_custom_ratio(Some(64), None, DEFAULT_FONT_RATIO, FilterType::Triangle)
//!     .ascii_art(&AsciiArtConverterOptions {
//!         // Put your other options here
//!         colored: true,
//!         ..Default::default()
//!     })?;
//!
//! println!("{}", result);
//! # Ok(())
//! # }
//! ```

pub mod renderers;
pub mod utils;

// #[deprecated(since = "3.4.0")]
// pub use renderers::ascii;

// #[allow(deprecated)]
// #[doc(inline)]
// #[deprecated(since = "3.4.0")]
// pub use renderers::ascii::{
//     AsciiArt, AsciiArtConverter, AsciiArtConverterError, AsciiArtConverterOptions, AsciiArtPixel,
//     AsciiStringError, DEFAULT_ASCII_STRING, ReverseString, SizeError, ToAsciiArtPixel,
//     ascii_character,
// };

// #[deprecated(since = "3.4.0")]
// pub use utils::resize;

// #[doc(inline)]
// #[cfg(feature = "player")]
// #[deprecated(since = "3.4.0")]
// pub use utils::player::{self, AsciiPlayer, AsciiPlayerError, AsciiPlayerOptions};

// #[doc(inline)]
// #[deprecated(since = "3.4.0")]
// pub use utils::resize::{CustomRatioResize, DEFAULT_FONT_RATIO};

// #[cfg(feature = "braille")]
// #[deprecated(since = "3.4.0")]
// pub use crate::renderers::braille;

// #[cfg(feature = "background-string")]
// #[deprecated(since = "3.4.0")]
// pub use crate::renderers::background_string;

// #[cfg(feature = "threshold-utils")]
// #[deprecated(since = "3.4.0")]
// pub use utils::threshold as threshold_utils;

#[cfg(feature = "player")]
pub mod cli;

pub mod prelude;
