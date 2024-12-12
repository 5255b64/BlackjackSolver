use bevy::prelude::*;

use crate::server::card::ECard;

#[derive(Event)]
pub struct EventClientPlayerSplitCards {
    pub hand_index: usize,
    pub card1: ECard,
    pub card2: ECard,
}

#[derive(Event)]
pub struct EventClientDealerDrawCard {
    pub card: ECard,
    pub is_dealer_stop: bool,
    pub is_revealed: bool,
}

#[derive(Event)]
pub struct EventClientPlayerDrawCard {
    pub card: ECard,
    pub hand_index: usize,
    pub is_player_stop: bool,
}

#[derive(Event)]
pub struct EventClientUpdateState {}

pub struct ClientEventsPlugin;

impl Plugin for ClientEventsPlugin {
    fn build(&self, app: &mut App) {
        app
            // Event
            .add_event::<EventClientPlayerSplitCards>()
            .add_event::<EventClientDealerDrawCard>()
            .add_event::<EventClientPlayerDrawCard>()
            .add_event::<EventClientUpdateState>();
    }
}
