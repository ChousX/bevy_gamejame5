use bevy::transform::commands;

use crate::prelude::*;

pub struct SplashScreenPlugin;
impl Plugin for SplashScreenPlugin{
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Enter), init).add_systems(OnExit(AppState::Enter), crate::helpers::despawn_all::<SplashScreen>)
    ;}
}

#[derive(Component)]
struct SplashScreen;

fn init(
    mut commands: Commands,
){
    let root = commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            ..default()
        }, 
        SplashScreen,
    )).id();

    let text_box = commands.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(80.0),
            height: Val::Percent(60.0),
            ..default()
        },
        ..default()
    }).id();

    let text = commands
        .spawn(TextBundle::from_section("Loading", TextStyle {
            color: css::BLACK.into(),
            ..default()
        })).id();

    commands.entity(root).add_child(text_box);
    commands.entity(text_box).add_child(text);

}
