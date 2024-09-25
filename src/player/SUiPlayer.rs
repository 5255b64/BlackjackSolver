use crate::player::EPlayerAction;
use crate::table::{ETableState, STable};

pub struct SUiPlayer {
    c:usize
}

impl SUiPlayer {
    pub fn new() -> Self {
        SUiPlayer {
            c:0
        }
    }
    pub fn action(&self, table: &STable) -> EPlayerAction {
        match table.get_state() {
            ETableState::PlayerBet => {
                // 永远bet1 todo
                EPlayerAction::Bet(1)
            }
            ETableState::PlayerBuyInsurance => {
                // 永远不买 todo
                EPlayerAction::BuyInsurance(0)
            }
            ETableState::PlayerSplit(_) => {
                // 必定split todo
                EPlayerAction::Split(true)
            }
            ETableState::PlayerDoubleDownOrHitOrStand(_) => {
                // todo
                match self.c%3 {
                    0 => EPlayerAction::DoubleDown,
                    1 => EPlayerAction::Hit,
                    _ => EPlayerAction::Stand,
                }
            }
            ETableState::PlayerHitOrStand(_) => {
                // todo
                match self.c%2 {
                    1 => EPlayerAction::Hit,
                    _ => EPlayerAction::Stand,
                }
            }
            _ => EPlayerAction::NoAction
        }
    }
}