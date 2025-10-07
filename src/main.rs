mod map;
mod map_builder;
mod player;

mod prelude {
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::player::*;
    pub use bracket_lib::prelude::*;

    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
}

use prelude::*;

use std::io::{stdout, Write};

struct State {
    map: Map,
    player: Player,
}

impl State {
    fn new() -> Self {
        // let seed = 1337;
        // let mut rng = RandomNumberGenerator::seeded(seed);
        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut rng);

        Self {
            map: map_builder.map,
            player: Player::new(map_builder.player_start),
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        // clear screen
        stdout().flush().expect("Command fail");
        ctx.cls();
        self.player.update(ctx, &self.map);
        self.map.render(ctx);
        self.player.render(ctx);
    }
}

fn main() -> BError {
    // TODO: For crossterm backend, have to specify *exact* terminal dimensions?
    // let context = BTermBuilder::simple(124, 32)?
    let context = BTermBuilder::simple80x50()
        .with_title("R O G U !")
        .with_fps_cap(30.0)
        .build()?;

    main_loop(context, State::new())
}
