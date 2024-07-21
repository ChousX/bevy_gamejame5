use strum::{VariantNames, EnumString, VariantArray};
use crate::prelude::*;
use bevy_ecs_tilemap::prelude::*;

pub struct DomainPlugin;
impl Plugin for DomainPlugin {
    fn name(&self) -> &str {
        "Domain Plugin"
    }

    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Game), add_domain)
    }
}

pub fn add_domain(mut commands: Commands, assets: Res<TerrainTextures>){
    let map_size = TilemapSize { x: 100, y: 100 };
    let mut tile_storage = TileStorage::empty(map_size);
    let tilemap_entity = commands.spawn_empty().id();
    let tilemap_id = TilemapId(tilemap_entity);
    let tile_size = TilemapTileSize { x: 64.0, y: 32.0 };
    let grid_size = tile_size.into();
    let map_type = TilemapType::Isometric(IsoCoordSystem::Staggered);

    fill_tilemap_rect(
        TileTextureIndex(0),
        TilePos { x: 0, y: 0 },
        map_size,
        tilemap_id,
        &mut commands,
        &mut tile_storage,
    );

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(texture_handle),
        tile_size,
        map_type,
        transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0),
        ..Default::default()
    });



}

#[derive(AssetCollection, Resource)]
pub struct TerrainTextures{
    #[asset(path = "todo!()")]
    tiles: Handle<Image>,
}


#[derive(Debug, Component, EnumString, VariantNames, VariantArray)]
#[strum(serialize_all = "kebab-case")]
pub enum DomainType{
    Island,
    Mountans,
    Plains,
}

pub struct DomainSize{
    tile_count: UVec2,
    tile_size: Vec2,
}

pub struct DomainBundle{

}

