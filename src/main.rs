mod camera;
mod components;
mod map;
mod map_builder;
mod spawner;
mod systems;

mod prelude {
    pub use crate::camera::*;
    pub use crate::components::*;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::spawner::*;
    pub use crate::systems::*;

    pub use bracket_lib::prelude::*;
    pub use legion::systems::CommandBuffer;
    pub use legion::world::SubWorld;
    pub use legion::*;

    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;
}

use prelude::*;

/// Game State
struct State {
    ecs: World, // entities and components
    resources: Resources,
    systems: Schedule,
}

/// State implementation
impl State {
    fn new() -> Self {
        let mut ecs = World::default();
        let mut resources = Resources::default();

        // let seed = 1337;
        // let mut rng = RandomNumberGenerator::seeded(seed);
        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut rng);

        spawn_player(&mut ecs, map_builder.player_start);

        resources.insert(map_builder.map);
        resources.insert(Camera::new(map_builder.player_start));

        Self {
            ecs,
            resources,
            systems: build_scheduler(),
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

        // add keyboard state as a resource
        self.resources.insert(ctx.key);
        // execute the systems scheduler
        self.systems.execute(&mut self.ecs, &mut self.resources);

        // TODO: render Draw buffer
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
