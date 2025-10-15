use crate::prelude::*;

#[system]
#[read_component(Carried)]
#[read_component(Health)]
#[read_component(Item)]
#[read_component(Name)]
#[read_component(Player)]
pub fn hud(ecs: &SubWorld) {
    const DRAW_LAYER: usize = 2;
    const SORT_ORDER: usize = 10_000;
    const BAR_FG: (u8, u8, u8) = RED;
    const BAR_BG: (u8, u8, u8) = BLACK;
    const HEALTH_FG: (u8, u8, u8) = WHITE;
    const HEALTH_BG: (u8, u8, u8) = RED;

    let player_health = <&Health>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .nth(0)
        .unwrap();

    // Draw the HUD on layer 2
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(DRAW_LAYER);
    draw_batch.print_centered(
        1,
        "Explore the Dungeon, you ROGU!!. <hjkl> or arrows to move.",
    );

    draw_batch.bar_horizontal(
        Point::zero(),
        SCREEN_WIDTH * 2,
        player_health.current,
        player_health.max,
        ColorPair::new(RED, BLACK),
    );

    draw_batch.print_color_centered(
        0,
        format!(
            " Health: {} / {} ",
            player_health.current, player_health.max
        ),
        ColorPair::new(WHITE, RED),
    );

    // Display inventory
    let player = <(Entity, &Player)>::query()
        .iter(ecs)
        .find_map(|(entity, _)| Some(*entity))
        .unwrap();
    let mut y = 3;
    let _ = <(&Item, &Name, &Carried)>::query()
        .iter(ecs)
        .filter(|(_, _, carried)| carried.0 == player)
        .for_each(|(_, name, _)| {
            draw_batch.print(Point::new(3, y), format!("{} : {}", y - 2, &name.0));
            y += 1;
        });

    if y > 3 {
        draw_batch.print_color(
            Point::new(3, 2),
            "Items carried:",
            ColorPair::new(YELLOW, BLACK),
        );
    }

    draw_batch.submit(SORT_ORDER).expect("Batch error");
}
