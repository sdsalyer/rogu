use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Player)]
pub fn player_input(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] turn_state: &mut TurnState,
) {
    if let Some(key) = key {
        let delta = match key {
            VirtualKeyCode::Left | VirtualKeyCode::H => Point::new(-1, 0),
            VirtualKeyCode::Right | VirtualKeyCode::L => Point::new(1, 0),
            VirtualKeyCode::Up | VirtualKeyCode::K => Point::new(0, -1),
            VirtualKeyCode::Down | VirtualKeyCode::J => Point::new(0, 1),
            _ => Point::zero(),
        };

        // Query the ECS for the player component (tag)
        let _ = <(Entity, &Point)>::query() // find all Point components
            .filter(component::<Player>()) // filter to only Player tag
            .iter(ecs) // loop over SubWorld results
            .for_each(|(entity, pos)| {
                let dest = *pos + delta;
                commands.push((
                    (),
                    WantsToMove {
                        entity: *entity,
                        destination: dest,
                    },
                ));
            });
        *turn_state = TurnState::PlayerTurn;
    }
}
