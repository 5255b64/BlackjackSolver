pub mod diy_deck;
pub mod queue_deck;
pub mod random_deck;

use super::card::{ECard, ECardNumber, ECardPoint};
use fraction::Fraction;
use std::collections::HashMap;
use strum::IntoEnumIterator;

/// 卡池
pub trait TDeck {
    fn draw(&mut self) -> Option<ECard>;

    fn draw_specific(&mut self, card: ECardNumber) -> Option<ECardNumber>;

    fn shuffle(&mut self);

    /// 获取每种点数的抽取概率
    /// 使用分数Fraction作为结果，提高精度。
    fn get_point_probability_map(&self) -> &HashMap<ECardPoint, Fraction>;

    fn remain_cards_num(&self) -> ECardNum;
    fn cards_num(&self) -> ECardNum;
}

#[derive(PartialEq, Eq, Debug)]
pub enum ECardNum {
    Some(usize),
    Infinite
}

/// 根据给定卡牌数组 计算概率map
fn new_probability_map_from_cards(
    cards: &[ECard],
) -> (HashMap<ECardNumber, usize>, HashMap<ECardPoint, Fraction>) {
    let cards_len = cards.len();
    let mut number_map = HashMap::<ECardNumber, usize>::new();
    for card in cards {
        let num = card.value;
        if number_map.contains_key(&num) {
            number_map.insert(num, number_map.get(&num).unwrap() + 1);
        } else {
            number_map.insert(num, 1);
        }
    }
    let point_probability_map = new_probability_map_from_number_map(&number_map, cards_len);
    (number_map, point_probability_map)
}

fn new_probability_map_from_number_map(
    number_map: &HashMap<ECardNumber, usize>,
    cards_len: usize,
) -> HashMap<ECardPoint, Fraction> {
    let mut point_probability_map: HashMap<ECardPoint, Fraction> = HashMap::new();

    for e in ECardNumber::iter() {
        let num = match number_map.get(&e) {
            Some(r) => *r,
            None => 0,
        };
        let point: ECardPoint = e.into();
        if point_probability_map.contains_key(&point) {
            point_probability_map.insert(
                point,
                point_probability_map.get(&point).unwrap()
                    + Fraction::new(num as u64, cards_len as u64),
            );
        } else {
            point_probability_map.insert(e.into(), Fraction::new(num as u64, cards_len as u64));
        }
    }

    point_probability_map
}
