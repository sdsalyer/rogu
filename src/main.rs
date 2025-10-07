mod map;

mod prelude {
    pub use crate::map::*;
    pub use bracket_lib::prelude::*;

    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
}

use prelude::*;

use std::io::{stdout, Write};

struct State {
    map: Map,
}

impl State {
    fn new() -> Self {
        Self {
            map: Map::new()
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        stdout().flush().expect("Command fail");
        ctx.cls();
        self.map.render(ctx);
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
