use crate::prelude::*;

mod living_things;
pub use living_things::*;

pub struct BodyPlugin;
impl Plugin for BodyPlugin {
    fn build(&self, app: &mut App) {
        
    }
}

#[derive(Component)]
pub struct Body;

