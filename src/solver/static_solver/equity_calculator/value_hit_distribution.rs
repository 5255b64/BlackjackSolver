use std::collections::HashMap;
use std::fmt::{Display};
use fraction::Fraction;
use strum::IntoEnumIterator;
use tabled::{Tabled};
use crate::card::{ECard, ECardPoint};
use crate::deck::TDeck;
use crate::value::EValue;

#[derive(Tabled)]
pub struct SValueHitDistributionCell {
    pub pre_hit_value: EValue,
    pub s12: Fraction,
    pub s13: Fraction,
    pub s14: Fraction,
    pub s15: Fraction,
    pub s16: Fraction,
    pub s17: Fraction,
    pub s18: Fraction,
    pub s19: Fraction,
    pub s20: Fraction,
    pub s21: Fraction,
    pub h4: Fraction,
    pub h5: Fraction,
    pub h6: Fraction,
    pub h7: Fraction,
    pub h8: Fraction,
    pub h9: Fraction,
    pub h10: Fraction,
    pub h11: Fraction,
    pub h12: Fraction,
    pub h13: Fraction,
    pub h14: Fraction,
    pub h15: Fraction,
    pub h16: Fraction,
    pub h17: Fraction,
    pub h18: Fraction,
    pub h19: Fraction,
    pub h20: Fraction,
    pub h21: Fraction,
    pub bust: Fraction,
}

impl SValueHitDistributionCell {
    fn new() -> Self {
        Self {
            pre_hit_value: EValue::None,
            h4: Default::default(),
            h5: Default::default(),
            h6: Default::default(),
            h7: Default::default(),
            h8: Default::default(),
            h9: Default::default(),
            h10: Default::default(),
            h11: Default::default(),
            h12: Default::default(),
            h13: Default::default(),
            h14: Default::default(),
            h15: Default::default(),
            h16: Default::default(),
            h17: Default::default(),
            h18: Default::default(),
            h19: Default::default(),
            h20: Default::default(),
            h21: Default::default(),
            s12: Default::default(),
            s13: Default::default(),
            s14: Default::default(),
            s15: Default::default(),
            s16: Default::default(),
            s17: Default::default(),
            s18: Default::default(),
            s19: Default::default(),
            s20: Default::default(),
            s21: Default::default(),
            bust: Default::default(),
        }
    }
}

pub type SValueHitDistribution = Vec<SValueHitDistributionCell>;

pub fn get_map(probability_map: &HashMap<ECardPoint, Fraction>) -> HashMap<EValue, HashMap::<EValue, Fraction>> {
    let mut map = HashMap::<EValue, HashMap::<EValue, Fraction>>::new();
    for value in EValue::iter() {
        match value {
            EValue::None => {
                // pass
            }
            v1 => {
                let mut m = HashMap::<EValue, Fraction>::new();
                for card in ECard::iter() {
                    match card {
                        ECard::Jack | ECard::Queen | ECard::King => {}
                        _ => {
                            let v2 = v1 + card;
                            m.insert(
                                v2,
                                m.get(&v2).unwrap_or(&Fraction::new(0u64, 1u64)).clone() +
                                probability_map.get(&ECardPoint::from(card)).unwrap_or(&Fraction::new(0u64, 1u64)).clone()
                            );
                        }
                    }
                }
                map.insert(value, m);
            }
        }
    }
    map
}

pub fn get_table(probability_map: &HashMap<ECardPoint, Fraction>) -> SValueHitDistribution {
    let mut table = Vec::<SValueHitDistributionCell>::new();
    let map = get_map(probability_map);
    for value in EValue::iter() {
        match value {
            EValue::None => {
                // pass
            }
            _ => {
                let m = map.get(&value).unwrap();
                table.push(SValueHitDistributionCell {
                    pre_hit_value: value,
                    h4: m.get(&EValue::H4).unwrap_or(&Fraction::new(0u64, 1u64)).clone(),
                    h5: m.get(&EValue::H5).unwrap_or(&Fraction::new(0u64, 1u64)).clone(),
                    h6: m.get(&EValue::H6).unwrap_or(&Fraction::new(0u64, 1u64)).clone(),
                    h7: m.get(&EValue::H7).unwrap_or(&Fraction::new(0u64, 1u64)).clone(),
                    h8: m.get(&EValue::H8).unwrap_or(&Fraction::new(0u64, 1u64)).clone(),
                    h9: m.get(&EValue::H9).unwrap_or(&Fraction::new(0u64, 1u64)).clone(),
                    h10: m.get(&EValue::H10).unwrap_or(&Fraction::new(0u64, 1u64)).clone(),
                    h11: m.get(&EValue::H11).unwrap_or(&Fraction::new(0u64, 1u64)).clone(),
                    s12: m.get(&EValue::S12).unwrap_or(&Fraction::new(0u64, 1u64)).clone(),
                    s13: m.get(&EValue::S13).unwrap_or(&Fraction::new(0u64, 1u64)).clone(),
                    s14: m.get(&EValue::S14).unwrap_or(&Fraction::new(0u64, 1u64)).clone(),
                    s15: m.get(&EValue::S15).unwrap_or(&Fraction::new(0u64, 1u64)).clone(),
                    s16: m.get(&EValue::S16).unwrap_or(&Fraction::new(0u64, 1u64)).clone(),
                    s17: m.get(&EValue::S17).unwrap_or(&Fraction::new(0u64, 1u64)).clone(),
                    s18: m.get(&EValue::S18).unwrap_or(&Fraction::new(0u64, 1u64)).clone(),
                    s19: m.get(&EValue::S19).unwrap_or(&Fraction::new(0u64, 1u64)).clone(),
                    s20: m.get(&EValue::S20).unwrap_or(&Fraction::new(0u64, 1u64)).clone(),
                    s21: m.get(&EValue::S21).unwrap_or(&Fraction::new(0u64, 1u64)).clone(),
                    h12: m.get(&EValue::H12).unwrap_or(&Fraction::new(0u64, 1u64)).clone(),
                    h13: m.get(&EValue::H13).unwrap_or(&Fraction::new(0u64, 1u64)).clone(),
                    h14: m.get(&EValue::H14).unwrap_or(&Fraction::new(0u64, 1u64)).clone(),
                    h15: m.get(&EValue::H15).unwrap_or(&Fraction::new(0u64, 1u64)).clone(),
                    h16: m.get(&EValue::H16).unwrap_or(&Fraction::new(0u64, 1u64)).clone(),
                    h17: m.get(&EValue::H17).unwrap_or(&Fraction::new(0u64, 1u64)).clone(),
                    h18: m.get(&EValue::H18).unwrap_or(&Fraction::new(0u64, 1u64)).clone(),
                    h19: m.get(&EValue::H19).unwrap_or(&Fraction::new(0u64, 1u64)).clone(),
                    h20: m.get(&EValue::H20).unwrap_or(&Fraction::new(0u64, 1u64)).clone(),
                    h21: m.get(&EValue::H21).unwrap_or(&Fraction::new(0u64, 1u64)).clone(),
                    bust: m.get(&EValue::Bust).unwrap_or(&Fraction::new(0u64, 1u64)).clone(),
                })
            }
        }
    }
    table
}

#[cfg(test)]
mod test {
    use tabled::Table;
    use crate::deck::random_deck::SRandomDeck;
    use crate::deck::TDeck;
    use crate::solver::static_solver::equity_caculator::value_hit_distribution::get_table;

    #[test]
    fn test1() {
        let deck = SRandomDeck::new();
        let v = get_table(deck.get_point_probability_map());
        let table = Table::new(v).to_string();
        print!("{table}");
    }
}