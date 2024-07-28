use crate::prelude::*;
use std::time::Duration;

pub struct DifficultyPlugin;
impl Plugin for DifficultyPlugin{
    fn build(&self, app: &mut App) {
        app
            .init_state::<DifficultyMode>()
            .init_resource::<DifficultyRampTimer>()
            .add_systems(
                OnEnter(GamePhase::SoulSelection), 
                apply_difficulty_mode_to_difficulty_starting_val)
            .add_systems(
                OnEnter(GamePhase::SoulSelection), 
                init_difficulty_ramp)
            .add_systems(
                Update,
                update_ramp.run_if(in_state(GamePhase::Tribulation)));
    }
}

#[derive(Default, Resource)]
pub struct Difficulty(pub u64);

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum DifficultyMode{
    Easy,
    #[default]
    Normal,
    Medium,
    Hard,
    Super,
    OhF___
}

fn apply_difficulty_mode_to_difficulty_starting_val(
    mode: Res<State<DifficultyMode>>,
    mut difficulty: ResMut<Difficulty>,
){
    let val = match mode.get() {
        DifficultyMode::Easy => 0,
        DifficultyMode::Normal => 10,
        DifficultyMode::Medium => 20,
        DifficultyMode::Hard => 30,
        DifficultyMode::Super => 40,
        DifficultyMode::OhF___ => 50,
    };
    difficulty.0 = val;
}

pub struct DifficultyRampTimer{
    pub timer: Timer,
    pub amount: u64,
}

impl DifficultyRampTimer{
    pub fn update(&mut self, delta: Duration) -> bool{
        self.timer.tick(delta);
        self.timer.finished()
    }
}

fn init_difficulty_ramp(
    mut ramp_timer: ResMut<DifficultyRampTimer>,
    mode: Res<State<DifficultyMode>>,
){
    let mode_new = match *mode{
        DifficultyMode::Easy=> DifficultyRampTimer {
            timer: Timer::from_seconds(60.0, TimerMode::Repeating),
            amount: 1,
        },
        DifficultyMode::Normal=> DifficultyRampTimer {
            timer: Timer::from_seconds(60.0, TimerMode::Repeating),
            amount: 5,
        },
        DifficultyMode::Medium=> DifficultyRampTimer {
            timer: Timer::from_seconds(60.0, TimerMode::Repeating),
            amount: 5,
        },
        DifficultyMode::Hard=> DifficultyRampTimer {
            timer: Timer::from_seconds(60.0, TimerMode::Repeating),
            amount: 10,
        },
        DifficultyMode::Super => DifficultyRampTimer {
            timer: Timer::from_seconds(30.0, TimerMode::Repeating),
            amount: 6,
        },
        DifficultyMode::OhF___ => DifficultyRampTimer{
            timer: Timer::from_seconds(30.0, TimerMode::Repeating),
            amount: 10,
        }
    };

}

fn update_ramp(
    mut ramp: ResMut<DifficultyRampTimer>,
    time: Res<Time>,
    mut difficulty: ResMut<Difficulty>,
){
    if ramp.update(time.delta()) {
        difficulty.0 += ramp.amount;
    }
}
