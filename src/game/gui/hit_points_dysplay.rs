use crate::{helpers, player::{BodyRoot, HitPoints}, prelude::*};

use super::{GamePhase, GameUiBottomSection};

pub struct HitPointsDysplayPlugin;
impl Plugin for HitPointsDysplayPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                OnEnter(GamePhase::Tribulation), 
                init_hit_point_dysplay)
            .add_systems(
                OnExit(GamePhase::Tribulation), 
                helpers::despawn_all::<HitPointsCleanUp>)
            .add_systems(
                Update, 
                bar_update.run_if(in_state(GamePhase::Tribulation)));
    }
}

#[derive(Component)]
struct HitPointsCleanUp;

fn init_hit_point_dysplay(
    mut commands: Commands,
    menu_pos: Query<Entity, With<GameUiBottomSection>>,
    hp: Query<&HitPoints, With<BodyRoot>>
){
    let root = commands.spawn((
        NodeBundle {
            style: Style {
                ..default()
            },
            ..default()
        },
        HitPointsCleanUp,
    )).id();

    let menu_pos = menu_pos.single();
    commands.entity(menu_pos).add_child(root);

    let hp_bar = commands.spawn((
            NodeBundle {
                style: Style {
                    width: Val::Px(400.0),
                    height: Val::Px(20.0),
                    ..default()
                },
                ..default()
            },

    )).id();

    let hp = hp.single();

    let percent_there = hp.remaining();

    let bar_empty = commands.spawn((
        NodeBundle {
            style: Style {
                height: Val::Percent(100.0),
                width: Val::Percent(100.0 - percent_there),
                ..default()
            },
            background_color: css::GRAY.into(),
            ..default()
        },
        BarEmpty,
    )).id();

    let bar_full = commands.spawn((
            NodeBundle {
                style: Style {
                    height: Val::Percent(100.0),
                    width: Val::Percent(percent_there),
                    ..default()
                },
                ..default()
            },
            BarFull,
    )).id();

    commands.entity(root).add_child(hp_bar);
    commands.entity(hp_bar).add_child(bar_full);
    commands.entity(hp_bar).add_child(bar_empty);
}

#[derive(Component)]
struct BarFull;

#[derive(Component)]
struct BarEmpty;

fn bar_update(
    mut bar_empty: Query<&mut Style, With<BarEmpty>>,
    mut bar_full: Query<&mut Style, With<BarFull>>,
    //a sneeky error is if there is more than one none will show
    hp: Query<&HitPoints, Changed<HitPoints>>
){
    if let Ok(hp) = hp.get_single(){
        let left = hp.remaining();
        let mut bar_empty = bar_empty.single_mut();
        let mut bar_full = bar_full.single_mut();

        bar_empty.width = Val::Percent(100.0 - left);
        bar_full.width = Val::Percent(left);
    }
}
