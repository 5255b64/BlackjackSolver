

#[cfg(test)]
mod tests {
    use crate::server::{card::{ECard, ECardColor, ECardNumber}, player::EPlayerAction, table::STable};

    #[tokio::test]
    async fn test_diy_deck() {
        let cards = vec![
            ECard {
                color: ECardColor::Hearts,
                value: ECardNumber::Six,
            },
            ECard {
                color: ECardColor::Hearts,
                value: ECardNumber::Ten,
            },
            ECard {
                color: ECardColor::Hearts,
                value: ECardNumber::Six,
            },
            ECard {
                color: ECardColor::Hearts,
                value: ECardNumber::Ace,
            },
            ECard {
                color: ECardColor::Hearts,
                value: ECardNumber::Ten,
            },
            ECard {
                color: ECardColor::Hearts,
                value: ECardNumber::Ten,
            },
            ECard {
                color: ECardColor::Hearts,
                value: ECardNumber::Ten,
            },
            ECard {
                color: ECardColor::Hearts,
                value: ECardNumber::Ten,
            },
            ECard {
                color: ECardColor::Hearts,
                value: ECardNumber::Ten,
            },
        ];
        // let mut deck = SDiyDeck::from(v);
        let mut table = STable::new_diy_deck(cards);
        table.buy_chips(10000);
        println!("player action: bet 2");
        let result = table.receive_player_action(EPlayerAction::Bet(2));
        println!("player bet result:{result:?}");

        println!("player action: wait");
        let result = table.receive_player_action(EPlayerAction::WaitNext);
        println!("player wait result:{result:?}");

        // println!("player action: buy insurance");
        // let result = table.receive_player_action(EPlayerAction::BuyInsurance(0));
        // println!("player buy insurance result:{result:?}");

        println!("player action: stand");
        let result = table.receive_player_action(EPlayerAction::Stand);
        println!("player stand result:{result:?}");

        println!("player action: wait");
        let result = table.receive_player_action(EPlayerAction::WaitNext);
        println!("player wait result:{result:?}");

        println!("player action: wait");
        let result = table.receive_player_action(EPlayerAction::WaitNext);
        println!("player wait result:{result:?}");

    }

    
    #[tokio::test]
    async fn test_diy_deck_player_bust() {
        let cards = vec![
            ECard {
                color: ECardColor::Hearts,
                value: ECardNumber::Six,
            },
            ECard {
                color: ECardColor::Hearts,
                value: ECardNumber::Six,
            },
            ECard {
                color: ECardColor::Hearts,
                value: ECardNumber::Six,
            },
            ECard {
                color: ECardColor::Hearts,
                value: ECardNumber::Six,
            },
            ECard {
                color: ECardColor::Hearts,
                value: ECardNumber::Ten,
            },
            ECard {
                color: ECardColor::Hearts,
                value: ECardNumber::Ten,
            },
            ECard {
                color: ECardColor::Hearts,
                value: ECardNumber::Ten,
            },
            ECard {
                color: ECardColor::Hearts,
                value: ECardNumber::Ten,
            },
            ECard {
                color: ECardColor::Hearts,
                value: ECardNumber::Ten,
            },
        ];
        // let mut deck = SDiyDeck::from(v);
        let mut table = STable::new_diy_deck(cards);
        table.buy_chips(10000);
        println!("player action: bet 2");
        let result = table.receive_player_action(EPlayerAction::Bet(2));
        println!("player bet result:{result:?}");

        println!("player action: wait");
        let result = table.receive_player_action(EPlayerAction::WaitNext);
        println!("player wait result:{result:?}");

        // println!("player action: buy insurance");
        // let result = table.receive_player_action(EPlayerAction::BuyInsurance(0));
        // println!("player buy insurance result:{result:?}");

        println!("player action: hit");
        let result = table.receive_player_action(EPlayerAction::Hit);
        println!("player hit result:{result:?}");

        println!("player action: wait");
        let result = table.receive_player_action(EPlayerAction::WaitNext);
        println!("player wait result:{result:?}");

        println!("player action: wait");
        let result = table.receive_player_action(EPlayerAction::WaitNext);
        println!("player wait result:{result:?}");

    }
    
    #[tokio::test]
    async fn test_diy_deck_player_stand_dealer_draw() {
        let cards = vec![
            ECard {
                color: ECardColor::Hearts,
                value: ECardNumber::Ten,
            },
            ECard {
                color: ECardColor::Hearts,
                value: ECardNumber::Two,
            },
            ECard {
                color: ECardColor::Hearts,
                value: ECardNumber::Ten,
            },
            ECard {
                color: ECardColor::Hearts,
                value: ECardNumber::Nine,
            },
            ECard {
                color: ECardColor::Hearts,
                value: ECardNumber::Ten,
            },
            ECard {
                color: ECardColor::Hearts,
                value: ECardNumber::Ten,
            },
            ECard {
                color: ECardColor::Hearts,
                value: ECardNumber::Ten,
            },
            ECard {
                color: ECardColor::Hearts,
                value: ECardNumber::Ten,
            },
            ECard {
                color: ECardColor::Hearts,
                value: ECardNumber::Ten,
            },
        ];
        // let mut deck = SDiyDeck::from(v);
        let mut table = STable::new_diy_deck(cards);
        table.buy_chips(10000);
        println!("player action: bet 2");
        let result = table.receive_player_action(EPlayerAction::Bet(2));
        println!("player bet result:{result:?}");

        println!("player action: wait");
        let result = table.receive_player_action(EPlayerAction::WaitNext);
        println!("player wait result:{result:?}");

        // println!("player action: buy insurance");
        // let result = table.receive_player_action(EPlayerAction::BuyInsurance(0));
        // println!("player buy insurance result:{result:?}");

        println!("player action: stand");
        let result = table.receive_player_action(EPlayerAction::Stand);
        println!("player stand result:{result:?}");

        println!("player action: wait");
        let result = table.receive_player_action(EPlayerAction::WaitNext);
        println!("player wait result:{result:?}");

        println!("player action: wait");
        let result = table.receive_player_action(EPlayerAction::WaitNext);
        println!("player wait result:{result:?}");

    }
}