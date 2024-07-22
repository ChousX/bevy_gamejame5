use crate::prelude::*;

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_sub_state::<GamePhase>()
    ;}
}

#[derive(SubStates, Clone, PartialEq, Eq, Hash, Debug, Default)]
#[source(AppState = AppState::Game)]
pub enum GamePhase{
    #[default]
    None,
    Pause,
}
