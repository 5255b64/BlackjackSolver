use bevy::prelude::*;

use crate::server::{card::ECard, value::EValue};

#[derive(Resource)]
pub struct DealerHand {
    pub point: EValue,
    pub cards: Vec<ECard>,
}

impl Default for DealerHand {
    fn default() -> Self {
        Self { point: Default::default(), cards: Default::default() }
    }
}

impl DealerHand {
    pub fn reset(&mut self) {
        self.point = EValue::default();
        self.cards.clear();
    }
}