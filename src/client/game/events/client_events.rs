use bevy::prelude::*;

use crate::{client::resources::Focus, server::card::ECard};

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
pub struct EventClientUpdateInfoBar {
    pub new_info: String,
}

#[derive(Event)]
pub struct EventClientUpdateState {}

#[derive(Event)]
pub struct EventClientGameOver {
    pub bet_chips: usize,
    pub win_chips: usize,
    pub player_chips: usize,
}

/// 焦点转换事件
/// 用于切换高亮展示
#[derive(Event)]
pub struct EventClientFocusChange {
    pub old_focus: Focus,
    pub new_focus: Focus,
}

pub struct ClientEventsPlugin;

impl Plugin for ClientEventsPlugin {
    fn build(&self, app: &mut App) {
        app
            // Event
            .add_event::<EventClientPlayerSplitCards>()
            .add_event::<EventClientDealerDrawCard>()
            .add_event::<EventClientPlayerDrawCard>()
            .add_event::<EventClientUpdateState>()
            .add_event::<EventClientUpdateInfoBar>()
            .add_event::<EventClientGameOver>()
            .add_event::<EventClientFocusChange>();
    }
}
