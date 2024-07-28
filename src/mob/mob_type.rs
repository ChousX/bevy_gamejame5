use crate::prelude::*;

use crate::helpers::AnimationTimer;

use strum::{
    VariantArray,
    IntoStaticStr,
};

#[derive(AssetCollection, Resource)]
pub struct MobTexture{
    #[asset(texture_atlas_layout(tile_size_x = 16, tile_size_y = 16, columns = 4, rows = 1))]
    pub grimlock_layout: Handle<TextureAtlasLayout>,
    #[asset(image(sampler = nearest))]
    #[asset(path = "sprites/BlindedGrimlock.png")]
    pub grimlock: Handle<Image>,

    #[asset(texture_atlas_layout(tile_size_x = 16, tile_size_y = 16, columns = 4, rows = 1))]
    pub eye_layout: Handle<TextureAtlasLayout>,
    #[asset(image(sampler = nearest))]
    #[asset(path = "sprites/BloodshotEye.png")]
    pub eye: Handle<Image>,

    #[asset(texture_atlas_layout(tile_size_x = 16, tile_size_y = 16, columns = 4, rows = 1))]
    pub ogre_layout: Handle<TextureAtlasLayout>,
    #[asset(image(sampler = nearest))]
    #[asset(path = "sprites/BrawnyOgre.png")]
    pub ogre: Handle<Image>,

    #[asset(texture_atlas_layout(tile_size_x = 16, tile_size_y = 16, columns = 4, rows = 1))]
    pub crimson_slaad_layout: Handle<TextureAtlasLayout>,
    #[asset(image(sampler = nearest))]
    #[asset(path = "sprites/CrimsonSlaad.png")]
    pub crimson_slaad: Handle<Image>,

    #[asset(texture_atlas_layout(tile_size_x = 16, tile_size_y = 16, columns = 4, rows = 1))]
    pub cyclops_layout: Handle<TextureAtlasLayout>,
    #[asset(image(sampler = nearest))]
    #[asset(path = "sprites/CrushingCyclops.png")]
    pub cyclops: Handle<Image>,

    #[asset(texture_atlas_layout(tile_size_x = 16, tile_size_y = 16, columns = 4, rows = 1))]
    pub slime_layout: Handle<TextureAtlasLayout>,
    #[asset(image(sampler = nearest))]
    #[asset(path = "sprites/DeathSlime.png")]
    pub slime: Handle<Image>,

    #[asset(texture_atlas_layout(tile_size_x = 16, tile_size_y = 16, columns = 4, rows = 1))]
    pub myconid_layout: Handle<TextureAtlasLayout>,
    #[asset(image(sampler = nearest))]
    #[asset(path = "sprites/FungalMyconid.png")]
    pub myconid: Handle<Image>,

    #[asset(texture_atlas_layout(tile_size_x = 16, tile_size_y = 16, columns = 4, rows = 1))]
    pub ettin_layout: Handle<TextureAtlasLayout>,
    #[asset(image(sampler = nearest))]
    #[asset(path = "sprites/HumongousEttin.png")]
    pub ettin: Handle<Image>,

    #[asset(texture_atlas_layout(tile_size_x = 16, tile_size_y = 16, columns = 4, rows = 1))]
    pub murky_slaad_layout: Handle<TextureAtlasLayout>,
    #[asset(image(sampler = nearest))]
    #[asset(path = "sprites/MurkySlaad.png")]
    pub murky_slaad: Handle<Image>,

    #[asset(texture_atlas_layout(tile_size_x = 16, tile_size_y = 16, columns = 4, rows = 1))]
    pub jelly_layout: Handle<TextureAtlasLayout>,
    #[asset(image(sampler = nearest))]
    #[asset(path = "sprites/OchreJelly.png")]
    pub jelly: Handle<Image>,

    #[asset(texture_atlas_layout(tile_size_x = 16, tile_size_y = 16, columns = 4, rows = 1))]
    pub watcher_layout: Handle<TextureAtlasLayout>,
    #[asset(image(sampler = nearest))]
    #[asset(path = "sprites/OcularWatcher.png")]
    pub watcher: Handle<Image>,

    #[asset(texture_atlas_layout(tile_size_x = 16, tile_size_y = 16, columns = 4, rows = 1))]
    pub red_cap_layout: Handle<TextureAtlasLayout>,
    #[asset(image(sampler = nearest))]
    #[asset(path = "sprites/RedCap.png")]
    pub red_cap: Handle<Image>,

    #[asset(texture_atlas_layout(tile_size_x = 16, tile_size_y = 16, columns = 4, rows = 1))]
    pub mushroom_layout: Handle<TextureAtlasLayout>,
    #[asset(image(sampler = nearest))]
    #[asset(path = "sprites/ShriekerMushroom.png")]
    pub mushroom: Handle<Image>,

    #[asset(texture_atlas_layout(tile_size_x = 16, tile_size_y = 16, columns = 4, rows = 1))]
    pub stone_troll_layout: Handle<TextureAtlasLayout>,
    #[asset(image(sampler = nearest))]
    #[asset(path = "sprites/StoneTroll.png")]
    pub stone_troll: Handle<Image>,

    #[asset(texture_atlas_layout(tile_size_x = 16, tile_size_y = 16, columns = 4, rows = 1))]
    pub swamp_troll_layout: Handle<TextureAtlasLayout>,
    #[asset(image(sampler = nearest))]
    #[asset(path = "sprites/SwampTroll.png")]
    pub swamp_troll: Handle<Image>,
}

impl MobTexture {
    pub fn get(&self, mob_type: &MobType) -> (Handle<Image>, Handle<TextureAtlasLayout>) {
        use MobType::*;
        match *mob_type{
            Grimlock => (self.grimlock.clone(), self.grimlock_layout.clone()),
            Eye => (self.eye.clone(), self.eye_layout.clone()),
            Ogre => (self.ogre.clone(), self.ogre_layout.clone()),
            CrimsonSlaad => (self.crimson_slaad.clone(), self.crimson_slaad_layout.clone()),
            Cyclops => (self.cyclops.clone(), self.cyclops_layout.clone()),
            Slime => (self.slime.clone(), self.slime_layout.clone()),
            Myconid => (self.myconid.clone(), self.myconid_layout.clone()),
            Ettin => (self.ettin.clone(), self.ettin_layout.clone()),
            MurkySlaad => (self.murky_slaad.clone(), self.murky_slaad_layout.clone()),
            Jelly => (self.jelly.clone(), self.jelly_layout.clone()),
            Watcher => (self.watcher.clone(), self.watcher_layout.clone()),
            RedCap => (self.red_cap.clone(), self.red_cap_layout.clone()),
            Mushroom => (self.mushroom.clone(), self.mushroom_layout.clone()),
            StoneTroll => (self.stone_troll.clone(), self.stone_troll_layout.clone()),
            SwampTroll => (self.swamp_troll.clone(), self.swamp_troll_layout.clone()),
        }
    }
}

#[derive(Debug, Component,  IntoStaticStr, VariantArray, Copy, Clone, Default)]
#[strum(serialize_all = "kebab-case")]
pub enum MobType{
    #[default]
    Grimlock,
    Eye,
    Ogre,
    CrimsonSlaad,
    Cyclops,
    Slime,
    Myconid,
    Ettin,
    MurkySlaad,
    Jelly,
    Watcher,
    RedCap,
    Mushroom,
    StoneTroll,
    SwampTroll,
}



pub fn update_mob_texutes_from_type(
    time: Res<Time>,
    mut sprites: Query<(&MobType, &mut AnimationTimer, &mut TextureAtlas)>,
){
    for (mob_type, mut timer, mut texture) in sprites.iter_mut() {
        let texture_num = match *mob_type {
            _ => 4,
        };

        timer.0.tick(time.delta());
        if timer.0.finished() {
            texture.index = (texture.index + 1) % texture_num;
        }
    }
}

