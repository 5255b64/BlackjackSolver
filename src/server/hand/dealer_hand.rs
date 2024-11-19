use super::super::card::ECard;
use super::super::value::EValue;
use super::hand::SHand;

#[derive(Debug, Clone)]
pub struct SDealerHand {
    pub hand: SHand,
}

impl SDealerHand {
    pub fn new() -> Self {
        SDealerHand { hand: SHand::new() }
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

    pub fn is_blackjack(&self) -> bool {
        self.hand.value().to_point() == 21 && self.hand.cards.len() == 2
    }
}

#[cfg(test)]
mod tests {
    use crate::server::card::{ECard, ECardColor};

    use super::super::super::card::ECardNumber;
    use super::SDealerHand;

    #[tokio::test]
    /// 随机生成两个card 相加求value
    async fn test1() {
        let mut dealer = SDealerHand::new();
        dealer.draw(ECard {
            color: ECardColor::Hearts,
            value: ECardNumber::Eight,
        });
        assert_eq!(dealer.point(), 8);
        dealer.draw(ECard {
            color: ECardColor::Hearts,
            value: ECardNumber::Eight,
        });
        assert_eq!(dealer.point(), 16);
        dealer.draw(ECard {
            color: ECardColor::Hearts,
            value: ECardNumber::Eight,
        });
        assert_eq!(dealer.point(), 1);
    }
}
