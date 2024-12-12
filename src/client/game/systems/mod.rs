pub mod events_handler;
pub mod keyboard;
pub mod layout;
pub mod systems;

use bevy::prelude::*;
use events_handler::EventsHandlerPlugin;
use keyboard::*;
use layout::{despawn_framework, spawn_framework};
use systems::update_server_state;

use crate::client::AppState;

pub struct SystemPlugin;

impl Plugin for SystemPlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            // States
            // Event
            // Plugin
            .add_plugins(EventsHandlerPlugin)
            // On Enter Systems
            .add_systems(OnEnter(AppState::Game), spawn_framework)
            // Systems
            .add_systems(
                Update,
                (
                    keyboard_esc_exit_game,
                    update_server_state,
                ),
            )
            // On Exit Systems
            .add_systems(OnExit(AppState::Game), despawn_framework);
    }
}
