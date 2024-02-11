//! Tool to convert your images into ASCII art
//!
//! Useful functions
//! - [`AsciiConverter::convert`]
//! - [`AsciiConverter::convert_raw`]

pub mod ascii;
#[cfg(feature = "player")]
pub mod player;

#[cfg(feature = "player")]
pub mod cli;

pub use ascii::*;
#[cfg(feature = "player")]
pub use player::*;
