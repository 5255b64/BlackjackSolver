use bevy::prelude::*;

// --- 主动触发事件 ---
#[derive(Event)]
pub struct EventRequestPlayerBet {
    pub value: usize,
}

#[derive(Event)]
pub struct EventRequestPlayerHit {}

#[derive(Event)]
pub struct EventRequestPlayerStand {}

#[derive(Event)]
pub struct EventRequestPlayerDoubleDown {}

#[derive(Event)]
pub struct EventRequestPlayerSplit {}

pub struct PlayerRequestEventsPlugin;

impl Plugin for PlayerRequestEventsPlugin {
    fn build(&self, app: &mut App) {
        app
            // Event
            .add_event::<EventRequestPlayerBet>()
            .add_event::<EventRequestPlayerHit>()
            .add_event::<EventRequestPlayerStand>()
            .add_event::<EventRequestPlayerSplit>()
            .add_event::<EventRequestPlayerDoubleDown>();
    }
}
