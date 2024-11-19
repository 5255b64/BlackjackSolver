use std::fmt::{Display, Formatter};

use super::super::card::ECard;
use super::super::value::EValue;

#[derive(Debug, Clone)]
pub struct SHand {
    pub cards: Vec<ECard>,
    pub value: EValue,
}

impl SHand {
    pub fn new() -> Self {
        SHand {
            cards: Vec::new(),
            value: EValue::None,
        }
    }

    /// 获得一张手牌 并且更新value
    pub fn draw(&mut self, card: ECard) {
        self.cards.push(card);
        self.value = card + self.value;
    }

    pub fn reset(&mut self) {
        self.cards.clear();
        self.value = EValue::None;
    }

    // /// 删除一张手牌 更新value 并返回删除的手牌
    // pub fn return_card(&mut self) -> Option<ECard> {
    //     self.cards.pop()
    // }
    //
    // /// 根据手牌 重新计算value
    // /// 计算量偏大
    // #[deprecated]
    // pub fn update_value(&mut self) {
    //     let mut value = EValue::None;
    //     for card in &mut self.cards {
    //         value = value + *card;
    //     }
    //     self.value = value;
    // }

    pub fn value(&self) -> EValue {
        self.value
    }
}

impl Display for SHand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:?}, {:?})", self.value, self.cards)
    }
}

#[cfg(test)]
mod tests {
    use crate::server::card::{ECard, ECardColor};

    use super::super::super::card::ECardNumber;
    use super::SHand;

    #[tokio::test]
    /// 随机生成两个card 相加求value
    async fn test1() {
        let mut hand = SHand::new();
        println!("{hand}");

        // add
        let card: ECard = ECard {
            color: rand::random(),
            value: rand::random(),
        };
        println!("add {card:?}");
        hand.draw(card);
        println!("{hand}");

        // add
        let card: ECard = ECard {
            color: rand::random(),
            value: rand::random(),
        };
        println!("add {card:?}");
        hand.draw(card);
        println!("{hand}");

        // add
        let card: ECard = ECard {
            color: rand::random(),
            value: rand::random(),
        };
        println!("add {card:?}");
        hand.draw(card);
        println!("{hand}");

        // add
        let card: ECard = ECard {
            color: rand::random(),
            value: rand::random(),
        };
        println!("add {card:?}");
        hand.draw(card);
        println!("{hand}");

        // add
        let card: ECard = ECard {
            color: rand::random(),
            value: rand::random(),
        };
        println!("add {card:?}");
        hand.draw(card);
        println!("{hand}");

        // reset
        println!("reset");
        hand.reset();
        println!("{hand}");

        // add
        let card: ECard = ECard {
            color: rand::random(),
            value: rand::random(),
        };
        println!("add {card:?}");
        hand.draw(card);
        println!("{hand}");

        // add
        let card: ECard = ECard {
            color: rand::random(),
            value: rand::random(),
        };
        println!("add {card:?}");
        hand.draw(card);
        println!("{hand}");

        // add
        let card: ECard = ECard {
            color: rand::random(),
            value: rand::random(),
        };
        println!("add {card:?}");
        hand.draw(card);
        println!("{hand}");

        // add
        let card: ECard = ECard {
            color: rand::random(),
            value: rand::random(),
        };
        println!("add {card:?}");
        hand.draw(card);
        println!("{hand}");

        // add
        let card: ECard = ECard {
            color: rand::random(),
            value: rand::random(),
        };
        println!("add {card:?}");
        hand.draw(card);
        println!("{hand}");
    }

    #[test]
    fn test_diy() {
        
        let card1 = ECard {
            color: ECardColor::Clubs,
            value: ECardNumber::Four,
        };
        let card2 = ECard {
            color: ECardColor::Diamonds,
            value: ECardNumber::Ace,
        };
        let card3 = ECard {
            color: ECardColor::Spades,
            value: ECardNumber::Three,
        };
        let mut hand = SHand::new();
        hand.draw(card1);
        println!("draw card1: {hand:?}");
        hand.draw(card2);
        println!("draw card2: {hand:?}");
        hand.draw(card3);
        println!("draw card3: {hand:?}");
    }
}
