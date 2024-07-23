use body_selection::BodySelectionPlugin;
use soul_selection::SoulSelectionPlugin;

use crate::prelude::*;

mod soul_selection;
mod body_selection;

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_sub_state::<GamePhase>()
            .add_plugins((
                BodySelectionPlugin,
                SoulSelectionPlugin,
            ))
    ;}
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
    // Stop Game and give options
    Pause,
}
