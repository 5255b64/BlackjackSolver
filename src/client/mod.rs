mod game;
mod main_menu;
mod systems;
mod events;

use game::*;
use main_menu::*;
use bevy::prelude::*;
use systems::*;

pub fn run() {
    App::new()
        // Bevy Plugins
        .add_plugins(DefaultPlugins)
        // My State
        .init_state::<AppState>()
        // My Plugins
        .add_plugins((MainMenuPlugin, GamePlugin))
        // Startup Systems
        .add_systems(Startup, spawn_camara)
        // Systems
        // .add_systems(Update, exit_game)
        .run();
}

#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}