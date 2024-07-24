use crate::prelude::*;

pub struct BodyPlugin;
impl Plugin for BodyPlugin{
    fn name(&self) -> &str {
        "Body Plugin"
    }

    fn build(&self, app: &mut App) {
        
    }
}


#[derive(Component, Default, Clone, Copy)]
pub struct BodyRoot;

#[derive(Component, Clone)]
pub struct Speed(pub f32);
impl Default for Speed{
    fn default() -> Self{
        Self(1.0)
    }
}

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


#[derive(Component, Clone, Copy)]
pub struct HitPointRegeneration(pub f32);
impl Default for HitPointRegeneration{
    fn default () -> Self{
        Self(0.1)
    }
}

#[derive(Component, Clone, Copy)]
pub struct Size(pub f32);
impl Default for Size{
    fn default() -> Self{
        Self(1.0)
    }
}

#[derive(Bundle, Default)]
pub struct BodyBundle{
    pub root: BodyRoot,
    pub speed: Speed,
    pub hit_points: HitPoints,
    pub hit_points_regen: HitPointRegeneration,
    pub size: Size,
}
