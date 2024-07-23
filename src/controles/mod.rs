use crate::{
    camera::{
        CameraMoveEvent,
        Direction
    },
    prelude::*
};

pub struct ControlesPlugin;
impl Plugin for ControlesPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_state::<ControlerInputType>()
            .init_resource::<KeyboardBindings>()
            .add_systems(
                Update,
                keyboard_camera_controler
                    .run_if(
                        in_state(ControlerInputType::KeyboardMouse)
                            .and_then(in_state(AppState::Game)))
            )
    ;}
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum ControlerInputType {
    None,
    #[default]
    KeyboardMouse,
    Gamepad,
}

#[derive(Resource)]
pub struct KeyboardBindings {
    pub forward: [KeyCode; 2],
    pub backward: [KeyCode; 2],
    pub left: [KeyCode; 2],
    pub right: [KeyCode; 2],
}

impl Default for KeyboardBindings { 
    fn default() -> Self {
        Self {
            forward: [KeyCode::KeyW, KeyCode::ArrowUp],
            backward: [KeyCode::KeyS, KeyCode::ArrowDown],
            left: [KeyCode::KeyA, KeyCode::ArrowLeft],
            right: [KeyCode::KeyD, KeyCode::ArrowRight],
        }
    }
}

pub fn keyboard_camera_controler(
    keys: Res<ButtonInput<KeyCode>>,
    keyb: Res<KeyboardBindings>,
    mut events: EventWriter<CameraMoveEvent>,
){
    if keys.any_pressed(keyb.forward){ events.send(CameraMoveEvent(Direction::Forward));}
    if keys.any_pressed(keyb.backward){ events.send(CameraMoveEvent(Direction::Backward));}
    if keys.any_pressed(keyb.left){ events.send(CameraMoveEvent(Direction::Left));}
    if keys.any_pressed(keyb.right){ events.send(CameraMoveEvent(Direction::Right));}
}
