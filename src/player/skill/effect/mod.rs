use crate::prelude::*;

mod spawner;
mod duration;
mod texture;
mod skill_effect_type;

use super::SkillType;
use spawner::*;
use duration::*;
use skill_effect_type::SkillEffectType;

pub use spawner::SpawnSkillEffect;
pub use texture::SkillEffectTexture;

pub struct SkillEffectPlugin;
impl Plugin for SkillEffectPlugin{
    fn build(&self, app: &mut App) {
        app
            .add_event::<SpawnSkillEffect>()
            .add_systems(
                Update, 
                spawn_skill_effect.run_if(on_event::<SpawnSkillEffect>()))
            .add_systems(
                Update, 
                skill_effect_duration_remover.run_if(in_state(AppState::Game)))
    ;}
}
