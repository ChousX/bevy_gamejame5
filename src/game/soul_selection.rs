use crate::prelude::*;

use super::GamePhase;

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
            );
    }
}

#[derive(Componant)]
pub struct SoulSelectionMenuRoot;

#[derive(Recorce)]
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
        SoulSelectionMenuRoot
    )).id();

    let menu = commands.spawn(
        NodeBundle{
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(60.0),
                ..default()
            },
            background_color: css::LIGHT_SLATE_GRAY.into(),
            ..default()
        }
    ).id();

    commands.entity(root).add_child(menu);

    for _ in 0..soul_amount.0{
        let soul = commands.spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                ..default()
            },
            //add more things
        )).id();

        commands.entity(menu).add_child(soul);
    }
}

fn menu_soul_select(){}