use bevy::prelude::*;

use crate::server::table::ETableState;

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

// 焦点(当前需要做出action的对象)
#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
pub enum Focus {
    #[default]
    None, // 牌局结束/等待开始 状态
    Dealer,     // 庄家行动
    Player(u8), // 玩家行动 对第n手牌进行操作
}
