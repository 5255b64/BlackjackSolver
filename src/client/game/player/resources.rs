use bevy::prelude::*;

use crate::server::{card::ECard, value::EValue};

#[derive(Resource)]
pub struct ResPlayer {
    pub point: EValue,
    pub hands: Vec<Vec<ECard>>,
}

impl Default for ResPlayer {
    fn default() -> Self {
        Self { point: Default::default(), hands: Default::default() }
    }
}

impl ResPlayer {
    pub fn reset(&mut self) {
        self.point = EValue::default();
        self.hands.clear();
    }
}
