use crate::prelude::*;

mod personality;
pub use personality::*;

mod gender;
pub use gender::*;

mod failings;
pub use failings::*;

pub struct SoulPlugin;
impl Plugin for SoulPlugin{
    fn build(&self, app: &mut App) {
        
    }
}
