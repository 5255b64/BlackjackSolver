use crate::card::ECard;
use crate::hand::hand::SHand;
use crate::value::EValue;

#[derive(Debug, Clone)]
pub struct SPlayerHand {
    pub hand:SHand,
    pub bet:usize,
    pub betting_box:usize,
}

impl SPlayerHand {
    pub fn new() -> Self {
        SPlayerHand{
            hand: SHand::new(),
            bet: 0,
            betting_box: 0,
        }
    }

    pub fn reset(&mut self) {
        self.hand.reset();
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
