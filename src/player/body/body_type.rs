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
    pub goat_layout: Handle<TextureAtlasLayout>,
    #[asset(image(sampler = nearest))]
    #[asset(path = "sprites/NibblingGoatIdleSide.png")]
    pub goat: Handle<Image>,

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
            Chicken => Speed(5.0),
            Crab=> Speed(7.0),
            Toad=> Speed(5.0),
            Monkey => Speed(8.0),
            Pig => Speed(10.0),
            Dog => Speed(14.0),
            Cow => Speed(6.0),
            Goose => Speed(10.0),
            Gorilla => Speed(9.0),
            Frog => Speed(6.0),
            Boar => Speed(9.0),
            Moose => Speed(12.0),
            Cat => Speed(11.0),
            Goat => Speed(8.0),
            Sheep => Speed(7.0),
            Turtle => Speed(1.0),
            SnowFox => Speed(16.0),
            Porcupine => Speed(3.0),
            Skunk => Speed(6.0),
            Wolf => Speed(15.0),
        }
    }

    pub fn gen_hp(&self) -> HitPoints {
        use BodyType::*;
        match *self {
            Chicken => HitPoints::new(25.0),
            Crab=> HitPoints::new(30.0),
            Toad=> HitPoints::new(20.0),
            Monkey => HitPoints::new(30.0),
            Pig => HitPoints::new(35.0),
            Dog => HitPoints::new(30.0),
            Cow => HitPoints::new(36.0),
            Goose => HitPoints::new(30.0),
            Gorilla => HitPoints::new(120.0),
            Frog => HitPoints::new(15.0),
            Boar => HitPoints::new(100.0),
            Moose => HitPoints::new(100.0),
            Cat => HitPoints::new(25.0),
            Goat => HitPoints::new(35.0),
            Sheep => HitPoints::new(35.0),
            Turtle => HitPoints::new(100.0),
            SnowFox => HitPoints::new(20.0),
            Porcupine => HitPoints::new(75.0),
            Skunk => HitPoints::new(30.0),
            Wolf => HitPoints::new(65.0),
        }
    }

    pub fn gen_hp_regen(&self) -> HitPointRegeneration {
        use BodyType::*;
        let ammount = match *self {
            Chicken => 0.01,
            Crab => 0.0,
            Toad => 0.01,
            Monkey => 0.01,
            Pig => 0.2,
            Dog => 0.01,
            Cow => 0.01,
            Goose => 0.0,
            Gorilla => 0.01,
            Frog => 0.01,
            Boar => 0.1,
            Moose => 0.05,
            Cat => 0.01,
            Goat => 0.01,
            Sheep => 0.01,
            Turtle => 0.01,
            SnowFox => 0.01,
            Porcupine => 0.05,
            Skunk => 0.01,
            Wolf => 0.01,
        };
        HitPointRegeneration(ammount)
    }

    pub fn gen_size(&self) -> Size {
        use BodyType::*;
        let ammount = match *self {
            Chicken => 0.5,
            Crab => 0.25,
            Toad => 0.25,
            Monkey => 1.0,
            Pig => 1.0,
            Dog => 0.5,
            Cow => 1.5,
            Goose => 0.75,
            Gorilla => 2.0,
            Frog => 0.2,
            Boar => 1.2,
            Moose => 3.00,
            Cat => 0.37,
            Goat => 1.0,
            Sheep => 1.0,
            Turtle => 0.35,
            SnowFox => 0.39,
            Porcupine => 0.43,
            Skunk => 0.35,
            Wolf => 1.0,
        };
        Size(ammount)
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
