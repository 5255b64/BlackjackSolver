pub mod random_deck;
use std::collections::HashMap;
use fraction::Fraction;
use crate::card::{ECard, ECardPoint};

pub trait TDeck {
    fn draw(&mut self) -> Option<ECard>;

    fn draw_specific(&mut self, card:ECard) -> Option<ECard>;

    fn shuffle(&mut self);

    /// 获取每种点数的抽取概率
    /// 使用分数Fraction作为结果，提高精度。
    fn get_point_probability_map(&self) -> &HashMap<ECardPoint, Fraction>;
}