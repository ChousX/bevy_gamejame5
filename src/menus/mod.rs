use crate::prelude::*;
mod main_menu;
pub use main_menu::*;

pub struct MenuPlugin;
impl Plugin for MenuPlugin{
    fn build(&self, app: &mut App) {
        app.add_plugins((
            main_menu::MainMenuPlugin,
        ));
    }
}

#[derive(Component)]
pub enum MenuAction{
    Play,
    Load,
    Options,
    About,
}

fn button_system_color(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut image) in &mut interaction_query {
        image.0 = match *interaction {
            Interaction::Pressed => css::BLACK.into(),
            Interaction::Hovered => css::WHITE_SMOKE.into(),
            Interaction::None => css::GRAY.into(),
        }
    }
}

fn button_system_action(
    mut interaction_query: Query<
        (&Interaction, &MenuAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<AppState>>,
){
    for (interaction, action) in &interaction_query {
        if let Interaction::Pressed = interaction {
            match action {
                MenuAction::Play => {
                    next_state.set(AppState::Game)
                },
                MenuAction::Load => {},
                MenuAction::Options => {},
                MenuAction::About => {},
            }
        }
    }
}

fn despawn_all<T: Component>( query: Query<Entity, With<T>>, mut commands: Commands) {
    for e in query.iter(){
        commands.entity(e).despawn_recursive()
    }
}