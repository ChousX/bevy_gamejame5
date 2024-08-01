use crate::{player::BodyRoot, prelude::*};
use super::*;
use bevy::utils::{HashMap, HashSet};
use rand::{thread_rng, Rng};


pub struct WavePlugin;
impl Plugin for WavePlugin{
    fn name(&self) -> &str {
        "Mob Wave Spawning Plugin"
    }
    fn build(&self, app: &mut App) {
        app
            .init_resource::<WaveTimer>()
            .add_systems(
                OnEnter(GamePhase::Prepration), 
                init_game_time)
            .add_systems(
                Update, 
                update_wave_timer.run_if(in_state(GamePhase::Tribulation)))
    ;}
}

#[derive(Resource)]
pub struct WaveTimer(pub Timer);
impl Default for WaveTimer {
    fn default() -> Self {
        Self(Self::timer_from_dif(0))
    }
}

fn init_wave_timer(
    difficulty: Res<Difficulty>,
    mut wave_timer: ResMut<WaveTimer>,
) {
    wave_timer.0 = WaveTimer::timer_from_dif(difficulty.0);
}

impl WaveTimer {
    pub fn timer_from_dif(dif: u64) -> Timer{
        let sec = match dif {
            0..10 => 10.0,
            10..20 => 8.0,
            20..30 => 6.0,
            30..40 => 4.0,
            40..50 => 3.0,
            50..60 => 2.0,
            60..70 => 1.0,
            70..80 => 0.8,
            80..90 => 0.6,
            90..100 => 0.4,
            100..110 => 0.3,
            110..120 => 0.2,
            _ => 0.1
        };
        Timer::from_seconds(sec, TimerMode::Once)
    }
}

fn update_wave_timer(
    difficulty: Res<Difficulty>,
    time: Res<Time>,
    mut wave_timer: ResMut<WaveTimer>,
    mut mob_spawner: EventWriter<SpawnMob>,
    body_pos: Query<&Transform, With<BodyRoot>>,
    mob_count: Res<MobCount>,
){
    wave_timer.0.tick(time.delta());
    if wave_timer.0.finished(){
        let mut rng = thread_rng();
        wave_timer.0 = WaveTimer::timer_from_dif(difficulty.0);
        let body_pos = body_pos.single().translation.xy();
        const OFFSET: f32 = 16.  * 40.0;
        let low_min = body_pos.x - OFFSET;
        let high_min = body_pos.x + OFFSET;
        let low_max = low_min - OFFSET;
        let high_max = high_min + OFFSET;
        let y_low_min = body_pos.y - OFFSET;
        let y_high_min = body_pos.y - OFFSET;
        let y_low_max = y_low_min - OFFSET;
        let y_high_max = y_high_min + OFFSET;
        let amount_to_spawn = difficulty.0 as usize * 12;
        
        let mut positions = HashMap::new();
        while positions.len() < amount_to_spawn {
            let x = rng.gen_range(low_max..high_max);
            let y = if x > low_min && x < high_min {
                if rng.gen() {
                    rng.gen_range(low_max..low_min)
                } else {
                    rng.gen_range(high_min..high_max)
                }
            } else {
                rng.gen_range(low_max..high_max)
            };
            positions.insert(IVec2::new(x as i32, y as i32),Vec2::new(x, y));
        }
        for (_, pos) in positions.drain(){
            mob_spawner.send(SpawnMob::Single { 
                mob_type: MobType::from_difficulty(difficulty.0), 
                pos
            });
        }
    }
}


