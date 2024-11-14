use std::collections::{HashMap};
use std::fmt::{Display};
use fraction::Fraction;
use rand::distributions::{Distribution};
use rand::Rng;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};
use tabled::Tabled;
use super::super::super::super::card::ECardPoint;
use super::super::super::super::deck::TDeck;
use super::value_hit_distribution;
use super::super::super::super::value::EValue;

#[derive(Tabled)]
pub struct SDealerValueFinalDistributionCell {
    pub value: EValue,
    pub p17: Fraction,
    pub p18: Fraction,
    pub p19: Fraction,
    pub p20: Fraction,
    pub p21: Fraction,
    pub bust: Fraction,
}
pub type SDealerValueFinalDistribution = Vec<SDealerValueFinalDistributionCell>;

#[derive(PartialEq, EnumIter, Hash, Eq, Display, Debug, Clone)]
pub enum EDealerFinalValue {
    None,
    P17,
    P18,
    P19,
    P20,
    P21,
    Blackjack,
    Bust,
}

impl EDealerFinalValue {
    pub fn to_point(&self) -> u8 {
        match self {
            EDealerFinalValue::None => 0,
            EDealerFinalValue::P17 => 17,
            EDealerFinalValue::P18 => 18,
            EDealerFinalValue::P19 => 19,
            EDealerFinalValue::P20 => 20,
            EDealerFinalValue::P21 => 21,
            EDealerFinalValue::Blackjack => 22,
            EDealerFinalValue::Bust => 1,
        }
    }
}

impl From<EValue> for EDealerFinalValue {
    fn from(value: EValue) -> Self {
        match value {
            EValue::S17 | EValue::H17 => EDealerFinalValue::P17,
            EValue::S18 | EValue::H18 => EDealerFinalValue::P18,
            EValue::S19 | EValue::H19 => EDealerFinalValue::P19,
            EValue::S20 | EValue::H20 => EDealerFinalValue::P20,
            EValue::S21 | EValue::H21 => EDealerFinalValue::P21,
            EValue::Bust => EDealerFinalValue::Bust,
            _ => EDealerFinalValue::None,
        }
    }
}

#[inline]
fn is_final_value(value: EValue) -> bool {
    EDealerFinalValue::from(value) != EDealerFinalValue::None
}

pub fn get_map(probability_map: &HashMap<ECardPoint, Fraction>) -> HashMap<EValue, HashMap::<EDealerFinalValue, Fraction>> {
    let mut map = HashMap::<EValue, HashMap::<EDealerFinalValue, Fraction>>::new();
    let hit_map = value_hit_distribution::get_map(probability_map);
    // println!("hit_map:{:?}", &hit_map);
    for value in EValue::iter().rev() {
        match value {
            EValue::None => {}
            _ => {
                let mut m = HashMap::<EDealerFinalValue, Fraction>::new();
                if is_final_value(value.clone()) {
                    m.insert(value.into(), Fraction::from(1u64));
                } else {
                    let transfer_map = hit_map.get(&value).unwrap();
                    // println!("value:{:?}", &value);
                    // println!("transfer_map:{:?}", &transfer_map);
                    for transfer_to_value in EValue::iter() {
                        match transfer_to_value {
                            EValue::None | EValue::S11 | EValue::H2 => {}
                            _ => {
                                // println!("transfer_to_value:{:?}", &transfer_to_value);
                                for target_value in EDealerFinalValue::iter() {
                                    // println!("target_value:{:?}", &target_value);
                                    if target_value != EDealerFinalValue::None {
                                        m.insert(
                                            target_value.clone(),
                                            map.get(&transfer_to_value).unwrap_or(&HashMap::new()).get(&target_value).unwrap_or(&Fraction::new(0u64, 1u64)).clone()
                                                * transfer_map.get(&transfer_to_value).unwrap_or(&Fraction::new(0u64, 1u64)).clone()
                                                + m.get(&target_value).unwrap_or(&Fraction::new(0u64, 1u64)).clone(),
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
                // println!("m:{:?}", &m);
                map.insert(value, m);
            }
        }
    }
    map
}

pub fn get_table(probability_map: &HashMap<ECardPoint, Fraction>) -> SDealerValueFinalDistribution {
    let mut table = Vec::<SDealerValueFinalDistributionCell>::new();
    let map = get_map(probability_map);
    for value in EValue::iter() {
        match value {
            EValue::None => {}
            _ => {
                let m = map.get(&value).unwrap();
                table.push(SDealerValueFinalDistributionCell {
                    value,
                    p17: m.get(&EDealerFinalValue::P17).unwrap_or(&Fraction::new(0u64, 1u64)).clone(),
                    p18: m.get(&EDealerFinalValue::P18).unwrap_or(&Fraction::new(0u64, 1u64)).clone(),
                    p19: m.get(&EDealerFinalValue::P19).unwrap_or(&Fraction::new(0u64, 1u64)).clone(),
                    p20: m.get(&EDealerFinalValue::P20).unwrap_or(&Fraction::new(0u64, 1u64)).clone(),
                    p21: m.get(&EDealerFinalValue::P21).unwrap_or(&Fraction::new(0u64, 1u64)).clone(),
                    bust: m.get(&EDealerFinalValue::Bust).unwrap_or(&Fraction::new(0u64, 1u64)).clone(),
                })
            }
        }
    }
    table
}

#[cfg(test)]
mod test {
    use fraction::Fraction;
    use tabled::{Table};
    use super::super::super::super::super::deck::random_deck::SRandomDeck;
    use super::super::super::super::super::deck::TDeck;
    use super::super::dealer_value_final_distribution::get_table;
    #[test]
    fn test_table() {
        let table = Table::new(get_table(SRandomDeck::new().get_point_probability_map())).to_string();
        println!("{table}");
        for row in get_table(SRandomDeck::new().get_point_probability_map()) {
            print!("Check {}:", row.value);
            // let sum = row.blackjack + row.p17 + row.p18 + row.p19 + row.p20 + row.p21 + row.bust;
            // if (Fraction::new(1u64, 1u64) == sum) {
            //     println!("pass");
            // } else {
            //     println!("fail: {}", sum);
            // }
            assert_eq!(Fraction::new(1u64, 1u64),
                       row.p17 + row.p18 + row.p19 + row.p20 + row.p21 + row.bust
            );
            println!("pass");
        }
    }
}