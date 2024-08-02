use crate::prelude::*;
use bevy_asset_loader::prelude::*;

pub struct SkillPlugin;
impl Plugin for SkillPlugin{
    fn build(&self, app: &mut App) {
        app
            .init_resource::<SkillTexture>()
    ;}
}

#[derive(AssetCollection, Resource)]
pub struct SkillTexture{
    #[asset(texture_atlas_layout(tile_size_x = 64, tile_size_y = 64, columns = 17, rows = 1))]
    pub boom_layout: Handle<TextureAtlasLayout>,

    #[asset(image(sampler = nearest))]
    #[asset(path = "skills/boom.png")]
    pub boom_texture: Handle<Image>,
}

pub struct SkillBundle{
    pub sprite: SpriteBundle,

}
