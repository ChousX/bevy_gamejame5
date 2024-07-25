use crate::prelude::*;

pub struct ControlesPlugin;
impl Plugin for ControlesPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_state::<ControlerInputType>()
            .init_state::<FlyCam>()
            .init_resource::<KeyboardBindings>()
    ;}
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum FlyCam{
    Enabled,
    #[default]
    Disabled,
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

/*
pub trait KeyboardEventResever: Event + Sized{
    type MyEvent: Event + Sized;

    fn keyboard_controler(
        keys: Res<ButtonInput<KeyCode>>,
        keyb: Res<KeyboardBindings>,
        mut events: EventWriter<Self::MyEvent>,
    ) {
        if keys.any_pressed(keyb.forward){ events.send(MyEvent(Direction::Forward));}
        if keys.any_pressed(keyb.backward){ events.send(MyEvent(Direction::Backward));}
        if keys.any_pressed(keyb.left){ events.send(MyEvent(Direction::Left));}
        if keys.any_pressed(keyb.right){ events.send(MyEvent(Direction::Right));}

    }
}

*/
