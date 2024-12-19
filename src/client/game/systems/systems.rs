use bevy::prelude::*;

use crate::{
    client::{
        // dealer::resources::ResDealer,
        game::{
            client_events::EventClientFocusChange,
            components::CompCard,
            server_response_events::{EventResponseDealerDrawCard, EventResponseGameOver},
            states::GameState,
        },
        // player::resources::ResPlayer,
        resources::{Focus, ResFrameworkHandler, ResGameTable},
        states::FocusState,
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
    mut res_focus_next_state: ResMut<NextState<FocusState>>,
    mut res_framework_handler: ResMut<ResFrameworkHandler>,
    mut event_writer: EventWriter<EventClientFocusChange>,
) {
    let action: Option<EPlayerAction> = match game_state.get() {
        GameState::DealerCheckBlackJack
        | GameState::DealerHitOrStand
        | GameState::CheckResultAndReset => Some(EPlayerAction::WaitNext),
        GameState::PlayerBuyInsurance => Some(EPlayerAction::BuyInsurance(0)),
        _ => None,
    };

    match action {
        Some(action) => match table.table.receive_player_action(action.clone()) {
            Ok(r) => {
                info!("Player Request: {action:?}");
                info!("Server Response: {r:?}");

                match r {
                    ETableOutputEvent::DealerHit { card } => {
                        dealer_draw_card_event_writer.send(EventResponseDealerDrawCard {
                            card,
                            is_revealed: true,
                        });
                    }
                    ETableOutputEvent::GameOver {
                        bet_chips,
                        win_chips,
                        player_chips,
                    } => {
                        game_over_event_writer.send(EventResponseGameOver {
                            bet_chips,
                            win_chips,
                            player_chips,
                        });
                    }
                    _ => {}
                }

                let game_state = table.table.get_state();
                info!("New GameState:{game_state:?}");
                game_state_next_state.set(game_state.into());

                update_client_state(
                    &table,
                    &mut game_state_next_state,
                    &mut res_focus_next_state,
                    &mut res_framework_handler,
                    &mut event_writer,
                );
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
    event_writer: &mut EventWriter<EventClientFocusChange>,
) {
    // 获取server状态
    let server_state: GameState = table.table.get_state().into();
    info!("New GameState:{server_state:?}");

    // 更新client的game状态
    game_state_next_state.set(server_state);

    // 更新focus状态（包括state和resource）
    let (new_focus_state, new_focus_res) = match server_state {
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
    info!("focus old:{:?}", res_framework_handler.focus);
    info!("focus new:{:?}", new_focus_res);
    if res_framework_handler.focus != new_focus_res {
        // focus changed
        event_writer.send(EventClientFocusChange {
            old_focus: res_framework_handler.focus.clone(),
            new_focus: new_focus_res.clone(),
        });

        info!("set focus_res:{new_focus_res:?}");
        res_framework_handler.focus = new_focus_res;
    }

    // 更新deck的remain card num
    let old_remain_card = res_framework_handler.remain_cards;
    let new_remain_card = match table.table.remain_cards_num() {
        crate::server::deck::ECardNum::Some(num) => Some(num),
        crate::server::deck::ECardNum::Infinite => None,
    };
    if old_remain_card != new_remain_card {
        info!("remain card old:{:?}", old_remain_card);
        info!("remain card new:{:?}", new_remain_card);
        res_framework_handler.remain_cards = new_remain_card;
    }
}
