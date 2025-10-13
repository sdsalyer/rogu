use crate::prelude::*;

#[system]
#[read_component(FieldOfView)]
#[read_component(Player)]
pub fn map_render(ecs: &SubWorld, #[resource] map: &Map, #[resource] camera: &Camera) {
    const DRAW_LAYER: usize = 0;
    const SORT_ORDER: usize = 0;
    const FLOOR_FG: (u8, u8, u8) = WHITE; // (50, 50, 50);
    const FLOOR_BG: (u8, u8, u8) = BLACK;
    const WALL_FG: (u8, u8, u8) = WHITE; // (30, 5, 25);
    const WALL_BG: (u8, u8, u8) = BLACK;

    let mut draw_batch = DrawBatch::new();
    draw_batch.target(DRAW_LAYER);

    let mut fov = <&FieldOfView>::query().filter(component::<Player>());
    let player_fov = fov.iter(ecs).nth(0).unwrap();

    // add each map tile to the draw batch for rendering
    for y in camera.top_y..camera.bottom_y {
        for x in camera.left_x..camera.right_x {
            let pt = Point::new(x, y);
            let offset = Point::new(camera.left_x, camera.top_y);
            if map.in_bounds(pt) && player_fov.visible_tiles.contains(&pt) {
                let idx = map_idx(x, y);
                let (cp, glyph) = match map.tiles[idx] {
                    TileType::Floor => (ColorPair::new(FLOOR_FG, FLOOR_BG), to_cp437('.')),
                    TileType::Wall => (ColorPair::new(WALL_FG, WALL_BG), to_cp437('#')),
                };

                draw_batch.set(pt - offset, cp, glyph);
            } // end if in bounds
        } // end x loop
    } // end y loop

    draw_batch.submit(SORT_ORDER).expect("Batch error");
}
