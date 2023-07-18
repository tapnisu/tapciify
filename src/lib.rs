pub mod ascii;
#[cfg(feature = "player")]
pub mod player;

pub use ascii::*;
#[cfg(feature = "player")]
pub use player::{calculate_frame_time, get_paths, Player};
