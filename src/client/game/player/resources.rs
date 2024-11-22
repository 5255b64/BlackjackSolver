use bevy::prelude::*;

use crate::server::{card::ECard, value::EValue};

#[derive(Resource)]
pub struct PlayerHand {
    pub point: EValue,
    pub cards: Vec<ECard>,
}

impl Default for PlayerHand {
    fn default() -> Self {
        Self { point: Default::default(), cards: Default::default() }
    }
}

impl PlayerHand {
    pub fn reset(&mut self) {
        self.point = EValue::default();
        self.cards.clear();
    }
}
