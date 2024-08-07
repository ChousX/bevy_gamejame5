use bevy::ecs::query;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use bevy::transform::commands;
use css::{CRIMSON, DARK_GRAY};
use ron::de;
use crate::helpers::despawn_all;

use crate::{helpers, player::{BodyRoot, HitPoints, Size}, prelude::*};

use super::{GamePhase, GameUiBottomSection};

pub struct HitPointsUiPlugin;
impl Plugin for HitPointsUiPlugin{
    fn build(&self, app: &mut App) {
        app
            .add_event::<SpawnHPBar>()
            .add_systems(
                OnEnter(GamePhase::Tribulation), 
                add_to_players)
            .add_systems(
                Update,
                (
                    init_entity_bar.run_if(in_state(GamePhase::Tribulation)),
                    update_hp_bar.run_if(in_state(GamePhase::Tribulation)),
            ))
    ;}
}

fn add_to_players(
    players: Query<Entity, With<HitPoints>>,
    mut events: EventWriter<SpawnHPBar>,
) {
    for e in players.iter(){
        events.send(SpawnHPBar(e));
    }
}

#[derive(Component, Clone)]
struct HPInnerBar(Entity);

#[derive(Component, Clone, Copy)]
struct HPBar;

#[derive(Event)]
pub struct SpawnHPBar(pub Entity);

fn init_entity_bar(
    players: Query<&HitPoints>,
    mut events: EventReader<SpawnHPBar>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let crimson: Color = CRIMSON.into();
    let dark_gray: Color = DARK_GRAY.into();
    for SpawnHPBar(entity) in events.read(){
        let hp = players.get(*entity).expect("A Spawn Event For HPBar on an Entity without HitPoints");
        let bar_back = commands.spawn((
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Rectangle::new(hp.max, 4.0))),
                material: materials.add(dark_gray),
                transform: Transform::from_xyz(0.0, 32.0, 1.),
                ..default()
            },
            HPBar,
        )).id();
        commands.entity(*entity).add_child(bar_back);
        let bar = commands.spawn((
            MaterialMesh2dBundle{
                mesh: Mesh2dHandle(meshes.add(Rectangle::new(hp.current, 4.0))),
                material: materials.add(crimson),
                transform: Transform::from_xyz(0.0, 0.0, 0.95),
                ..default()
            },
            HPInnerBar(*entity),
        )).id();
        commands.entity(bar_back).add_child(bar);
    }
}

fn update_hp_bar(
    mut bars: Query<(&Mesh2dHandle, &HPInnerBar)>,
    hps: Query<&HitPoints, Changed<HitPoints>>, 
    mut meshes: ResMut<Assets<Mesh>>,
){
    for (handle, bar) in bars.iter(){
        if let Ok(hp) = hps.get(bar.0){
            let mut mesh = meshes.get_mut(&handle.0).unwrap();
            let positions = mesh
                .attribute(Mesh::ATTRIBUTE_POSITION).unwrap();
            let mut pos_new: Vec<[f32; 3]> = positions.as_float3().unwrap()
                .iter().map(|x| {[x[0], x[1], x[2]]}).collect();
            //todo V
            let v = hp.get_v_pos();
            pos_new[0][0] = v;
            pos_new[3][0] = v;
            mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, pos_new);
        }
         
    }
}
