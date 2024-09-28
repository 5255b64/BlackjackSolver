use std::collections::HashMap;
use fraction::Fraction;
use strum::IntoEnumIterator;
use tabled::Tabled;
use crate::card::ECardPoint;
use crate::deck::TDeck;
use crate::solver::static_solver::equity_caculator::dealer_value_final_distribution::EDealerFinalValue;
use crate::solver::static_solver::equity_caculator::dealer_value_final_distribution;

#[derive(Tabled)]
pub struct SDealerFirstFinalDistributionCell {
    pub first_card_point: ECardPoint,
    pub p17: Fraction,
    pub p18: Fraction,
    pub p19: Fraction,
    pub p20: Fraction,
    pub p21: Fraction,
    pub bust: Fraction,
    pub blackjack: Fraction,
}

pub type SDealerFirstFinalDistribution = Vec<SDealerFirstFinalDistributionCell>;

pub fn get_map(probability_map: &HashMap<ECardPoint, Fraction>) -> HashMap<ECardPoint, HashMap::<EDealerFinalValue, Fraction>> {
    let mut map = HashMap::<ECardPoint, HashMap::<EDealerFinalValue, Fraction>>::new();
    let value_final_map = dealer_value_final_distribution::get_map(probability_map);
    for card_point in ECardPoint::iter() {
        
        let mut m = value_final_map.get(&card_point.into()).unwrap().clone();
        let black_jack = match card_point {
            ECardPoint::Ace => probability_map.get(&ECardPoint::Ten).unwrap().clone(),
            ECardPoint::Ten => probability_map.get(&ECardPoint::Ace).unwrap().clone(),
            _ => Fraction::new(0u64, 1u64),
        };
        m.insert(EDealerFinalValue::Blackjack, black_jack);
        m.insert(EDealerFinalValue::P21,
                 *m.get(&EDealerFinalValue::P21).unwrap_or(&Fraction::new(0u64, 1u64)) - black_jack
        );
        map.insert(card_point, m);
    }
    map
}

pub fn get_table(probability_map: &HashMap<ECardPoint, Fraction>) -> SDealerFirstFinalDistribution {
    let mut table = Vec::<SDealerFirstFinalDistributionCell>::new();
    let map = get_map(probability_map);
    for card_point in ECardPoint::iter() {
        let m = map.get(&card_point.into()).unwrap();
        table.push(SDealerFirstFinalDistributionCell {
            first_card_point: card_point,
            p17: m.get(&EDealerFinalValue::P17).unwrap_or(&Fraction::new(0u64, 1u64)).clone(),
            p18: m.get(&EDealerFinalValue::P18).unwrap_or(&Fraction::new(0u64, 1u64)).clone(),
            p19: m.get(&EDealerFinalValue::P19).unwrap_or(&Fraction::new(0u64, 1u64)).clone(),
            p20: m.get(&EDealerFinalValue::P20).unwrap_or(&Fraction::new(0u64, 1u64)).clone(),
            p21: m.get(&EDealerFinalValue::P21).unwrap_or(&Fraction::new(0u64, 1u64)).clone(),
            bust: m.get(&EDealerFinalValue::Bust).unwrap_or(&Fraction::new(0u64, 1u64)).clone(),
            blackjack: m.get(&EDealerFinalValue::Blackjack).unwrap_or(&Fraction::new(0u64, 1u64)).clone(),
        })
    }
    table
}

#[cfg(test)]
mod test {
    use fraction::Fraction;
    use tabled::Table;
    use crate::deck::random_deck::SRandomDeck;
    use crate::deck::TDeck;
    use crate::solver::static_solver::equity_caculator::dealer_first_final_distribution::get_table;

    #[test]
    fn test1() {
        let table = Table::new(get_table(SRandomDeck::new().get_point_probability_map())).to_string();
        println!("{}", &table);
        for row in get_table(SRandomDeck::new().get_point_probability_map()) {
            print!("Check {}:", row.first_card_point);

            // let sum = row.blackjack + row.p17 + row.p18 + row.p19 + row.p20 + row.p21 + row.bust;
            // if (Fraction::new(1u64, 1u64) == sum) {
            //     println!("pass");
            // } else {
            //     println!("fail: {}", sum);
            // }

            assert_eq!(Fraction::new(1u64, 1u64),
                       row.blackjack + row.p17 + row.p18 + row.p19 + row.p20 + row.p21 + row.bust
            );
            println!("pass");
        }
    }
}