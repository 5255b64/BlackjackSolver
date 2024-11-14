pub mod random_deck;
use std::collections::HashMap;
use fraction::Fraction;
use super::card::{ECardNumber, ECardPoint};

pub trait TDeck {
    fn draw(&mut self) -> Option<ECardNumber>;

    fn draw_specific(&mut self, card: ECardNumber) -> Option<ECardNumber>;

    fn shuffle(&mut self);

    /// 获取每种点数的抽取概率
    /// 使用分数Fraction作为结果，提高精度。
    fn get_point_probability_map(&self) -> &HashMap<ECardPoint, Fraction>;
}