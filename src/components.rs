pub use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Render {
    pub color: ColorPair,
    pub glyph: FontCharType,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WantsToMove {
    pub entity: Entity,
    pub destination: Point,
}


#[derive(Clone, Debug, PartialEq)]
pub struct Name(pub String);


/// Monsters "tag"
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Enemy;

/// Player "tag"
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MovingRandomly;

/// Player "tag"
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Player;
