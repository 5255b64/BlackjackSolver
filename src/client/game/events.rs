use bevy::prelude::*;

// --- Player主动事件 ---
#[derive(Event)]
pub struct UserBet {
    pub value: u32,
}

#[derive(Event)]
pub struct UserHit {}

#[derive(Event)]
pub struct UserStand {}

#[derive(Event)]
pub struct UserDoubleDown {}

#[derive(Event)]
pub struct UserSplit {}

// --- Player 被动事件 ---
#[derive(Event)]
pub struct PlayerBust {}

#[derive(Event)]
pub struct PlayerBlackjack {}

// --- Dealer 被动事件 ---
#[derive(Event)]
pub struct DealerBlackjack {}

#[derive(Event)]
pub struct DealerHit {}

#[derive(Event)]
pub struct DealerStandOrBust {}

