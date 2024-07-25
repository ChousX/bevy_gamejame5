use super::*;
use bevy::render::render_resource::*;


pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin{
    fn name(&self) -> &str {
        "Main Menu Plugin"
    }
    fn build(&self, app: &mut App) {
        app
            .add_plugins(UiMaterialPlugin::<CustomUiMaterial>::default())
            .add_systems(OnEnter(AppState::MainMenu), setup)
            .add_systems(
                Update,
                (button_system_action, button_system_color, main_menu_materials_update)
                    .distributive_run_if(
                        in_state(AppState::MainMenu)
                    )
            ).add_systems(
                OnExit(AppState::MainMenu), 
                despawn_all::<MainMenuRoot>
            );
    }
}

#[derive(Component)]
struct MainMenuRoot;

fn setup(
    mut commands: Commands,
    mut ui_materials: ResMut<Assets<CustomUiMaterial>>,
){

    let root = commands.spawn((MaterialNodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Row,
            ..default()
        },
        material: ui_materials.add(CustomUiMaterial {
            color: LinearRgba::WHITE.to_f32_array().into(),
        }),
        ..default()
    },
    MainMenuRoot,
    )).id();

    let padding0 = commands.spawn(NodeBundle {
        style: Style {
            height: Val::Percent(100.0),
            width: Val::Percent(30.0),
            ..default()
        },
        ..default()
    }).id();

    let padding1 = commands.spawn(NodeBundle {
        style: Style {
            height: Val::Percent(100.0),
            width: Val::Percent(30.0),
            ..default()
        },
        ..default()
    }).id();

    let padding2 = commands.spawn(NodeBundle {
        style: Style {
            height: Val::Percent(30.0),
            width: Val::Percent(100.0),
            ..default()
        },
        ..default()
    }).id();


    let padding3 = commands.spawn(NodeBundle {
        style: Style {
            height: Val::Percent(20.0),
            width: Val::Percent(100.0),
            ..default()
        },
        ..default()
    }).id();


    let main_menu_root = commands.spawn(
        NodeBundle {
            style: Style {
                width: Val::Percent(40.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        },
    ).id();

    let main_menu = commands.spawn(
            NodeBundle{
                style: Style {
                    width: Val::Px(500.0),
                    height: Val::Px(600.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                background_color: css::RED.into(),
                ..default()
            }
    ).id();

    {

        let title = commands.spawn(
            TextBundle::from_section(APP_NAME, TextStyle::default()).with_style(Style {
                ..default()
            }),
        ).id();
    
        let title_box = commands.spawn(NodeBundle {
            style: Style {
                width: Val::Px(500.0),
                height: Val::Px(100.0),
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Row,
                ..default()
            },
            ..default()
        }).id();

        let button_style = Style {
            width: Val::Px(250.0),
            height: Val::Px(65.0),
            margin: UiRect::all(Val::Px(20.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        };

        /*
        let button_icon_style = Style {
            width: Val::Px(30.0),
            // This takes the icons out of the flexbox flow, to be positioned exactly
            position_type: PositionType::Absolute,
            // The icon will be close to the left border of the button
            left: Val::Px(10.0),
            ..default()
        };
        */

        let button_text_style = TextStyle {
            font_size: 40.0,
            color: css::BLACK.into(),
            ..default()
        };

        let play = commands.spawn((
            ButtonBundle {
                style: button_style.clone(),
                background_color: css::NAVY.into(),
                ..default()
            },
            MenuAction::Play,
        )).id();

        let play_text = commands.spawn(TextBundle::from_section("Play", button_text_style.clone())).id();

        let load = commands.spawn((
            ButtonBundle {
                style: button_style.clone(),
                background_color: css::NAVY.into(),
                ..default()
            },
            MenuAction::Load,
        )).id();

        let load_text = commands.spawn(TextBundle::from_section("Load", button_text_style.clone())).id();

        let options = commands.spawn((
            ButtonBundle {
                style: button_style.clone(),
                background_color: css::NAVY.into(),
                ..default()
            },
            MenuAction::Options,
        )).id();

        let options_text = commands.spawn(TextBundle::from_section("Options", button_text_style.clone())).id();

        let options_box = commands.spawn(NodeBundle {
            style: Style {
                height: Val::Px(100.0),
                width: Val::Px(500.0),
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        }).id();

        let about = commands.spawn((
            ButtonBundle {
                style: button_style.clone(),
                background_color: css::NAVY.into(),
                ..default()
            },
            MenuAction::About,
        )).id();

        let about_text = commands.spawn(TextBundle::from_section("About", button_text_style.clone())).id();

        let about_box = commands.spawn(NodeBundle {
            style: Style {
                height: Val::Px(100.0),
                width: Val::Px(500.0),
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        }).id();

        let play_load_box = commands.spawn(NodeBundle {
            style: Style {
                height: Val::Px(100.0),
                width: Val::Px(500.0),
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            ..default()
        }).id();

        commands.entity(title_box).add_child(title);
        commands.entity(main_menu).add_child(title_box);

        commands.entity(play).add_child(play_text);
        commands.entity(load).add_child(load_text);
        commands.entity(play_load_box).add_child(play);
        commands.entity(play_load_box).add_child(load);
        commands.entity(main_menu).add_child(play_load_box);

        commands.entity(options).add_child(options_text);
        commands.entity(options_box).add_child(options);
        commands.entity(main_menu).add_child(options_box);

        commands.entity(about).add_child(about_text);
        commands.entity(about_box).add_child(about);
        commands.entity(main_menu).add_child(about_box);

    }

    commands.entity(root).add_child(padding0);
    commands.entity(root).add_child(main_menu_root);
    commands.entity(root).add_child(padding1);

    commands.entity(main_menu_root).add_child(padding2);
    commands.entity(main_menu_root).add_child(main_menu);
    commands.entity(main_menu_root).add_child(padding3);

}


fn main_menu_materials_update(time: Res<Time>, mut ui_materials: ResMut<Assets<CustomUiMaterial>>) {
    for (_, material) in ui_materials.iter_mut() {
        // rainbow color effect
        let new_color = Color::hsl((time.elapsed_seconds() * 60.0) % 360.0, 1., 0.5);
        material.color = LinearRgba::from(new_color).to_f32_array().into();
    }
}

#[derive(Component)]
pub struct MainMenu;

#[derive(AsBindGroup, Asset, TypePath, Debug, Clone)]
struct CustomUiMaterial {
    #[uniform(0)]
    color: Vec4,
}

impl UiMaterial for CustomUiMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/main_menu.wgsl".into()
    }
}

