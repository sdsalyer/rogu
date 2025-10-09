use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(MovingRandomly)]
pub fn random_move(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let _ = <(Entity, &Point, &MovingRandomly)>::query()
        .iter(ecs)
        .for_each(|(entity, pos, _)| {
            // TODO: why isn't this passed in?
            let mut rng = RandomNumberGenerator::new();
            let dest = match rng.range(0, 4) {
                0 => Point::new(-1, 0),
                1 => Point::new(1, 0),
                2 => Point::new(0, -1),
                _ => Point::new(0, 1),
            } + *pos;

            // if map.can_enter_tile(dest) {
            //     *pos = dest;
            // }
            commands.push((
                (),
                WantsToMove {
                    entity: *entity,
                    destination: dest,
                },
            ));
        });
}
