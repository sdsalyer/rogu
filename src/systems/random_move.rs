use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(MovingRandomly)]
#[read_component(Health)]
#[read_component(Player)]
pub fn random_move(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    // Make our queries up front
    let mut movers = <(Entity, &Point, &MovingRandomly)>::query();
    let mut targets = <(Entity, &Point, &Health)>::query();

    // handle all movement
    let _ = movers
        .iter(ecs)
        .for_each(|(entity, pos, _)| {
            // TODO: why isn't RNG passed in?
            let mut rng = RandomNumberGenerator::new();
            let dest = match rng.range(0, 4) {
                0 => Point::new(-1, 0),
                1 => Point::new(1, 0),
                2 => Point::new(0, -1),
                _ => Point::new(0, 1),
            } + *pos;

            // handle all potential attacks
            let mut did_attack = false;
            let _ = targets
                .iter(ecs)
                .filter(|(_, target_pos, _)| **target_pos == dest)
                .for_each(|(target, _, _)| {
                    // Attack if there's a target player
                    if ecs
                        .entry_ref(*target)
                        .unwrap()
                        .get_component::<Player>()
                        .is_ok()
                    {
                        commands.push((
                            (),
                            WantsToAttack {
                                attacker: *entity,
                                target: *target,
                            },
                        ));
                    }
                    // Set to true even if no player target
                    // to prevent enemies from overlapping
                    did_attack = true;
                });

            // Move if not attacking
            if !did_attack {
                commands.push((
                    (),
                    WantsToMove {
                        entity: *entity,
                        destination: dest,
                    },
                ));
            }
        });
}
