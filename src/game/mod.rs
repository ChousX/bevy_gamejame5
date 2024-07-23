use bevy::core_pipeline::deferred;
use body_selection::BodySelectionPlugin;
use soul_selection::SoulSelectionPlugin;

use crate::prelude::*;
use std::time::Duration;


mod gui;
mod soul_selection;
mod body_selection;

use gui::*;

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_sub_state::<GamePhase>()
            .add_sub_state::<GameState>()
            .init_resource::<GameTimeMode>()
            .add_systems(
                OnEnter(GamePhase::Tribulation), init_game_time)
            .add_systems(
                Update, 
                update_game_time
                    .run_if(in_state(GamePhase::Tribulation)
                        .and_then(in_state(GameState::Running)))
            )
            .add_plugins((
                BodySelectionPlugin,
                SoulSelectionPlugin,
            ))
    ;}
}

#[derive(SubStates, Clone, PartialEq, Eq, Hash, Debug, Default)]
#[source(AppState = AppState::Game)]
pub enum GameState{
    Paused,
    #[default]
    Running,
}

#[derive(SubStates, Clone, PartialEq, Eq, Hash, Debug, Default)]
#[source(AppState = AppState::Game)]
pub enum GamePhase{
    #[default]
    None,
    // Chouse a soul and move on to BodySelection
    SoulSelection,
    // Chouse a body to house your chosen soul
    BodySelection,
    //Setup Defensis and what not 
    Prepration,
    // Pick Trials to forge your Servivor gameplay 
    Tribulation,
    // Long turm advancment
    SoulReinforcement,
    // Run turm advancment
    BodyReinforcement,
    // Trigers once death. stats and unlocks
    LifeRecap,
}


#[derive(Resource, Default)]
pub enum GameTimeMode {
    #[default]
    Short,
    Normal,
    Long,
    Endless
}

impl GameTimeMode {
    pub fn get_time(&self) -> Timer {
        use GameTimeMode::*;
        match *self {
            Short => Timer::new(Duration::from_secs(60 * 15), TimerMode::Once),
            Normal => Timer::new(Duration::from_secs(60 * 15), TimerMode::Once),
            Long => Timer::new(Duration::from_secs(60 * 15), TimerMode::Once),
            Endless => Timer::new(Duration::from_secs(60 * 60 * 24), TimerMode::Once),
        }
    }
}

#[derive(Resource)]
pub struct GameTime(pub Timer);

fn init_game_time(
    mut commands: Commands,
    time_mode: Res<GameTimeMode>,
    mut ui: EventWriter<NewTimeUi>
) {
    commands.insert_resource(GameTime(time_mode.get_time()));
    ui.send(NewTimeUi);
}

///If there is a game time update it 
///if finished remove game time and move to the next state
fn update_game_time(
    mut commands: Commands,
    mut game_time: ResMut<GameTime>,
    time: Res<Time>,
    mut next_state: ResMut<NextState<GamePhase>>,
    mut ui: EventWriter<DeleteTimeUi>,
) {
    game_time.0.tick(time.delta());

    if game_time.0.finished() {
        commands.remove_resource::<GameTime>();
        next_state.set(GamePhase::LifeRecap);
    }
}
