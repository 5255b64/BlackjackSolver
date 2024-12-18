use crate::server::card::{ECard, ECardColor};
use fraction::Fraction;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use std::collections::HashMap;
use strum::IntoEnumIterator;

use super::super::card::{ECardNumber, ECardPoint};
use super::super::deck::TDeck;

/// 手动构建卡池
/// 手动指定卡池顺序
/// 有状态：当某张牌从牌库中抽出后，影响后续抽牌的概率。
pub struct SDiyDeck {
    pub cards: Vec<ECard>,
    pub number_probability_map: HashMap<ECardNumber, Fraction>,
    pub point_probability_map: HashMap<ECardPoint, Fraction>,
    // 指针 指向下一张抽到的牌
    draw_ptr: usize,
}

impl SDiyDeck {
    pub fn from(cards: Vec<ECard>) -> Self {
        let cards_len = cards.len();
        let mut map = HashMap::<ECardNumber, usize>::new();
        for card in &cards {
            let num = card.value;
            if map.contains_key(&num) {
                map.insert(num, map.get(&num).unwrap() + 1);
            }
        }

        let mut number_probability_map: HashMap<ECardNumber, Fraction> = HashMap::new();

        for e in ECardNumber::iter() {
            let num = match map.get(&e) {
                Some(r) => *r,
                None => 0,
            };
            number_probability_map.insert(
                ECardNumber::Ace,
                Fraction::new(num as u64, cards_len as u64),
            );
        }

        // number_probability_map.insert(ECardNumber::Ace, Fraction::new(1u64, 13u64));
        // number_probability_map.insert(ECardNumber::Two, Fraction::new(1u64, 13u64));
        // number_probability_map.insert(ECardNumber::Three, Fraction::new(1u64, 13u64));
        // number_probability_map.insert(ECardNumber::Four, Fraction::new(1u64, 13u64));
        // number_probability_map.insert(ECardNumber::Five, Fraction::new(1u64, 13u64));
        // number_probability_map.insert(ECardNumber::Six, Fraction::new(1u64, 13u64));
        // number_probability_map.insert(ECardNumber::Seven, Fraction::new(1u64, 13u64));
        // number_probability_map.insert(ECardNumber::Eight, Fraction::new(1u64, 13u64));
        // number_probability_map.insert(ECardNumber::Nine, Fraction::new(1u64, 13u64));
        // number_probability_map.insert(ECardNumber::Ten, Fraction::new(1u64, 13u64));
        // number_probability_map.insert(ECardNumber::Jack, Fraction::new(1u64, 13u64));
        // number_probability_map.insert(ECardNumber::Queen, Fraction::new(1u64, 13u64));
        // number_probability_map.insert(ECardNumber::King, Fraction::new(1u64, 13u64));

        let mut point_probability_map: HashMap<ECardPoint, Fraction> = HashMap::new();

        // point_probability_map.insert(ECardPoint::Ace, Fraction::new(1u64, 13u64));
        // point_probability_map.insert(ECardPoint::Two, Fraction::new(1u64, 13u64));
        // point_probability_map.insert(ECardPoint::Three, Fraction::new(1u64, 13u64));
        // point_probability_map.insert(ECardPoint::Four, Fraction::new(1u64, 13u64));
        // point_probability_map.insert(ECardPoint::Five, Fraction::new(1u64, 13u64));
        // point_probability_map.insert(ECardPoint::Six, Fraction::new(1u64, 13u64));
        // point_probability_map.insert(ECardPoint::Seven, Fraction::new(1u64, 13u64));
        // point_probability_map.insert(ECardPoint::Eight, Fraction::new(1u64, 13u64));
        // point_probability_map.insert(ECardPoint::Nine, Fraction::new(1u64, 13u64));
        // point_probability_map.insert(ECardPoint::Ten, Fraction::new(4u64, 13u64));

        SDiyDeck {
            cards,
            number_probability_map,
            point_probability_map,
            draw_ptr: 0,
        }
    }

    #[inline]
    pub fn cards_num(&self) -> usize {
        self.cards.len()
    }

    #[inline]
    pub fn remain_cards_num(&self) -> usize {
        self.cards_num() - self.draw_ptr
    }
}

impl TDeck for SDiyDeck {
    fn draw(&mut self) -> Option<ECard> {
        match self.cards.get(self.draw_ptr) {
            None => None,
            Some(x) => {
                self.draw_ptr += 1;
                Some(*x)
            }
        }
    }

    fn draw_specific(&mut self, card_num: ECardNumber) -> Option<ECardNumber> {
        for ptr in self.draw_ptr..self.cards.len() {
            if self.cards.get(ptr).unwrap().value == card_num {
                // swap
                let tmp = self.cards.get(ptr).unwrap().clone();
                self.cards.remove(ptr);
                self.cards
                    .insert(ptr, self.cards.get(self.draw_ptr).unwrap().clone());
                self.cards.remove(self.draw_ptr);
                self.cards.insert(self.draw_ptr, tmp);
                return match self.draw() {
                    Some(card) => Some(card.value),
                    None => None,
                };
            }
        }
        None
    }

    fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
        self.draw_ptr = 0;
    }

    fn get_point_probability_map(&self) -> &HashMap<ECardPoint, Fraction> {
        &self.point_probability_map
    }
}

#[cfg(test)]
mod tests {
    use crate::server::{
        card::{ECard, ECardColor},
        deck::diy_deck::*,
    };

    use super::super::super::card::ECardNumber;

    #[tokio::test]
    async fn test_draw() {
        let v = vec![
            ECard {
                color: ECardColor::Hearts,
                value: ECardNumber::Ace,
            },
            ECard {
                color: ECardColor::Hearts,
                value: ECardNumber::Ten,
            },
            ECard {
                color: ECardColor::Hearts,
                value: ECardNumber::Six,
            },
            ECard {
                color: ECardColor::Hearts,
                value: ECardNumber::Six,
            },
        ];
        let mut deck = SDiyDeck::from(v);
        for _ in 0..deck.cards_num() {
            println!("{:?}", deck.draw());
            println!("remain_cards:{:?}", deck.remain_cards_num())
        }
    }

    #[tokio::test]
    async fn test_draw_specific() {
        let v = vec![
            ECard {
                color: ECardColor::Hearts,
                value: ECardNumber::Ace,
            },
            ECard {
                color: ECardColor::Hearts,
                value: ECardNumber::Ten,
            },
            ECard {
                color: ECardColor::Hearts,
                value: ECardNumber::Six,
            },
            ECard {
                color: ECardColor::Hearts,
                value: ECardNumber::Six,
            },
        ];
        let mut deck = SDiyDeck::from(v);
        println!("cards:{:?}", deck.cards);
        println!("ptr:{:?}", deck.draw_ptr);
        println!("draw:{:?}", deck.draw_specific(ECardNumber::Six));
        println!("cards:{:?}", deck.cards);
        println!("ptr:{:?}", deck.draw_ptr);
    }
}
