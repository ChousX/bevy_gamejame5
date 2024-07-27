use crate::prelude::*;

mod mob_type;

pub use mob_type::*;

pub struct MobPlugin;
impl Plugin for MobPlugin{
    fn name(&self) -> &str {
        "Mob Plugin"
    }
    fn build(&self, app: &mut App) {

    }
}
