use fraction::Fraction;

#[derive(Debug)]
pub struct SGameRule {
    pub blackjack_pay: Fraction,
    pub insurance_pay: Fraction,
    pub bet_min: usize,
    pub bet_max: usize,
    pub bet_step: usize,
    pub insurance_max: Fraction,
    pub bet_options: Vec<usize>,
    pub shuffle_threshold: Fraction,
}

impl Default for SGameRule {
    fn default() -> Self {
        let blackjack_pay = Fraction::new(3u64, 2u64);
        let insurance_pay = Fraction::new(2u64, 1u64);
        let bet_min = 1;
        let bet_max = 10;
        let bet_step = 1;
        let insurance_max = Fraction::new(1u64, 2u64);
        let shuffle_threshold = Fraction::new(1u64, 5u64);
        // 参数校验
        assert!(bet_min <= bet_step);
        assert!(bet_max >= bet_step);
        assert_eq!(bet_step % bet_min, 0);
        assert_eq!(bet_max % bet_step, 0);
        assert!(shuffle_threshold.ge(&Fraction::from(0)));
        assert!(shuffle_threshold.le(&Fraction::from(1)));
        let mut bet_options = Vec::new();
        for x in (bet_min..=bet_max).step_by(bet_step) {
            bet_options.push(x);
        }
        Self {
            blackjack_pay,
            insurance_pay,
            bet_min,
            bet_max,
            bet_step,
            insurance_max,
            bet_options,
            shuffle_threshold,
        }
    }
}

impl SGameRule {
    /// 判断player的bet的amount是否合法
    pub fn check_bet(&self, bet: usize) -> bool {
        self.bet_options.contains(&bet)
    }

    /// 判断player buy insurance的amount是否合法
    pub fn check_insurance(&self, bet: usize, insurance: usize) -> bool {
        let max_insurance = self.insurance_max * bet;
        if insurance > 0 && max_insurance >= Fraction::from(insurance) {
            true
        } else {
            false
        }
    }

    /// 判断剩余牌数是否触发阈值
    /// cards_num 牌库的初始牌数
    /// cards_remain 剩余未发出的牌数
    /// return: bool true表示触发阈值 false表示未触发
    pub fn check_threshold(&self, cards_num: usize, cards_remain: usize) -> bool {
        Fraction::new(cards_remain as u64, cards_num as u64) <= self.shuffle_threshold
            // || cards_remain <= 0
    }
}

#[cfg(test)]
mod tests {
    use super::SGameRule;

    #[tokio::test]
    async fn test() {
        let game_rule = SGameRule::default();
        println!("{:?}", game_rule);
    }
}
