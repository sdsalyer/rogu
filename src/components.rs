pub use crate::prelude::*;

use std::collections::HashSet;

/// Tiles an entity can see
#[derive(Clone, Debug, PartialEq)]
pub struct FieldOfView {
    pub visible_tiles: HashSet<Point>,
    pub radius: i32,
    pub is_dirty: bool,
}

impl FieldOfView {
    pub fn new(radius: i32) -> Self {
        Self {
            visible_tiles: HashSet::new(),
            radius,
            is_dirty: true,
        }
    }

    pub fn clone_dirty(&self) -> Self {
        Self {
            visible_tiles: HashSet::new(),
            radius: self.radius,
            is_dirty: true,
        }
    }
}

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
pub struct ActivateItem {
    pub user: Entity,
    pub item: Entity,
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

#[derive(Clone, Debug, PartialEq)]
pub struct Carried(pub Entity);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProvidesDungeonMap;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProvidesHealing {
    pub amount: i32,
}


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

/// Weapons 
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Weapon;

/// deals damage 
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Damage(pub i32);


/// Enemies 
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Enemy;

/// Player
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Player {
    pub map_level: u32,
}
