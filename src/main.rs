mod camera;
mod components;
mod map;
mod map_builder;
mod spawner;
mod systems;
mod turn_state;

mod prelude {
    pub use crate::camera::*;
    pub use crate::components::*;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::spawner::*;
    pub use crate::systems::*;
    pub use crate::turn_state::*;

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
    input_systems: Schedule,
    player_systems: Schedule,
    enemy_systems: Schedule,
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

        // Populate player and enemies
        spawn_player(&mut ecs, map_builder.player_start);
        spawn_amulet_of_yala(&mut ecs, map_builder.amulet_start);
        /*
                map_builder
                    .rooms
                    .iter()
                    .skip(1) // 1 monster per room except the first
                    .map(|r| r.center())
                    .for_each(|pos| {
                        spawn_enemy(&mut ecs, &mut rng, pos);
                    });
        */
        map_builder
            .enemy_spawns
            .iter()
            .for_each(|pos| spawn_enemy(&mut ecs, &mut rng, *pos));

        resources.insert(map_builder.map);
        resources.insert(Camera::new(map_builder.player_start));
        resources.insert(TurnState::AwaitingInput);

        Self {
            ecs,
            resources,
            input_systems: build_input_scheduler(),
            player_systems: build_player_scheduler(),
            enemy_systems: build_enemy_scheduler(),
        }
    }

    fn game_over(&mut self, ctx: &mut BTerm) {
        const DRAW_LAYER: usize = 2;

        ctx.set_active_console(DRAW_LAYER);
        ctx.print_color_centered(2, RED, BLACK, "Your quest has ended!");
        ctx.print_color_centered(
            4,
            WHITE,
            BLACK,
            "Slain by a monster, your hero's journey has come to a premature end...",
        );
        ctx.print_color_centered(
            5,
            WHITE,
            BLACK,
            "The Amulet of Yala remains unclaimed, and your hometown is not saved. :(",
        );
        ctx.print_color_centered(
            8,
            YELLOW,
            BLACK,
            "Don't worry, though -- you can try again with a new hero!",
        );
        ctx.print_color_centered(9, GREEN, BLACK, "Press 1 to play again!");

        if let Some(VirtualKeyCode::Key1) = ctx.key {
            self.reset()
        };
    }

    fn victory(&mut self, ctx: &mut BTerm) {
        const DRAW_LAYER: usize = 2;

        ctx.set_active_console(DRAW_LAYER);
        ctx.print_color_centered(2, GREEN, BLACK, "You have won!@!");
        ctx.print_color_centered(
            4,
            WHITE,
            BLACK,
            "You put on the Amulet of Yala and feel its power course through your veins...",
        );
        ctx.print_color_centered(
            5,
            WHITE,
            BLACK,
            "Your town is saved! You can return to your normal life!",
        );
        ctx.print_color_centered(7, GREEN, BLACK, "Press 1 to play again!");

        if let Some(VirtualKeyCode::Key1) = ctx.key {
            self.reset()
        };
    }

    /// Reset the game state
    fn reset(&mut self) {
        self.ecs = World::default();
        self.resources = Resources::default();

        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut rng);

        spawn_player(&mut self.ecs, map_builder.player_start);
        spawn_amulet_of_yala(&mut self.ecs, map_builder.amulet_start);
        /*
                map_builder
                    .rooms
                    .iter()
                    .skip(1)
                    .map(|r| r.center())
                    .for_each(|pos| spawn_enemy(&mut self.ecs, &mut rng, pos));
        */
        map_builder
            .enemy_spawns
            .iter()
            .for_each(|pos| spawn_enemy(&mut self.ecs, &mut rng, *pos));

        self.resources.insert(map_builder.map);
        self.resources.insert(Camera::new(map_builder.player_start));
        self.resources.insert(TurnState::AwaitingInput);
    }
}

/// Tick implementation
impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        // clear screen if running in terminal?
        // use std::io::{stdout, Write};
        // stdout().flush().expect("Command fail");

        // Clear every layer
        for c in 0..=2 {
            ctx.set_active_console(c);
            ctx.cls();
        }

        // add keyboard state as a resource
        self.resources.insert(ctx.key);

        // Set console for correct mouse coords
        ctx.set_active_console(0);
        self.resources.insert(Point::from_tuple(ctx.mouse_pos()));

        // execute the systems schedulers
        let current_state = self.resources.get::<TurnState>().unwrap().clone();
        match current_state {
            TurnState::AwaitingInput => self
                .input_systems
                .execute(&mut self.ecs, &mut self.resources),
            TurnState::PlayerTurn => self
                .player_systems
                .execute(&mut self.ecs, &mut self.resources),
            TurnState::EnemyTurn => self
                .enemy_systems
                .execute(&mut self.ecs, &mut self.resources),
            TurnState::GameOver => self.game_over(ctx),
            TurnState::Victory => self.victory(ctx),
        }

        // render Draw buffer
        render_draw_buffer(ctx).expect("Render error");
    }
}

/// R O G U !
/// Main (and only?) entry point
fn main() -> BError {
    // TODO: For crossterm backend, have to specify *exact* terminal dimensions?
    // let context = BTermBuilder::simple(124, 32)?
    //let context = BTermBuilder::simple80x50()

    const FONT_TILE: (&str, i32, i32) = ("dungeonfont.png", 32, 32);
    const FONT_TEXT: (&str, i32, i32) = ("terminal8x8.png", 8, 8);

    let context = BTermBuilder::new()
        .with_title("R O G U !")
        .with_fps_cap(30.0)
        // .with_fullscreen(true)
        // .with_fitscreen(true)
        // .with_tile_dimensions(24, 24)
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
        .with_tile_dimensions(32, 32)
        .with_resource_path("resources/")
        .with_font(FONT_TILE.0, FONT_TILE.1, FONT_TILE.2)
        .with_font(FONT_TEXT.0, FONT_TEXT.1, FONT_TEXT.2)
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, FONT_TILE.0) // layer 0
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, FONT_TILE.0) // layer 1
        .with_simple_console_no_bg(SCREEN_WIDTH * 2, SCREEN_HEIGHT * 2, FONT_TEXT.0) // layer 2
        .build()?;

    main_loop(context, State::new())
}
