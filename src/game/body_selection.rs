use crate::player::*;

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
struct BodySelectionMenuCleanUp;

#[derive(Resource)]
pub struct BodySelectionAmmount(pub u8);
impl Default for BodySelectionAmmount {
    fn default() -> Self {
        Self(3)
    }
}

fn init_menu(
    mut commands: Commands,
    body_amount: Res<BodySelectionAmmount>,
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
        BodySelectionMenuCleanUp,
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
        BodySelectionMenuCleanUp,
    )).id();

    commands.entity(root).add_child(menu);
    

    for i in 0..body_amount.0 {
        let body_type = BodyType::gen_rand();
        let name_text = body_type.get_text();
        let body = commands.spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                background_color: css::GRAY.into(),
                ..default()
            },
            BodyBundle {
                speed: body_type.gen_speed(),
                hit_points: body_type.gen_hp(),
                hit_points_regen: body_type.gen_hp_regen(),
                size: body_type.gen_size(),
                body_type,
                ..default()
            },
        )).id();
        commands.entity(menu).add_child(body);

        let body_type_node = commands.spawn((
            TextBundle::from_section(name_text, TextStyle {
                ..default()
            }),
            BodySelectionMenuCleanUp
        )).id();
        commands.entity(menu).add_child(body_type_node);
    }
}

fn menu_selection(
    mut commands: Commands,
    mut interaction_query: Query<(&Interaction, &mut BackgroundColor, Entity), (With<BodyRoot>, Changed<Interaction>)>,
){
    for (interaction, mut color, entity) in interaction_query.iter_mut(){
        match *interaction {
            Interaction::None => {
                *color = css::GRAY.into();
            },
            Interaction::Hovered => {
                *color = css::WHITE_SMOKE.into();
            },
            Interaction::Pressed => {
                *color = css::BLUE.into();
                commands.entity(entity).insert(Selected);
            },
        }
    }
}

fn cleanup(){}
