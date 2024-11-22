use bevy::app::Plugin;

pub mod buttons;
pub mod card;
pub mod chip;
pub mod dealer;
mod events;
pub mod player;
pub mod resources;
pub mod styles;
mod systems;

use bevy::prelude::*;

use buttons::ButtonPlugin;
use dealer::DealerPlugin;
use events::*;
use player::PlayerPlugin;
use resources::GameTable;
use systems::*;

use crate::server::table::ETableState;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .init_resource::<GameTable>()
            // States
            .init_state::<SimulationState>()
            .init_state::<GameState>()
            // Event
            .add_plugins(EventPlugin)
            // Plugin
            .add_plugins(ButtonPlugin)
            .add_plugins(PlayerPlugin)
            .add_plugins(DealerPlugin)
            .add_plugins(SystemPlugin)
            // On Enter Systems
            // Systems
            // On Exit Systems
        ;
    }
}

/// 控制游戏暂停
/// Deprecated
#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
pub enum SimulationState {
    Running,
    #[default]
    Paused,
}

/// 控制游戏状态
/// Deprecated
#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
pub enum GameState {
    #[default]
    PlayerBet,
    DealerCheckBlackJack,
    PlayerBuyInsurance, // TODO 添加Insurance功能
    PlayerSplitOrDoubleDownOrHitOrStand,
    PlayerDoubleDownOrHitOrStand,
    PlayerHitOrStand,
    DealerHitOrStand,
    CheckResultAndReset,
}

impl From<ETableState> for GameState {
    fn from(value: ETableState) -> Self {
        match value {
            ETableState::PlayerBet => GameState::PlayerBet,
            ETableState::DealerCheckBlackJack => GameState::DealerCheckBlackJack,
            ETableState::PlayerBuyInsurance => GameState::PlayerBuyInsurance,
            ETableState::PlayerSplitOrDoubleDownOrHitOrStand(_) => {
                GameState::PlayerSplitOrDoubleDownOrHitOrStand
            }
            ETableState::PlayerDoubleDownOrHitOrStand(_) => GameState::PlayerDoubleDownOrHitOrStand,
            ETableState::PlayerHitOrStand(_) => GameState::PlayerHitOrStand,
            ETableState::DealerHitOrStand => GameState::DealerHitOrStand,
            ETableState::CheckResultAndReset => GameState::CheckResultAndReset,
        }
    }
}
