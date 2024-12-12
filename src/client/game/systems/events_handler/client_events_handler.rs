pub struct ClientEventsHandlerPlugin;

use bevy::prelude::*;

use crate::client::{
    game::systems::systems::update_client_state,
    resources::{ResFrameworkHandler, ResGameTable},
    states::{FocusState, GameState},
};

use super::{
    EventClientDealerDrawCard, EventClientPlayerDrawCard, EventClientPlayerSplitCards,
    EventClientUpdateState,
};

pub fn handle_client_player_split_cards(
    mut event_reader: EventReader<EventClientPlayerSplitCards>,
    mut event_writer: EventWriter<EventClientUpdateState>,
    mut commands: Commands,
    assert_server: Res<AssetServer>,
    mut res_framework_handler: ResMut<ResFrameworkHandler>,
    mut q_text: Query<(&mut Text, Entity)>,
) {
    for event in event_reader.read().into_iter() {
        let EventClientPlayerSplitCards {
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

        event_writer.send(EventClientUpdateState {});
    }
}

pub fn handle_client_player_draw_card(
    mut event_writer: EventWriter<EventClientUpdateState>,
    mut event_reader: EventReader<EventClientPlayerDrawCard>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut q_text: Query<(&mut Text, Entity)>,
    mut res_framework_handler: ResMut<ResFrameworkHandler>,
) {
    for event in event_reader.read().into_iter() {
        let EventClientPlayerDrawCard {
            card,
            hand_index,
            is_player_stop,
        } = event;
        info!(
            "Receive Event: EventClientPlayerDrawCard {:?}\tis_stop:{:?}",
            card, is_player_stop
        );
        res_framework_handler.player_draw_new_card(
            &mut commands,
            &asset_server,
            &mut q_text,
            *hand_index as usize,
            *card,
        );
        event_writer.send(EventClientUpdateState {});
    }
}

pub fn handle_client_dealer_draw_card(
    mut event_writer: EventWriter<EventClientUpdateState>,
    mut event_reader: EventReader<EventClientDealerDrawCard>,
    mut commands: Commands,
    assert_server: Res<AssetServer>,
    mut q_text: Query<(&mut Text, Entity)>,
    mut res_framework_handler: ResMut<ResFrameworkHandler>,
    mut q_img: Query<(&mut UiImage, &Parent)>,
) {
    for event in event_reader.read().into_iter() {
        let EventClientDealerDrawCard {
            card,
            is_dealer_stop,
            is_revealed,
        } = event;
        info!(
            "Receive Event: EventClientDealerDrawCard {:?}\tis_stop:{:?}",
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
        event_writer.send(EventClientUpdateState {});
    }
}

pub fn handle_client_update_state(
    mut event_reader: EventReader<EventClientUpdateState>,
    table: ResMut<ResGameTable>,
    mut game_state_next_state: ResMut<NextState<GameState>>,
    mut res_focus_next_state: ResMut<NextState<FocusState>>,
    mut res_framework_handler: ResMut<ResFrameworkHandler>,
) {
    for _ in event_reader.read().into_iter() {
        info!("Receive Event: EventClientUpdateState");

        update_client_state(
            &table,
            &mut game_state_next_state,
            &mut res_focus_next_state,
            &mut res_framework_handler,
        );
    }
}

impl Plugin for ClientEventsHandlerPlugin {
    fn build(&self, app: &mut App) {
        app
            // Systems
            .add_systems(
                Update,
                (
                    handle_client_player_split_cards,
                    handle_client_player_draw_card,
                    handle_client_dealer_draw_card,
                    handle_client_update_state,
                ),
            );
    }
}
