use crate::{player::{BodyDamageEvent, PlayerRoot}, prelude::*};

mod mob_type;
mod spawner;

use bevy::ecs::component::StorageType;
pub use mob_type::*;
use spawner::MobSpawnerPlugin;
pub use spawner::SpawnMob;

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

#[derive(Component, Default, Clone, Copy)]
pub struct MobSeed(pub f32);

#[derive(Component, Default, Clone, Copy)]
pub struct Melee{
    damge: f32
}

fn melee_move(
    mut units: Query<(&mut Transform, &MobSeed), With<Melee>>,
    player_root: Query<&Transform, (Without<Melee>, With<PlayerRoot>)>,
    time: Res<Time>,
){
    let p0 = player_root.single().xy();
    for (&mut transform, speed) in units.iter_mut(){
        let p1 = transformer.translation.xy();
        transform.translation = (p0 - p1).normalize() * speed.0 * time.delta_seconds();
    }
}

fn melle_attack(
    units: Query<(&Transform, Entity, &Melee)>,
    attack_event: EventWriter<BodyDamageEvent>,
){}

#[derive(Bundle, Default)]
pub struct MobBundle{
    mob: Mob,
    pub sprite: SpriteBundle,
}
