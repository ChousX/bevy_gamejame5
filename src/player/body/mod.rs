use crate::prelude::*;

pub struct BodyPlugin;
impl Plugin for BodyPlugin{
    fn name(&self) -> &str {
        "Body Plugin"
    }

    fn build(&self, app: &mut App) {
        
    }
}


#[derive(Component, Default)]
pub struct BodyRoot;

pub struct BodySpeed(pub f32);
impl Default for BodySpeed{
    fn default() -> Default{
        Self(1.0)
    }
}

pub struct BodyHitPoints{
    pub max: f32,
    pub current: f32,
}
impl  Default for BodyHitPoints {
    fn default () -> Self{
        const VAL: f32 = 100.0;
        Self{
            max: VAL,
            current: VAL,
        }
    }
}


pub struct BodyHitPointRegeneration(pub f32);
impl Default for BodyHitPointRegeneration{
    fn default () -> Self{
        Self(0.1)
    }
}

pub struct BodySize(pub f32);
impl Default for BodySize{
    fn default() -> Self{
        Self(1.0)
    }
}

#[derive(Bundle, Default)]
pub struct BodyBundle{
    pub root: BodyRoot,
    pub speed: BodySpeed,
    pub hit_points: BodyHitPoints,
    pub hit_points_regen: BodyHitPointRegeneration,
    pub size: BodySize,
}
