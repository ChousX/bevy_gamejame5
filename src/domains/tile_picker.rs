use crate::{cursor, game::GamePhase, prelude::*};
use bevy_ecs_tilemap::prelude::*;

pub struct TilePickerPlugin;
impl Plugin for TilePickerPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<OverTile>()
            .init_resource::<TileWorldPos>()
            .add_systems(
                Update, 
                update_over_tile_and_tile_world_pos.run_if(in_state(AppState::Game)));
    }
}

#[derive(Default, Resource)]
pub struct OverTile(pub Option<Entity>);

#[derive(Default, Resource)]
pub struct TileWorldPos(pub Option<Vec2>);

fn update_over_tile_and_tile_world_pos(
    cursor_pos: Res<CursorPosition>,
    mut over_tile: ResMut<OverTile>,
    mut tile_world_pos: ResMut<TileWorldPos>,
    tilemap_q: Query<(
        &TilemapSize,
        &TilemapGridSize,
        &TilemapType,
        &TileStorage,
        &Transform,
    )>,
){
    if cursor_pos.is_changed(){
        let (map_size, grid_size, map_type, tile_storage, map_transform) = tilemap_q.single();
        if let Some(cursor_pos) = cursor_pos.0 {
            let cursor_in_map_pos: Vec2 = {
                // Extend the cursor_pos vec3 by 0.0 and 1.0
                let cursor_pos = Vec4::from((cursor_pos, 0.0, 1.0));
                let cursor_in_map_pos = map_transform.compute_matrix().inverse() * cursor_pos;
                cursor_in_map_pos.xy()
            };
            tile_world_pos.0 = Some(cursor_in_map_pos);

            if let Some(tile_pos) = 
                TilePos::from_world_pos(&cursor_in_map_pos, map_size, grid_size, map_type) {
                    if let Some(tile_entity) = tile_storage.get(&tile_pos){
                        over_tile.0 = Some(tile_entity)
                    }            }
        } else {
            tile_world_pos.0 = None;
            over_tile.0 = None;
        }
    }
}
