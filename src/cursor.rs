use bevy::window::PrimaryWindow;

use crate::{camera::MainCamera, prelude::*};

pub struct CursorPlugin;
impl Plugin for CursorPlugin{
    fn build(&self, app: &mut App) {
        app
            .init_resource::<CursorPosition>()
            .add_systems(
                First,
                update_cursor_position);
    }
}


#[derive(Resource, Default)]
pub struct CursorPosition(pub Option<Vec2>);

fn update_cursor_position(
    mut cursor_pos: ResMut<CursorPosition>,
    window: Query<&Window, With<PrimaryWindow>>,
) {
    if let Some(window_pos) = window.single().cursor_position(){
        if let Some(cursor_p) = cursor_pos.0{
            if window_pos != cursor_p {
                cursor_pos.0 = Some(window_pos);
            }
        } else {
            cursor_pos.0 = Some(window_pos);
        }
    } else {
        if cursor_pos.0.is_some(){
            cursor_pos.0 = None
        }
    }
}
/*
fn update_cursor_position(
    mut cursor_pos: ResMut<CursorPosition>,
    window: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&GlobalTransform, &Camera), With<MainCamera>>,
){
    if let Some(window_pos) = window.single().cursor_position(){
        if let Ok((camera_trans, camera)) = camera.get_single(){
            let pos = camera.viewport_to_world_2d(camera_trans, window_pos);
            if pos != cursor_pos.0 {
                    cursor_pos.0 = pos;
            }
        }
    } else {
        if cursor_pos.0.is_some(){
            cursor_pos.0 = None;
        }
    };
}
*/
