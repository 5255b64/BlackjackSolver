use super::card::ECard;
use super::card::ECardPoint;
use super::config::SGameRule;
use super::deck::random_deck::SRandomDeck;
use super::deck::TDeck;
use super::hand::dealer_hand::SDealerHand;
use super::hand::player_hand::SPlayerHand;
use super::player::EPlayerAction;
use super::value::EValue;
use fraction::{Fraction, ToPrimitive};
use std::borrow::BorrowMut;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum ETableState {
    PlayerBet,
    DealerCheckBlackJack,
    PlayerBuyInsurance,
    // usize代表hand的index
    PlayerSplitOrDoubleDownOrHitOrStand(usize),
    PlayerDoubleDownOrHitOrStand(usize),
    PlayerHitOrStand(usize),
    DealerHitOrStand,
    CheckResultAndReset,
}

#[derive(Debug)]
pub enum EPlayerActionError {
    // player action与table状态不符
    ActionStatusError,
    // bet值不符合和要求
    CheckBetError,
    // insurance值不符合和要求
    CheckInsuranceError,
    //
    HandLengthError,
    // 下注时筹码不足
    ChipsNotEnoughError,
    // 不符合Split要求
    SplitError,
}

#[derive(Debug)]
pub enum ETableRunError {
    AbnormalStateError,
    HandLengthError,
}

pub struct STable {
    pub state: ETableState,
    pub rule: SGameRule,
    pub dealer_hand: SDealerHand,
    pub player_hands: Vec<SPlayerHand>,
    pub player_chips: usize,
    pub deck: Box<dyn TDeck + Sync + Send>,
}

impl Default for STable {
    fn default() -> Self {
        let rule = SGameRule::default();
        let dealer_hand = SDealerHand::new();
        let player_hands = vec![SPlayerHand::new()];
        let deck = SRandomDeck::new();
        STable::new(rule, dealer_hand, player_hands, 0, Box::new(deck))
    }
}

#[derive(Debug)]
pub enum ETableOutputEvent {
    InitGameWithCards {
        player_cards: [ECard; 2],
        dealer_cards: [ECard; 2],
    },
    WaitPlayerBuyInsurance,
    InsuranceResult {
        is_dealer_blackjack: bool,
    },
    PlayerSplitCards {
        card1: ECard,
        card2: ECard,
    },
    PlayerDrawCard {
        card: ECard,
        hand_index: usize,
        is_player_stop: bool,
    },
    PlayerStand {
        is_player_stop: bool,
    },
    DealerDrawCard {
        card: ECard,
        is_dealer_stop: bool,
    }, // 等于 DealerHit
    GameOver {
        player_chips: usize,
        bet_chips: usize,
        win_chips: usize,
    },
    WaitForPlayerAction,
}

impl STable {
    pub fn new(
        rule: SGameRule,
        dealer_hand: SDealerHand,
        player_hands: Vec<SPlayerHand>,
        player_chips: usize,
        deck: Box<dyn TDeck + Sync + Send>,
    ) -> Self {
        let state = ETableState::PlayerBet;
        STable {
            state,
            rule,
            dealer_hand,
            player_hands,
            player_chips,
            deck,
        }
    }

    pub fn new_random_deck() -> Self {
        let rule = SGameRule::default();
        let dealer_hand = SDealerHand::new();
        let player_hands = vec![SPlayerHand::new()];
        let deck = SRandomDeck::new();
        STable::new(rule, dealer_hand, player_hands, 0, Box::new(deck))
    }

    pub fn reset(&mut self) {
        self.state = ETableState::PlayerBet;
        self.dealer_hand.reset();
        self.player_hands.clear();
        self.player_hands.push(SPlayerHand::new());
    }

    pub fn shuffle(&mut self) {
        self.reset();
        self.deck.shuffle();
    }

    pub fn get_point_probability_map(&self) -> &HashMap<ECardPoint, Fraction> {
        self.deck.get_point_probability_map()
    }

    pub fn receive_player_action(
        &mut self,
        action: EPlayerAction,
    ) -> Result<ETableOutputEvent, EPlayerActionError> {
        println!(
            "SERVER\t--\tserver_state:{:?}\tuser_action:{:?}",
            self.state.clone(),
            action
        );
        match (self.state.clone(), action) {
            (ETableState::PlayerBet, EPlayerAction::Bet(value)) => {
                if self.player_hands.len() != 1 {
                    return Err(EPlayerActionError::HandLengthError);
                }
                let hand = self.player_hands.get_mut(0).unwrap();
                // 下注
                if self.player_chips <= value {
                    return Err(EPlayerActionError::ChipsNotEnoughError);
                }
                if self.rule.check_bet(value) {
                    self.player_chips -= value;
                    hand.bet(value);
                    // 抽牌
                    // todo 多路抽牌
                    let card1 = self.deck.draw().unwrap();
                    hand.draw(card1);
                    let card2 = self.deck.draw().unwrap();
                    self.dealer_hand.draw(card2);
                    let card3 = self.deck.draw().unwrap();
                    hand.draw(card3);
                    let card4 = self.deck.draw().unwrap();
                    self.dealer_hand.draw(card4);
                    // 状态转移
                    self.state = ETableState::DealerCheckBlackJack;
                    Ok(ETableOutputEvent::InitGameWithCards {
                        player_cards: [card1, card3],
                        dealer_cards: [card2, card4],
                    })
                } else {
                    Err(EPlayerActionError::CheckBetError)
                }
            }
            (ETableState::PlayerBuyInsurance, EPlayerAction::BuyInsurance(value)) => {
                if self.player_hands.len() != 1 {
                    return Err(EPlayerActionError::HandLengthError);
                }
                let hand = self.player_hands.get_mut(0).unwrap();
                if value != 0 {
                    // 排除不买保险的情况
                    if self.rule.check_insurance(hand.get_bet(), value) {
                        hand.insurance(value);
                        self.player_chips -= value;
                    } else {
                        return Err(EPlayerActionError::CheckInsuranceError);
                    }
                }
                // 状态转移
                // self.state = ETableState::DealerCheckInsurance;
                // self.run();

                // 判断是否blackjack
                let second_card = self.dealer_hand.hand.cards.get(1).unwrap().clone();
                if EValue::from(second_card.into()) == EValue::H10 {
                    // blackjack 直接进入结算状态
                    // 状态转移
                    self.state = ETableState::CheckResultAndReset;
                    let win_insurance_amount = ((self.rule.insurance_pay + 1)
                        * Fraction::from(value))
                    .floor()
                    .to_usize()
                    .unwrap();
                    self.player_chips += win_insurance_amount;
                    println!("保险成功 赢得:{win_insurance_amount}");
                    Ok(ETableOutputEvent::InsuranceResult {
                        is_dealer_blackjack: true,
                    })
                } else {
                    // 非blackjack 进入用户操作状态
                    if self.player_hands.get(0).unwrap().hand.cards.len() != 2 {
                        return Err(EPlayerActionError::HandLengthError);
                    }
                    // 状态转移
                    if self.player_hands.get(0).unwrap().should_split() {
                        self.state = ETableState::PlayerSplitOrDoubleDownOrHitOrStand(0);
                    } else {
                        self.state = ETableState::PlayerDoubleDownOrHitOrStand(0);
                    }
                    println!("保险失败");
                    Ok(ETableOutputEvent::InsuranceResult {
                        is_dealer_blackjack: false,
                    })
                }
            }
            (ETableState::PlayerSplitOrDoubleDownOrHitOrStand(index), EPlayerAction::Split) => {
                if self.player_hands.len() <= index {
                    return Err(EPlayerActionError::HandLengthError);
                }
                // 判断chips是否足够
                let hand = self.player_hands.get_mut(index).unwrap();
                if self.player_chips <= hand.get_bet() {
                    return Err(EPlayerActionError::ChipsNotEnoughError);
                }

                let hand = self.player_hands.get(index).unwrap();
                if hand.hand.cards.len() != 2
                    || hand.hand.cards.get(0).unwrap().value
                        != hand.hand.cards.get(1).unwrap().value
                {
                    return Err(EPlayerActionError::SplitError);
                }
                let old_hand = self.player_hands.get_mut(index).unwrap();
                let mut new_hand = SPlayerHand::from(old_hand.split());
                new_hand.bet(old_hand.get_bet());
                self.player_chips -= old_hand.get_bet();
                // 发牌
                let card1 = self.deck.draw().unwrap();
                let card2 = self.deck.draw().unwrap();
                old_hand.draw(card1);
                new_hand.draw(card2);
                // 新的手牌插入队列
                self.player_hands.push(new_hand);
                // 状态转移
                self.state = ETableState::PlayerDoubleDownOrHitOrStand(index);
                Ok(ETableOutputEvent::PlayerSplitCards { card1, card2 })
            }
            (
                ETableState::PlayerSplitOrDoubleDownOrHitOrStand(index)
                | ETableState::PlayerDoubleDownOrHitOrStand(index),
                EPlayerAction::DoubleDown,
            ) => {
                let hand = self.player_hands.get_mut(index).unwrap();
                // 判断chips是否足够
                if self.player_chips <= hand.get_bet() {
                    return Err(EPlayerActionError::ChipsNotEnoughError);
                }
                self.player_chips -= hand.get_bet();
                let card = self.deck.draw().unwrap();
                hand.double_down(card);
                // 判断是否有下一手牌
                if index + 1 < self.player_hands.len() {
                    if self.player_hands.get(index + 1).unwrap().should_split() {
                        self.state = ETableState::PlayerSplitOrDoubleDownOrHitOrStand(index + 1);
                    } else {
                        self.state = ETableState::PlayerDoubleDownOrHitOrStand(index + 1);
                    }
                    Ok(ETableOutputEvent::PlayerDrawCard {
                        card,
                        hand_index: index,
                        is_player_stop: false,
                    })
                } else {
                    // 没有下一手牌 进入dealer行动环节
                    self.state = ETableState::DealerHitOrStand;
                    Ok(ETableOutputEvent::PlayerDrawCard {
                        card,
                        hand_index: index,
                        is_player_stop: true,
                    })
                }
            }
            (
                ETableState::PlayerSplitOrDoubleDownOrHitOrStand(index)
                | ETableState::PlayerDoubleDownOrHitOrStand(index)
                | ETableState::PlayerHitOrStand(index),
                EPlayerAction::Hit,
            ) => {
                let hand = self.player_hands.get_mut(index).unwrap();
                let card = self.deck.draw().unwrap();
                hand.draw(card);
                // 判断是否bust
                if hand.is_bust() {
                    // 当前牌bust 判断是否有下一手牌
                    if index + 1 < self.player_hands.len() {
                        if self.player_hands.get(index + 1).unwrap().should_split() {
                            self.state =
                                ETableState::PlayerSplitOrDoubleDownOrHitOrStand(index + 1);
                        } else {
                            self.state = ETableState::PlayerDoubleDownOrHitOrStand(index + 1);
                        }
                        Ok(ETableOutputEvent::PlayerDrawCard {
                            card,
                            hand_index: index,
                            is_player_stop: false,
                        })
                    } else {
                        // 没有下一手牌 进入dealer行动环节
                        self.state = ETableState::DealerHitOrStand;
                        Ok(ETableOutputEvent::PlayerDrawCard {
                            card,
                            hand_index: index,
                            is_player_stop: true,
                        })
                    }
                } else {
                    // 当前牌未bust
                    self.state = ETableState::PlayerHitOrStand(index);
                    Ok(ETableOutputEvent::PlayerDrawCard {
                        card,
                        hand_index: index,
                        is_player_stop: false,
                    })
                }
            }
            (
                ETableState::PlayerSplitOrDoubleDownOrHitOrStand(index)
                | ETableState::PlayerDoubleDownOrHitOrStand(index)
                | ETableState::PlayerHitOrStand(index),
                EPlayerAction::Stand,
            ) => {
                // 判断是否有下一手牌
                if index + 1 < self.player_hands.len() {
                    if self.player_hands.get(index + 1).unwrap().should_split() {
                        self.state = ETableState::PlayerSplitOrDoubleDownOrHitOrStand(index + 1);
                    } else {
                        self.state = ETableState::PlayerDoubleDownOrHitOrStand(index + 1);
                    }
                    Ok(ETableOutputEvent::PlayerStand {
                        is_player_stop: false,
                    })
                } else {
                    // 没有下一手牌 进入dealer行动环节
                    self.state = ETableState::DealerHitOrStand;
                    Ok(ETableOutputEvent::PlayerStand {
                        is_player_stop: true,
                    })
                }
            }
            (table_state, EPlayerAction::WaitNext) => {
                match table_state {
                    ETableState::DealerCheckBlackJack => {
                        if self.dealer_hand.hand.cards.len() != 2 {
                            return Err(EPlayerActionError::HandLengthError);
                        }
                        // 判断是否buy insurance
                        let first_card = self.dealer_hand.hand.cards.get(0).unwrap().clone();
                        if EValue::from(first_card.into()) == EValue::S11 {
                            // 状态转移
                            self.state = ETableState::PlayerBuyInsurance;
                            Ok(ETableOutputEvent::WaitPlayerBuyInsurance)
                        } else {
                            // 非blackjack 进入用户操作状态
                            if self.player_hands.len() != 1
                                || self.player_hands.get(0).unwrap().hand.cards.len() != 2
                            {
                                return Err(EPlayerActionError::HandLengthError);
                            }
                            // 状态转移
                            if self.player_hands.get(0).unwrap().should_split() {
                                self.state = ETableState::PlayerSplitOrDoubleDownOrHitOrStand(0);
                            } else {
                                self.state = ETableState::PlayerDoubleDownOrHitOrStand(0);
                            }
                            Ok(ETableOutputEvent::WaitForPlayerAction)
                        }
                    }
                    ETableState::DealerHitOrStand => {
                        // 不足17且不爆牌时 继续拿牌
                        let hand = &mut self.dealer_hand;
                        let card = self.deck.draw().unwrap();
                        hand.draw(card);
                        let point = hand.point();
                        let is_dealer_stop = point > 17 || point == 1;
                        // 状态转移
                        if is_dealer_stop {
                            self.state = ETableState::CheckResultAndReset;
                        }
                        Ok(ETableOutputEvent::DealerDrawCard {
                            card,
                            is_dealer_stop,
                        })
                    }
                    ETableState::CheckResultAndReset => Ok(self.check_result_and_reset()),
                    _ => Ok(ETableOutputEvent::WaitForPlayerAction),
                }
            }
            (table_state, player_state) => {
                // ignore
                // println!(
                //     "状态错误：table状态-{:?} player状态-{:?}",
                //     table_state, player_state
                // );
                log::info!(
                    "状态错误：table状态-{:?} player状态-{:?}",
                    table_state,
                    player_state
                );
                // println!("状态错误：table状态-{:?} player状态-{:?}", table_state, player_state);
                Err(EPlayerActionError::ActionStatusError)
            }
        }
    }

    pub fn get_state(&self) -> ETableState {
        self.state.clone()
    }

    pub fn buy_chips(&mut self, chips: usize) {
        self.player_chips += chips;
    }

    pub fn reset_player_hand(&mut self) {
        self.player_hands.clear();
        self.player_hands.push(SPlayerHand::new());
    }

    pub fn reset_dealer_hand(&mut self) {
        self.dealer_hand.reset();
    }

    // fn is_player_blackjack(&self) -> bool {
    //     self.player_hands.len() == 1
    //         && self.player_hands.get(0).unwrap().hand.cards.len() == 2
    //         && self.player_hands.get(0).unwrap().value() == EValue::S21
    // }

    #[inline]
    fn is_dealer_blackjack(&self) -> bool {
        self.dealer_hand.is_blackjack()
    }

    fn check_result_and_reset(&mut self) -> ETableOutputEvent {
        // 判断结果
        let mut win_chips_amount = 0;
        let mut bet_chips_amount = 0;

        let dealer_point = self.dealer_hand.point();
        let is_dealer_blackjack = self.is_dealer_blackjack();

        // 计算输赢
        for player_hand in &mut self.player_hands {
            // 计算下注总筹码量(包括下注数量和保险)
            bet_chips_amount += player_hand.betting_box + player_hand.insurance;

            let is_player_blackjack = player_hand.is_blackjack();
            let is_player_bust = player_hand.is_bust();
            let player_point = player_hand.point();

            if is_player_bust || dealer_point > player_point {
                // 玩家失败情况
                if is_dealer_blackjack {
                    // 计算保险
                    win_chips_amount += ((self.rule.insurance_pay)
                        * Fraction::from(player_hand.insurance))
                    .floor()
                    .to_usize()
                    .unwrap();
                }
            } else if dealer_point == player_point {
                // 平局情况
                win_chips_amount += player_hand.get_bet();
            } else {
                // 玩家获胜情况
                win_chips_amount += match is_player_blackjack {
                    true => {
                        ((self.rule.blackjack_pay) * Fraction::from(player_hand.get_bet()))
                            .floor()
                            .to_usize()
                            .unwrap()
                            + player_hand.get_bet()
                    }
                    false => player_hand.get_bet() * 2,
                };
            }
        }
        self.player_chips += win_chips_amount;

        // let is_player_blackjack = self.is_player_blackjack();
        // if is_player_blackjack && is_dealer_blackjack {
        //     // 庄家与玩家均BJ
        //     // Push
        //     // self.player_chips += self.player_hands.get(0).unwrap().get_bet();
        //     win_value += self.player_hands.get(0).unwrap().get_bet();
        // } else if is_player_blackjack && !is_dealer_blackjack {
        //     // 庄家不BJ且玩家BJ
        //     // 玩家胜利
        //     let win_value = (self.rule.blackjack_pay + 1)
        //         * Fraction::from(self.player_hands.get(0).unwrap().get_bet());
        //     let win_value = win_value.floor().to_usize().unwrap();
        //     self.player_hands
        //         .get_mut(0)
        //         .unwrap()
        //         .borrow_mut()
        //         .win(win_value);
        //     win_chips_amount += win_value;
        // } else {
        //     // 庄家与玩家均不BJ
        //     for player_hand in &mut self.player_hands {
        //         let player_point = player_hand.point();
        //         if player_point > dealer_point {
        //             // 玩家胜利
        //             let win_value = player_hand.get_bet();
        //             player_hand.win(win_value);
        //             win_chips_amount += win_value;
        //         } else if player_point < dealer_point
        //             || player_hand.is_bust()
        //             || is_dealer_blackjack
        //         {
        //             // 玩家失败
        //             player_hand.lose();
        //         } else {
        //             // 平局push 无需处理
        //         }
        //         // 将筹码返还给玩家
        //         self.player_chips += player_hand.get_bet();
        //     }
        // }

        // 重置状态
        self.reset_dealer_hand();
        self.reset_player_hand();
        // 状态转移
        self.state = ETableState::PlayerBet;
        ETableOutputEvent::GameOver {
            bet_chips: bet_chips_amount,
            win_chips: win_chips_amount,
            player_chips: self.player_chips,
        }
    }
}
