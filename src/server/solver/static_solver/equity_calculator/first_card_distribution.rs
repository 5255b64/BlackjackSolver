use std::collections::HashMap;
use std::fmt::{Display};
use fraction::Fraction;
use strum::IntoEnumIterator;
use tabled::{Tabled};
use super::super::super::super::card::{ECardNumber, ECardPoint};
use super::super::super::super::deck::TDeck;

#[derive(Tabled)]
pub struct SFirstCardDistributionCell {
    pub first_card_point: ECardPoint,
    pub probability: Fraction,
}

pub type SFirstCardDistribution = Vec<SFirstCardDistributionCell>;

pub fn get(probability_map: &HashMap<ECardPoint, Fraction>) -> SFirstCardDistribution {
    let mut table = Vec::<SFirstCardDistributionCell>::new();
    for card_point in ECardPoint::iter() {
        table.push(SFirstCardDistributionCell {
            first_card_point: card_point.clone(),
            probability: probability_map.get(&card_point).unwrap().clone(),
        })
    }
    table
}

#[cfg(test)]
mod test {
    use tabled::Table;
    use super::super::super::super::super::deck::random_deck::SRandomDeck;
    use super::super::super::super::super::deck::TDeck;
    use super::super::first_card_distribution::{get};

    #[test]
    fn test1() {
        let deck = SRandomDeck::new();
        let v = get(deck.get_point_probability_map());
        let table = Table::new(v).to_string();
        print!("{table}");
    }
}