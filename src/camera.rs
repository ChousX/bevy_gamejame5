use crate::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_2d_camera);
    }
}

fn init_2d_camera(mut commands: Commands){
    commands.spawn(Camera2dBundle::default());
}
