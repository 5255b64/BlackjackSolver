use super::super::table::{ETableState, STable};
use super::EPlayerAction;

pub struct SStaticStrategyPlayer {
    c: usize,
}

/// 静态策略用户
impl SStaticStrategyPlayer {
    pub fn new() -> Self {
        SStaticStrategyPlayer { c: 0 }
    }
    pub fn action(&self, table: &STable) -> EPlayerAction {
        match table.get_state() {
            ETableState::PlayerBet => {
                // 永远bet 2 todo
                EPlayerAction::Bet(2)
            }
            ETableState::PlayerBuyInsurance => {
                // 永远不买 todo
                EPlayerAction::BuyInsurance(0)
            }
            ETableState::PlayerSplitOrDoubleDownOrHitOrStand(_) => {
                // 必定split todo
                EPlayerAction::Split
            }
            ETableState::PlayerDoubleDownOrHitOrStand(_) => {
                // todo
                match self.c % 3 {
                    0 => EPlayerAction::DoubleDown,
                    1 => EPlayerAction::Hit,
                    _ => EPlayerAction::Stand,
                }
            }
            ETableState::PlayerHitOrStand(_) => {
                // todo
                match self.c % 2 {
                    1 => EPlayerAction::Hit,
                    _ => EPlayerAction::Stand,
                }
            }
            _ => EPlayerAction::WaitNext
        }
    }
}
