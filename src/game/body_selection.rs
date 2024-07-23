use super::*;

pub struct BodySelectionPlugin;
impl Plugin for BodySelectionPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<BodySelectionAmmount>()
            .add_systems(
                OnEnter(GamePhase::BodySelection), 
                init_menu
            ).add_systems(
                Update, 
                menu_selection.run_if(in_state(GamePhase::BodySelection))
            );
    }
}

#[derive(Component)]
struct BodySelectionMenuRoot;

#[derive(Resource)]
pub struct BodySelectionAmmount(pub u8);
impl Default for BodySelectionAmmount {
    fn default() -> Self {
        Self(3)
    }
}

fn init_menu(){}

fn menu_selection(){}
