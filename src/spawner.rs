pub use crate::prelude::*;

pub fn spawn_player(ecs: &mut World, pos: Point) {
    const PLAYER_FG: (u8, u8, u8) = PURPLE;
    const PLAYER_BG: (u8, u8, u8) = BLACK;
    ecs.push(
        (
            Player,
            pos,
            Render {
                color: ColorPair::new(PLAYER_FG, PLAYER_BG),
                glyph: to_cp437('@')
            }
    )
    );
}
