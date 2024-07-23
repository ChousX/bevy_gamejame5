use super::*;
use crate::prelude::*;

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
                time_ui_clean_up.run_if(on_event::<DeleteTimeUi>()))
    ;}
}

#[derive(Event)]
pub struct DeleteTimeUi;

#[derive(Event)]
pub struct NewTimeUi;

#[derive(Component)]
pub struct TimeDysplay;

fn init_time_ui(
    mut commands: Commands,
){

}

fn time_ui_clean_up(
    mut commands: Commands,
){

}

fn update_time_ui(
    mut time_ui: Query<Entity, With<TimeDysplay>>,
    game_time: Res<GameTime>,
){
    if !game_time.is_changed() {return;}
    let time_ui = time_ui.single();
}
