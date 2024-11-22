use bevy::prelude::*;

use crate::{
    client::{
        game::{GameState, ResponseDealerDrawCard, ResponseGameOver},
        resources::GameTable,
    },
    server::{
        player::EPlayerAction,
        table::{ETableOutputEvent, ETableState},
    },
};

pub fn update_game_state(
    game_state: Res<State<GameState>>,
    mut table: ResMut<GameTable>,
    mut game_state_next_state: ResMut<NextState<GameState>>,
    mut dealer_draw_card_event_writer: EventWriter<ResponseDealerDrawCard>,
    mut game_over_event_writer: EventWriter<ResponseGameOver>,
) {
    let action: Option<EPlayerAction> = match game_state.get() {
        GameState::DealerCheckBlackJack
        | GameState::DealerHitOrStand
        | GameState::CheckResultAndReset => Some(EPlayerAction::WaitNext),
        GameState::PlayerBuyInsurance => Some(EPlayerAction::BuyInsurance(0)),
        _ => None,
    };

    match action {
        Some(action) => match table.table.receive_player_action(action) {
            Ok(r) => {
                println!("(Dealer Response)Table Output: {r:?}");

                match r {
                    ETableOutputEvent::DealerDrawCard {
                        card,
                        is_dealer_stop,
                    } => {
                        dealer_draw_card_event_writer.send(ResponseDealerDrawCard {
                            card,
                            is_dealer_stop,
                        });
                    }
                    ETableOutputEvent::GameOver { win_chips } => {
                        game_over_event_writer.send(ResponseGameOver { win_chips });
                    }
                    _ => {}
                }

                let game_state = table.table.get_state();
                println!("New GameState:{game_state:?}");
                game_state_next_state.set(game_state.into());
            }
            Err(e) => {
                println!("Error: Update Game State - {e:?}");
            }
        },
        None => {}
    };
}
