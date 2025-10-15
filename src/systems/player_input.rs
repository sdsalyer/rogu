use crate::prelude::*;

#[system]
#[read_component(Carried)]
#[read_component(Item)]
#[read_component(Enemy)]
#[read_component(Point)]
#[read_component(Player)]
#[write_component(Health)]
pub fn player_input(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] turn_state: &mut TurnState,
) {
    if let Some(key) = key {
        // Query the ECS for the player component (tag)
        let mut players = <(Entity, &Point)>::query().filter(component::<Player>());
        let (player, dest) = players
            .iter(ecs) // loop over SubWorld results
            .find_map(|(entity, pos)| Some((*entity, *pos)))
            .unwrap();

        // Find any delta and add it to the dest
        let delta = match key {
            VirtualKeyCode::H | VirtualKeyCode::Left => Point::new(-1, 0),
            VirtualKeyCode::L | VirtualKeyCode::Right => Point::new(1, 0),
            VirtualKeyCode::K | VirtualKeyCode::Up => Point::new(0, -1),
            VirtualKeyCode::J | VirtualKeyCode::Down => Point::new(0, 1),
            VirtualKeyCode::G => {
                let _ = <(Entity, &Item, &Point)>::query()
                    .iter(ecs)
                    .filter(|(_, _, item_pos)| **item_pos == dest)
                    .for_each(|(entity, _, _)| {
                        commands.remove_component::<Point>(*entity);
                        commands.add_component(*entity, Carried(player));
                    });
                Point::zero()
            }
            _ => Point::zero(),
        };
        let dest = dest + delta;

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
