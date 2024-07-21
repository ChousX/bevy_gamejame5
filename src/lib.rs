pub mod prelude {
  pub use bevy::prelude::*;
  pub use bevy::color::palettes::css as css;
  pub use bevy_asset_loader::prelude::*;
  pub use rand::prelude::*;
  pub use crate::app::*;

}

use prelude::*;

mod menus;
mod camera;
mod soul;
mod body;
mod names;

pub mod plugins {
    pub use crate::menus::MenuPlugin;
    pub use crate::camera::CameraPlugin;
}

mod app {
  use crate::prelude::*;
  #[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
  pub enum AppState {
    #[default]
    Entry,
    MainMenu,
    Game,
  }

#[derive(Resource)]
pub struct AppSeed(pub u32);
impl Default for AppSeed{
    fn default() -> Self{
        let mut rng = thread_rng();
        let seed: u32 = rng.gen();
        Self(seed)
    }
}

  pub const APP_NAME: &str = "Kani-Kai";
}

