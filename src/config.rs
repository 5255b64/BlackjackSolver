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
}

impl Default for SGameRule {
    fn default() -> Self {
        let blackjack_pay = Fraction::new(3u64, 2u64);
        let insurance_pay = Fraction::new(2u64, 1u64);
        let bet_min = 1;
        let bet_max = 10;
        let bet_step = 1;
        let insurance_max = Fraction::new(1u64, 2u64);
        assert!(bet_min <= bet_step);
        assert!(bet_max >= bet_step);
        assert_eq!(bet_step % bet_min, 0);
        assert_eq!(bet_max % bet_step, 0);
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
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::config::SGameRule;

    #[tokio::test]
    async fn test() {
        let game_rule = SGameRule::default();
        println!("{:?}", game_rule);
    }
}
