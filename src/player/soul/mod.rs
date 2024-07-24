use crate::prelude::*;

pub struct SoulPlugin;
impl Plugin for SoulPlugin{
    fn name(&self) -> &str {
        "Soul Plugin"    
    }

    fn build(&self, app: &mut App) {
        
    }
}

pub struct SoulRoot;

pub struct SoulName(pub String);

pub struct SoulMagnetism(pub f32);

pub struct SoulPresence(pub f32);

pub struct SoulBundle{
    pub root: SoulRoot,
    pub name: SoulName,
    pub magnetism: SoulMagnetism,
    pub presence: SoulPresence,
}
