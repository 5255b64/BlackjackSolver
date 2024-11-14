pub mod StaticStrategyPlayer;

#[derive(Debug)]
pub enum EPlayerAction {
    Bet(usize),
    BuyInsurance(usize),
    Split(bool),
    DoubleDown,
    Hit,
    Stand,
    NoAction
}