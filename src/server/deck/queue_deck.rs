use crate::server::card::{ECard, ECardColor};
use fraction::Fraction;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashMap;
use strum::IntoEnumIterator;

use super::super::card::{ECardNumber, ECardPoint};
use super::super::deck::TDeck;
use super::{new_probability_map_from_cards, new_probability_map_from_number_map, ECardNum};

/// 队列卡池
/// 有状态：当某张牌从牌库中抽出后，影响后续抽牌的概率。
pub struct SQueueDeck {
    pub cards: Vec<ECard>,
    pub number_map: HashMap<ECardNumber, usize>,
    pub point_probability_map: HashMap<ECardPoint, Fraction>,
    // 指针 指向下一张抽到的牌
    draw_ptr: usize,
}

impl SQueueDeck {
    pub fn new(num_of_deck: u8) -> Self {
        let mut cards = Vec::new();

        for _ in 0..num_of_deck {
            for value in ECardNumber::iter() {
                for color in ECardColor::iter() {
                    cards.push(ECard { color, value });
                }
            }
        }

        let (number_map, point_probability_map) = new_probability_map_from_cards(&cards);

        let mut deck = SQueueDeck {
            cards,
            number_map,
            point_probability_map,
            draw_ptr: 0,
        };

        deck.shuffle();

        deck
    }

    fn inner_remain_cards_num(&self) -> usize {
        self.cards.len() - self.draw_ptr
    }

    fn inner_cards_num(&self) -> usize {
        self.cards.len()
    }
}

impl TDeck for SQueueDeck {
    fn draw(&mut self) -> Option<ECard> {
        // 判断是否shuffle
        // 手牌不足时需要shuffle
        if self.inner_remain_cards_num() <= 0 {
            self.shuffle();
        }

        // 抽出一张卡
        let result = match self.cards.get(self.draw_ptr) {
            None => None,
            Some(x) => {
                self.draw_ptr += 1;
                Some(*x)
            }
        };
        // 重新计算map
        if let Some(x) = result {
            let value = x.value;
            self.number_map
                .insert(value, *self.number_map.get(&value).unwrap() - 1);
            let num = match self.remain_cards_num() {
                ECardNum::Some(x) => x,
                ECardNum::Infinite => 0,
            };
            self.point_probability_map = new_probability_map_from_number_map(&self.number_map, num);
        }

        result
    }

    fn draw_specific(&mut self, card_num: ECardNumber) -> Option<ECardNumber> {
        for ptr in self.draw_ptr..self.cards.len() {
            if self.cards.get(ptr).unwrap().value == card_num {
                // swap
                let ele1 = self.cards.get(ptr).unwrap().clone();
                let ele2 = self.cards.get(self.draw_ptr).unwrap().clone();
                let ptr1 = self.cards.get_mut(ptr).unwrap();
                *ptr1 = ele2;
                let ptr2 = self.cards.get_mut(self.draw_ptr).unwrap();
                *ptr2 = ele1;
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
        (self.number_map, self.point_probability_map) = new_probability_map_from_cards(&self.cards);
    }

    fn get_point_probability_map(&self) -> &HashMap<ECardPoint, Fraction> {
        &self.point_probability_map
    }

    fn remain_cards_num(&self) -> ECardNum {
        ECardNum::Some(self.inner_remain_cards_num())
    }

    fn cards_num(&self) -> super::ECardNum {
        ECardNum::Some(self.inner_cards_num())
    }
}

#[cfg(test)]
mod tests {
    use crate::server::deck::{queue_deck::SQueueDeck, TDeck};

    #[tokio::test]
    pub async fn test_draw_and_remain() {
        let mut deck = SQueueDeck::new(2);
        let draw_num = 10;
        for _ in 0..draw_num {
            println!("card_num:{:?}", deck.cards_num());
            println!("draw_ptr:{:?}", deck.draw_ptr);
            println!("draw_card:{:?}", deck.draw());
            println!("remain_cards:{:?}", deck.remain_cards_num());
            let map = deck.get_point_probability_map();
            println!("prob map:",);
            for key in map.keys() {
                println!("key:{key:?}, value:{:?}", map.get(&key));
            }
        }
    }
}
