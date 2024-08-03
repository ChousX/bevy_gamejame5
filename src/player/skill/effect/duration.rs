use crate::prelude::*;
use std::time::Duration;

#[derive(Component, Clone)]
pub struct SkillEffectDuration{
    timer: Timer,
    tick_count: u32,
    current: u32,
    finished: bool,
}

impl SkillEffectDuration {
    pub fn new(seconds: f32, tick_count: u32) -> Self{
        let timer = Timer::from_seconds(seconds, TimerMode::Repeating);
        Self {
            timer,
            tick_count,
            current: 0,
            finished: false
        }
    }

    pub fn tick(&mut self, duration: Duration){
        self.timer.tick(duration);
        self.finished = self.timer.finished();
        if self.finished {
            self.current += 1; 
        }
    }

    pub fn is_finished(&self) -> Option<bool>{
        if self.current < self.tick_count {
            Some(self.finished)
        } else {
            None
        }
    }

    pub fn get_index(&self) -> usize {
        if self.current == 0 {
            dbg!();
        } 
        self.current as usize
    }
}

/// Animates all skills with duration 
pub fn animate_duration_skills(
    time: Res<Time>,
    mut skills: Query<(&mut TextureAtlas, &mut SkillEffectDuration)>
){
    for (mut texture, mut duration) in skills.iter_mut(){
        duration.tick(time.delta());
        if let Some(true) = duration.is_finished() {
            texture.index = duration.get_index();
        }
    }
}

///A system that removes the entity and its children of an
///expired skill effect duration
pub fn skill_effect_duration_remover(
    mut commands: Commands,
    durations: Query<(&SkillEffectDuration, Entity), Changed<SkillEffectDuration>>
){
    for (duration, id) in durations.iter() {
        if let None = duration.is_finished() {
            commands.entity(id).despawn_recursive();
        }
    }
}



