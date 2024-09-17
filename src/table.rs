use std::collections::HashMap;
use fraction::Fraction;
use crate::card::ECardPoint;
use crate::config::SGameRule;
use crate::deck::random_deck::SRandomDeck;
use crate::hand::dealer_hand::SDealerHand;
use crate::hand::player_hand::SPlayerHand;
use crate::deck::TDeck;

pub enum ETableState {
    PlayerBet,
    DealerCheckBlackJack,
    PlayerBuyInsurance,
    PlayerSplit,
    PlayerDoubleDown,
    PlayerHitOrStand,
    DealerHitOrStand,
    CheckResultAndReset,
}

pub struct STable<T: TDeck + Sized> {
    pub state: ETableState,
    pub rule: SGameRule,
    pub dealer: SDealerHand,
    pub player: SPlayerHand,
    pub deck: T,
}

impl<T: TDeck + Sized> Default for STable<T> {
    fn default() -> Self {
        let rule = SGameRule::default();
        let dealer = SDealerHand::new();
        let player = SPlayerHand::new();
        let deck = SRandomDeck::new();
        STable::new(rule, dealer, player, deck)
    }
}

impl<T: TDeck + Sized> STable<T> {
    pub fn new(rule: SGameRule, dealer: SDealerHand, player: SPlayerHand, deck: T) -> Self {
        let state = ETableState::PlayerBet;
        STable {
            state,
            rule,
            dealer,
            player,
            deck,
        }
    }

    pub fn play(&mut self) {
        todo!()
    }

    pub fn reset(&mut self) {
        todo!()
    }

    pub fn shuffle(&mut self) {
        todo!()
    }

    pub fn get_point_probability_map(&self) -> &HashMap<ECardPoint, Fraction> {
        todo!()
    }
}