use crate::prelude::*;
mod personality;
pub use personality::*;

mod gender;
pub use gender::*;

pub struct SoulPlugin;
impl Plugin for SoulPlugin{
    fn build(&self, app: &mut App) {
        
    }
}
