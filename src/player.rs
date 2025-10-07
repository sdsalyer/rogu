use crate::prelude::*;

pub struct Player {
    pub position: Point,
}

impl Player {
    /// create a new player as given coords
    pub fn new(position: Point) -> Self {
        Self { position }
    }

    /// draw the player at current location
    pub fn render(&self, ctx: &mut BTerm) {
        ctx.set(
            self.position.x,
            self.position.y,
            WHITE,
            BLACK,
            to_cp437('@'),
        );
    }

    /// update player position on keypress
    pub fn update(&mut self, ctx: &mut BTerm, map: &Map) {
        if let Some(key) = ctx.key {
            let delta = match key {
                VirtualKeyCode::Left | VirtualKeyCode::H => Point::new(-1, 0),
                VirtualKeyCode::Right | VirtualKeyCode::L => Point::new(1, 0),
                VirtualKeyCode::Up | VirtualKeyCode::K => Point::new(0, -1),
                VirtualKeyCode::Down | VirtualKeyCode::J => Point::new(0, 1),
                _ => Point::zero(),
            };

            let new_position = self.position + delta;
            if map.can_enter_tile(new_position) {
                self.position = new_position;
            }
        }
    }
}
