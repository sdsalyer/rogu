mod camera;
mod map;
mod map_builder;
mod player;

mod prelude {
    pub use crate::camera::*;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::player::*;
    pub use bracket_lib::prelude::*;

    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;
}

use prelude::*;


/// Game State
struct State {
    map: Map,
    player: Player,
    camera: Camera,
}

/// State implementation
impl State {
    fn new() -> Self {
        // let seed = 1337;
        // let mut rng = RandomNumberGenerator::seeded(seed);
        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut rng);

        Self {
            map: map_builder.map,
            player: Player::new(map_builder.player_start),
            camera: Camera::new(map_builder.player_start),
        }
    }
}

/// Tick implementation
impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        // clear screen if running in terminal?
        // use std::io::{stdout, Write};
        // stdout().flush().expect("Command fail");
        ctx.set_active_console(0);
        ctx.cls();
        ctx.set_active_console(1);
        ctx.cls();
        self.player.update(ctx, &self.map, &mut self.camera);
        self.map.render(ctx, &self.camera);
        self.player.render(ctx, &self.camera);
    }
}

/// R O G U !
/// Main (and only?) entry point
fn main() -> BError {
    // TODO: For crossterm backend, have to specify *exact* terminal dimensions?
    // let context = BTermBuilder::simple(124, 32)?
    //let context = BTermBuilder::simple80x50()

    const FONT_FILE: &str = "dungeonfont.png";

    let context = BTermBuilder::new()
        .with_title("R O G U !")
        .with_fps_cap(30.0)
        // .with_fullscreen(true)
        // .with_fitscreen(true)
        // .with_tile_dimensions(24, 24)
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
        .with_tile_dimensions(32, 32)
        .with_resource_path("resources/")
        .with_font(FONT_FILE, 32, 32)
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, FONT_FILE)
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, FONT_FILE)
        .build()?;

    main_loop(context, State::new())
}
