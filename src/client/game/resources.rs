use bevy::prelude::*;

use crate::server::table::STable;

#[derive(Resource)]
pub struct GameTable {
    pub table: STable,
}

impl Default for GameTable {
    fn default() -> Self {
        Self { table: Default::default() }
    }
}