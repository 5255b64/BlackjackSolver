use bevy::prelude::*;

use crate::client::{
    resources::{ResFrameworkHandler, ResGameTable},
    states::{FocusState, GameState},
};

use super::{
    super::super::events::server_response_events::*, EventClientGameOver,
    EventClientPlayerDrawCard, EventClientPlayerSplitCards, EventClientUpdateState,
};
use super::EventClientDealerDrawCard;

pub fn handle_response_init_game_with_cards(
    mut event_reader: EventReader<EventResponseInitGameWithCards>,
    // table: ResMut<ResGameTable>,
    // mut game_state_next_state: ResMut<NextState<GameState>>,
    mut event_writer_player_draw_card: EventWriter<EventResponsePlayerDrawCard>,
    mut event_writer_dealer_draw_card: EventWriter<EventResponseDealerDrawCard>,
    // mut res_focus_next_state: ResMut<NextState<FocusState>>,
    // mut res_framework_handler: ResMut<ResFrameworkHandler>,
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
        // update_client_state(
        //     &table,
        //     &mut game_state_next_state,
        //     &mut res_focus_next_state,
        //     &mut res_framework_handler,
        // );
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
        // update_client_state(
        //     &table,
        //     &mut game_state_next_state,
        //     &mut res_focus_next_state,
        //     &mut res_framework_handler,
        // );
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
        // update_client_state(
        //     &table,
        //     &mut game_state_next_state,
        //     &mut res_focus_next_state,
        //     &mut res_framework_handler,
        // );
        todo!()
    }
}

pub fn handle_response_player_split_cards(
    mut event_reader: EventReader<EventResponsePlayerSplitCards>,
    mut event_writer: EventWriter<EventClientPlayerSplitCards>,
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
        event_writer.send(EventClientPlayerSplitCards {
            hand_index: *hand_index,
            card1: *card1,
            card2: *card2,
        });
    }
}

pub fn handle_response_player_stand(
    mut event_reader: EventReader<EventResponsePlayerStand>,
    mut event_writer: EventWriter<EventClientUpdateState>,
) {
    for event in event_reader.read().into_iter() {
        info!(
            "Receive Event: ResponsePlayerStand\tis_stop:{:?}",
            event.is_player_stop
        );
        event_writer.send(EventClientUpdateState {});
    }
}

pub fn handle_response_game_over(
    mut event_reader: EventReader<EventResponseGameOver>,
    mut event_writer: EventWriter<EventClientGameOver>,
) {
    for event in event_reader.read().into_iter() {
        let EventResponseGameOver {
            bet_chips,
            win_chips,
            player_chips,
        } = event;
        info!(
            "Receive Event: ResponseGameOver\tbet:{:?}\twin:{:?}",
            bet_chips, win_chips
        );
        event_writer.send(EventClientGameOver {
            bet_chips: *bet_chips,
            win_chips: *win_chips,
            player_chips: *player_chips,
        });
    }
}

pub fn handle_response_player_draw_card(
    mut event_reader: EventReader<EventResponsePlayerDrawCard>,
    mut event_writer: EventWriter<EventClientPlayerDrawCard>,
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
        event_writer.send(EventClientPlayerDrawCard {
            card: *card,
            hand_index: *hand_index,
            is_player_stop: *is_player_stop,
        });
    }
}

pub fn handle_response_dealer_draw_card(
    mut event_reader: EventReader<EventResponseDealerDrawCard>,
    mut event_writer_client_dealer_draw_card: EventWriter<EventClientDealerDrawCard>,
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
        event_writer_client_dealer_draw_card.send(EventClientDealerDrawCard {
            card: *card,
            is_dealer_stop: *is_dealer_stop,
            is_revealed: *is_revealed,
        });
    }
}

pub struct ServerResponseEventsHandlerPlugin;

impl Plugin for ServerResponseEventsHandlerPlugin {
    fn build(&self, app: &mut App) {
        app
            // Systems
            .add_systems(
                Update,
                (
                    handle_response_init_game_with_cards,
                    handle_response_wait_player_buy_insurance,
                    handle_response_insurance_result,
                    handle_response_player_split_cards,
                    handle_response_player_stand,
                    handle_response_game_over,
                    handle_response_player_draw_card,
                    handle_response_dealer_draw_card,
                ),
            );
    }
}
