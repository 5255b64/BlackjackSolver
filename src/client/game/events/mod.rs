pub mod client_events;
pub mod player_request_events;
pub mod server_response_events;

use bevy::prelude::*;
use client_events::ClientEventsPlugin;
use player_request_events::PlayerRequestEventsPlugin;
use server_response_events::ServerResponseEventsPlugin;

pub struct EventPlugin;

impl Plugin for EventPlugin {
    fn build(&self, app: &mut App) {
        app
            // Event
            .add_plugins(ServerResponseEventsPlugin)
            .add_plugins(PlayerRequestEventsPlugin)
            .add_plugins(ClientEventsPlugin);
    }
}
