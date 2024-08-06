use bevy::ecs::query;
use crate::helpers::despawn_all;

use crate::{helpers, player::{BodyRoot, HitPoints, Size}, prelude::*};

use super::{GamePhase, GameUiBottomSection};

pub struct HitPointsUiPlugin;
impl Plugin for HitPointsUiPlugin{
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                OnEnter(GamePhase::Tribulation), 
                init_hp_ui)
            .add_systems(
                Update, 
                update_hp_ui.run_if(in_state(GamePhase::Tribulation)))
            .add_systems(
                OnExit(GamePhase::Tribulation), 
                despawn_all::<HPBar>)
    ;}
}

#[derive(Component, Clone, Copy)]
struct HPInnerBar;

#[derive(Component, Clone, Copy)]
struct HPBar;

fn init_hp_ui(
    mut commands: Commands,
    players: Query<(Entity, &HitPoints, &Size)>,
    root: Query<Entity, With<GameUiBottomSection>>
) {
    let entity = root.single();
    for (_, hp, _) in players.iter(){
        let bar = commands.spawn((
            NodeBundle {
                style: Style {
                    width: Val::Px(hp.max + 100.0),
                    height: Val::Px(25.0),
                    ..default()
                },
                background_color: css::SNOW.into(),
                ..default()
            },
            HPBar,
        )).id();

        commands.entity(entity).add_child(bar);

        let red_stuff = commands.spawn((
                NodeBundle{
                    style: Style {
                        width: Val::Percent(hp.remaining()),
                        height: Val::Percent(100.0),
                        ..default()
                    },
                    background_color: css::CRIMSON.into(),
                    ..default()
                },
                HPInnerBar,
        )).id();

        commands.entity(bar).add_child(red_stuff);
    }
}

fn update_hp_ui(
    players: Query<&HitPoints, Changed<HitPoints>>,
    mut red_stuff: Query<&mut Style, With<HPInnerBar>>
){
    let hp = players.single();
    let mut red_stuff = red_stuff.single_mut();
    red_stuff.width = Val::Percent(hp.remaining());
}
