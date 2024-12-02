use bevy::prelude::*;

use crate::server::card::ECard;

// --- 主动触发事件 ---
#[derive(Event)]
pub struct RequestPlayerBet {
    pub value: usize,
}

#[derive(Event)]
pub struct RequestPlayerHit {}

#[derive(Event)]
pub struct RequestPlayerStand {}

#[derive(Event)]
pub struct RequestPlayerDoubleDown {}

#[derive(Event)]
pub struct RequestPlayerSplit {}

// --- 被动触发事件 由server回传 ---
#[derive(Event)]
pub struct ResponseInitGameWithCards {
    pub player_cards: [ECard; 2],
    pub dealer_cards: [ECard; 2],
}

#[derive(Event)]
pub struct ResponseWaitPlayerBuyInsurance {}

#[derive(Event)]
pub struct ResponseInsuranceResult {
    pub is_dealer_blackjack: bool,
}

#[derive(Event)]
pub struct ResponsePlayerSplitCards {
    pub card1: ECard,
    pub card2: ECard,
}

#[derive(Event)]
pub struct ResponsePlayerDrawCard {
    pub card: ECard,
    pub is_player_stop: bool,
}

#[derive(Event)]
pub struct ResponsePlayerStand {
    pub is_player_stop: bool,
}

#[derive(Event)]
pub struct ResponseGameOver {
    pub win_chips: usize,
}

#[derive(Event)]
pub struct ResponseDealerDrawCard {
    pub card: ECard,
    pub is_dealer_stop: bool,
    pub is_revealed: bool,
}

pub struct EventPlugin;

impl Plugin for EventPlugin {
    fn build(&self, app: &mut App) {
        app
            // Event
            .add_event::<RequestPlayerBet>()
            .add_event::<RequestPlayerHit>()
            .add_event::<RequestPlayerStand>()
            .add_event::<RequestPlayerSplit>()
            .add_event::<RequestPlayerDoubleDown>()
            .add_event::<ResponseInitGameWithCards>()
            .add_event::<ResponseWaitPlayerBuyInsurance>()
            .add_event::<ResponseInsuranceResult>()
            .add_event::<ResponsePlayerSplitCards>()
            .add_event::<ResponsePlayerStand>()
            .add_event::<ResponseGameOver>()
            .add_event::<ResponsePlayerDrawCard>()
            .add_event::<ResponseDealerDrawCard>();
    }
}
