use crate::server::card::{ECard, ECardColor};
use fraction::Fraction;
use rand::Rng;
use std::collections::HashMap;

use super::super::card::{ECardNumber, ECardPoint};
use super::super::deck::TDeck;
use super::ECardNum;

/// 随机卡池
/// 根据卡牌的初始占比，按概率抽牌。
/// 无状态：当某张牌从牌库中抽出后，不影响后续抽牌的概率。
pub struct SRandomDeck {
    pub cards: Vec<ECard>,
    pub point_probability_map: HashMap<ECardPoint, Fraction>,
}

impl SRandomDeck {
    pub fn new() -> Self {
        let card_num_vec = vec![
            ECardNumber::Ace,
            ECardNumber::Two,
            ECardNumber::Three,
            ECardNumber::Four,
            ECardNumber::Five,
            ECardNumber::Six,
            ECardNumber::Seven,
            ECardNumber::Eight,
            ECardNumber::Nine,
            ECardNumber::Ten,
            ECardNumber::Jack,
            ECardNumber::Queen,
            ECardNumber::King,
        ];
        let card_color_vec = vec![
            ECardColor::Hearts,
            ECardColor::Diamonds,
            ECardColor::Clubs,
            ECardColor::Spades,
        ];
        let mut cards: Vec<ECard> = Vec::new();
        for value in &card_num_vec {
            for color in &card_color_vec {
                cards.push(ECard {
                    color: *color,
                    value: *value,
                });
            }
        }
        let mut point_probability_map: HashMap<ECardPoint, Fraction> = HashMap::new();

        point_probability_map.insert(ECardPoint::Ace, Fraction::new(1u64, 13u64));
        point_probability_map.insert(ECardPoint::Two, Fraction::new(1u64, 13u64));
        point_probability_map.insert(ECardPoint::Three, Fraction::new(1u64, 13u64));
        point_probability_map.insert(ECardPoint::Four, Fraction::new(1u64, 13u64));
        point_probability_map.insert(ECardPoint::Five, Fraction::new(1u64, 13u64));
        point_probability_map.insert(ECardPoint::Six, Fraction::new(1u64, 13u64));
        point_probability_map.insert(ECardPoint::Seven, Fraction::new(1u64, 13u64));
        point_probability_map.insert(ECardPoint::Eight, Fraction::new(1u64, 13u64));
        point_probability_map.insert(ECardPoint::Nine, Fraction::new(1u64, 13u64));
        point_probability_map.insert(ECardPoint::Ten, Fraction::new(4u64, 13u64));

        SRandomDeck {
            cards,
            point_probability_map,
        }
    }
}

impl TDeck for SRandomDeck {
    fn draw(&mut self) -> Option<ECard> {
        match self
            .cards
            .get(rand::thread_rng().gen_range(0..self.cards.len()))
        {
            None => None,
            Some(x) => Some(*x),
        }
    }

    fn draw_specific(&mut self, card_num: ECardNumber) -> Option<ECardNumber> {
        Some(card_num)
    }

    fn shuffle(&mut self) {
        // do nothing
        ()
    }

    fn get_point_probability_map(&self) -> &HashMap<ECardPoint, Fraction> {
        &self.point_probability_map
    }
    fn remain_cards_num(&self) -> ECardNum {
        ECardNum::Infinite
    }
    
    fn cards_num(&self) -> ECardNum {
        ECardNum::Infinite
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::card::{ECardNumber, ECardPoint};
    use super::super::super::deck::TDeck;
    use super::SRandomDeck;

    #[tokio::test]
    async fn test_draw() {
        let mut deck = SRandomDeck::new();
        for _ in 0..10 {
            println!("{:?}", deck.draw())
        }
    }
    #[tokio::test]
    async fn test_draw_specific() {
        let mut deck = SRandomDeck::new();
        println!("{:?}", deck.draw_specific(ECardNumber::Eight));
        println!("{:?}", deck.draw_specific(ECardNumber::Ace));
        println!("{:?}", deck.draw_specific(ECardNumber::Jack));
        println!("{:?}", deck.draw_specific(ECardNumber::Five));
        println!("{:?}", deck.draw_specific(ECardNumber::King));
    }
    #[tokio::test]
    async fn test_get_probability_map() {
        let deck = SRandomDeck::new();
        let map = deck.get_point_probability_map();
        println!("{:?}", map);
        println!("{:?}", map.get(&ECardPoint::Ace));
        println!("{:?}", map.get(&ECardPoint::Two));
        println!("{:?}", map.get(&ECardPoint::Seven));
        println!("{:?}", map.get(&ECardPoint::Ten));
    }
}
