pub mod prelude {
  pub use bevy::prelude::*;
  pub use bevy::color::palettes::css as css;
  pub use bevy_asset_loader::prelude::*;
  pub use crate::app::*;
  pub use crate::domains::TerrainTextures;
}

use prelude::*;

mod menus;
mod camera;
mod soul;
mod body;
mod names;
mod domains;
mod game;
mod splash_screan;
mod helpers;
mod controles;

pub mod plugins {
    pub use crate::menus::MenuPlugin;
    pub use crate::camera::CameraPlugin;
    pub use crate::game::GamePlugin;
    pub use crate::domains::DomainPlugin;
    pub use crate::soul::SoulPlugin;
    pub use crate::body::BodyPlugin;
    pub use crate::splash_screan::SplashScreenPlugin;
}

mod app {
    use crate::prelude::*;

    #[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
    pub enum AppState {
        #[default]
        Enter,
        MainMenu,
        Game,
    }


    pub const APP_NAME: &str = "Kani-Kai";
}

