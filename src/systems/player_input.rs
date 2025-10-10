use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(Enemy)]
#[write_component(Health)]
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

        let mut did_action = false;
        let mut enemies = <(Entity, &Point)>::query().filter(component::<Enemy>());
        if delta.x != 0 || delta.y != 0 {
            let mut did_hit = false;
            enemies
                .iter(ecs)
                .filter(|(_, pos)| **pos == dest)
                .for_each(|(entity, _)| {
                    did_hit = true;
                    did_action = true;
                    commands.push((
                        (),
                        WantsToAttack {
                            attacker: player,
                            target: *entity,
                        },
                    ));
                });

            if !did_hit {
                did_action = true;
                commands.push((
                    (),
                    WantsToMove {
                        entity: player,
                        destination: dest,
                    },
                ));
            }
        }

        if !did_action {
            if let Ok(health) = ecs.entry_mut(player).unwrap().get_component_mut::<Health>() {
                health.current = i32::min(health.max, health.current + 1);
            }
        }

        *turn_state = TurnState::PlayerTurn;
    }
}
