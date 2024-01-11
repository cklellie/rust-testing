//! # Art
//!
//! A library for modeling artistic concepts.

pub mod kinds {
    //! The module kinds - not sure why it's called that.

    /// The primary colours according to the RYB colour model.
    pub enum PrimaryColour{
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colours according to the RYB colour model.
    pub enum SecondaryColour{
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    //! The module utils for a horrid name!

    use crate::kinds::*;

    /// Combines two primary colours in equal amounts to create
    /// a secondary colour.
    pub fn mix( _c1: PrimaryColour, _c2: PrimaryColour ) -> SecondaryColour {
        return SecondaryColour::Orange;
    }
}