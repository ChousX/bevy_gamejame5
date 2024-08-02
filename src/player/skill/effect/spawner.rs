use crate::prelude::*;
use super::SkillEffectType;

pub struct SkillEffectSpawnerPlugin;
impl Plugin for SkillEffectSpawnerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<SpawnSkillEffect>()
            .add_systems(
                Update, 
                spawn_skill_effect.run_if(on_event::<SpawnSkillEffect>()))
    ;}
}

#[derive(Event)]
pub struct SpawnSkillEffect(pub SkillEffectType);

pub fn spawn_skill_effect(
    mut commands: Commands,
    mut spawn_events: EventReader<SpawnSkillEffect>,
    textures: Res<SkillEffectTexture>,
) {
    use SkillEffectType::*;
    for SpawnSkillEffect(skill_type) in spawn_events.read(){
        match *skill_type {
            Boom => {
                
            },
        }
    }
}
