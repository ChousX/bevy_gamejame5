pub mod prelude {
  pub use bevy::prelude::*;
  pub use bevy::color::palettes::css as css;
  pub use css::GOLD;
  pub use crate::app::APP_NAME;
}

use prelude::*;

pub mod plugins {

}

mod app {
  use crate::prelude::*;
  #[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
  pub enum AppState {
    #[default]
    Entry,
  }

  pub const APP_NAME: &str = "Kani-Kai";
}

