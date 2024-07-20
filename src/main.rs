use bevy_gamejame5::{plugins::*, prelude::*, test_basic};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin{
            primary_window: Some(Window{ 
                title: APP_NAME.into(), 
                fit_canvas_to_parent: true,
                resizable: true,
                ..default()
            }),
            ..default()
        }))
        .run();
}
