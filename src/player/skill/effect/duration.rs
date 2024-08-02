use crate::prelude::*;
use std::time::Duration;

#[derive(Component, Clone)]
pub struct SkillEffectDuration{
    timer: Timer,
    tick_count: u32,
    finished: bool,
}

impl SkillEffectDuration {
    pub fn tick(&mut self, duration: Duration){
        self.timer.tick(duration);
        self.finished = self.timer.finished();
        if self.finished {
            self.tick_count -= 1; //if this errors I am not removing it in time
        }
    }

    pub fn is_finished(&self) -> Option<bool>{
        if self.tick_count != 0 {
            Some(self.finished)
        } else {
            None
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



