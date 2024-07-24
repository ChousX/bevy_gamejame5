use crate::prelude::*;

mod living_things;
pub use living_things::*;

pub struct BodyPlugin;
impl Plugin for BodyPlugin {
    fn build(&self, app: &mut App) {
        
    }
}

#[derive(Event)]
pub struct BodyMoveEvent(Direction);

#[derive(Component)]
pub struct BodySpeed(pub f32);

#[derive(Component)]
pub struct BodyVelocity(pub Vec2);

fn update_speed(
    mut body_data: Query<(&mut BodyVelocity, &mut Transform, &BodySpeed)>,
    mut events: EventReader<BodyMoveEvent>,
    time: Res<Time>,
) {
    let (mut body_v, mut pos, speed) = body_data.single_mut();
    for BodyMoveEvent(dir) in events.read(){
        let dir = match dir {
            Direction::Forward => pos.forward(),
            Direction::Backward => pos.back(),
            Direction::Left => pos.left(),
            Direction::Right => pos.right(),
            Direction::Analog(d) => {
                Dir3::new_unchecked(d.extend(0.0))
            }
        }.as_vec3();
        body_v.0 += dir.xy() * speed.0 * time.delta_seconds();
    }
    let dif = body_v.0 * time.delta_seconds();
    body_v.0 -= dif;
    pos.translation += dif.extend(0.0);
}

