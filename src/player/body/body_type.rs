use crate::{helpers:: AnimationTimer, prelude::*};
use rand::{
    prelude::*,
    seq::SliceRandom,
};

use strum::{
    VariantArray,
    IntoStaticStr,
};

use bevy_asset_loader::prelude::*;

use super::*;





#[derive(AssetCollection, Resource)]
pub struct BodyTexture{

    #[asset(texture_atlas_layout(tile_size_x = 16, tile_size_y = 16, columns = 4, rows = 1))]
    pub chicken_layout: Handle<TextureAtlasLayout>,
    #[asset(image(sampler = nearest))]
    #[asset(path = "sprites/CluckingChickenIdleSide.png")]
    pub chicken: Handle<Image>,


    #[asset(texture_atlas_layout(tile_size_x = 16, tile_size_y = 16, columns = 4, rows = 1))]
    pub crab_layout:  Handle<TextureAtlasLayout>,
    #[asset(image(sampler = nearest))]
    #[asset(path = "sprites/CoralCrabIdleSide.png")]
    pub crab:  Handle<Image>,

    #[asset(texture_atlas_layout(tile_size_x = 16, tile_size_y = 16, columns = 4, rows = 1))]
    pub toad_layout: Handle<TextureAtlasLayout>,
    #[asset(image(sampler = nearest))]
    #[asset(path = "sprites/CroakingToadIdleSide.png")]
    pub toad: Handle<Image>,

    #[asset(texture_atlas_layout(tile_size_x = 16, tile_size_y = 16, columns = 4, rows = 1))]
    pub monkey_layout: Handle<TextureAtlasLayout>,
    #[asset(image(sampler = nearest))]
    #[asset(path = "sprites/CuriousMonkeyIdleSide.png")]
    pub monkey: Handle<Image>,

    #[asset(texture_atlas_layout(tile_size_x = 16, tile_size_y = 16, columns = 4, rows = 1))]
    pub pig_layout: Handle<TextureAtlasLayout>,
    #[asset(image(sampler = nearest))]
    #[asset(path = "sprites/DaintyPigIdleSide.png")]
    pub pig: Handle<Image>,

    #[asset(texture_atlas_layout(tile_size_x = 16, tile_size_y = 16, columns = 4, rows = 1))]
    pub dog_layout: Handle<TextureAtlasLayout>,
    #[asset(image(sampler = nearest))]
    #[asset(path = "sprites/FaithfulDogIdleSide.png")]
    pub dog: Handle<Image>,

    #[asset(texture_atlas_layout(tile_size_x = 16, tile_size_y = 16, columns = 4, rows = 1))]
    pub cow_layout: Handle<TextureAtlasLayout>,
    #[asset(image(sampler = nearest))]
    #[asset(path = "sprites/GrazingCowIdleSide.png")]
    pub cow: Handle<Image>,

    #[asset(texture_atlas_layout(tile_size_x = 16, tile_size_y = 16, columns = 4, rows = 1))]
    pub goose_layout: Handle<TextureAtlasLayout>,
    #[asset(image(sampler = nearest))]
    #[asset(path = "sprites/HonkingGooseIdleSide.png")]
    pub goose: Handle<Image>,

    #[asset(texture_atlas_layout(tile_size_x = 32, tile_size_y = 32, columns = 8, rows = 1))]
    pub gorilla_layout: Handle<TextureAtlasLayout>,
    #[asset(image(sampler = nearest))]
    #[asset(path = "sprites/HulkingGorillaIdleSide.png")]
    pub gorilla: Handle<Image>,

    #[asset(texture_atlas_layout(tile_size_x = 16, tile_size_y = 16, columns = 4, rows = 1))]
    pub frog_layout: Handle<TextureAtlasLayout>,
    #[asset(image(sampler = nearest))]
    #[asset(path = "sprites/LeapingFrogIdleSide.png")]
    pub frog: Handle<Image>,

    #[asset(texture_atlas_layout(tile_size_x = 16, tile_size_y = 16, columns = 4, rows = 1))]
    pub boar_layout: Handle<TextureAtlasLayout>,
    #[asset(image(sampler = nearest))]
    #[asset(path = "sprites/MadBoarIdleSide.png")]
    pub boar: Handle<Image>,

    #[asset(texture_atlas_layout(tile_size_x = 32, tile_size_y = 32, columns = 8, rows = 1))]
    pub moose_layout: Handle<TextureAtlasLayout>,
    #[asset(image(sampler = nearest))]
    #[asset(path = "sprites/MajesticMooseIdleSide.png")]
    pub moose: Handle<Image>,

    #[asset(texture_atlas_layout(tile_size_x = 16, tile_size_y = 16, columns = 4, rows = 1))]
    pub cat_layout: Handle<TextureAtlasLayout>,
    #[asset(image(sampler = nearest))]
    #[asset(path = "sprites/MeowingCatIdleSide.png")]
    pub cat: Handle<Image>,

    #[asset(texture_atlas_layout(tile_size_x = 16, tile_size_y = 16, columns = 4, rows = 1))]
    pub gaot_layout: Handle<TextureAtlasLayout>,
    #[asset(image(sampler = nearest))]
    #[asset(path = "sprites/NibblingGoatIdleSide.png")]
    pub gaot: Handle<Image>,

    #[asset(texture_atlas_layout(tile_size_x = 16, tile_size_y = 16, columns = 4, rows = 1))]
    pub sheep_layout: Handle<TextureAtlasLayout>,
    #[asset(image(sampler = nearest))]
    #[asset(path = "sprites/PasturingSheepIdleSide.png")]
    pub sheep: Handle<Image>,

    #[asset(texture_atlas_layout(tile_size_x = 16, tile_size_y = 16, columns = 4, rows = 1))]
    pub turtle_layout: Handle<TextureAtlasLayout>,
    #[asset(image(sampler = nearest))]
    #[asset(path = "sprites/SlowTurtleIdleSide.png")]
    pub turtle: Handle<Image>,

    #[asset(texture_atlas_layout(tile_size_x = 16, tile_size_y = 16, columns = 4, rows = 1))]
    pub snow_fox_layout: Handle<TextureAtlasLayout>,
    #[asset(image(sampler = nearest))]
    #[asset(path = "sprites/SnowFoxIdleSide.png")]
    pub snow_fox: Handle<Image>,

    #[asset(texture_atlas_layout(tile_size_x = 16, tile_size_y = 16, columns = 4, rows = 1))]
    pub porcupine_layout: Handle<TextureAtlasLayout>,
    #[asset(image(sampler = nearest))]
    #[asset(path = "sprites/SpikeyPorcupineIdleSide.png")]
    pub porcupine: Handle<Image>,

    #[asset(texture_atlas_layout(tile_size_x = 16, tile_size_y = 16, columns = 4, rows = 1))]
    pub skunk_layout: Handle<TextureAtlasLayout>,
    #[asset(image(sampler = nearest))]
    #[asset(path = "sprites/StinkySkunkIdleSide.png")]
    pub skunk: Handle<Image>,

    #[asset(texture_atlas_layout(tile_size_x = 16, tile_size_y = 16, columns = 4, rows = 1))]
    pub wolf_layout: Handle<TextureAtlasLayout>,
    #[asset(image(sampler = nearest))]
    #[asset(path = "sprites/TimberWolfIdleSide.png")]
    pub wolf: Handle<Image>,

}


#[derive(Debug, Component,  IntoStaticStr, VariantArray, Copy, Clone, Default)]
#[strum(serialize_all = "kebab-case")]
pub enum BodyType {
    Chicken,
    #[default]
    Crab,
    Toad,
    Monkey,
    Pig,
    Dog,
    Cow,
    Goose,
    Gorilla,
    Frog,
    Boar,
    Moose,
    Cat,
    Goat,
    Sheep,
    Turtle,
    SnowFox,
    Porcupine,
    Skunk,
    Wolf
}

impl BodyType {
    pub fn get_text(&self) -> &'static str {
        let num = *self as usize;
        Self::VARIANTS[num].into()
    }

    pub fn gen_rand() -> Self {
        let mut rng = thread_rng();
        Self::VARIANTS.choose(&mut rng).expect("No VARIANTS").clone()
    }

    pub fn gen_speed(&self) -> Speed {
        use BodyType::*;
        match *self {
            Human => Speed(10.0),
            Dog => Speed(15.0),
            Bee => Speed(8.0),
            _ => Speed::default(),
        }
    }

    pub fn gen_hp(&self) -> HitPoints {
        use BodyType::*;
        match *self {
            Human => HitPoints::new(120.0),
            _ => HitPoints::default(),
        }
    }

    pub fn gen_hp_regen(&self) -> HitPointRegeneration {
        use BodyType::*;
        match *self {
            Human => HitPointRegeneration(1.0),
            _ => HitPointRegeneration::default(),
        }
    }

    pub fn gen_size(&self) -> Size {
        use BodyType::*;
        match *self {
            Human => Size(5.0),
            _ => Size::default(),
        }
    }
}

pub fn update_body_texutes_from_type(
    time: Res<Time>,
    mut sprites: Query<(&BodyType, &mut AnimationTimer, &mut TextureAtlas)>,
){
    for (body_type, mut timer, mut texture) in sprites.iter_mut() {
        let texture_num = match *body_type {
            BodyType::Crab => 4,
            _ => 4,
        };

        timer.0.tick(time.delta());
        if timer.0.finished() {
            texture.index = (texture.index + 1) % texture_num;
        }
    }
}
