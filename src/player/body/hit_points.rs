use crate::prelude::*;

#[derive(Component, Clone, Copy)]
pub struct HitPoints{
    pub max: f32,
    pub current: f32,
}
impl  Default for HitPoints {
    fn default () -> Self{
        const VAL: f32 = 100.0;
        Self{
            max: VAL,
            current: VAL,
        }
    }
}

impl HitPoints {
    pub const fn new(ammount: f32) -> Self {
        Self {
            max: ammount,
            current: ammount,
        }
    }

    pub fn damage(&mut self, ammount: f32) {
        //no healling form damge
        self.current -= ammount.abs();
    }

    pub fn is_dead(&self) -> bool{
        self.current < 0.0
    }

    pub fn is_full(&self) -> bool {
        self.current >= self.max
    }

    pub fn heal(&mut self, ammount: f32) {
        self.current += ammount;
        if self.current > self.max {
            self.current = self.max;
        }
    }

    pub fn remaining(&self) -> f32{
        (self.current / self.max) * 100.0
    }
}


