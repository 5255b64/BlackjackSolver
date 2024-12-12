use bevy::prelude::*;

use crate::{
    client::{
        // dealer::resources::ResDealer,
        game::{
            states::GameState,
            EventResponseDealerDrawCard, EventResponseGameOver,
        },
        // player::resources::ResPlayer,
        resources::ResGameTable,
    },
    server::{player::EPlayerAction, table::ETableOutputEvent},
};

/// 根据当前gamestate 在特定state下 向gametable发送请求
pub fn update_game_state(
    game_state: Res<State<GameState>>,
    mut table: ResMut<ResGameTable>,
    mut game_state_next_state: ResMut<NextState<GameState>>,
    mut dealer_draw_card_event_writer: EventWriter<EventResponseDealerDrawCard>,
    mut game_over_event_writer: EventWriter<EventResponseGameOver>,
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
                info!("(Dealer Response)Table Output: {r:?}");

                match r {
                    ETableOutputEvent::DealerDrawCard {
                        card,
                        is_dealer_stop,
                    } => {
                        dealer_draw_card_event_writer.send(EventResponseDealerDrawCard {
                            card,
                            is_dealer_stop,
                            is_revealed: true,
                        });
                    }
                    ETableOutputEvent::GameOver { win_chips } => {
                        game_over_event_writer.send(EventResponseGameOver { win_chips });
                    }
                    _ => {}
                }

                let game_state = table.table.get_state();
                info!("New GameState:{game_state:?}");
                game_state_next_state.set(game_state.into());
            }
            Err(e) => {
                info!("Error: Update Game State - {e:?}");
            }
        },
        None => {}
    };
}
