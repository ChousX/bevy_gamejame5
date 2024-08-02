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

#[derive(Component, Clone)]
pub struct SkillActionRate(pub Timer);

#[derive(Component, Copy, Clone)]
pub struct SkillDamage(pub f32);

#[derive(Component, Default, Copy, Clone)]
pub enum SkillType{
    #[default]
    Boom
}


