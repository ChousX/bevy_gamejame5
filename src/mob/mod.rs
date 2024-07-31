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
            .add_plugins(
                    MobSpawnerPlugin
            )
            .add_systems(
                Update, 
                melee_move.run_if(in_state(GamePhase::Tribulation))
            )
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
pub struct MobSpeed(pub f32);

#[derive(Component, Default, Clone, Copy)]
pub struct Melee{
    damage: f32
}

fn melee_move(
    mut units: Query<(&mut Transform, &MobSpeed), With<Melee>>,
    player_root: Query<&Transform, (Without<Melee>, With<PlayerRoot>)>,
    time: Res<Time>,
){
    let p0 = player_root.single().translation.xy();
    for (mut transform, speed) in units.iter_mut(){
        let p1 = transform.translation.xy();
        transform.translation += 
            ((p0 - p1).normalize() * speed.0 * time.delta_seconds()).extend(0.9);
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
