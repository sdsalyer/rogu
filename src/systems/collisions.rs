use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(Enemy)]
pub fn collisions(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    // get player position
    if let Some(player_pos) = <&Point>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .map(|pos| *pos)
        // TODO: we only have one player, right?
        .nth(0)
    {
        // get all the enemies and check if same pos as player
        let _ = <(Entity, &Point)>::query()
            .filter(component::<Enemy>())
            .iter(ecs)
            .filter(|(_, p)| **p == player_pos)
            .for_each(|(e, _)| commands.remove(*e));
    }
}
