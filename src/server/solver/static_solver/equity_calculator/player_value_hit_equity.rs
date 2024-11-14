use std::collections::HashMap;

use fraction::Fraction;
use strum::IntoEnumIterator;
use tabled::Tabled;

use super::super::super::super::card::ECardPoint;
use super::super::super::super::deck::TDeck;
use super::{player_value_stand_equity, value_hit_distribution};
use super::super::super::super::value::EValue;

#[derive(Tabled)]
pub struct SPlayerValueHitEquityCell {
    pub value: EValue,
    pub two: Fraction,
    pub three: Fraction,
    pub four: Fraction,
    pub five: Fraction,
    pub six: Fraction,
    pub seven: Fraction,
    pub eight: Fraction,
    pub nine: Fraction,
    pub ten: Fraction,
    pub ace: Fraction,
}

pub type SPlayerValueHitEquity = Vec<SPlayerValueHitEquityCell>;

pub fn get_map(probability_map: &HashMap<ECardPoint, Fraction>) -> HashMap<EValue, HashMap::<ECardPoint, Fraction>> {
    let mut map = HashMap::<EValue, HashMap::<ECardPoint, Fraction>>::new();
    let player_value_stand_equity_map = player_value_stand_equity::get_map(probability_map);
    let value_hit_map = value_hit_distribution::get_map(probability_map);
    for player_value_before_hit in EValue::iter() {
        if player_value_before_hit == EValue::None {
            continue;
        }
        let after_hit_map = value_hit_map.get(&player_value_before_hit).unwrap().clone();
        let mut tmp_map = HashMap::<ECardPoint, Fraction>::new();
        // 将after_hit_map与player_value_stand_equity_map相乘
        for player_value_after_hit in EValue::iter() {
            if player_value_after_hit == EValue::None {
                continue;
            }
            let hit_prob = after_hit_map.get(&player_value_after_hit).unwrap_or(&Fraction::new(0u64, 1u64)).clone();
            let equity_map = player_value_stand_equity_map.get(&player_value_after_hit).unwrap();
            for dealer_card_point in ECardPoint::iter() {
                tmp_map.insert(
                    dealer_card_point,
                    tmp_map.get(&dealer_card_point).unwrap_or(&Fraction::new(0u64, 1u64)).clone()
                        + hit_prob * equity_map.get(&dealer_card_point).unwrap_or(&Fraction::new(0u64, 1u64)).clone(),
                );
            }
        }
        map.insert(player_value_before_hit, tmp_map);
    }
    map
}

pub fn get_table(probability_map: &HashMap<ECardPoint, Fraction>) -> SPlayerValueHitEquity {
    let mut table = Vec::<SPlayerValueHitEquityCell>::new();
    let map = get_map(probability_map);
    for player_value in EValue::iter().rev() {
        if player_value == EValue::None {
            continue;
        }
        let equity_map = map.get(&player_value).unwrap();

        table.push(SPlayerValueHitEquityCell {
            value: player_value,
            two: equity_map.get(&ECardPoint::Two).unwrap_or(&Fraction::new(0u64, 1u64)).clone(),
            three: equity_map.get(&ECardPoint::Three).unwrap_or(&Fraction::new(0u64, 1u64)).clone(),
            four: equity_map.get(&ECardPoint::Four).unwrap_or(&Fraction::new(0u64, 1u64)).clone(),
            five: equity_map.get(&ECardPoint::Five).unwrap_or(&Fraction::new(0u64, 1u64)).clone(),
            six: equity_map.get(&ECardPoint::Six).unwrap_or(&Fraction::new(0u64, 1u64)).clone(),
            seven: equity_map.get(&ECardPoint::Seven).unwrap_or(&Fraction::new(0u64, 1u64)).clone(),
            eight: equity_map.get(&ECardPoint::Eight).unwrap_or(&Fraction::new(0u64, 1u64)).clone(),
            nine: equity_map.get(&ECardPoint::Nine).unwrap_or(&Fraction::new(0u64, 1u64)).clone(),
            ten: equity_map.get(&ECardPoint::Ten).unwrap_or(&Fraction::new(0u64, 1u64)).clone(),
            ace: equity_map.get(&ECardPoint::Ace).unwrap_or(&Fraction::new(0u64, 1u64)).clone(),
        })
    }
    table
}

#[cfg(test)]
mod test {
    use tabled::Table;

    use super::super::super::super::super::deck::random_deck::SRandomDeck;
    use super::super::super::super::super::deck::TDeck;
    use super::super::player_value_hit_equity::get_table;

    #[test]
    fn test1() {
        let table = Table::new(get_table(SRandomDeck::new().get_point_probability_map())).to_string();
        println!("{}", &table);
        // for row in get_table(Box::new(SRandomDeck::new())) {
        //     print!("Check {}:", row.value);
        // 
        //     assert_eq!(Fraction::new(1u64, 1u64),
        //                row.blackjack + row.p17 + row.p18 + row.p19 + row.p20 + row.p21 + row.bust
        //     );
        //     println!("pass");
        // }
    }
}