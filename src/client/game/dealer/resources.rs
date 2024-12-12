// use bevy::prelude::*;

// use crate::server::{card::ECard, value::EValue};

// #[derive(Resource)]
// pub struct ResDealer {
//     pub point: EValue,
//     pub cards: Vec<ECard>,
// }

// impl Default for ResDealer {
//     fn default() -> Self {
//         Self { point: Default::default(), cards: Default::default() }
//     }
// }

// impl ResDealer {
//     pub fn reset(&mut self) {
//         self.point = EValue::default();
//         self.cards.clear();
//     }
// }