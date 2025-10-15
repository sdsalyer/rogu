use crate::prelude::*;

#[system]
#[read_component(ActivateItem)]
#[read_component(ProvidesDungeonMap)]
#[read_component(ProvidesHealing)]
#[write_component(Health)]
pub fn use_items(ecs: &mut SubWorld, #[resource] map: &mut Map, commands: &mut CommandBuffer) {
    let mut heals = Vec::<(Entity, i32)>::new();
    let _ = <(Entity, &ActivateItem)>::query()
        .iter(ecs)
        .for_each(|(entity, act_req)| {
            let item = ecs.entry_ref(act_req.item);
            if let Ok(item) = item {
                if let Ok(healing) = item.get_component::<ProvidesHealing>() {
                    heals.push((act_req.user, healing.amount));
                };

                if let Ok(_mapper) = item.get_component::<ProvidesDungeonMap>() {
                    map.revealed_tiles.iter_mut().for_each(|t| *t = true);
                };
            };

            // Remove the consumed item
            commands.remove(act_req.item);

            // Consume the message
            commands.remove(*entity);
        });

    for heal in heals.iter() {
        if let Ok(mut target) = ecs.entry_mut(heal.0) {
            if let Ok(health) = target.get_component_mut::<Health>() {
                health.current = i32::min(health.max, health.current + heal.1);
            };
        };
    }
}
