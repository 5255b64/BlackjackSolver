pub mod StaticStrategyPlayer;

#[derive(Debug)]
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