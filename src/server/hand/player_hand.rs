use super::super::card::ECard;
use super::super::value::EValue;
use super::hand::SHand;

#[derive(Debug, Clone)]
pub struct SPlayerHand {
    pub hand: SHand,
    // 本轮玩家下注
    pub betting_box: usize,
    // 保险
    pub insurance: usize,
}
impl SPlayerHand {
    pub fn new() -> Self {
        SPlayerHand {
            hand: SHand::new(),
            betting_box: 0,
            insurance: 0,
        }
    }

    pub fn reset(&mut self) {
        self.hand.reset();
        self.betting_box = 0;
        self.insurance = 0;
    }

    pub fn value(&self) -> EValue {
        self.hand.value()
    }

    pub fn draw(&mut self, card: ECard) {
        self.hand.draw(card);
    }

    /// 获取u8类型点数 用于比较
    pub fn point(&self) -> u8 {
        match self.value().to_point() {
            1 => 0,
            x => x,
        }
    }

    pub fn bet(&mut self, bet: usize) {
        self.betting_box = bet;
    }

    pub fn insurance(&mut self, bet: usize) {
        self.insurance = bet;
    }

    pub fn double_down(&mut self, card: ECard) {
        self.betting_box *= 2;
        self.draw(card);
    }

    pub fn get_bet(&self) -> usize {
        self.betting_box
    }

    pub fn split(&mut self) -> SHand {
        let card = self.hand.cards.pop().unwrap();
        let mut hand = SHand::new();
        hand.draw(card);
        hand
    }

    pub fn is_bust(&self) -> bool {
        self.value() == EValue::Bust
    }

    pub fn should_split(&self) -> bool {
        let cards = &self.hand.cards;
        cards.len() == 2 && cards.get(0).unwrap().value == cards.get(1).unwrap().value
    }

    pub fn win(&mut self, value: usize) {
        self.betting_box += value;
    }

    pub fn lose(&mut self) {
        self.betting_box = 0;
    }
}

impl From<SHand> for SPlayerHand {
    fn from(value: SHand) -> Self {
        SPlayerHand {
            hand: value,
            betting_box: 0,
            insurance: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::server::card::{ECard, ECardColor};

    use super::super::super::card::ECardNumber;
    use super::SPlayerHand;

    #[tokio::test]
    /// 随机生成两个card 相加求value
    async fn test1() {
        let mut player = SPlayerHand::new();
        player.draw(ECard {
            color: ECardColor::Hearts,
            value: ECardNumber::Eight,
        });
        assert_eq!(player.point(), 8);
        player.draw(ECard {
            color: ECardColor::Hearts,
            value: ECardNumber::Eight,
        });
        assert_eq!(player.point(), 16);
        player.draw(ECard {
            color: ECardColor::Hearts,
            value: ECardNumber::Eight,
        });
        assert_eq!(player.point(), 0);
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
        let mut hand = SPlayerHand::new();
        hand.draw(card1);
        println!("draw card1: {hand:?}");
        hand.draw(card2);
        println!("draw card2: {hand:?}");
        hand.draw(card3);
        println!("draw card3: {hand:?}");
    }
}
