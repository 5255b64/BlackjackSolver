use crate::server::card::ECard;
use fraction::Fraction;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashMap;

use super::super::card::{ECardNumber, ECardPoint};
use super::super::deck::TDeck;
use super::{new_probability_map_from_cards, new_probability_map_from_number_map, ECardNum};

/// 手动构建卡池
/// 手动指定卡池顺序
/// 有状态：当某张牌从牌库中抽出后，影响后续抽牌的概率。
pub struct SDiyDeck {
    pub cards: Vec<ECard>,
    pub number_map: HashMap<ECardNumber, usize>,
    pub point_probability_map: HashMap<ECardPoint, Fraction>,
    // 指针 指向下一张抽到的牌
    pub draw_ptr: usize,
}

impl SDiyDeck {
    pub fn from(cards: Vec<ECard>) -> Self {
        let (number_map, point_probability_map) = new_probability_map_from_cards(&cards);

        SDiyDeck {
            cards,
            number_map,
            point_probability_map,
            draw_ptr: 0,
        }
    }

    fn inner_remain_cards_num(&self) -> usize {
        self.cards.len() - self.draw_ptr
    }

    fn inner_cards_num(&self) -> usize {
        self.cards.len()
    }
}

impl TDeck for SDiyDeck {
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
    use strum::IntoEnumIterator;

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
            ECard {
                color: ECardColor::Hearts,
                value: ECardNumber::Six,
            },
            ECard {
                color: ECardColor::Hearts,
                value: ECardNumber::Six,
            },
            ECard {
                color: ECardColor::Hearts,
                value: ECardNumber::Six,
            },
            ECard {
                color: ECardColor::Hearts,
                value: ECardNumber::Six,
            },
            ECard {
                color: ECardColor::Hearts,
                value: ECardNumber::Nine,
            },
        ];
        let mut deck = SDiyDeck::from(v);
        let cards_num = match deck.cards_num() {
            ECardNum::Some(num) => num,
            ECardNum::Infinite => 0,
        };
        for _ in 0..cards_num {
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

    #[tokio::test]
    async fn test_probability_map() {
        let v = vec![
            ECard {
                color: ECardColor::Hearts,
                value: ECardNumber::Ace,
            },
            ECard {
                color: ECardColor::Hearts,
                value: ECardNumber::Two,
            },
            ECard {
                color: ECardColor::Hearts,
                value: ECardNumber::Three,
            },
            ECard {
                color: ECardColor::Hearts,
                value: ECardNumber::Four,
            },
            ECard {
                color: ECardColor::Hearts,
                value: ECardNumber::Five,
            },
            ECard {
                color: ECardColor::Hearts,
                value: ECardNumber::Six,
            },
            ECard {
                color: ECardColor::Hearts,
                value: ECardNumber::Seven,
            },
            ECard {
                color: ECardColor::Hearts,
                value: ECardNumber::Eight,
            },
            ECard {
                color: ECardColor::Hearts,
                value: ECardNumber::Nine,
            },
            ECard {
                color: ECardColor::Hearts,
                value: ECardNumber::Ten,
            },
            ECard {
                color: ECardColor::Hearts,
                value: ECardNumber::Jack,
            },
            ECard {
                color: ECardColor::Hearts,
                value: ECardNumber::Queen,
            },
            ECard {
                color: ECardColor::Hearts,
                value: ECardNumber::King,
            },
        ];
        let deck = SDiyDeck::from(v);
        let point_map = deck.get_point_probability_map();
        let number_map = &deck.number_map;
        for point in ECardPoint::iter() {
            println!("point_map:{:?}:{:?}", point, point_map.get(&point));
        }
        for number in ECardNumber::iter() {
            println!("number_map:{:?}:{:?}", number, number_map.get(&number));
        }
    }
}
