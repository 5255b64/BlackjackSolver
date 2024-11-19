use std::{borrow::BorrowMut, process::Output};

use bevy::{ecs::world::error, prelude::*};

use crate::{
    client::{dealer::{resources::DealerHand, systems::spawn_new_dealer_card}, game::events::*, resources::GameTable},
    server::{card::ECard, player::EPlayerAction, table::ETableOutputEvent},
};

pub fn handle_request_player_bet(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut dealer_hand: ResMut<DealerHand>,
    mut event_reader: EventReader<RequestPlayerBet>,
    mut table: ResMut<GameTable>,
) {
    for event in event_reader.read().into_iter() {
        println!("Receive Event: RequestPlayerBet {}", event.value);
        let result = table
            .table
            .receive_player_action(EPlayerAction::Bet(event.value));
        match result {
            Ok(ETableOutputEvent::InitGameWithCards { player_cards, dealer_cards }) => {
                for card in dealer_cards {
                    spawn_new_dealer_card(&mut commands, &asset_server, &mut dealer_hand, card);
                }
            },
            Ok(r) => {
                error!("EPlayerAction::Bet 返回类型错误:{r:?}");
            }
            Err(e) => {error!("{e:?}")},
        }
    }
}

pub fn handle_request_player_hit(mut event_reader: EventReader<RequestPlayerHit>) {
    for event in event_reader.read().into_iter() {
        println!("Receive Event: RequestPlayerHit");
        todo!()
    }
}

pub fn handle_request_player_stand(mut event_reader: EventReader<RequestPlayerStand>) {
    for event in event_reader.read().into_iter() {
        println!("Receive Event: RequestPlayerStand");
        todo!()
    }
}

pub fn handle_request_player_split(mut event_reader: EventReader<RequestPlayerSplit>) {
    for event in event_reader.read().into_iter() {
        println!("Receive Event: RequestPlayerSplit");
        todo!()
    }
}

pub fn handle_request_player_double_down(mut event_reader: EventReader<RequestPlayerDoubleDown>) {
    for event in event_reader.read().into_iter() {
        println!("Receive Event: RequestPlayerDoubleDown");
        todo!()
    }
}

pub fn handle_response_init_game_with_cards(
    mut event_reader: EventReader<ResponseInitGameWithCards>,
) {
    for event in event_reader.read().into_iter() {
        println!("Receive Event: ResponseInitGameWithCards");
        println!("dealer cards:{:?}", event.dealer_cards);
        println!("player cards:{:?}", event.player_cards);
        todo!()
    }
}

pub fn handle_response_wait_player_buy_insurance(
    mut event_reader: EventReader<ResponseWaitPlayerBuyInsurance>,
) {
    for event in event_reader.read().into_iter() {
        println!("Receive Event: ResponseWaitPlayerBuyInsurance");
        todo!()
    }
}

pub fn handle_response_insurance_result(mut event_reader: EventReader<ResponseInsuranceResult>) {
    for event in event_reader.read().into_iter() {
        println!(
            "Receive Event: ResponseInsuranceResult\tis dealer blackjack:{:?}",
            event.is_dealer_blackjack
        );
        todo!()
    }
}

pub fn handle_response_player_split_cards(mut event_reader: EventReader<ResponsePlayerSplitCards>) {
    for event in event_reader.read().into_iter() {
        println!(
            "Receive Event: ResponsePlayerSplitCards {:?} {:?}",
            event.card1, event.card2
        );
        todo!()
    }
}

pub fn handle_response_player_stand(mut event_reader: EventReader<ResponsePlayerStand>) {
    for event in event_reader.read().into_iter() {
        println!(
            "Receive Event: ResponsePlayerStand\tis_stop:{:?}",
            event.is_player_stop
        );
        todo!()
    }
}

pub fn handle_response_game_over(mut event_reader: EventReader<ResponseGameOver>) {
    for event in event_reader.read().into_iter() {
        println!(
            "Receive Event: ResponseGameOver\twin chips:{:?}",
            event.win_chips
        );
        todo!()
    }
}

pub fn handle_response_player_draw_card(mut event_reader: EventReader<ResponsePlayerDrawCard>) {
    for event in event_reader.read().into_iter() {
        println!(
            "Receive Event: ResponsePlayerDrawCard {:?}\tis_stop:{:?}",
            event.card, event.is_player_stop
        );
        todo!()
    }
}

pub fn handle_response_dealer_draw_card(mut event_reader: EventReader<ResponseDealerDrawCard>) {
    for event in event_reader.read().into_iter() {
        println!(
            "Receive Event: ResponseDealerDrawCard {:?}\tis_stop:{:?}",
            event.card, event.is_dealer_stop
        );
        todo!()
    }
}
