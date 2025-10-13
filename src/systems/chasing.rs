use crate::prelude::*;

#[system]
#[read_component(ChasingPlayer)]
#[read_component(FieldOfView)]
#[read_component(Health)]
#[read_component(Player)]
#[read_component(Point)]
// TODO: Why is this function signature different from the others?
pub fn chasing(#[resource] map: &Map, ecs: &SubWorld, commands: &mut CommandBuffer) {
    let mut movers = <(Entity, &Point, &ChasingPlayer, &FieldOfView)>::query();
    let mut positions = <(Entity, &Point, &Health)>::query();
    let mut player = <(&Point, &Player)>::query();

    let (player_pos, _player) = player.iter(ecs).nth(0).unwrap();
    let player_idx = map_idx(player_pos.x, player_pos.y);

    // Populate Dijkstra map for pathfinding
    // map "overlay" with each "tile" indicating distance to
    // a target point
    // Note: big ones can be slow, and we also supply a max search
    //       depth to prevent searching the whole map.
    const MAX_DEPTH: f32 = 1024.0;
    let search_targets = vec![player_idx];
    let dijkstra_map =
        DijkstraMap::new(SCREEN_WIDTH, SCREEN_HEIGHT, &search_targets, map, MAX_DEPTH);

    movers.iter(ecs).for_each(|(entity, pos, _, fov)| {
        // Don't move if entity can't see the player
        if !fov.visible_tiles.contains(&player_pos) {
            return;
        }

        let idx = map_idx(pos.x, pos.y);
        if let Some(dest) = DijkstraMap::find_lowest_exit(&dijkstra_map, idx, map) {
            // Choose target tile based on player distance
            // Choose the next smallest target tile on the dijkstra map
            // unless very close to the player
            let dist = DistanceAlg::Pythagoras.distance2d(*pos, *player_pos);
            // 1.4 is diagonal, so move again to avoid unfair advantage vs player
            let dest = if dist > 1.2 {
                map.index_to_point2d(dest)
            } else {
                *player_pos
            };

            let mut did_attack = false;
            positions
                .iter(ecs)
                .filter(|(_, target_pos, _)| **target_pos == dest)
                .for_each(|(target, _, _)| {
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
                    did_attack = true;
                });

            if !did_attack {
                commands.push((
                    (),
                    WantsToMove {
                        entity: *entity,
                        destination: dest,
                    },
                ));
            }
        };
    });
}
