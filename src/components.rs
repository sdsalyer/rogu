pub use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Render {
    pub color: ColorPair,
    pub glyph: FontCharType,
}

/// Player "tag"
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Player;
