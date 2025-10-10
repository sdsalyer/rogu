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
        let mut players = <(Entity, &Point)>::query().filter(component::<Player>());
        let (player, dest) = players
            .iter(ecs) // loop over SubWorld results
            .find_map(|(entity, pos)| Some((*entity, *pos + delta)))
            .unwrap();

        let mut enemies = <(Entity, &Point)>::query().filter(component::<Enemy>());
        if delta.x != 0 || delta.y != 0 {
            let mut did_hit = false;
            enemies
                .iter(ecs)
                .filter(|(_, pos)| **pos == dest)
                .for_each(|(entity, _)| {
                    did_hit = true;
                    commands.push((
                        (),
                        WantsToAttack {
                            attacker: player,
                            target: *entity,
                        },
                    ));
                });

            if !did_hit {
                commands.push((
                    (),
                    WantsToMove {
                        entity: player,
                        destination: dest,
                    },
                ));
            }
        }

        *turn_state = TurnState::PlayerTurn;
    }
}
