use crate::prelude::*;

#[system]
#[write_component(Point)]
#[read_component(Player)]
pub fn player_input(
    ecs: &mut SubWorld,
    #[resource] map: &Map,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] camera: &mut Camera,
) {
    if let Some(key) = key {
        let delta = match key {
            VirtualKeyCode::Left | VirtualKeyCode::H => Point::new(-1, 0),
            VirtualKeyCode::Right | VirtualKeyCode::L => Point::new(1, 0),
            VirtualKeyCode::Up | VirtualKeyCode::K => Point::new(0, -1),
            VirtualKeyCode::Down | VirtualKeyCode::J => Point::new(0, 1),
            _ => Point::zero(),
        };

        if delta.x != 0 || delta.y != 0 {
            // Query the ECS for the player component (tag)
            let _ = <&mut Point>::query()         // find all Point components
                .filter(component::<Player>())    // filter to only Player tag
                .iter_mut(ecs)                    // loop over SubWorld results
                .for_each(|pos| {
                    let dest = *pos + delta;
                    if map.can_enter_tile(dest) {
                        *pos = dest;
                        camera.on_player_move(dest);
                    }
                });
        }
    }
}
