use crate::prelude::*;

mod personality_traits;
pub use personality_traits::*;

mod gender;
pub use gender::*;

pub struct SoulPlugin;
impl Plugin for SoulPlugin{
    fn build(&self, app: &mut App) {
        
    }
}
