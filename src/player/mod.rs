use crate::prelude::*;

mod soul;
mod body;

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
    commands: Commands,
    selected_soul: Query<Entity, With<(PlayerSelected, SoulRoot)>>,
    selected_body: Query<Entity, With<(PlayerSelected, BodyRoot)>>,
    body_speed: Query<&BodySpeed>,
    body_hit_points: Query<&BodyHitPoits>,
    body_size: Query<&BodySize>,
    soul_presence: Query<&SoulPresence>,
    soul_magnetism: Query<&SoulMagnetism>,
){
    let (body, soul)  = if !selected_body.is_empty() && !selected_soul.is_empty() {
            (selected_body.single(), selected_soul.single())
    } else {
        return;
    };
    //Get Data to Build Player
    let move_speed = if let Ok(body_speed) = body_speeds.get(body){
        PlayerMoveSpeed(body_speed.0)
    } else {
        PlayerMoveSpeed::default()
    };

    let hit_points = if let Ok(hit_points) = body_hit_points.get(body) {
        PlayerHitPoints(hit_points.0)
    } else {
        PlayerHitPoints::default()
    };

    let pick_up_range = {
        let mut accum = 0.0;
        if let Ok(body_range) = body_size.get(body){
            accum += body_range.0;
        }
        if let Ok(soul_range) = soul_magnetism.get(soul){
            accum += soul_range.0;
        }
        PlayerPickupRange(accum)
    };
    
    let aoe = {
        let mut accum = 0.0;
        if let Ok(body_aoe) = body_size.get(body){
            accum += body_aoe.0;
        }
        if let Ok(soul_aoe) = soul_presence.get(soul){
            accum += soul_aoe.0;
        }
        PlayerAreaOfEffect(accum)
    };

    //Build Player
    let player = commands.spawn(
        PlayerBundle {
            root: PlayerRoot,
            move_speed,
            pick_up_range,
            hit_points,
            aoe,
        },
    ).id();

    //Move Entitys to Player
    commands.entity(player).add_child(body);
    commands.entity(player).add_child(soul);

    //Remove selected form body and soul
    commands.entity(body).remove::<PlayerSelected>();
    commands.entity(soul).remove::<PlayerSelected>();

}

#[derive(Component, Default)]
pub struct PlayerRoot;

#[derive(Component)]
pub struct PlayerMoveSpeed(pub f32);
impl Default for PlayerMoveSpeed{
    fn default() -> Self{
        Self(1.0)
    }
}

#[derive(Component, Default)]
pub struct EffectsMoveSpeed;

#[derive(Component)]
pub struct PlayerPickupRange(pub f32);
impl Default for PlayerPickupRange {
    fn default() -> Self{
        Self(5.0)
    }
}

#[derive(Component, Default)]
pub struct EffectsPickupRange;

#[derive(Component)]
pub struct PlayerHitPoints(pub f32);
impl Default for PlayerHitPoints {
    fn default() -> Self{
        Self(100.0)
    }
}

#[derive(Component, Default)]
pub struct EffectsHitPoints;

#[derive(Component)]
pub struct PlayerAreaOfEffect(pub f32);
impl Default for PlayerAreaOfEffect {
    fn default() -> Self{
        Self(1.0)
    }
}


#[derive(Component, Default)]
pub struct EffectsAreaOfEffect;

#[derive(Bundle, Default)]
pub struct PlayerBundle {
    pub root: PlayerRoot,
    pub move_speed: PlayerMoveSpeed,
    pub pick_up_range: PlayerPickupRange,
    pub hit_points: PlayerHitPoints,
    pub aoe: PlayerAreaOfEffect,
}


