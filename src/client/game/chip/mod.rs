pub mod components;
mod systems;

use bevy::prelude::*;

use super::super::AppState;
use systems::*;

pub struct ChipPlugin;

impl Plugin for ChipPlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            // On Enter State
            .add_systems(OnEnter(AppState::Game), spawn_chips)
            // Systems
            // .add_systems(Update, update_score.run_if(in_state(AppState::Game)))
            // On Exit State
            .add_systems(OnExit(AppState::Game), despawn_chips)
            ;
    }
}
