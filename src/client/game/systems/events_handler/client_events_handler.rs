pub struct ClientEventsHandlerPlugin;

use bevy::prelude::*;

use crate::client::{
    game::{
        client_events::{EventClientFocusChange, EventClientUpdateInfoBar},
        systems::systems::update_client_state,
    },
    resources::{Focus, ResFrameworkHandler, ResGameTable},
    states::{FocusState, GameState},
};

use super::{
    EventClientDealerDrawCard, EventClientGameOver, EventClientPlayerDrawCard,
    EventClientPlayerSplitCards, EventClientUpdateState,
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
        let EventClientDealerDrawCard { card, is_revealed } = event;
        info!("Receive Event: EventClientDealerDrawCard {:?}", card);
        res_framework_handler.dealer_reveal_card(&assert_server, &mut q_text, &mut q_img);
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
    mut event_writer: EventWriter<EventClientFocusChange>,
) {
    for _ in event_reader.read().into_iter() {
        info!("Receive Event: EventClientUpdateState");

        update_client_state(
            &table,
            &mut game_state_next_state,
            &mut res_focus_next_state,
            &mut res_framework_handler,
            &mut event_writer,
        );
    }
}

pub fn handle_client_update_info_bar(
    mut event_reader: EventReader<EventClientUpdateInfoBar>,
    mut res_framework_handler: ResMut<ResFrameworkHandler>,
    mut q_text: Query<(&mut Text, Entity)>,
) {
    for event in event_reader.read().into_iter() {
        let EventClientUpdateInfoBar { new_info } = event;
        info!("Receive Event: EventClientUpdateInfoBar");

        res_framework_handler.infobar_set_new_info(&mut q_text, new_info.clone());
    }
}

pub fn handle_client_game_over(
    mut event_reader: EventReader<EventClientGameOver>,
    mut event_writer: EventWriter<EventClientUpdateInfoBar>,
    mut res_framework_handler: ResMut<ResFrameworkHandler>,
    assert_server: Res<AssetServer>,
    mut q_text: Query<(&mut Text, Entity)>,
    mut q_img: Query<(&mut UiImage, &Parent)>,
) {
    for event in event_reader.read().into_iter() {
        let EventClientGameOver {
            bet_chips,
            win_chips,
            player_chips,
        } = event;
        info!(
            "Receive Event: EventClientGameOver\t chips:{:?}\t bet:{:?}\t win:{:?}",
            player_chips, bet_chips, win_chips
        );
        // 避免blackjack的情况下未揭示庄家底牌
        res_framework_handler.dealer_reveal_card(&assert_server, &mut q_text, &mut q_img);
        // 修改info bar 文案
        event_writer.send(EventClientUpdateInfoBar {
            new_info: format!(
                "Game Over\nchips:{:?}  bet:{:?}  win:{:?}",
                player_chips, bet_chips, win_chips
            )
            .into(),
        });
    }
}

pub fn handle_client_focus_change(
    mut event_reader: EventReader<EventClientFocusChange>,
    mut res_framework_handler: ResMut<ResFrameworkHandler>,
    mut q_text: Query<(&mut Text, Entity)>,
) {
    for event in event_reader.read().into_iter() {
        let EventClientFocusChange {
            old_focus: old_focus_hand,
            new_focus: new_focus_hand,
        } = event;
        info!(
            "Receive Event: EventClientFocusChange\t old:{:?}\t new:{:?}",
            old_focus_hand, new_focus_hand
        );
        // old取消高亮
        match old_focus_hand {
            Focus::Dealer => res_framework_handler.dealer_de_highlight(&mut q_text),
            Focus::Player(hand_num) => {
                res_framework_handler.player_de_highlight_hand(&mut q_text, *hand_num)
            }
            Focus::None => {}
        }
        // new高亮
        match new_focus_hand {
            Focus::Dealer => res_framework_handler.dealer_highlight(&mut q_text),
            Focus::Player(hand_num) => {
                res_framework_handler.player_highlight_hand(&mut q_text, *hand_num)
            }
            Focus::None => {}
        }
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
                    handle_client_update_info_bar,
                    handle_client_game_over,
                    handle_client_focus_change,
                ),
            );
    }
}
