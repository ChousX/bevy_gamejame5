use crate::prelude::*;

pub struct SoulPlugin;
impl Plugin for SoulPlugin{
    fn name(&self) -> &str {
        "Soul Plugin"    
    }

    fn build(&self, app: &mut App) {
        
    }
}

#[derive(Component, Default, Copy, Clone)]
pub struct SoulRoot;

#[derive(Component, Clone)]
pub struct SoulName(pub String);
impl Default for SoulName{
    fn default() -> Self{
        Self("Anima".into())
    }
}

#[derive(Component, Copy, Clone)]
pub struct Magnetism(pub f32);
impl Default for Magnetism{
    fn default() -> Self{
        Self(1.0)
    }
}

#[derive(Component, Copy, Clone)]
pub struct Presence(pub f32);
impl Default for Presence{
    fn default() -> Self{
        Self(1.0)
    }
}

#[derive(Bundle, Default)]
pub struct SoulBundle{
    pub root: SoulRoot,
    pub name: SoulName,
    pub magnetism: Magnetism,
    pub presence: Presence,
}
