use crate::card::ECard;
use crate::hand::hand::SHand;
use crate::value::EValue;

#[derive(Debug, Clone)]
pub struct SDealerHand {
    pub hand:SHand
}

impl SDealerHand {
    pub fn new() -> Self {
        SDealerHand{
            hand: SHand::new(),
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
        self.value().to_point()
    }
}

#[cfg(test)]
mod tests {
    use crate::card::ECard;
    use crate::hand::dealer_hand::SDealerHand;

    #[tokio::test]
    /// 随机生成两个card 相加求value
    async fn test1() {
        let mut dealer = SDealerHand::new();
        dealer.draw(ECard::Eight);
        assert_eq!(dealer.point(), 8);
        dealer.draw(ECard::Eight);
        assert_eq!(dealer.point(), 16);
        dealer.draw(ECard::Eight);
        assert_eq!(dealer.point(), 1);
    }
}

