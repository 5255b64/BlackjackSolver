pub mod components;
mod systems;

use bevy::prelude::*;
use systems::*;

use crate::client::AppState;

pub struct ButtonPlugin;

impl Plugin for ButtonPlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            // Event
            // Plugin
            // On Enter Systems
            .add_systems(OnEnter(AppState::Game), spawn_buttons)
            // Systems
            .add_systems(
                Update,
                (
                    interact_with_start_button,
                    interact_with_split_button,
                    interact_with_double_down_button,
                    interact_with_hit_button,
                    interact_with_stand_button,
                ),
            )
            // On Exit Systems
            // .add_systems(OnExit(AppState::Game), despawn_buttons)
            ;
    }
}