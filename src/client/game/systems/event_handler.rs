use bevy::prelude::*;

use crate::{
    client::{
        game::events::*,
        resources::{Focus, ResFrameworkHandler, ResGameTable},
        states::{FocusState, GameState},
    },
    server::{player::EPlayerAction, table::ETableOutputEvent},
};

pub fn handle_request_player_bet(
    mut commands: Commands,
    mut q_text: Query<(&mut Text, Entity)>,
    assert_server: Res<AssetServer>,
    mut res_framework_handler: ResMut<ResFrameworkHandler>,
    mut event_reader: EventReader<EventRequestPlayerBet>,
    mut event_writer: EventWriter<EventResponseInitGameWithCards>,
    mut table: ResMut<ResGameTable>,
) {
    for event in event_reader.read().into_iter() {
        info!("Receive Event: RequestPlayerBet {}", event.value);
        // println!("Receive Event: RequestPlayerBet {}", event.value);
        // 清理上一局

        res_framework_handler.reset_hands(&mut commands, &assert_server, &mut q_text);

        // 开始下一局
        let result = table
            .table
            .receive_player_action(EPlayerAction::Bet(event.value));
        match result {
            Ok(ETableOutputEvent::InitGameWithCards {
                player_cards,
                dealer_cards,
            }) => {
                event_writer.send(EventResponseInitGameWithCards {
                    player_cards,
                    dealer_cards,
                });
            }
            Ok(r) => {
                error!("EPlayerAction::Bet 返回类型错误:{r:?}");
            }
            Err(e) => {
                error!("{e:?}")
            }
        }
    }
}

pub fn handle_request_player_hit(
    mut event_reader: EventReader<EventRequestPlayerHit>,
    mut event_writer: EventWriter<EventResponsePlayerDrawCard>,
    mut table: ResMut<ResGameTable>,
) {
    for _ in event_reader.read().into_iter() {
        info!("Receive Event: RequestPlayerHit");
        let result = table.table.receive_player_action(EPlayerAction::Hit);
        match result {
            Ok(ETableOutputEvent::PlayerDrawCard {
                card,
                hand_index,
                is_player_stop,
            }) => {
                event_writer.send(EventResponsePlayerDrawCard {
                    card,
                    hand_index,
                    is_player_stop,
                });
            }
            Ok(r) => {
                error!("EPlayerAction::Hit 返回类型错误:{r:?}");
            }
            Err(e) => {
                error!("{e:?}")
            }
        }
    }
}

pub fn handle_request_player_stand(
    mut event_reader: EventReader<EventRequestPlayerStand>,
    mut event_writer: EventWriter<EventResponsePlayerStand>,
    mut table: ResMut<ResGameTable>,
) {
    for _ in event_reader.read().into_iter() {
        info!("Receive Event: RequestPlayerStand");
        let result = table.table.receive_player_action(EPlayerAction::Stand);
        match result {
            Ok(ETableOutputEvent::PlayerStand { is_player_stop }) => {
                event_writer.send(EventResponsePlayerStand { is_player_stop });
            }
            Ok(r) => {
                error!("EPlayerAction::Stand 返回类型错误:{r:?}");
            }
            Err(e) => {
                error!("{e:?}")
            }
        }
    }
}

pub fn handle_request_player_split(
    mut event_reader: EventReader<EventRequestPlayerSplit>,
    mut event_writer: EventWriter<EventResponsePlayerSplitCards>,
    mut table: ResMut<ResGameTable>,
    res_framework_handler: Res<ResFrameworkHandler>,
) {
    for _ in event_reader.read().into_iter() {
        info!("Receive Event: RequestPlayerSplit");
        let result = table.table.receive_player_action(EPlayerAction::Split);
        match result {
            Ok(ETableOutputEvent::PlayerSplitCards { card1, card2 }) => {
                match res_framework_handler.focus {
                    Focus::Player(hand_index) => {
                        event_writer.send(EventResponsePlayerSplitCards {
                            card1,
                            card2,
                            hand_index: hand_index,
                        });
                    }
                    _ => {
                        error!("focus error: not focus on player hand!")
                    }
                }
            }
            Ok(r) => {
                error!("EPlayerAction::Split 返回类型错误:{r:?}");
            }
            Err(e) => {
                error!("{e:?}")
            }
        }
    }
}

pub fn handle_request_player_double_down(
    mut event_reader: EventReader<EventRequestPlayerDoubleDown>,
    mut event_writer: EventWriter<EventResponsePlayerDrawCard>,
    mut table: ResMut<ResGameTable>,
) {
    for _ in event_reader.read().into_iter() {
        info!("Receive Event: RequestPlayerDoubleDown");
        let result = table.table.receive_player_action(EPlayerAction::DoubleDown);
        match result {
            Ok(ETableOutputEvent::PlayerDrawCard {
                card,
                hand_index,
                is_player_stop,
            }) => {
                event_writer.send(EventResponsePlayerDrawCard {
                    card,
                    hand_index,
                    is_player_stop,
                });
            }
            Ok(r) => {
                error!("EPlayerAction::DoubleDown 返回类型错误:{r:?}");
            }
            Err(e) => {
                error!("{e:?}")
            }
        }
    }
}

fn update_state(
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

pub fn handle_response_init_game_with_cards(
    mut event_reader: EventReader<EventResponseInitGameWithCards>,
    table: ResMut<ResGameTable>,
    mut game_state_next_state: ResMut<NextState<GameState>>,
    mut event_writer_player_draw_card: EventWriter<EventResponsePlayerDrawCard>,
    mut event_writer_dealer_draw_card: EventWriter<EventResponseDealerDrawCard>,
    mut res_focus_next_state: ResMut<NextState<FocusState>>,
    mut res_framework_handler: ResMut<ResFrameworkHandler>,
) {
    for event in event_reader.read().into_iter() {
        let EventResponseInitGameWithCards {
            dealer_cards,
            player_cards,
        } = event;
        info!("Receive Event: ResponseInitGameWithCards");
        info!("dealer cards:{:?}", dealer_cards);
        info!("player cards:{:?}", player_cards);

        event_writer_dealer_draw_card.send(EventResponseDealerDrawCard {
            card: dealer_cards[0],
            is_dealer_stop: false,
            is_revealed: true,
        });
        event_writer_dealer_draw_card.send(EventResponseDealerDrawCard {
            card: dealer_cards[1],
            is_dealer_stop: false,
            is_revealed: false,
        });
        for card in player_cards {
            event_writer_player_draw_card.send(EventResponsePlayerDrawCard {
                card: card.clone(),
                hand_index: 0, // todo 多路開局
                is_player_stop: false,
            });
        }
        update_state(
            &table,
            &mut game_state_next_state,
            &mut res_focus_next_state,
            &mut res_framework_handler,
        );
    }
}

pub fn handle_response_wait_player_buy_insurance(
    mut event_reader: EventReader<EventResponseWaitPlayerBuyInsurance>,
    table: ResMut<ResGameTable>,
    mut game_state_next_state: ResMut<NextState<GameState>>,
    mut res_focus_next_state: ResMut<NextState<FocusState>>,
    mut res_framework_handler: ResMut<ResFrameworkHandler>,
) {
    for _event in event_reader.read().into_iter() {
        info!("Receive Event: ResponseWaitPlayerBuyInsurance");
        update_state(
            &table,
            &mut game_state_next_state,
            &mut res_focus_next_state,
            &mut res_framework_handler,
        );
        todo!();
    }
}

pub fn handle_response_insurance_result(
    mut event_reader: EventReader<EventResponseInsuranceResult>,
    table: ResMut<ResGameTable>,
    mut game_state_next_state: ResMut<NextState<GameState>>,
    mut res_focus_next_state: ResMut<NextState<FocusState>>,
    mut res_framework_handler: ResMut<ResFrameworkHandler>,
) {
    for event in event_reader.read().into_iter() {
        info!(
            "Receive Event: ResponseInsuranceResult\tis dealer blackjack:{:?}",
            event.is_dealer_blackjack
        );
        update_state(
            &table,
            &mut game_state_next_state,
            &mut res_focus_next_state,
            &mut res_framework_handler,
        );
        todo!()
    }
}

pub fn handle_response_player_split_cards(
    mut commands: Commands,
    assert_server: Res<AssetServer>,
    mut event_reader: EventReader<EventResponsePlayerSplitCards>,
    table: ResMut<ResGameTable>,
    mut res_framework_handler: ResMut<ResFrameworkHandler>,
    mut game_state_next_state: ResMut<NextState<GameState>>,
    mut res_focus_next_state: ResMut<NextState<FocusState>>,
    mut q_text: Query<(&mut Text, Entity)>,
) {
    for event in event_reader.read().into_iter() {
        let EventResponsePlayerSplitCards {
            card1,
            card2,
            hand_index,
        } = event;
        info!(
            "Receive Event: ResponsePlayerSplitCards index: {:?}\tcards:{:?} {:?}",
            hand_index, card1, card2
        );

        res_framework_handler.player_split(
            &mut commands,
            &assert_server,
            &mut q_text,
            *hand_index,
            *card1,
            *card2,
        );

        update_state(
            &table,
            &mut game_state_next_state,
            &mut res_focus_next_state,
            &mut res_framework_handler,
        );
    }
}

pub fn handle_response_player_stand(
    mut event_reader: EventReader<EventResponsePlayerStand>,
    table: ResMut<ResGameTable>,
    mut game_state_next_state: ResMut<NextState<GameState>>,
    mut res_focus_next_state: ResMut<NextState<FocusState>>,
    mut res_framework_handler: ResMut<ResFrameworkHandler>,
) {
    for event in event_reader.read().into_iter() {
        info!(
            "Receive Event: ResponsePlayerStand\tis_stop:{:?}",
            event.is_player_stop
        );
        update_state(
            &table,
            &mut game_state_next_state,
            &mut res_focus_next_state,
            &mut res_framework_handler,
        );
    }
}

pub fn handle_response_game_over(
    mut event_reader: EventReader<EventResponseGameOver>,
    assert_server: Res<AssetServer>,
    table: ResMut<ResGameTable>,
    mut game_state_next_state: ResMut<NextState<GameState>>,
    mut res_focus_next_state: ResMut<NextState<FocusState>>,
    mut res_framework_handler: ResMut<ResFrameworkHandler>,
    mut q_img: Query<(&mut UiImage, &Parent)>,
) {
    for event in event_reader.read().into_iter() {
        info!(
            "Receive Event: ResponseGameOver\twin chips:{:?}",
            event.win_chips
        );
        // blackjack的情况
        res_framework_handler.dealer_reveal_card(&assert_server, &mut q_img);
        update_state(
            &table,
            &mut game_state_next_state,
            &mut res_focus_next_state,
            &mut res_framework_handler,
        );
    }
}

pub fn handle_response_player_draw_card(
    mut event_reader: EventReader<EventResponsePlayerDrawCard>,

    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut q_text: Query<(&mut Text, Entity)>,
    mut res_framework_handler: ResMut<ResFrameworkHandler>,

    table: ResMut<ResGameTable>,
    mut game_state_next_state: ResMut<NextState<GameState>>,
    mut res_focus_next_state: ResMut<NextState<FocusState>>,
) {
    for event in event_reader.read().into_iter() {
        let EventResponsePlayerDrawCard {
            card,
            hand_index,
            is_player_stop,
        } = event;
        info!(
            "Receive Event: ResponsePlayerDrawCard {:?}\tis_stop:{:?}",
            card, is_player_stop
        );
        res_framework_handler.player_draw_new_card(
            &mut commands,
            &asset_server,
            &mut q_text,
            *hand_index as usize,
            *card,
        );
        update_state(
            &table,
            &mut game_state_next_state,
            &mut res_focus_next_state,
            &mut res_framework_handler,
        );
    }
}

pub fn handle_response_dealer_draw_card(
    mut event_reader: EventReader<EventResponseDealerDrawCard>,

    mut commands: Commands,
    assert_server: Res<AssetServer>,
    mut q_text: Query<(&mut Text, Entity)>,
    mut res_framework_handler: ResMut<ResFrameworkHandler>,

    table: ResMut<ResGameTable>,
    mut game_state_next_state: ResMut<NextState<GameState>>,
    mut res_focus_next_state: ResMut<NextState<FocusState>>,
    mut q_img: Query<(&mut UiImage, &Parent)>,
) {
    for event in event_reader.read().into_iter() {
        let EventResponseDealerDrawCard {
            card,
            is_dealer_stop,
            is_revealed,
        } = event;
        info!(
            "Receive Event: ResponseDealerDrawCard {:?}\tis_stop:{:?}",
            card, is_dealer_stop
        );
        res_framework_handler.dealer_reveal_card(&assert_server, &mut q_img);
        res_framework_handler.dealer_draw_new_card(
            &mut commands,
            &assert_server,
            &mut q_text,
            *card,
            *is_revealed,
        );
        update_state(
            &table,
            &mut game_state_next_state,
            &mut res_focus_next_state,
            &mut res_framework_handler,
        );
    }
}
