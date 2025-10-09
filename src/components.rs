pub use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Render {
    pub color: ColorPair,
    pub glyph: FontCharType,
}

/// Monsters "tag"
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Enemy;

/// Player "tag"
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Player;
