use bevy::{prelude::*, utils::info};

use crate::{
    client::{
        dealer::{resources::ResDealer, systems::spawn_new_dealer_card},
        game::{
            components::{CompCards, CompDealer, CompHand, CompHands, CompPlayer, CompValue},
            events::*,
            systems::systems::reset_game,
        },
        player::{resources::ResPlayer, systems::spawn_new_player_card},
        resources::GameTable,
        states::{Focus, GameState},
    },
    server::{player::EPlayerAction, table::ETableOutputEvent},
};

pub fn handle_request_player_bet(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut dealer_hand: ResMut<ResDealer>,
    mut player_hand: ResMut<ResPlayer>,
    q_hands: Query<Entity, With<CompHands>>,
    q_dealer: Query<Entity, With<CompDealer>>,
    q_player: Query<Entity, With<CompPlayer>>,

    mut event_reader: EventReader<RequestPlayerBet>,
    mut event_writer: EventWriter<ResponseInitGameWithCards>,
    mut table: ResMut<GameTable>,
) {
    for event in event_reader.read().into_iter() {
        debug!("Receive Event: RequestPlayerBet {}", event.value);
        // 清理上一局
        reset_game(
            &mut commands,
            &asset_server,
            &mut dealer_hand,
            &mut player_hand,
            &q_hands,
            &q_dealer,
            &q_player,
        );

        // 开始下一局
        let result = table
            .table
            .receive_player_action(EPlayerAction::Bet(event.value));
        match result {
            Ok(ETableOutputEvent::InitGameWithCards {
                player_cards,
                dealer_cards,
            }) => {
                event_writer.send(ResponseInitGameWithCards {
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
    mut event_reader: EventReader<RequestPlayerHit>,
    mut event_writer: EventWriter<ResponsePlayerDrawCard>,
    mut table: ResMut<GameTable>,
) {
    for _ in event_reader.read().into_iter() {
        debug!("Receive Event: RequestPlayerHit");
        let result = table.table.receive_player_action(EPlayerAction::Hit);
        match result {
            Ok(ETableOutputEvent::PlayerDrawCard {
                card,
                is_player_stop,
            }) => {
                event_writer.send(ResponsePlayerDrawCard {
                    card,
                    is_player_stop,
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

pub fn handle_request_player_stand(
    mut event_reader: EventReader<RequestPlayerStand>,
    mut event_writer: EventWriter<ResponsePlayerStand>,
    mut table: ResMut<GameTable>,
) {
    for _ in event_reader.read().into_iter() {
        debug!("Receive Event: RequestPlayerStand");
        let result = table.table.receive_player_action(EPlayerAction::Stand);
        match result {
            Ok(ETableOutputEvent::PlayerStand { is_player_stop }) => {
                event_writer.send(ResponsePlayerStand { is_player_stop });
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

pub fn handle_request_player_split(
    mut event_reader: EventReader<RequestPlayerSplit>,
    mut event_writer: EventWriter<ResponsePlayerSplitCards>,
    mut table: ResMut<GameTable>,
) {
    for _ in event_reader.read().into_iter() {
        debug!("Receive Event: RequestPlayerSplit");
        let result = table.table.receive_player_action(EPlayerAction::Stand);
        match result {
            Ok(ETableOutputEvent::PlayerSplitCards { card1, card2 }) => {
                event_writer.send(ResponsePlayerSplitCards { card1, card2 });
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

pub fn handle_request_player_double_down(
    mut event_reader: EventReader<RequestPlayerDoubleDown>,
    mut event_writer: EventWriter<ResponsePlayerDrawCard>,
    mut table: ResMut<GameTable>,
) {
    for _ in event_reader.read().into_iter() {
        debug!("Receive Event: RequestPlayerDoubleDown");
        let result = table.table.receive_player_action(EPlayerAction::DoubleDown);
        match result {
            Ok(ETableOutputEvent::PlayerDrawCard {
                card,
                is_player_stop,
            }) => {
                event_writer.send(ResponsePlayerDrawCard {
                    card,
                    is_player_stop,
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

fn update_state(
    table: &ResMut<GameTable>,
    game_state_next_state: &mut ResMut<NextState<GameState>>,
) {
    let game_state = table.table.get_state();
    debug!("New GameState:{game_state:?}");
    game_state_next_state.set(game_state.into());
    debug!("");
}

pub fn handle_response_init_game_with_cards(
    mut event_reader: EventReader<ResponseInitGameWithCards>,
    table: ResMut<GameTable>,
    mut game_state_next_state: ResMut<NextState<GameState>>,
    mut event_writer_player_draw_card: EventWriter<ResponsePlayerDrawCard>,
    mut event_writer_dealer_draw_card: EventWriter<ResponseDealerDrawCard>,
    mut res_focus: ResMut<NextState<Focus>>,
) {
    for event in event_reader.read().into_iter() {
        let ResponseInitGameWithCards {
            dealer_cards,
            player_cards,
        } = event;
        debug!("Receive Event: ResponseInitGameWithCards");
        debug!("dealer cards:{:?}", dealer_cards);
        debug!("player cards:{:?}", player_cards);

        info("set focus: Focus::Player(0)");
        res_focus.set(Focus::Player(0));

        event_writer_dealer_draw_card.send(ResponseDealerDrawCard {
            card: dealer_cards[0],
            is_dealer_stop: false,
            is_revealed: true,
        });
        event_writer_dealer_draw_card.send(ResponseDealerDrawCard {
            card: dealer_cards[1],
            is_dealer_stop: false,
            is_revealed: false,
        });
        // for card in dealer_cards {
        //     event_writer_dealer_draw_card.send(ResponseDealerDrawCard {
        //         card: card.clone(),
        //         is_dealer_stop: false,
        //     });
        //     // spawn_new_dealer_card(&mut commands, &asset_server, &mut res_dealer, card.clone());
        // }
        for card in player_cards {
            event_writer_player_draw_card.send(ResponsePlayerDrawCard {
                card: card.clone(),
                is_player_stop: false,
            });
            // spawn_new_player_card(&mut commands, &asset_server, &mut res_player, card.clone());
        }
        update_state(&table, &mut game_state_next_state);
    }
}

pub fn handle_response_wait_player_buy_insurance(
    mut event_reader: EventReader<ResponseWaitPlayerBuyInsurance>,
    table: ResMut<GameTable>,
    mut game_state_next_state: ResMut<NextState<GameState>>,
) {
    for _event in event_reader.read().into_iter() {
        debug!("Receive Event: ResponseWaitPlayerBuyInsurance");
        update_state(&table, &mut game_state_next_state);
        todo!();
    }
}

pub fn handle_response_insurance_result(
    mut event_reader: EventReader<ResponseInsuranceResult>,
    table: ResMut<GameTable>,
    mut game_state_next_state: ResMut<NextState<GameState>>,
) {
    for event in event_reader.read().into_iter() {
        debug!(
            "Receive Event: ResponseInsuranceResult\tis dealer blackjack:{:?}",
            event.is_dealer_blackjack
        );
        update_state(&table, &mut game_state_next_state);
        todo!()
    }
}

pub fn handle_response_player_split_cards(
    mut event_reader: EventReader<ResponsePlayerSplitCards>,
    table: ResMut<GameTable>,
    mut game_state_next_state: ResMut<NextState<GameState>>,
) {
    for event in event_reader.read().into_iter() {
        let ResponsePlayerSplitCards { card1, card2 } = event;
        debug!(
            "Receive Event: ResponsePlayerSplitCards {:?} {:?}",
            card1, card2
        );
        // todo!() split相关
        update_state(&table, &mut game_state_next_state);
    }
}

pub fn handle_response_player_stand(
    mut event_reader: EventReader<ResponsePlayerStand>,
    table: ResMut<GameTable>,
    mut game_state_next_state: ResMut<NextState<GameState>>,
) {
    for event in event_reader.read().into_iter() {
        debug!(
            "Receive Event: ResponsePlayerStand\tis_stop:{:?}",
            event.is_player_stop
        );
        update_state(&table, &mut game_state_next_state);
    }
}

pub fn handle_response_game_over(
    mut event_reader: EventReader<ResponseGameOver>,
    table: ResMut<GameTable>,
    mut game_state_next_state: ResMut<NextState<GameState>>,
) {
    for event in event_reader.read().into_iter() {
        debug!(
            "Receive Event: ResponseGameOver\twin chips:{:?}",
            event.win_chips
        );
        update_state(&table, &mut game_state_next_state);
    }
}

pub fn handle_response_player_draw_card(
    mut event_reader: EventReader<ResponsePlayerDrawCard>,

    mut commands: Commands,
    res_asset_server: Res<AssetServer>,
    mut res_player: ResMut<ResPlayer>,
    res_focus: Res<State<Focus>>,
    q_player: Query<&Children, With<CompPlayer>>,
    q_player_hands: Query<&Children, With<CompHands>>,
    mut q_player_hand: Query<(&mut CompHand, &Children), With<CompHand>>,
    q_player_hand_cards: Query<Entity, With<CompCards>>,
    mut q_player_hand_value: Query<&mut Text, With<CompValue>>,

    table: ResMut<GameTable>,
    mut game_state_next_state: ResMut<NextState<GameState>>,
) {
    for event in event_reader.read().into_iter() {
        let ResponsePlayerDrawCard {
            card,
            is_player_stop,
        } = event;
        debug!(
            "Receive Event: ResponsePlayerDrawCard {:?}\tis_stop:{:?}",
            card, is_player_stop
        );
        spawn_new_player_card(
            &mut commands,
            &res_asset_server,
            &mut res_player,
            &res_focus,
            card.clone(),
            &q_player,
            &q_player_hands,
            &mut q_player_hand,
            &q_player_hand_cards,
            &mut q_player_hand_value,
        );
        update_state(&table, &mut game_state_next_state);
    }
}

pub fn handle_response_dealer_draw_card(
    mut event_reader: EventReader<ResponseDealerDrawCard>,

    mut commands: Commands,
    res_asset_server: Res<AssetServer>,
    mut res_dealer: ResMut<ResDealer>,
    q_dealer: Query<&Children, With<CompDealer>>,
    q_dealer_hands: Query<&Children, With<CompHands>>,
    mut q_dealer_hand: Query<(&mut CompHand, &Children), With<CompHand>>,
    q_dealer_hand_cards: Query<Entity, With<CompCards>>,
    mut q_dealer_hand_value: Query<&mut Text, With<CompValue>>,

    table: ResMut<GameTable>,
    mut game_state_next_state: ResMut<NextState<GameState>>,
) {
    for event in event_reader.read().into_iter() {
        let ResponseDealerDrawCard {
            card,
            is_dealer_stop,
            is_revealed,
        } = event;
        debug!(
            "Receive Event: ResponseDealerDrawCard {:?}\tis_stop:{:?}",
            card, is_dealer_stop
        );
        spawn_new_dealer_card(
            &mut commands,
            &res_asset_server,
            &mut res_dealer,
            card.clone(),
            &q_dealer,
            &q_dealer_hands,
            &mut q_dealer_hand,
            &q_dealer_hand_cards,
            &mut q_dealer_hand_value,
            *is_revealed,
        );
        update_state(&table, &mut game_state_next_state);
    }
}
