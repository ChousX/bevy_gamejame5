use crate::{player::{SkillActionRate, SkillLevel}, prelude::*};
use super::SkillType;

pub struct SkillSpawnerPlugin;
impl Plugin for SkillSpawnerPlugin{
    fn build(&self, app: &mut App) {
        app
            .add_event::<SpawnSkill>()
            .add_systems(
                Update, 
                spawn_skill.run_if(on_event::<SpawnSkill>()))
    ;}
}

#[derive(Event, Default)]
pub struct SpawnSkill(pub SkillType);


pub fn spawn_skill(
    mut commands: Commands,
    mut spawn_events: EventReader<SpawnSkill>,
) {
    use SkillType::*;
    for SpawnSkill(skill_type) in spawn_events.read(){
        match *skill_type {
            MagicBoom => {
                commands.spawn((
                    SkillActionRate(Timer::from_seconds(1.0, TimerMode::Repeating)),
                    SkillType::MagicBoom,
                    SkillLevel::new(10),
                ));
            },
        }
    }
}
