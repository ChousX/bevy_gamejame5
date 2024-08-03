use crate::prelude::*;
use bevy_asset_loader::prelude::*;
use std::time::Duration;

mod spawner;
mod effect;

use spawner::SkillSpawnerPlugin;
use effect::SkillEffectPlugin;
pub use effect::SkillEffectTexture;

pub struct SkillPlugin;
impl Plugin for SkillPlugin{
    fn build(&self, app: &mut App) {
        app.add_plugins((
            SkillSpawnerPlugin,
            SkillEffectPlugin,
        ))
    ;}
}

#[derive(Component, Copy, Clone)]
pub enum SkillTarget{
    Entity(Entity),
    Position(Vec2),
}

#[derive(Component, Copy, Clone)]
pub struct SkillAreaOfEffectSize(pub f32);

#[derive(Component, Clone)]
pub struct SkillActionRate(pub Timer);

#[derive(Component, Copy, Clone)]
pub struct SkillDamage(pub f32);

#[derive(Component, Copy, Clone)]
pub struct SkillLevel{
    max: u8,
    current: u8,
}

impl SkillLevel {
    fn new(max: u8) -> Self{
        Self {
            max: max.into(),
            current: 0,
        }
    }

    ///Incroment Level if posible else return false
    pub fn level_up(&mut self) -> bool {
        if self.current < self.max{
            self.current += 1;
            true
        } else {
            false
        }
    }

    fn get_level(&self) -> u8 {
        self.current
    }
}

#[derive(Component, Default, Copy, Clone)]
pub enum SkillType{
    #[default]
    MagicBoom,
}


