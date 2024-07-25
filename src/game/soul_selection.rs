use bevy::{transform::commands, ui::FocusPolicy};

use crate::{player::{SoulBundle, SoulName, SoulRoot}, prelude::*};

use super::{GamePhase, Selected};

pub struct SoulSelectionPlugin;
impl Plugin for SoulSelectionPlugin {
    fn name(&self) -> &str {
        "Soul Selection Plugin"
    }

    fn build(&self, app: &mut App) {
        app
            .init_resource::<SoulSelectionAmount>()
            .add_systems(
                OnEnter(GamePhase::SoulSelection),
                init_menu
            ).add_systems(
                Update,
                menu_soul_select.run_if(in_state(GamePhase::SoulSelection))
            ).add_systems(
            OnExit(GamePhase::SoulSelection), cleanup);
    }
}

#[derive(Component)]
pub struct SoulSelectionMenuCleanUp;

#[derive(Resource)]
pub struct SoulSelectionAmount(pub u8);
impl Default for SoulSelectionAmount {
    fn default () -> Self {
        Self(3)
    }
}


fn init_menu(
    mut commands: Commands,
    soul_amount: Res<SoulSelectionAmount>,
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
        SoulSelectionMenuCleanUp
    )).id();

    let menu = commands.spawn((
        NodeBundle{
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(60.0),
                ..default()
            },
            background_color: css::LIGHT_SLATE_GRAY.into(),
            ..default()
        },
        SoulSelectionMenuCleanUp
    )).id();

    commands.entity(root).add_child(menu);

    for i in 0..soul_amount.0{
        let name_text = crate::names::FIRST_NAMES[i as usize];
        let soul = commands.spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                background_color: css::GRAY.into(),
                ..default()
            },
            //add more things
            SoulBundle {
                name: SoulName(name_text.to_string()),
                ..default()
            },
        )).id();
        commands.entity(menu).add_child(soul);


        let name_node = commands.spawn((
            TextBundle::from_section(name_text, TextStyle {
                ..default()
            }),
            SoulSelectionMenuCleanUp
        )
        ).id();
        commands.entity(menu).add_child(name_node);
    }
}

fn menu_soul_select(
    mut commands: Commands,
    mut next_state: ResMut<NextState<GamePhase>>,
    mut interaction_query: Query<(&Interaction, &mut BackgroundColor, Entity), (With<SoulRoot>, Changed<Interaction>)>,
){
    for (interaction, mut color, entity) in interaction_query.iter_mut(){
        match *interaction{
            Interaction::None => {
                *color = css::GRAY.into();
            },
            Interaction::Hovered => {
                *color = css::WHITE_SMOKE.into();
            },
            Interaction::Pressed => {
                *color = css::BLUE.into();
                next_state.set(GamePhase::BodySelection);
                commands.entity(entity).insert(Selected);
                //cleanup menu
                commands.entity(entity)
                    .remove::<Node>()
                    .remove::<Button>()
                    .remove::<Style>()
                    .remove::<Interaction>()
                    .remove::<FocusPolicy>()
                    .remove::<BorderColor>()
                    .remove::<BorderRadius>()
                    .remove::<UiImage>()
                    .remove::<BackgroundColor>()
                    .remove::<Transform>()
                    .remove::<GlobalTransform>()
                    .remove::<Visibility>()
                    .remove::<InheritedVisibility>()
                    .remove::<ViewVisibility>()
                    .remove::<ZIndex>();
           },
        }
    }
}

fn cleanup(
    nodes: Query<Entity, With<SoulSelectionMenuCleanUp>>,
    buttons: Query<Entity, (With<Button>, Without<Selected>)>,
    mut commands: Commands,
){
    for node in nodes.iter(){
        commands.entity(node).despawn();
    }
    for button in buttons.iter(){
        commands.entity(button).despawn();
    }
}
