mod game;
mod main_menu;
mod systems;
mod events;
pub mod test;

use game::*;
use main_menu::*;

use bevy::prelude::*;
use resources::{GameTable};
use states::{Focus, GameState, SimulationState};
use systems::*;

pub fn run() {
    App::new()
        // Bevy Plugins
        .add_plugins(DefaultPlugins)
        // Resources
        .init_resource::<GameTable>()
        // My State
        .init_state::<Focus>()
        .init_state::<AppState>()
        .init_state::<SimulationState>()
        .init_state::<GameState>()
        // My Plugins
        .add_plugins((MainMenuPlugin, GamePlugin))
        // Startup Systems
        .add_systems(Startup, spawn_camara)
        // Systems
        .run();
}

#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}