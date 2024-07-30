use crate::{player::PlayerRoot, prelude::*};
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                Startup,
                init_2d_camera
            ).add_systems(
                Update, 
                player_lock.run_if(in_state(AppState::Game)))
    ;}
}

#[derive(Component, Default)]
pub struct MainCamera;

fn init_2d_camera(mut commands: Commands){
    commands.spawn((
            Camera2dBundle::default(),
            MainCamera::default(),
    ));
}

fn player_lock(
    mut camera: Query<&mut Transform, (With<MainCamera>, Without<PlayerRoot>)>,
    player_pos: Query<&Transform, (With<PlayerRoot>, Changed<Transform>)>,
) {
    if let Some(pos) = player_pos.iter().next(){
        camera.single_mut().translation = pos.translation;
    }
}
