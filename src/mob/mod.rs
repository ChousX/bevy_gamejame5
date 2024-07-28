use crate::prelude::*;

mod mob_type;
mod spawner;

use bevy::ecs::component::StorageType;
pub use mob_type::*;
use spawner::MobSpawnerPlugin;

pub struct MobPlugin;
impl Plugin for MobPlugin{
    fn name(&self) -> &str {
        "Mob Plugin"
    }
    fn build(&self, app: &mut App) {
        app
            .init_resource::<MobCount>()
            .add_plugins((
                    MobSpawnerPlugin
            ))
    ;}
}

#[derive(Default, Resource)]
pub struct MobCount(pub usize);

#[derive(Default)]
pub struct Mob;
impl Component for Mob{
    const STORAGE_TYPE: StorageType = StorageType::Table;
    fn register_component_hooks(hooks: &mut bevy::ecs::component::ComponentHooks) {
        hooks.on_add(|mut world, _targeted_entity, _component_id|{
            let mut count = world.resource_mut::<MobCount>();
            count.0 += 1;
        });
        
        hooks.on_remove(|mut world, _targeted_entity, _component_id|{
            let mut count = world.resource_mut::<MobCount>();
            count.0 -= 1;
        });
    }
}

#[derive(Bundle, Default)]
pub struct MobBundle{
    mob: Mob,
    pub sprite: SpriteBundle,
}
