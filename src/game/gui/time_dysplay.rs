use super::*;
use crate::{helpers, prelude::*};

pub struct TimeDyplayPlugin;
impl Plugin for TimeDyplayPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<NewTimeUi>()
            .add_systems(
                Update,
                init_time_ui.run_if(on_event::<NewTimeUi>()))
            .add_systems(
                Update, 
                update_time_ui.run_if(in_state(GamePhase::Tribulation)))
            .add_systems(
                Update, 
                helpers::despawn_all::<TimeUiCleanUp>.run_if(on_event::<DeleteTimeUi>()))
    ;}
}

#[derive(Event)]
pub struct DeleteTimeUi;

#[derive(Event)]
pub struct NewTimeUi;

#[derive(Component)]
pub struct TimeDysplay;

#[derive(Component)]
pub struct TimeUiCleanUp;

fn init_time_ui(
    mut commands: Commands,
    game_time: Res<GameTime>,
    top_middle: Query<Entity, With<GameUiTopSectionMiddle>>
){
    let root = commands.spawn((
        NodeBundle{
            style: Style {
                width: Val::Percent(20.0),
                height: Val::Percent(100.0),
                ..default()
            },
            ..default()
        },
        TimeUiCleanUp
    )).id();
    
    let time_text = game_time.time_left_text();
    let text = commands.spawn((
        TextBundle::from_section(&time_text, TextStyle {..default()}),
        TimeDysplay,
    )).id();

    let top_middle = top_middle.single();
    commands.entity(top_middle).add_child(root);
    commands.entity(root).add_child(text);

}


fn update_time_ui(
    mut time_ui: Query<&mut Text, With<TimeDysplay>>,
    game_time: Res<GameTime>,
){
    if !game_time.is_changed() {return;}
    let mut time_ui = time_ui.single_mut();
    let time_text = game_time.time_left_text();
    *time_ui = Text::from_section(&time_text, TextStyle {..default()});
}
