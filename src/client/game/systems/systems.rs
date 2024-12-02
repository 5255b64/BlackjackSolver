use bevy::prelude::*;

use crate::{
    client::{
        dealer::resources::ResDealer,
        game::{
            components::{CompCards, CompDealer, CompHand, CompHands, CompPlayer, CompValue},
            states::GameState,
            ResponseDealerDrawCard, ResponseGameOver,
        },
        player::resources::ResPlayer,
        resources::GameTable,
    },
    server::{player::EPlayerAction, table::ETableOutputEvent},
};

/// 根据当前gamestate 在特定state下 向gametable发送请求
pub fn update_game_state(
    game_state: Res<State<GameState>>,
    mut table: ResMut<GameTable>,
    mut game_state_next_state: ResMut<NextState<GameState>>,
    mut dealer_draw_card_event_writer: EventWriter<ResponseDealerDrawCard>,
    mut game_over_event_writer: EventWriter<ResponseGameOver>,
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
                debug!("(Dealer Response)Table Output: {r:?}");

                match r {
                    ETableOutputEvent::DealerDrawCard {
                        card,
                        is_dealer_stop,
                    } => {
                        dealer_draw_card_event_writer.send(ResponseDealerDrawCard {
                            card,
                            is_dealer_stop,
                            is_revealed: true,
                        });
                    }
                    ETableOutputEvent::GameOver { win_chips } => {
                        game_over_event_writer.send(ResponseGameOver { win_chips });
                    }
                    _ => {}
                }

                let game_state = table.table.get_state();
                debug!("New GameState:{game_state:?}");
                game_state_next_state.set(game_state.into());
            }
            Err(e) => {
                debug!("Error: Update Game State - {e:?}");
            }
        },
        None => {}
    };
}

pub fn reset_game(
    mut commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    dealer_hand: &mut ResMut<ResDealer>,
    player_hand: &mut ResMut<ResPlayer>,
    q_hands: &Query<Entity, With<CompHands>>,
    q_dealer: &Query<Entity, With<CompDealer>>,
    q_player: &Query<Entity, With<CompPlayer>>,
) {
    dealer_hand.reset();
    player_hand.reset();

    // 清除原有的children
    for hands in q_hands.iter() {
        commands.entity(hands).despawn_recursive();
    }

    // 重新构建dealer
    if let Ok(entity_dealer) = q_dealer.get_single() {
        debug!("重新构建dealer");
        let entity_cards = CompCards::get_entity(&mut commands);
        let entity_value = CompValue::get_entity(&mut commands, Default::default(), asset_server);
        let entity_hand = CompHand::get_entity(&mut commands, Default::default(), Default::default());
        let entity_hands = CompHands::get_entity(&mut commands);
        commands.entity(entity_hand).push_children(&[entity_cards, entity_value]);
        commands.entity(entity_hands).push_children(&[entity_hand]);
        commands
            .entity(entity_dealer)
            .push_children(&[entity_hands]);
    }

    // 重新构建player
    if let Ok(entity_player) = q_player.get_single() {
        debug!("重新构建player");
        // todo 多路牌时要发多个hand
        player_hand.as_mut().hands.push(Vec::new());
        let entity_cards = CompCards::get_entity(&mut commands);
        let entity_value = CompValue::get_entity(&mut commands, Default::default(), asset_server);
        let entity_hand = CompHand::get_entity(&mut commands, Default::default(), Default::default());
        let entity_hands = CompHands::get_entity(&mut commands);
        commands.entity(entity_hand).push_children(&[entity_cards, entity_value]);
        commands.entity(entity_hands).push_children(&[entity_hand]);
        commands
            .entity(entity_player)
            .push_children(&[entity_hands]);
    }
}
