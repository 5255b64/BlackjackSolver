use std::ops::Add;
use strum_macros::{Display, EnumIter};
use crate::value::EValue;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

#[derive(Debug, Copy, Clone, EnumIter, Hash, Eq, PartialEq, Display)]
pub enum ECard {
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
    Jack,
    Queen,
    King,
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
impl From<ECard> for ECardPoint {
    fn from(value: ECard) -> Self {
        match value {
            ECard::Ace => ECardPoint::Ace,
            ECard::Two => ECardPoint::Two,
            ECard::Three => ECardPoint::Three,
            ECard::Four => ECardPoint::Four,
            ECard::Five => ECardPoint::Five,
            ECard::Six => ECardPoint::Six,
            ECard::Seven => ECardPoint::Seven,
            ECard::Eight => ECardPoint::Eight,
            ECard::Nine => ECardPoint::Nine,
            ECard::Ten => ECardPoint::Ten,
            ECard::Jack => ECardPoint::Ten,
            ECard::Queen => ECardPoint::Ten,
            ECard::King => ECardPoint::Ten,
        }
    }
}

impl From<ECardPoint> for EValue {
    fn from(value: ECardPoint) -> Self {
        match value {
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
        EValue::from(rhs)  + EValue::from(self)
    }
}

impl Add<ECardPoint> for ECardPoint {
    type Output = EValue;

    fn add(self, rhs: ECardPoint) -> Self::Output {
        EValue::from(self) + EValue::from(rhs)
    }
}

impl Add<ECardPoint> for EValue {
    type Output = EValue;

    fn add(self, rhs: ECardPoint) -> Self::Output {
        self + EValue::from(rhs)
    }
}

// ECard impl----------------------------------------------------------------------------------
impl From<ECard> for EValue {
    fn from(value: ECard) -> Self {
        // match value {
        //     ECard::Ace => EValue::S11,
        //     ECard::Two => EValue::H2,
        //     ECard::Three => EValue::H3,
        //     ECard::Four => EValue::H4,
        //     ECard::Five => EValue::H5,
        //     ECard::Six => EValue::H6,
        //     ECard::Seven => EValue::H7,
        //     ECard::Eight => EValue::H8,
        //     ECard::Nine => EValue::H9,
        //     ECard::Ten => EValue::H10,
        //     ECard::Jack => EValue::H10,
        //     ECard::Queen => EValue::H10,
        //     ECard::King => EValue::H10,
        // }
        EValue::from(ECardPoint::from(value))
    }
}

impl Add<EValue> for ECard {
    type Output = EValue;

    fn add(self, rhs: EValue) -> Self::Output {
        EValue::from(rhs)  + EValue::from(self)
    }
}

impl Add<ECard> for ECard {
    type Output = EValue;

    fn add(self, rhs: ECard) -> Self::Output {
        EValue::from(self) + EValue::from(rhs)
    }
}

impl Add<ECard> for EValue {
    type Output = EValue;

    fn add(self, rhs: ECard) -> Self::Output {
        self + EValue::from(rhs)
    }
}

/// 随机生成一个ECard枚举
impl Distribution<ECard> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> ECard {
        match rng.gen_range(0..=12) {
            0 => ECard::Ace,
            1 => ECard::Two,
            2 => ECard::Three,
            3 => ECard::Four,
            4 => ECard::Five,
            5 => ECard::Six,
            6 => ECard::Seven,
            7 => ECard::Eight,
            8 => ECard::Nine,
            9 => ECard::Ten,
            10 => ECard::Jack,
            11 => ECard::Queen,
            _ => ECard::King,
        }
    }
}

impl ECard {
    pub fn to_point(&self) -> ECardPoint {
        match self {
            ECard::Ace => ECardPoint::Ace,
            ECard::Two => ECardPoint::Two,
            ECard::Three => ECardPoint::Three,
            ECard::Four => ECardPoint::Four,
            ECard::Five => ECardPoint::Five,
            ECard::Six => ECardPoint::Six,
            ECard::Seven => ECardPoint::Seven,
            ECard::Eight => ECardPoint::Eight,
            ECard::Nine => ECardPoint::Nine,
            ECard::Ten => ECardPoint::Ten,
            ECard::Jack => ECardPoint::Ten,
            ECard::Queen => ECardPoint::Ten,
            ECard::King => ECardPoint::Ten,
        }
    }
}

#[cfg(test)]
mod tests{
    use strum::IntoEnumIterator;
    use crate::card::ECard;
    use crate::value::EValue;

    #[tokio::test]
    async fn test_add_value(){
        let mut error_list = vec![];
        for value in EValue::iter() {
            for card in ECard::iter() {
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
    async fn test_add_card(){
        let mut error_list = vec![];
        for card1 in ECard::iter() {
            for card2 in ECard::iter() {
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
}
