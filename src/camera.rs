use crate::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<CameraMoveEvent>()
            .init_resource::<CameraSpeed>()
            .add_systems(
                Startup,
                init_2d_camera
            ).add_systems(
                Update,
                update_speed.run_if(on_event::<CameraMoveEvent>()
            )).add_systems(
                Update,
                camera_bounder.run_if(in_state(AppState::Game))
            );
    }
}

#[derive(Component, Default)]
pub struct MainCamera(pub Vec2);

pub enum Direction {
    Forward,
    Backward,
    Left,
    Right,
    Analog(Vec2),
}

#[derive(Event)]
pub struct CameraMoveEvent(pub Direction);

#[derive(Resource)]
pub struct CameraBounds{
    pub min: Vec2,
    pub max: Vec2,
}

#[derive(Resource)]
pub struct CameraSpeed(pub f32);

impl Default for CameraSpeed {
    fn default() -> Self{
        Self(1.0)
    }
}
fn init_2d_camera(mut commands: Commands){
    commands.spawn((
            Camera2dBundle::default(),
            MainCamera::default(),
    ));
}

fn update_speed(
    mut main_camera: Query<(&mut MainCamera, &mut Transform)>,
    mut events: EventReader<CameraMoveEvent>,
    speed: Res<CameraSpeed>,
    time: Res<Time>,
) {
    let (mut camera, mut pos) = main_camera.single_mut();
    for CameraMoveEvent(dir) in events.read(){
        let dir = match dir {
            Direction::Forward => pos.forward(),
            Direction::Backward => pos.back(),
            Direction::Left => pos.left(),
            Direction::Right => pos.right(),
            Direction::Analog(d) => {
                Dir3::new_unchecked(d.extend(0.0))
            }
        }.as_vec3();
        camera.0 += dir.xy() * speed.0 * time.delta_seconds();
    }
    let dif = camera.0 * time.delta_seconds();
    camera.0 -= dif;
    pos.translation += dif.extend(0.0);
}

fn camera_bounder(
    mut camera_pos: Query<&mut Transform, (With<MainCamera>, Changed<Transform>)>,
    bounds: Res<CameraBounds>,
) {
    let mut pos = camera_pos.single_mut();
    if pos.translation.x > bounds.max.x {
        pos.translation.x = bounds.max.x;
    }
    if pos.translation.y > bounds.max.y {
        pos.translation.y = bounds.max.y
    }
    if pos.translation.x < bounds.min.x {
        pos.translation.x = bounds.min.x
    }
    if pos.translation.y < bounds.min.y {
        pos.translation.y = bounds.min.y;
    }
}
