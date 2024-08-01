use crate::{player::{BodyDamageEvent, PlayerRoot, Size}, prelude::*};

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
                (melee_move, melle_attack).chain().distributive_run_if(in_state(GamePhase::Tribulation))
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

#[derive(Component, Default, Clone, Copy)]
pub struct MobSize(pub f32);

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
    units: Query<(&Transform, Entity, &Melee, &MobSize)>,
    player_root: Query<(&Transform, &Size), (Without<Melee>, With<PlayerRoot>)>,
    mut attack_event: EventWriter<BodyDamageEvent>,
){
    let (player_tans, Size(player_size)) = if let Ok(t) = player_root.get_single() {t} else {return;};
    for (mob_trans, id, melee, MobSize(mob_size)) in units.iter(){
        if is_in_circle(player_tans.translation.xy(), player_size + mob_size, mob_trans) {
            attack_event.send(BodyDamageEvent(melee.damage, id));
        }
    }
}

fn is_in_circle(centrum: Vec2, radius: f32, sample: Vec2) -> bool{
    let d = (sample - centrum).abs();
    let r = radius;

    //trying to keep comput down 
    if d.x > r || d.y > r{
        return false;
    } 
    
    if d.x.powi(2) + d.y.powi(2) <= r.powi(2){
        true
    } else {
        false
    }
}

#[derive(Bundle, Default)]
pub struct MobBundle{
    mob: Mob,
    pub sprite: SpriteBundle,
}
