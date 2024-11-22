use bevy::prelude::*;

use crate::{
    client::{
        dealer::{resources::DealerHand, systems::spawn_new_dealer_card},
        game::events::*,
        player::{resources::PlayerHand, systems::spawn_new_player_card},
        resources::GameTable,
        GameState,
    },
    server::{player::EPlayerAction, table::ETableOutputEvent},
};

pub fn handle_request_player_bet(
    mut event_reader: EventReader<RequestPlayerBet>,
    mut event_writer: EventWriter<ResponseInitGameWithCards>,
    mut table: ResMut<GameTable>,
) {
    for event in event_reader.read().into_iter() {
        println!("Receive Event: RequestPlayerBet {}", event.value);
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
        println!("Receive Event: RequestPlayerHit");
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
        println!("Receive Event: RequestPlayerStand");
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
    mut game_state_next_state: ResMut<NextState<GameState>>,
) {
    for _ in event_reader.read().into_iter() {
        println!("Receive Event: RequestPlayerSplit");
        let result = table.table.receive_player_action(EPlayerAction::Stand);
        match result {
            Ok(ETableOutputEvent::PlayerSplitCards { card1, card2 }) => {
                event_writer.send(ResponsePlayerSplitCards{ card1, card2 });
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
        println!("Receive Event: RequestPlayerDoubleDown");
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
    println!("New GameState:{game_state:?}");
    game_state_next_state.set(game_state.into());
    println!("");
}

pub fn handle_response_init_game_with_cards(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut dealer_hand: ResMut<DealerHand>,
    mut player_hand: ResMut<PlayerHand>,
    mut event_reader: EventReader<ResponseInitGameWithCards>,
    table: ResMut<GameTable>,
    mut game_state_next_state: ResMut<NextState<GameState>>,
) {
    for event in event_reader.read().into_iter() {
        let ResponseInitGameWithCards {
            dealer_cards,
            player_cards,
        } = event;
        println!("Receive Event: ResponseInitGameWithCards");
        println!("dealer cards:{:?}", dealer_cards);
        println!("player cards:{:?}", player_cards);
        for card in dealer_cards {
            spawn_new_dealer_card(&mut commands, &asset_server, &mut dealer_hand, card.clone());
        }
        for card in player_cards {
            spawn_new_player_card(&mut commands, &asset_server, &mut player_hand, card.clone());
        }
        update_state(&table, &mut game_state_next_state);
    }
}

pub fn handle_response_wait_player_buy_insurance(
    mut event_reader: EventReader<ResponseWaitPlayerBuyInsurance>,
    table: ResMut<GameTable>,
    mut game_state_next_state: ResMut<NextState<GameState>>,
) {
    for event in event_reader.read().into_iter() {
        println!("Receive Event: ResponseWaitPlayerBuyInsurance");
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
        println!(
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
        let ResponsePlayerSplitCards{ card1, card2 } = event;
        println!(
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
        println!(
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
        println!(
            "Receive Event: ResponseGameOver\twin chips:{:?}",
            event.win_chips
        );
        update_state(&table, &mut game_state_next_state);
    }
}

pub fn handle_response_player_draw_card(
    mut event_reader: EventReader<ResponsePlayerDrawCard>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut player_hand: ResMut<PlayerHand>,
    table: ResMut<GameTable>,
    mut game_state_next_state: ResMut<NextState<GameState>>,
) {
    for event in event_reader.read().into_iter() {
        let ResponsePlayerDrawCard {
            card,
            is_player_stop,
        } = event;
        println!(
            "Receive Event: ResponsePlayerDrawCard {:?}\tis_stop:{:?}",
            card, is_player_stop
        );
        spawn_new_player_card(&mut commands, &asset_server, &mut player_hand, card.clone());
        update_state(&table, &mut game_state_next_state);
    }
}

pub fn handle_response_dealer_draw_card(
    mut event_reader: EventReader<ResponseDealerDrawCard>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut dealer_hand: ResMut<DealerHand>,
    table: ResMut<GameTable>,
    mut game_state_next_state: ResMut<NextState<GameState>>,
) {
    for event in event_reader.read().into_iter() {
        let ResponseDealerDrawCard {
            card,
            is_dealer_stop,
        } = event;
        println!(
            "Receive Event: ResponseDealerDrawCard {:?}\tis_stop:{:?}",
            card, is_dealer_stop
        );
        spawn_new_dealer_card(&mut commands, &asset_server, &mut dealer_hand, card.clone());
        update_state(&table, &mut game_state_next_state);
    }
}
