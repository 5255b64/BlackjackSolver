use tokio::{self, time};
use BlackjackSolver::card::ECard;
use BlackjackSolver::player::SUiPlayer::SUiPlayer;
use BlackjackSolver::table::{EPlayerActionError, ETableRunError, STable};

#[tokio::main]
async fn main() {
    println!("{:?}", ECard::Seven);

    let player = SUiPlayer::new();
    let mut table = STable::new_random_deck();
    table.but_chips(10000);
    while true {
        print_table(&table);
        let action = player.action(&table);
        println!("Player操作：{action:?}");
        let mut result = table.receive_player_action(action);
        // println!("Table Result: {result:?}\n\n");
        while true {
            match result {
                Ok(_) => { break }
                Err(_) => {
                    match table.run() {
                        Ok(_) => { break }
                        Err(_) => {}
                    }
                }
            }
        }
        time::sleep(time::Duration::from_millis(100)).await;
    };
    ()
}

fn print_table(table: &STable) {
    println!("Table State：{:?}", table.state);
    println!("Player Chips：{:?}", table.player_chips);

    print!("Dealer手牌-{:?}：\t", table.dealer_hand.value());
    for card in &table.dealer_hand.hand.cards {
        print!("{:?}\t", card);
    }
    println!();

    print!("Player手牌：\n");
    let mut counter = 1;
    for hand in &table.player_hand {
        print!("{counter}-{:?}-{:?}: \t", hand.betting_box, hand.value());
        for card in &hand.hand.cards {
            print!("{:?}\t", card);
        }
        println!();
    }
    println!();
}