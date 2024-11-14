use super::value::EValue;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use std::ops::Add;
use strum_macros::{Display, EnumIter};

#[derive(Debug, Copy, Clone)]
pub struct ECard {
    pub color: ECardColor,
    pub value: ECardNumber,
}

#[derive(Debug, Copy, Clone, EnumIter, Hash, Display)]
pub enum ECardColor {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

#[derive(Debug, Copy, Clone, EnumIter, Hash, Eq, PartialEq, Display)]
pub enum ECardNumber {
    Ace = 0,
    Two = 1,
    Three = 2,
    Four = 3,
    Five = 4,
    Six = 5,
    Seven = 6,
    Eight = 7,
    Nine = 8,
    Ten = 9,
    Jack = 10,
    Queen = 11,
    King = 12,
}

/// 所有卡牌的点数 TJQK视为同一类枚举
#[derive(Debug, Copy, Clone, EnumIter, Hash, Eq, PartialEq, Display)]
pub enum ECardPoint {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
}

// ECardPoint impl ----------------------------------------------------------------------------------
impl Into<EValue> for ECardPoint {
    fn into(self) -> EValue {
        match self {
            ECardPoint::Ace => EValue::S11,
            ECardPoint::Two => EValue::H2,
            ECardPoint::Three => EValue::H3,
            ECardPoint::Four => EValue::H4,
            ECardPoint::Five => EValue::H5,
            ECardPoint::Six => EValue::H6,
            ECardPoint::Seven => EValue::H7,
            ECardPoint::Eight => EValue::H8,
            ECardPoint::Nine => EValue::H9,
            ECardPoint::Ten => EValue::H10,
        }
    }
}

impl Add<EValue> for ECardPoint {
    type Output = EValue;

    fn add(self, rhs: EValue) -> Self::Output {
        let tmp: EValue = self.into();
        rhs + tmp
    }
}

impl Add<ECardPoint> for EValue {
    type Output = EValue;

    fn add(self, rhs: ECardPoint) -> Self::Output {
        rhs + self
    }
}

// ECardNumber impl----------------------------------------------------------------------------------
impl Into<ECardPoint> for ECardNumber {
    fn into(self) -> ECardPoint {
        match self {
            ECardNumber::Ace => ECardPoint::Ace,
            ECardNumber::Two => ECardPoint::Two,
            ECardNumber::Three => ECardPoint::Three,
            ECardNumber::Four => ECardPoint::Four,
            ECardNumber::Five => ECardPoint::Five,
            ECardNumber::Six => ECardPoint::Six,
            ECardNumber::Seven => ECardPoint::Seven,
            ECardNumber::Eight => ECardPoint::Eight,
            ECardNumber::Nine => ECardPoint::Nine,
            ECardNumber::Ten => ECardPoint::Ten,
            ECardNumber::Jack => ECardPoint::Ten,
            ECardNumber::Queen => ECardPoint::Ten,
            ECardNumber::King => ECardPoint::Ten,
        }
    }
}

impl Into<EValue> for ECardNumber {
    fn into(self) -> EValue {
        let tmp: ECardPoint = self.into();
        tmp.into()
    }
}

impl Add<EValue> for ECardNumber {
    type Output = EValue;

    fn add(self, rhs: EValue) -> Self::Output {
        let tmp: EValue = self.into();
        rhs + tmp
    }
}

impl Add<ECardNumber> for ECardNumber {
    type Output = EValue;

    fn add(self, rhs: ECardNumber) -> Self::Output {
        let tmp1: EValue = self.into();
        let tmp2: EValue = rhs.into();
        tmp1 + tmp2
    }
}

impl Add<ECardNumber> for EValue {
    type Output = EValue;

    fn add(self, rhs: ECardNumber) -> Self::Output {
        let tmp: EValue = rhs.into();
        self + tmp
    }
}

/// 随机生成一个ECardNumber枚举
impl Distribution<ECardNumber> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> ECardNumber {
        match rng.gen_range(0..=12) {
            0 => ECardNumber::Ace,
            1 => ECardNumber::Two,
            2 => ECardNumber::Three,
            3 => ECardNumber::Four,
            4 => ECardNumber::Five,
            5 => ECardNumber::Six,
            6 => ECardNumber::Seven,
            7 => ECardNumber::Eight,
            8 => ECardNumber::Nine,
            9 => ECardNumber::Ten,
            10 => ECardNumber::Jack,
            11 => ECardNumber::Queen,
            _ => ECardNumber::King,
        }
    }
}


// ECardColor impl ----------------------------------------------------------------------------------
/// 随机生成一个ECardColor枚举
/// #[deprecated]
impl Distribution<ECardColor> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> ECardColor {
        match rng.gen_range(0..=3) {
            0 => ECardColor::Hearts,
            1 => ECardColor::Diamonds,
            2 => ECardColor::Clubs,
            _ => ECardColor::Spades,
        }
    }
}

// ECard impl ----------------------------------------------------------------------------------
impl Into<ECardPoint> for ECard {
    fn into(self) -> ECardPoint {
        self.value.into()
    }
}

impl Into<EValue> for ECard {
    fn into(self) -> EValue {
        self.value.into()
    }
}

impl Add<EValue> for ECard {
    type Output = EValue;

    fn add(self, rhs: EValue) -> Self::Output {
        rhs + self.value
    }
}

impl Add<Self> for ECard {
    type Output = EValue;

    fn add(self, rhs: Self) -> Self::Output {
        let tmp1:EValue = self.into();
        let tmp2:EValue = rhs.into();
        tmp1 + tmp2
    }
}

#[cfg(test)]
mod tests {
    use crate::server::card::{ECard, ECardColor};

    use super::super::value::EValue;
    use super::ECardNumber;
    use strum::IntoEnumIterator;

    #[tokio::test]
    async fn test_add_number_value() {
        let mut error_list = vec![];
        for value in EValue::iter() {
            for card in ECardNumber::iter() {
                let sum1 = value + card;
                let sum2 = card + value;
                if sum1 != sum2 {
                    error_list.push((value, card, sum1, sum2));
                    println!("{:?}\t{:?}\t{:?}\t{:?}", value, card, sum1, sum2);
                }
            }
        }
        assert_eq!(error_list.len(), 0);
    }

    #[tokio::test]
    async fn test_add_number_number() {
        let mut error_list = vec![];
        for card1 in ECardNumber::iter() {
            for card2 in ECardNumber::iter() {
                let sum1 = card1 + card2;
                let sum2 = card2 + card1;
                if sum1 != sum2 {
                    error_list.push((card1, card2, sum1, sum2));
                    println!("{:?}\t{:?}\t{:?}\t{:?}", card1, card2, sum1, sum2);
                }
            }
        }
        assert_eq!(error_list.len(), 0);
    }

    #[tokio::test]
    async fn test_add_card_card() {
        let mut error_list = vec![];
        for value1 in ECardNumber::iter() {
            for color1 in ECardColor::iter() {
                for value2 in ECardNumber::iter() {
                    for color2 in ECardColor::iter() {
                        let card1 = ECard{
                            color:color1,
                            value:value1,
                        };
                        let card2 = ECard{
                            color:color2,
                            value:value2,
                        };
                        let sum1 = card1 + card2;
                        let sum2 = card2 + card1;
                        if sum1 != sum2 {
                            error_list.push((card1, card2, sum1, sum2));
                            println!("{:?}\t{:?}\t{:?}\t{:?}", card1, card2, sum1, sum2);
                        }
                    }
                }
            }
        }
        assert_eq!(error_list.len(), 0);
    }
}
