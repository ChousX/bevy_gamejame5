use bevy_gamejame5::{
    plugins::*, 
    prelude::*
};


fn main() {
    App::new()
        .insert_resource(ClearColor(css::PLUM.into()))
        .init_resource::<AppSeed>()
        .add_plugins(DefaultPlugins.set(WindowPlugin{
            primary_window: Some(Window{ 
                title: APP_NAME.into(), 
                fit_canvas_to_parent: true,
                resizable: true,
                ..default()
            }),
            ..default()
        }))
        .init_state::<AppState>()
        .add_loading_state(
            LoadingState::new(AppState::Entry)
                .continue_to_state(AppState::MainMenu)
                .load_collection::<TerrainTextures>(),
        ).add_plugins((
                CameraPlugin,
                MenuPlugin,
        ))
        .run();
}
