pub mod event_handler;
pub mod keyboard;
pub mod systems;

use bevy::prelude::*;
use keyboard::*;
use event_handler::*;

pub struct SystemPlugin;


impl Plugin for SystemPlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            // States
            // Event
            // Plugin
            // On Enter Systems
            // Systems
            .add_systems(Update, (
                keyboard_esc_exit_game,
                handle_request_player_bet,
            ));
        // On Exit Systems
    }
}
