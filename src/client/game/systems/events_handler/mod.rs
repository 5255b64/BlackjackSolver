pub mod client_events_handler;
pub mod player_request_events_handler;
pub mod server_response_events_handler;

use bevy::prelude::*;
use client_events_handler::ClientEventsHandlerPlugin;
use player_request_events_handler::PlayerRequestEventsHandlerPlugin;
use server_response_events_handler::ServerResponseEventsHandlerPlugin;

use crate::client::game::events::client_events::*;

pub struct EventsHandlerPlugin;

impl Plugin for EventsHandlerPlugin {
    fn build(&self, app: &mut App) {
        app
            // Systems
            .add_plugins(PlayerRequestEventsHandlerPlugin)
            .add_plugins(ServerResponseEventsHandlerPlugin)
            .add_plugins(ClientEventsHandlerPlugin);
    }
}