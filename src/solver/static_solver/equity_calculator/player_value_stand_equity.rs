use std::collections::HashMap;
use fraction::Fraction;
use strum::IntoEnumIterator;
use tabled::Tabled;
use crate::card::ECardPoint;
use crate::deck::TDeck;
use crate::solver::static_solver::equity_caculator::dealer_value_final_distribution::EDealerFinalValue;
use crate::solver::static_solver::equity_caculator::dealer_first_final_distribution;
use crate::value::EValue;

#[derive(Tabled)]
pub struct SPlayerValueStandEquityCell {
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

pub type SPlayerValueStandEquity = Vec<SPlayerValueStandEquityCell>;

pub fn get_map(probability_map: &HashMap<ECardPoint, Fraction>) -> HashMap<EValue, HashMap::<ECardPoint, Fraction>> {
    let mut map = HashMap::<EValue, HashMap::<ECardPoint, Fraction>>::new();
    let dealer_first_final_map = dealer_first_final_distribution::get_map(probability_map);
    for player_value in EValue::iter().rev() {
        match player_value {
            EValue::None => {}
            _ => {
                let mut equity_map = HashMap::<ECardPoint, Fraction>::new();
                for dealer_card_point in ECardPoint::iter() {
                    let dealer_final_map = dealer_first_final_map.get(&dealer_card_point.into()).unwrap();
                    let mut equity = Fraction::from(0);
                    for dealer_final_value in EDealerFinalValue::iter() {
                        let prob = dealer_final_map.get(&dealer_final_value).unwrap_or(&Fraction::new(0u64, 1u64)).clone();
                        if player_value == EValue::Bust || dealer_final_value.to_point() > player_value.to_point() {
                            equity -= prob;
                        } else if dealer_final_value.to_point() < player_value.to_point() {
                            equity += prob;
                        };
                    }
                    equity_map.insert(dealer_card_point, equity);
                }
                map.insert(player_value, equity_map);
            }
        }
    }
    map
}

pub fn get_table(probability_map: &HashMap<ECardPoint, Fraction>) -> SPlayerValueStandEquity {
    let mut table = Vec::<SPlayerValueStandEquityCell>::new();
    let map = get_map(probability_map);
    for player_value in EValue::iter().rev() {
        if player_value == EValue::None {
            continue;
        }
        let equity_map = map.get(&player_value).unwrap();

        table.push(SPlayerValueStandEquityCell {
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
    use crate::deck::random_deck::SRandomDeck;
    use crate::deck::TDeck;
    use crate::solver::static_solver::equity_caculator::player_value_stand_equity::get_table;

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