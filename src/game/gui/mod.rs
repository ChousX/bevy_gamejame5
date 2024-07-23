use super::*;

mod stats_dysplay;
mod time_dysplay;
mod mini_map_dysplay;

pub use stats_dysplay::*;
pub use time_dysplay::*;
pub use mini_map_dysplay::*;

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                Startup,
                init_gui
            )
    ;}
}

#[derive(Component)]
pub struct GameUiRoot;


#[derive(Component)]
pub struct GameUiTopSection;

#[derive(Component)]
pub struct GameUiTopSectionLeft;

#[derive(Component)]
pub struct GameUiTopSectionMiddle;

#[derive(Component)]
pub struct GameUiTopSectionRight;


#[derive(Component)]
pub struct GameUiMidSection;

#[derive(Component)]
pub struct GameUiMidSectionLeft;

#[derive(Component)]
pub struct GameUiMidSectionRight;


#[derive(Component)]
pub struct GameUiBottomSection;

fn init_gui(
    mut commands: Commands,
) {
    let root = commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        },
        GameUiRoot,
    )).id();

    let top_section = commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(20.0),
                flex_direction: FlexDirection::Row,
                ..default()
            },
            ..default()
        },
        GameUiTopSection,
    )).id();

    let middle_section = commands.spawn((
        NodeBundle{
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(50.0),
                flex_direction: FlexDirection::Row,
                ..default()
            },
            ..default()
        },
        GameUiMidSection
    )).id();

    let bottom_section = commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(30.0),
                flex_direction: FlexDirection::Row,
                ..default()
            },
            ..default()
        },
        GameUiBottomSection,
    )).id();

    commands.entity(root).add_child(top_section);
    commands.entity(root).add_child(middle_section);
    commands.entity(root).add_child(bottom_section);

    let top_section_left = commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            ..default()
        },
        GameUiTopSectionLeft,
    )).id();

    let top_section_middle = commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            ..default()
        },
        GameUiTopSectionLeft,
    )).id();

    let top_section_right = commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            ..default()
        },
        GameUiTopSectionLeft,
    )).id();

    commands.entity(top_section).add_child(top_section_left);
    commands.entity(top_section).add_child(top_section_middle);
    commands.entity(top_section).add_child(top_section_right);

    let middle_section_left = commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(20.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        },
        GameUiMidSectionLeft,
    )).id();

    let middle_section_right = commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(20.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        },
        GameUiMidSectionRight,
    )).id();

    let middle_section_padding = commands.spawn(
        NodeBundle {
            style: Style {
                width: Val::Percent(60.0),
                height: Val::Percent(100.0),
                ..default()
            },
            ..default()
        }
    ).id();

    commands.entity(middle_section).add_child(middle_section_left);
    commands.entity(middle_section).add_child(middle_section_padding);
    commands.entity(middle_section).add_child(middle_section_right);
}

