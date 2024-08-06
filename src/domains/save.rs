use crate::prelude::*;
use serde::{Deserialize, Serialize};
use bevy_ecs_tilemap::prelude::*;
use std::path::PathBuf;

pub const SAVE_DOMAIN_FOLDER: &str = "Saves/Domain";

pub struct DomainSavePlugin;
impl Plugin for DomainSavePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<SaveDomainEvent>()
            .add_systems(
                Update, 
                save_the_domain.run_if(on_event::<SaveDomainEvent>()))
    ;}
}

#[derive(Event, Default)]
pub struct SaveDomainEvent{
    pub name: String
}


#[derive(Debug, Deserialize, Serialize)]
pub struct StoredDomain(
    Vec<Vec<u32>>
);

fn save_the_domain(
    tile_storage: Query<(&TileStorage, &TilemapSize)>,
    tiles: Query<&TileTextureIndex>,
    mut events: EventReader<SaveDomainEvent>,
){
    for sde in events.read(){
        let (storage, size) = tile_storage.single();
        let mut out = Vec::with_capacity(size.y as usize);
        for y in 0..size.y {
            let mut row = Vec::with_capacity(size.x as usize);
            for x in 0..size.x {
                let pos = TilePos{x, y};
                if let Some(entity) = storage.get(&pos){
                    if let Ok(texture_index) = tiles.get(entity){
                        row.push(texture_index.0);
                    }
                }
            }
            out.push(row);
        }
    
        let save_data =  ron::to_string(&out).expect("faled to make .ron");
        let mut save_pos = PathBuf::from(SAVE_DOMAIN_FOLDER);
    
        // was planing on checking for file overlap but blahhh
        if let Ok(dir) = std::fs::read_dir(save_pos.as_path()){
            for entry in dir{
                //let entry = if let Ok(x) = entry {x} else {continue;};
            }
        } else {
            std::fs::create_dir_all(save_pos.as_path());
        };

        save_pos.push(sde.name.clone());


        std::fs::write(save_pos.as_path(), save_data);
    }
}

