#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

pub mod prelude {
  pub use bevy::prelude::*;
  pub use bevy::color::palettes::css as css;
  pub use bevy_asset_loader::prelude::*;
  pub use crate::app::*;
  pub use crate::player::BodyType;
  pub use crate::cursor::CursorPosition;
  pub use crate::mob::{MobCount, MobType, SpawnMob};
  pub use crate::game::{GamePhase, Difficulty};
  //assets
  pub use crate::domains::TerrainTextures;
  pub use crate::mob::MobTexture;
  pub use crate::player::BodyTexture;
  pub use crate::player::SkillEffectTexture;
}

use prelude::*;

mod menus;
mod camera;
mod names;
mod domains;
mod game;
mod splash_screan;
mod helpers;
mod controles;
mod player;
mod mob;
mod cursor;

pub mod plugins {
    pub use crate::menus::MenuPlugin;
    pub use crate::camera::CameraPlugin;
    pub use crate::game::GamePlugin;
    pub use crate::domains::DomainPlugin;
    pub use crate::splash_screan::SplashScreenPlugin;
    pub use crate::player::PlayerPlugin;
    pub use crate::controles::ControlesPlugin;
    pub use crate::cursor::CursorPlugin;
    pub use crate::mob::MobPlugin;
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

