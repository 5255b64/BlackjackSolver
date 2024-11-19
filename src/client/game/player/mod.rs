use resources::PlayerHand;

pub mod components;
pub mod resources;
mod systems;

use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .init_resource::<PlayerHand>()
            // States
            // Event
            // Plugin
            // On Enter Systems
            // Systems
            // .add_systems(
            //     Update,
            //     (
            //         // update_dealer_hand,
            //         // update_dealer_hand,
            //         // update_player_hand,
            //     ),
            // )
            // On Exit Systems
            ;
    }
}