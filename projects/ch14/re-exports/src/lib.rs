//! # Art
//!
//! A library for modeling artistic concepts.

// The following re-exports helps a user of this library to
// import `PrimaryColor`, `SecondaryColor`, and `mix` using:
// use art::mix;
// use art::PrimaryColor;
// use art::SecondaryColor;
//
// instead of using:
//
// use art::kinds::PrimaryColor;
// use art::kinds::SecondaryColor;
// use art::utils::mix;

pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds {
    /// The primary colors according to the RYB color model.
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB color mode
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}
pub mod utils {
    use crate::kinds::*;

    /// Combines two primary colors in equal amounts to create
    /// a secondary color.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        SecondaryColor::Green
    }
}
