use bevy::prelude::*;

use crate::{
    client::resources::{Focus, ResFrameworkHandler, ResGameTable},
    server::{player::EPlayerAction, table::ETableOutputEvent},
};

use super::super::super::events::{player_request_events::*, server_response_events::*};

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

pub struct PlayerRequestEventsHandlerPlugin;

impl Plugin for PlayerRequestEventsHandlerPlugin {
    fn build(&self, app: &mut App) {
        app
            // Systems
            .add_systems(
                Update,
                (
                    handle_request_player_bet,
                    handle_request_player_hit,
                    handle_request_player_stand,
                    handle_request_player_split,
                    handle_request_player_double_down,
                ),
            );
    }
}
