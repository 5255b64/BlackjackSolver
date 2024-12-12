use bevy::prelude::*;

use crate::{
    client::{
        // dealer::resources::ResDealer,
        game::{
            server_response_events::{EventResponseDealerDrawCard, EventResponseGameOver}, states::GameState
        },
        // player::resources::ResPlayer,
        resources::{Focus, ResFrameworkHandler, ResGameTable}, states::FocusState,
    },
    server::{player::EPlayerAction, table::ETableOutputEvent},
};

/// 向server发送请求，并更新server状态。
pub fn update_server_state(
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

/// 更新client中的state
pub fn update_client_state(
    table: &ResMut<ResGameTable>,
    game_state_next_state: &mut ResMut<NextState<GameState>>,
    res_focus_next_state: &mut ResMut<NextState<FocusState>>,
    res_framework_handler: &mut ResMut<ResFrameworkHandler>,
) {
    let game_state: GameState = table.table.get_state().into();
    info!("New GameState:{game_state:?}");
    game_state_next_state.set(game_state);
    let (new_focus_state, new_focus_res) = match game_state {
        GameState::PlayerBet | GameState::CheckResultAndReset => (FocusState::None, Focus::None),
        GameState::DealerHitOrStand
        | GameState::DealerCheckBlackJack
        | GameState::PlayerBuyInsurance => (FocusState::Dealer, Focus::Dealer),
        GameState::PlayerSplitOrDoubleDownOrHitOrStand(hand_index)
        | GameState::PlayerDoubleDownOrHitOrStand(hand_index)
        | GameState::PlayerHitOrStand(hand_index) => {
            (FocusState::Player(hand_index), Focus::Player(hand_index))
        }
    };
    info!("set focus_state:{new_focus_state:?}");
    res_focus_next_state.set(new_focus_state);
    info!("set focus_res:{new_focus_res:?}");
    res_framework_handler.focus = new_focus_res;
}
