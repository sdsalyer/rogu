use crate::prelude::*;

#[system(for_each)]
#[read_component(FieldOfView)]
#[read_component(Player)]
pub fn movement(
    entity: &Entity,
    move_req: &WantsToMove,
    #[resource] map: &Map,
    #[resource] camera: &mut Camera,
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
) {
    if map.can_enter_tile(move_req.destination) {
        commands.add_component(move_req.entity, move_req.destination);

        if let Ok(entry) = ecs.entry_ref(move_req.entity) {
            if let Ok(fov) = entry.get_component::<FieldOfView>() {
                commands.add_component(move_req.entity, fov.clone_dirty());
            }

            if entry.get_component::<Player>().is_ok() {
                camera.on_player_move(move_req.destination);
            }
        }
    }

    // remove this message now that it's processed
    commands.remove(*entity);
}
