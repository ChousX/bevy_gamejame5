use crate::{controles::KeyboardBindings, game::GamePhase, prelude::*};

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
        dir += pos.forward().as_vec3();
    }
    if keys.any_pressed(keyb.backward){ 
        dir += pos.back().as_vec3();
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

#[derive(Component, Clone, Copy)]
pub struct HitPoints{
    pub max: f32,
    pub current: f32,
}
impl  Default for HitPoints {
    fn default () -> Self{
        const VAL: f32 = 100.0;
        Self{
            max: VAL,
            current: VAL,
        }
    }
}

impl HitPoints {
    pub fn damage(&mut self, ammount: f32) {
        //no healling form damge
        self.current -= ammount.abs();
    }

    pub fn is_dead(&self) -> bool{
        self.current < 0.0
    }

    pub fn is_full(&self) -> bool {
        self.current >= self.max
    }

    pub fn heal(&mut self, ammount: f32) {
        self.current += ammount;
        if self.current > self.max {
            self.current = self.max;
        }
    }
}

#[derive(Event, Default, Copy, Clone, Debug)]
pub struct DeathEvent;

#[derive(Event, Default, Copy, Clone, Debug)]
pub struct BodyDamageEvent(f32);

fn body_damage(
    mut hit_points: Query<&mut HitPoints, With<BodyRoot>>,
    mut damage_events: EventReader<BodyDamageEvent>,
    mut death: EventWriter<DeathEvent>,
) {
    let mut hit_points = hit_points.single_mut();
    for BodyDamageEvent(ammount) in damage_events.read(){
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
    pub speed: Speed,
    pub hit_points: HitPoints,
    pub hit_points_regen: HitPointRegeneration,
    pub size: Size,
}
