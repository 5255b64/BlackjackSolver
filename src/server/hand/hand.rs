use std::fmt::{Display, Formatter};
use super::super::card::ECardNumber;
use super::super::value::EValue;

#[derive(Debug, Clone)]
pub struct SHand {
    pub cards: Vec<ECardNumber>,
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
    pub fn draw(&mut self, card: ECardNumber) {
        self.cards.push(card);
        self.value = self.value + card;
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

    pub fn is_blackjack(&self) -> bool {
        self.value().to_point() == 21 && self.cards.len() == 2
    }
}

impl Display for SHand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:?}, {:?})", self.value, self.cards)
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::card::ECardNumber;
    use super::SHand;

    #[tokio::test]
    /// 随机生成两个card 相加求value
    async fn test1() {
        let mut hand = SHand::new();
        println!("{hand}");

        // add
        let card: ECardNumber = rand::random();
        println!("add {card:?}");
        hand.draw(card);
        println!("{hand}");

        // add
        let card: ECardNumber = rand::random();
        println!("add {card:?}");
        hand.draw(card);
        println!("{hand}");

        // add
        let card: ECardNumber = rand::random();
        println!("add {card:?}");
        hand.draw(card);
        println!("{hand}");

        // add
        let card: ECardNumber = rand::random();
        println!("add {card:?}");
        hand.draw(card);
        println!("{hand}");

        // add
        let card: ECardNumber = rand::random();
        println!("add {card:?}");
        hand.draw(card);
        println!("{hand}");

        // reset
        println!("reset");
        hand.reset();
        println!("{hand}");

        // add
        let card: ECardNumber = rand::random();
        println!("add {card:?}");
        hand.draw(card);
        println!("{hand}");

        // add
        let card: ECardNumber = rand::random();
        println!("add {card:?}");
        hand.draw(card);
        println!("{hand}");

        // add
        let card: ECardNumber = rand::random();
        println!("add {card:?}");
        hand.draw(card);
        println!("{hand}");

        // add
        let card: ECardNumber = rand::random();
        println!("add {card:?}");
        hand.draw(card);
        println!("{hand}");

        // add
        let card: ECardNumber = rand::random();
        println!("add {card:?}");
        hand.draw(card);
        println!("{hand}");
    }
}

