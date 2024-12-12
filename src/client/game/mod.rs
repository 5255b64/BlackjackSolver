use bevy::app::Plugin;

pub mod buttons;
pub mod card;
pub mod chip;
pub mod dealer;
pub mod player;
pub mod resources;
pub mod styles;
pub mod states;
mod events;
mod components;
mod systems;

use bevy::prelude::*;

use buttons::ButtonPlugin;
// use dealer::DealerPlugin;
use events::*;
// use player::PlayerPlugin;
use systems::*;


pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            // States
            // Event
            .add_plugins(EventPlugin)
            // Plugin
            .add_plugins(ButtonPlugin)
            // .add_plugins(PlayerPlugin)
            // .add_plugins(DealerPlugin)
            .add_plugins(SystemPlugin)
            // On Enter Systems
            // Systems
            // On Exit Systems
        ;
    }
}