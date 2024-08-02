use super::*;


#[derive(Event, Default)]
pub struct SpawnSkill(pub SkillType);


#[derive(Event)]
pub struct SpawnSkillEffect(pub SkillType);


pub fn spawn_skill(
    mut commands: Commands,
    mut spawn_events: EventReader<SpawnSkill>,
) {
    use SkillType::*;
    for SpawnSkill(skill_type) in spawn_events.read(){
        match *skill_type {
            Boom => {
                
            },
        }
    }
}

pub fn spawn_skill_effect(
    mut commands: Commands,
    mut spawn_events: EventReader<SpawnSkillEffect>,
    textures: Res<SkillTexture>,
) {
    use SkillType::*;
    for SpawnSkillEffect(skill_type) in spawn_events.read(){
        match *skill_type {
            Boom => {
                
            },
        }
    }
}
