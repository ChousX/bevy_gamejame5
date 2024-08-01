use crate::{controles::KeyboardBindings, game::GamePhase, prelude::*};

mod hit_points;
mod body_type;

pub use body_type::*;
pub use hit_points::*;

pub struct BodyPlugin;
impl Plugin for BodyPlugin{
    fn name(&self) -> &str {
        "Body Plugin"
    }

    fn build(&self, app: &mut App) {
        app
            .add_event::<DeathEvent>()
            .add_event::<BodyDamageEvent>()
            .add_systems(
                Update,
                move_body.run_if(in_state(GamePhase::Tribulation)))
            .add_systems(
                Update, 
                body_damage.run_if(on_event::<BodyDamageEvent>()))
            .add_systems(
                Update,
                death_to_life_recap.run_if(on_event::<DeathEvent>()))
            .add_systems(
                Update, 
                hit_point_regeneration.run_if(
                    in_state(GamePhase::Tribulation))
                        //I really don't know how after works
                        //like if body_damage does't run can hit_point_regeneration?
                        .after(body_damage))
            .add_systems(
                Update, 
                update_body_texutes_from_type.run_if(in_state(AppState::Game)))
    ;}
}

#[derive(Component, Default, Clone, Copy)]
pub struct BodyRoot;

#[derive(Component, Clone)]
pub struct Speed(pub f32);
impl Default for Speed{
    fn default() -> Self{
        Self(1.0)
    }
}

fn move_body(
    mut bodys_q: Query<(&mut Transform, &Speed), With<BodyRoot>>,
    keyb: Res<KeyboardBindings>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>
){
    let (mut pos, speed) = bodys_q.single_mut();
    let mut dir = Vec3::ZERO;
    if keys.any_pressed(keyb.forward){ 
        let mut temp = pos.forward().as_vec3();
        dir += Vec3::new(temp.x, -temp.z, 0.0);
    }
    if keys.any_pressed(keyb.backward){ 
        let mut temp = pos.back().as_vec3();
        dir += Vec3::new(temp.x, -temp.z, 0.0);
    }
    if keys.any_pressed(keyb.left){
        dir += pos.left().as_vec3();
    }
    if keys.any_pressed(keyb.right){
        dir += pos.right().as_vec3();
    }

    let v = dir.normalize_or(Vec3::ZERO) * speed.0 * time.delta_seconds();
    let z = pos.translation.z;
    pos.translation += v;
    pos.translation.z = z;
}

#[derive(Event, Default, Copy, Clone, Debug)]
pub struct DeathEvent;

#[derive(Event, Copy, Clone, Debug)]
pub struct BodyDamageEvent(pub f32, pub Entity);

fn body_damage(
    mut hit_points: Query<&mut HitPoints, With<BodyRoot>>,
    mut damage_events: EventReader<BodyDamageEvent>,
    mut death: EventWriter<DeathEvent>,
) {
    let mut hit_points = hit_points.single_mut();
    for BodyDamageEvent(ammount, entity) in damage_events.read(){
        hit_points.damage(*ammount);
        if hit_points.is_dead() {
            death.send(DeathEvent);
        }
    }

}

fn death_to_life_recap(
    mut next_state: ResMut<NextState<GamePhase>>
) {
    next_state.set(GamePhase::LifeRecap);
}


#[derive(Component, Clone, Copy)]
pub struct HitPointRegeneration(pub f32);
impl Default for HitPointRegeneration{
    fn default () -> Self{
        Self(0.1)
    }
}


fn hit_point_regeneration(
    mut hit_points: Query<(&mut HitPoints, &HitPointRegeneration), With<BodyRoot>>,
    time: Res<Time>,
) {
    for (mut hp, regen) in hit_points.iter_mut(){
        hp.heal(regen.0 * time.delta_seconds())
    }
}

#[derive(Component, Clone, Copy)]
pub struct Size(pub f32);
impl Default for Size{
    fn default() -> Self{
        Self(1.0)
    }
}

#[derive(Bundle, Default)]
pub struct BodyBundle{
    pub root: BodyRoot,
    pub body_type: BodyType,
    pub speed: Speed,
    pub hit_points: HitPoints,
    pub hit_points_regen: HitPointRegeneration,
    pub size: Size,
}
