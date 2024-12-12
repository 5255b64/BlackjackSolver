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
#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default, Copy)]
pub enum GameState {
    #[default]
    PlayerBet,
    DealerCheckBlackJack,
    PlayerBuyInsurance, // TODO 添加Insurance功能
    PlayerSplitOrDoubleDownOrHitOrStand(usize),
    PlayerDoubleDownOrHitOrStand(usize),
    PlayerHitOrStand(usize),
    DealerHitOrStand,
    CheckResultAndReset,
}

impl From<ETableState> for GameState {
    fn from(value: ETableState) -> Self {
        match value {
            ETableState::PlayerBet => GameState::PlayerBet,
            ETableState::DealerCheckBlackJack => GameState::DealerCheckBlackJack,
            ETableState::PlayerBuyInsurance => GameState::PlayerBuyInsurance,
            ETableState::PlayerSplitOrDoubleDownOrHitOrStand(hand_index) => {
                GameState::PlayerSplitOrDoubleDownOrHitOrStand(hand_index)
            }
            ETableState::PlayerDoubleDownOrHitOrStand(hand_index) => GameState::PlayerDoubleDownOrHitOrStand(hand_index),
            ETableState::PlayerHitOrStand(hand_index) => GameState::PlayerHitOrStand(hand_index),
            ETableState::DealerHitOrStand => GameState::DealerHitOrStand,
            ETableState::CheckResultAndReset => GameState::CheckResultAndReset,
        }
    }
}

/// 焦点(当前需要做出action的对象)
/// 焦点对象需要显著展示
#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
pub enum FocusState {
    #[default]
    None, // 牌局结束/等待开始 状态
    Dealer,     // 庄家行动
    Player(usize), // 玩家行动 对第n手牌进行操作
}
