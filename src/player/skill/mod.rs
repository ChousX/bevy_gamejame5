use crate::prelude::*;
use bevy_asset_loader::prelude::*;
use std::time::Duration;

mod spawner;
mod texture;
mod skill_effect_duration;

pub use spawner::*;
pub use texture::*;
pub use skill_effect_duration::*;

pub struct SkillPlugin;
impl Plugin for SkillPlugin{
    fn build(&self, app: &mut App) {
        app
            .add_event::<SpawnSkill>()
            .add_event::<SpawnSkillEffect>()
            .add_systems(
                Update, 
                skill_effect_entity_remover.run_if(in_state(AppState::Game)))
            .add_systems(
                Update, 
                (
                    spawn_skill.run_if(on_event::<SpawnSkill>()),
                    spawn_skill_effect.run_if(on_event::<SpawnSkillEffect>())
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


