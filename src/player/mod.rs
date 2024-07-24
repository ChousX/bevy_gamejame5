use crate::prelude::*;

mod soul;
mod body;

use soul::*;
use body::*;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn name(&self) -> &str {
        "Player Plugin"
    }

    fn build(&self, app: &mut App) {
        
    }
}

#[derive(Component)]
pub struct PlayerSelected;

fn player_builder(
    mut commands: Commands,
    selected_soul: Query<(&SoulName, &Magnetism, &Presence, Entity), (With<PlayerSelected>, With<SoulRoot>)>,
    selected_body: Query<(&Speed, &HitPoints, &HitPointRegeneration, &Size, Entity), (With<PlayerSelected>, With<BodyRoot>)>,
){
    let (body, soul)  = if !selected_body.is_empty() && !selected_soul.is_empty() {
            (selected_body.single(), selected_soul.single())
    } else {
        return;
    };
    //Get Data to Build Player
    let (name, magnetism, presence, soul_entity) = soul;
    let (speed, hit_points, hp_regen, size, body_entity) = body;

    let soul = SoulBundle {
        root: SoulRoot,
        name: name.clone(),
        magnetism: magnetism.clone(),
        presence: presence.clone(),
    };

    let body = BodyBundle {
        root: BodyRoot,
        speed: speed.clone(),
        hit_points: hit_points.clone(),
        hit_points_regen: hp_regen.clone(),
        size: size.clone(),
    };

    let pick_up_range = PickupRange(size.0 + magnetism.0);
    let aoe = AreaOfEffect(size.0 + presence.0);

    //Build Player
    let player = commands.spawn(
        PlayerBundle {
            root: PlayerRoot,
            pick_up_range,
            aoe,
            body,
            soul,
        },
    ).id();

    //Despawn selected 
    commands.entity(body_entity).despawn_recursive();
    commands.entity(soul_entity).despawn_recursive();

}

#[derive(Component, Default)]
pub struct PlayerRoot;

#[derive(Component)]
pub struct PickupRange(pub f32);
impl Default for PickupRange {
    fn default() -> Self{
        Self(5.0)
    }
}

#[derive(Component)]
pub struct AreaOfEffect(pub f32);
impl Default for AreaOfEffect {
    fn default() -> Self{
        Self(1.0)
    }
}

#[derive(Bundle, Default)]
pub struct PlayerBundle {
    pub root: PlayerRoot,
    pub pick_up_range: PickupRange,
    pub aoe: AreaOfEffect,
    pub soul: SoulBundle,
    pub body: BodyBundle,
}


