use super::*;

pub struct TimeDyplayPlugin;
impl Plugin for TimeDyplayPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                Update,
                init_time_ui.run_if(resource_added::<GameTime>))
            .add_systems(
                Update, 
                update_time_ui.run_if(resource_changed::<GameTime>))
            .add_systems(
                Update, 
                time_ui_clean_up.run_if(resource_removed::<GameTime>))
    ;}
}

#[derive(Componant)]
pub struct TimeDysplay;

fn init_time_ui(){

}

fn time_ui_clean_up(){

}

fn update_time_ui(){

}
