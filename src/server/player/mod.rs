pub mod static_strategy_player;

#[derive(Debug, Clone)]
pub enum EPlayerAction {
    Bet(usize),
    BuyInsurance(usize),
    Split,
    DoubleDown,
    Hit,
    Stand,
    WaitNext,
    NoAction
}