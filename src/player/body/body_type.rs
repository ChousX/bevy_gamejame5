use crate::prelude::*;
use rand::prelude::*;
use strum::{
    EnumString,
    VariantNames,

};

use super::*;

#[derive(Debug, Component, EnumString, VariantNames, Copy, Clone, Default)]
#[strum(serialize_all = "kebab-case")]
pub enum BodyType {
    #[default]
    Human,
    Dog,
    Bee,
    Cat,
}

impl BodyType {
    pub fn get_text(&self) -> &'static str {
        let num = *self as usize;
        Self::VARIANTS[num]
    }

    pub fn gen_rand() -> Self {
        let rng = thread_rng();
        Self::Human
    }

    pub fn gen_speed(&self) -> Speed {
           use BodyType::*;
        match *self {
            Human => Speed(10.0),
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


