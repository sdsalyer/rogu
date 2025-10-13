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
pub struct WantsToAttack {
    pub attacker: Entity,
    pub target: Entity,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WantsToMove {
    pub entity: Entity,
    pub destination: Point,
}



#[derive(Clone, Debug, PartialEq)]
pub struct Name(pub String);



/// Chase player 
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ChasingPlayer;

/// Erratic behavior
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MovingRandomly;



/// THE item
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AmuletOfYala;

/// an item
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Item;

/// Monsters "tag"
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Enemy;

/// Player "tag"
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Player;
