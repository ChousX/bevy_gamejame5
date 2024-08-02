use super::*;

#[derive(AssetCollection, Resource)]
pub struct SkillEffectTexture{
    #[asset(texture_atlas_layout(tile_size_x = 64, tile_size_y = 64, columns = 17, rows = 1))]
    pub boom_layout: Handle<TextureAtlasLayout>,

    #[asset(image(sampler = nearest))]
    #[asset(path = "skills/boom.png")]
    pub boom_texture: Handle<Image>,
}


