use std::collections::HashMap;
use fraction::{Fraction, ToPrimitive};
use log::log;
use super::card::ECardPoint;
use super::config::SGameRule;
use super::deck::random_deck::SRandomDeck;
use super::hand::dealer_hand::SDealerHand;
use super::hand::player_hand::SPlayerHand;
use super::deck::TDeck;
use super::player::EPlayerAction;
use super::value::EValue;

#[derive(Debug, Clone, PartialEq)]
pub enum ETableState {
    PlayerBet,
    DealerCheckBlackJack,
    PlayerBuyInsurance,
    DealerCheckInsurance,
    // usize代表hand的index
    PlayerSplit(usize),
    PlayerDoubleDownOrHitOrStand(usize),
    PlayerHitOrStand(usize),
    DealerHitOrStand,
    CheckResultAndReset,
}

#[derive(Debug)]
pub enum EPlayerActionError {
    // player action与table状态不符
    ActionStatueError,
    // bet值不符合和要求
    CheckBetError,
    // insurance值不符合和要求
    CheckInsuranceError,
    //
    HandLengthError,
    // 下注时筹码不足
    ChipsNotEnoughError,
}

pub enum ETableRunError {
    AbnormalStateError,
    HandLengthError,
}

pub struct STable {
    pub state: ETableState,
    pub rule: SGameRule,
    pub dealer_hand: SDealerHand,
    pub player_hand: Vec<SPlayerHand>,
    pub player_chips: usize,
    pub deck: Box<dyn TDeck>,
}

impl Default for STable {
    fn default() -> Self {
        let rule = SGameRule::default();
        let dealer = SDealerHand::new();
        let player = SPlayerHand::new();
        let deck = SRandomDeck::new();
        STable::new(rule, dealer, player, 0, Box::new(deck))
    }
}

impl STable {
    pub fn new(
        rule: SGameRule,
        dealer_hand: SDealerHand,
        player_hand: SPlayerHand,
        player_chips: usize,
        deck: Box<dyn TDeck>,
    ) -> Self {
        let state = ETableState::PlayerBet;
        STable {
            state,
            rule,
            dealer_hand,
            player_hand: vec![player_hand],
            player_chips,
            deck,
        }
    }

    pub fn new_random_deck() -> Self {
        let rule = SGameRule::default();
        let dealer = SDealerHand::new();
        let player = SPlayerHand::new();
        let deck = SRandomDeck::new();
        STable::new(rule, dealer, player, 0, Box::new(deck))
    }

    pub fn run(&mut self) -> Result<(), ETableRunError> {
        if self.state == ETableState::DealerCheckBlackJack
            || self.state == ETableState::DealerCheckInsurance
            || self.state == ETableState::DealerHitOrStand
            || self.state == ETableState::CheckResultAndReset {
            match self.state.clone() {
                ETableState::DealerCheckBlackJack => {
                    if self.dealer_hand.hand.cards.len() != 2 {
                        return Err(ETableRunError::HandLengthError);
                    }
                    // 判断是否buy insurance
                    let first_card = self.dealer_hand.hand.cards.get(0).unwrap().clone();
                    if EValue::from(first_card.into()) == EValue::S11 {
                        // 状态转移
                        self.state = ETableState::PlayerBuyInsurance;
                    } else {
                        // 非blackjack 进入用户操作状态
                        if self.player_hand.len() != 1 || self.player_hand.get(0).unwrap().hand.cards.len() != 2 {
                            return Err(ETableRunError::HandLengthError);
                        }
                        // 状态转移
                        if self.player_hand.get(0).unwrap().should_split() {
                            self.state = ETableState::PlayerSplit(0);
                        } else {
                            self.state = ETableState::PlayerDoubleDownOrHitOrStand(0);
                        }
                    }
                }
                ETableState::DealerCheckInsurance => {
                    // 判断是否blackjack
                    let second_card = self.dealer_hand.hand.cards.get(1).unwrap().clone();
                    if EValue::from(second_card.into()) == EValue::H10 {
                        // blackjack 直接进入结算状态
                        // 状态转移
                        self.state = ETableState::CheckResultAndReset;
                    } else {
                        // 非blackjack 进入用户操作状态
                        if self.player_hand.get(0).unwrap().hand.cards.len() != 2 {
                            return Err(ETableRunError::HandLengthError);
                        }
                        // 状态转移
                        if self.player_hand.get(0).unwrap().should_split() {
                            self.state = ETableState::PlayerSplit(0);
                        } else {
                            self.state = ETableState::PlayerDoubleDownOrHitOrStand(0);
                        }
                    }
                }
                ETableState::DealerHitOrStand => {
                    // 不足17且不爆牌时 继续拿牌
                    let hand = &mut self.dealer_hand;
                    let mut point = hand.point();
                    while point < 17 && point != 1 {
                        hand.draw(self.deck.draw().unwrap());
                        point = hand.point()
                    }
                    // 状态转移
                    self.state = ETableState::CheckResultAndReset
                }
                ETableState::CheckResultAndReset => {
                    // 判断结果
                    let dealer_point = self.dealer_hand.point();
                    for player_hand in &mut self.player_hand {
                        // 判断大小
                        let player_point = player_hand.point();
                        if player_point > dealer_point {
                            // 玩家胜利
                            // 判断player是否blackjack
                            if player_hand.is_blackjack() {
                                let win_value = (self.rule.blackjack_pay + 1) * Fraction::from(player_hand.get_bet());
                                player_hand.win(win_value.floor().to_usize().unwrap());
                            } else {
                                let win_value = player_hand.get_bet();
                                player_hand.win(win_value);
                            }
                        } else if player_point < dealer_point || player_hand.is_bust() || (self.dealer_hand.is_blackjack() && !player_hand.is_blackjack()) {
                            // 玩家失败
                            player_hand.lose();
                        }
                        // 平局push 无需处理
                        // 将筹码返还给玩家
                        self.player_chips += player_hand.get_bet();
                    }
                    // 重置状态
                    self.dealer_hand.reset();
                    self.reset_player_hand();
                    // 状态转移
                    self.state = ETableState::PlayerBet
                }
                table_state => {
                    // ignore
                    log::info!("状态错误：table状态-{:?}", table_state);
                    return Err(ETableRunError::AbnormalStateError);
                }
            }
        }
        Ok(())
    }

    pub fn reset(&mut self) {
        self.state = ETableState::PlayerBet;
        self.dealer_hand.reset();
        self.player_hand.clear();
        self.player_hand.push(SPlayerHand::new());
    }

    pub fn shuffle(&mut self) {
        self.reset();
        self.deck.shuffle();
    }

    pub fn get_point_probability_map(&self) -> &HashMap<ECardPoint, Fraction> {
        self.deck.get_point_probability_map()
    }

    pub fn receive_player_action(&mut self, action: EPlayerAction) -> Result<(), EPlayerActionError> {
        match (self.state.clone(), action) {
            (ETableState::PlayerBet, EPlayerAction::Bet(value)) => {
                if self.player_hand.len() != 1 {
                    return Err(EPlayerActionError::HandLengthError);
                }
                let hand = self.player_hand.get_mut(0).unwrap();
                // 下注
                if self.player_chips <= value {
                    return Err(EPlayerActionError::ChipsNotEnoughError);
                }
                if self.rule.check_bet(value) {
                    self.player_chips -= value;
                    hand.bet(value);
                    // 抽牌
                    hand.draw(self.deck.draw().unwrap());
                    self.dealer_hand.draw(self.deck.draw().unwrap());
                    hand.draw(self.deck.draw().unwrap());
                    self.dealer_hand.draw(self.deck.draw().unwrap());
                    // 状态转移
                    self.state = ETableState::DealerCheckBlackJack;
                    self.run();
                    Ok(())
                } else {
                    Err(EPlayerActionError::CheckBetError)
                }
            }
            (ETableState::PlayerBuyInsurance, EPlayerAction::BuyInsurance(value)) => {
                if self.player_hand.len() != 1 {
                    return Err(EPlayerActionError::HandLengthError);
                }
                let hand = self.player_hand.get_mut(0).unwrap();
                if value != 0 {
                    // 排除不买保险的情况
                    if self.rule.check_insurance(hand.get_bet(), value) {
                        hand.insurance(value);
                    } else {
                        return Err(EPlayerActionError::CheckInsuranceError);
                    }
                }
                // 状态转移
                self.state = ETableState::DealerCheckInsurance;
                self.run();

                Ok(())
            }
            (ETableState::PlayerSplit(index), EPlayerAction::Split(is_split)) => {
                if self.player_hand.len() <= index {
                    return Err(EPlayerActionError::HandLengthError);
                }
                let old_hand = self.player_hand.get_mut(index).unwrap();
                if is_split {
                    let mut new_hand = old_hand.split();
                    // 发牌
                    old_hand.draw(self.deck.draw().unwrap());
                    new_hand.draw(self.deck.draw().unwrap());
                    // 新的手牌插入队列
                    self.player_hand.push(SPlayerHand::from(new_hand));
                }
                // 状态转移
                self.state = ETableState::PlayerDoubleDownOrHitOrStand(index);
                self.run();
                Ok(())
            }
            (ETableState::PlayerDoubleDownOrHitOrStand(index), EPlayerAction::DoubleDown) => {
                let hand = self.player_hand.get_mut(index).unwrap();
                // 判断chips是否足够
                if self.player_chips <= hand.get_bet() {
                    return Err(EPlayerActionError::ChipsNotEnoughError);
                }
                self.player_chips -= hand.get_bet();
                hand.double_down(self.deck.draw().unwrap());
                // 判断是否有下一手牌
                if index + 1 < self.player_hand.len() {
                    if self.player_hand.get(index + 1).unwrap().should_split() {
                        self.state = ETableState::PlayerSplit(index + 1);
                    } else {
                        self.state = ETableState::PlayerDoubleDownOrHitOrStand(index + 1);
                    }
                } else {
                    // 没有下一手牌 进入dealer行动环节
                    self.state = ETableState::DealerHitOrStand;
                }
                Ok(())
            }
            (ETableState::PlayerDoubleDownOrHitOrStand(index) | ETableState::PlayerHitOrStand(index), EPlayerAction::Hit) => {
                let hand = self.player_hand.get_mut(index).unwrap();
                hand.draw(self.deck.draw().unwrap());
                // 判断是否bust
                if hand.is_bust() {
                    // 当前牌bust 判断是否有下一手牌
                    if index + 1 < self.player_hand.len() {
                        if self.player_hand.get(index + 1).unwrap().should_split() {
                            self.state = ETableState::PlayerSplit(index + 1);
                        } else {
                            self.state = ETableState::PlayerDoubleDownOrHitOrStand(index + 1);
                        }
                    } else {
                        // 没有下一手牌 进入dealer行动环节
                        self.state = ETableState::DealerHitOrStand;
                    }
                } else {
                    // 当前牌未bust
                    self.state = ETableState::PlayerHitOrStand(index);
                }
                Ok(())
            }
            (ETableState::PlayerDoubleDownOrHitOrStand(index) | ETableState::PlayerHitOrStand(index), EPlayerAction::Stand) => {
                // 判断是否有下一手牌
                if index + 1 < self.player_hand.len() {
                    if self.player_hand.get(index + 1).unwrap().should_split() {
                        self.state = ETableState::PlayerSplit(index + 1);
                    } else {
                        self.state = ETableState::PlayerDoubleDownOrHitOrStand(index + 1);
                    }
                } else {
                    // 没有下一手牌 进入dealer行动环节
                    self.state = ETableState::DealerHitOrStand;
                }
                Ok(())
            }
            (table_state, player_state) => {
                // ignore
                log::info!("状态错误：table状态-{:?} player状态-{:?}", table_state, player_state);
                // println!("状态错误：table状态-{:?} player状态-{:?}", table_state, player_state);
                Err(EPlayerActionError::ActionStatueError)
            }
        }
    }

    pub fn get_state(&self) -> ETableState {
        self.state.clone()
    }

    pub fn but_chips(&mut self, chips: usize) {
        self.player_chips += chips;
    }

    pub fn reset_player_hand(&mut self) {
        self.player_hand.clear();
        self.player_hand.push(SPlayerHand::new());
    }
}