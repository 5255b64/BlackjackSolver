use std::collections::HashMap;
use fraction::Fraction;
use rand::Rng;
use crate::card::{ECard, ECardPoint};
use crate::deck::TDeck;

/// 随即卡池
/// 根据卡牌的初始占比，按概率抽牌。
/// 无状态：当某张牌从牌库中抽出后，不影响后续抽牌的概率。
pub struct SRandomDeck {
    pub cards: Vec<ECard>,
    pub card_probability_map: HashMap<ECard, Fraction>,
    pub point_probability_map: HashMap<ECardPoint, Fraction>,
}

impl SRandomDeck {
    pub fn new() -> Self {
        let cards = vec![
            ECard::Ace,
            ECard::Two,
            ECard::Three,
            ECard::Four,
            ECard::Five,
            ECard::Six,
            ECard::Seven,
            ECard::Eight,
            ECard::Nine,
            ECard::Ten,
            ECard::Jack,
            ECard::Queen,
            ECard::King,
        ];
        let mut card_probability_map: HashMap<ECard, Fraction> = HashMap::new();

        card_probability_map.insert(ECard::Ace, Fraction::new(1u64, 13u64));
        card_probability_map.insert(ECard::Two, Fraction::new(1u64, 13u64));
        card_probability_map.insert(ECard::Three, Fraction::new(1u64, 13u64));
        card_probability_map.insert(ECard::Four, Fraction::new(1u64, 13u64));
        card_probability_map.insert(ECard::Five, Fraction::new(1u64, 13u64));
        card_probability_map.insert(ECard::Six, Fraction::new(1u64, 13u64));
        card_probability_map.insert(ECard::Seven, Fraction::new(1u64, 13u64));
        card_probability_map.insert(ECard::Eight, Fraction::new(1u64, 13u64));
        card_probability_map.insert(ECard::Nine, Fraction::new(1u64, 13u64));
        card_probability_map.insert(ECard::Ten, Fraction::new(1u64, 13u64));
        card_probability_map.insert(ECard::Jack, Fraction::new(1u64, 13u64));
        card_probability_map.insert(ECard::Queen, Fraction::new(1u64, 13u64));
        card_probability_map.insert(ECard::King, Fraction::new(1u64, 13u64));


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
            card_probability_map,
            point_probability_map
        }
    }
}

impl TDeck for SRandomDeck {
    fn draw(&mut self) -> Option<ECard> {
        match self.cards.get(rand::thread_rng().gen_range(0..self.cards.len())) {
            None => None,
            Some(x) => Some(*x)
        }
    }

    fn draw_specific(&mut self, card: ECard) -> Option<ECard> {
        Some(card)
    }

    fn shuffle(&mut self) {
        // do nothing
        ()
    }

    fn get_point_probability_map(&self) -> &HashMap<ECardPoint, Fraction> {
        &self.point_probability_map
    }
}


#[cfg(test)]
mod tests {
    use crate::card::{ECard, ECardPoint};
    use crate::deck::random_deck::SRandomDeck;
    use crate::deck::TDeck;

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
        println!("{:?}", deck.draw_specific(ECard::Eight));
        println!("{:?}", deck.draw_specific(ECard::Ace));
        println!("{:?}", deck.draw_specific(ECard::Jack));
        println!("{:?}", deck.draw_specific(ECard::Five));
        println!("{:?}", deck.draw_specific(ECard::King));
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
