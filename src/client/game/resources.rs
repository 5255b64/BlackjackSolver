use bevy::prelude::*;

use crate::server::table::STable;

/// 调用后端的Stable
#[derive(Resource)]
pub struct GameTable {
    pub table: STable,
}

impl Default for GameTable {
    fn default() -> Self {
        Self {
            table: Default::default(),
        }
    }
}