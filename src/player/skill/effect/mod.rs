use crate::prelude::*;

mod spawner;
mod duration;
mod texture;
mod skill_effect_type;

use super::SkillType;
use super::SkillTarget;
use spawner::*;
use duration::*;
use skill_effect_type::SkillEffectType;

pub use spawner::SpawnSkillEffect;
pub use texture::SkillEffectTexture;

pub struct SkillEffectPlugin;
impl Plugin for SkillEffectPlugin{
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                Update, 
                (
                    skill_effect_duration_remover,
                    animate_duration_skills
                ).distributive_run_if(in_state(AppState::Game)))
            .add_plugins(
                SkillEffectSpawnerPlugin
            )
    ;}
}
