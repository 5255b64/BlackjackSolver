use crate::card::ECard;
use crate::hand::hand::SHand;
use crate::value::EValue;

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

    pub fn get_bet(&mut self) -> usize {
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
        cards.len() == 2 && cards.get(0).unwrap() == cards.get(1).unwrap()
    }

    pub fn win(&mut self, value: usize) {
        self.betting_box += value;
    }

    pub fn lose(&mut self) {
        self.betting_box = 0;
    }

    pub fn is_blackjack(&self) -> bool {
        self.hand.is_blackjack()
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
    use crate::card::ECard;
    use crate::hand::player_hand::SPlayerHand;

    #[tokio::test]
    /// 随机生成两个card 相加求value
    async fn test1() {
        let mut player = SPlayerHand::new();
        player.draw(ECard::Eight);
        assert_eq!(player.point(), 8);
        player.draw(ECard::Eight);
        assert_eq!(player.point(), 16);
        player.draw(ECard::Eight);
        assert_eq!(player.point(), 0);
    }
}
