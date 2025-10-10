use crate::prelude::*;

#[system]
#[read_component(WantsToAttack)]
#[write_component(Health)]
pub fn combat(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut attackers = <(Entity, &WantsToAttack)>::query();

    // create a Vec of targets
    let targets: Vec<(Entity, Entity)> = attackers
        .iter(ecs)
        .map(|(entity, attack)| (*entity, attack.target))
        .collect();

    // give each target a whack
    targets.iter().for_each(|(message, target)| {
        if let Ok(health) = ecs
            .entry_mut(*target)
            .unwrap()
            .get_component_mut::<Health>()
        {
            // println!("Health before attack: {}", health.current);
            health.current -= 1;
            if health.current < 1 {
                // dead
                commands.remove(*target);
            }
            // println!("Health after attack: {}", health.current);
        }

        // consume the message
        commands.remove(*message);
    });
}
