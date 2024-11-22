pub mod components;
pub mod systems;

use bevy::prelude::*;
use systems::*;

use super::super::AppState;
pub struct CardPlugin;

impl Plugin for CardPlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            // Startup Systems
            // Enter State Systems
            // .add_systems(OnEnter(AppState::Game), spawn_cards)
            // Systems
            // Exit State Systems
            .add_systems(OnExit(AppState::Game), system_despawn_cards);
    }
}
