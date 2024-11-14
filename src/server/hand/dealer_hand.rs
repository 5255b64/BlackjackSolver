use super::super::card::ECardNumber;
use super::hand::SHand;
use super::super::value::EValue;

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

    pub fn draw(&mut self, card: ECardNumber) {
        self.hand.draw(card);
    }

    /// 获取u8类型点数 用于比较
    pub fn point(&self) -> u8 {
        self.value().to_point()
    }

    pub fn is_blackjack(&self) -> bool {
        self.hand.is_blackjack()
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::card::ECardNumber;
    use super::SDealerHand;

    #[tokio::test]
    /// 随机生成两个card 相加求value
    async fn test1() {
        let mut dealer = SDealerHand::new();
        dealer.draw(ECardNumber::Eight);
        assert_eq!(dealer.point(), 8);
        dealer.draw(ECardNumber::Eight);
        assert_eq!(dealer.point(), 16);
        dealer.draw(ECardNumber::Eight);
        assert_eq!(dealer.point(), 1);
    }
}

