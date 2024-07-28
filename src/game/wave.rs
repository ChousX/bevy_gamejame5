use crate::prelude::*;

pub struct WavePlugin;
impl Plugin for WavePlugin{
    fn name(&self) -> &str {
        "Mob Wave Spawning Plugin"
    }
    fn build(&self, app: &mut App) {
        
    }
}

pub struct WaveTimer(pub Timer);

fn update_wave_timer(){}


