use strum::{VariantNames, EnumString, VariantArray};
use crate::{game::GamePhase, prelude::*};
use bevy_ecs_tilemap::prelude::*;

pub struct DomainPlugin;
impl Plugin for DomainPlugin {
    fn name(&self) -> &str {
        "Domain Plugin"
    }

    fn build(&self, app: &mut App) {
        app
            .add_systems(
                OnEnter(GamePhase::Prepration),
                add_domain
            ).add_plugins(
                TilemapPlugin
            )
    ;}
}

pub fn add_domain(
    mut commands: Commands,
    assets: Res<TerrainTextures>
) {
    let map_size = TilemapSize { x: 10, y: 10 };
    let mut tile_storage = TileStorage::empty(map_size);
    let tilemap_entity = commands.spawn_empty().id();
    let tilemap_id = TilemapId(tilemap_entity);
    let tile_size = TilemapTileSize { x: 64.0, y: 32.0 };
    let grid_size = tile_size.into();
    let domain_type = ();
    let map_type = TilemapType::Isometric(IsoCoordSystem::Staggered);
    let TilemapSize { x: max_x, y: max_y } = map_size.clone();
    for y in 0..max_y {
        for x in 0..max_x {
            let tile_pos = TilePos {x, y};
            let tile = commands.spawn((
                    TileBundle {
                        position: tile_pos,
                        texture_index: TileTextureIndex(0),
                        tilemap_id: tilemap_id.clone(),
                        visible: TileVisible(true),
                        ..default()
                    },
            )).id();
            tile_storage.set(&tile_pos, tile);
        }
    }

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(assets.tiles.clone()),
        tile_size,
        map_type,
        transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0),
        ..Default::default()
    }).insert((

    ));
}

#[derive(AssetCollection, Resource)]
pub struct TerrainTextures{
    #[asset(path = "tilemaps/iso_color.png")]
    pub tiles: Handle<Image>,
}


#[derive(Debug, Component, EnumString, VariantNames, VariantArray)]
#[strum(serialize_all = "kebab-case")]
pub enum DomainType{
    Island,
    Mountans,
    Plains,
}

