use crate::server::card::ECard;

use bevy::prelude::*;

// --- 被动触发事件 由server回传 ---
#[derive(Event)]
pub struct EventResponseInitGameWithCards {
    pub player_cards: [ECard; 2],
    pub dealer_cards: [ECard; 2],
}

#[derive(Event)]
pub struct EventResponseWaitPlayerBuyInsurance {}

#[derive(Event)]
pub struct EventResponseInsuranceResult {
    pub is_dealer_blackjack: bool,
}

#[derive(Event)]
pub struct EventResponsePlayerSplitCards {
    pub hand_index: usize,
    pub card1: ECard,
    pub card2: ECard,
}

#[derive(Event)]
pub struct EventResponsePlayerStand {
    pub is_player_stop: bool,
}

#[derive(Event)]
pub struct EventResponseGameOver {
    pub bet_chips: usize,
    pub win_chips: usize,
    pub player_chips: usize,
}

#[derive(Event)]
pub struct EventResponsePlayerDrawCard {
    pub card: ECard,
    pub hand_index: usize,
    pub is_player_stop: bool,
}

#[derive(Event)]
pub struct EventResponseDealerDrawCard {
    pub card: ECard,
    pub is_revealed: bool,
}

pub struct ServerResponseEventsPlugin;

impl Plugin for ServerResponseEventsPlugin {
    fn build(&self, app: &mut App) {
        app
            // Event
            .add_event::<EventResponseInitGameWithCards>()
            .add_event::<EventResponseWaitPlayerBuyInsurance>()
            .add_event::<EventResponseInsuranceResult>()
            .add_event::<EventResponsePlayerSplitCards>()
            .add_event::<EventResponsePlayerStand>()
            .add_event::<EventResponseGameOver>()
            .add_event::<EventResponsePlayerDrawCard>()
            .add_event::<EventResponseDealerDrawCard>();
    }
}
