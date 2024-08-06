use bevy::ecs::query;
use ron::de;
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
                    width: Val::Percent(hp.remaining()),
                    height: Val::Percent(5.0),
                    ..default()
                },
                background_color: css::SNOW.into(),
                ..default()
            },
            HPBar,
        )).id();

        let space0 = commands.spawn(NodeBundle{style: Style {width: Val::Percent(45.0), height: Val::Percent(100.0), ..default()}, ..default()}).id();
        let space1 = commands.spawn(NodeBundle{style: Style {width: Val::Percent(10.0), height: Val::Percent(100.0), ..default()}, ..default()}).id();
        let space2 = commands.spawn(NodeBundle{style: Style {width: Val::Percent(45.0), height: Val::Percent(100.0), ..default()}, ..default()}).id();
        let space3 = commands.spawn(NodeBundle{style: Style {width: Val::Percent(95.0), height: Val::Percent(100.0), ..default()}, ..default()}).id();
        let remaining = hp.remaining();
        let red_stuff = commands.spawn((
                NodeBundle{
                    style: Style {
                        width: Val::Percent(remaining),
                        height: Val::Percent(100.0),
                        ..default()
                    },
                    background_color: css::CRIMSON.into(),
                    ..default()
                },
                HPInnerBar,
        )).id();
        commands.entity(entity).add_child(space0);
        commands.entity(entity).add_child(space1);
        commands.entity(entity).add_child(space2);
        commands.entity(space1).add_child(space3);
        commands.entity(space1).add_child(bar);
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
