// use bevy::prelude::*;

// use crate::{
//     client::{
//         game::components::{CompCard, CompCards, CompHand, CompHands, CompPlayer, CompValue},
//         states::Focus, styles::get_title_text_style,
//     },
//     server::card::ECard,
// };

// // use super::resources::ResPlayer;

// pub fn spawn_new_player_card(
//     commands: &mut Commands,
//     res_asset_server: &Res<AssetServer>,
//     res_player: &mut ResMut<ResPlayer>,
//     state_focus: &Res<State<Focus>>,
//     card: ECard,
//     q_player: &Query<&Children, With<CompPlayer>>,
//     q_player_hands: &Query<&Children, With<CompHands>>,
//     q_player_hand: &mut Query<(&mut CompHand, &Children), With<CompHand>>,
//     q_player_hand_cards: &Query<Entity, With<CompCards>>,
//     q_player_hand_value: &mut Query<&mut Text, With<CompValue>>,
// ) {
//     if let Focus::Player(focus_num) = state_focus.get() {
//         let hands = &mut res_player.hands;
//         let cards = hands.get_mut((*focus_num) as usize).unwrap();
//         cards.push(card.clone());

//         // 遍历player
//         for player_children in q_player.iter() {
//             // 遍历player的children
//             for &player_child in player_children.iter() {
//                 // 检索player hands的children中的hands
//                 if let Ok(hands_children) = q_player_hands.get(player_child) {
//                     // 遍历hands的children
//                     for &hands_child in hands_children.iter() {
//                         // 检索hands的children中的hand
//                         if let Ok((mut comp_hand, hand_children)) = q_player_hand.get_mut(hands_child) {
//                             // 判断 hand 判断是否为当前聚焦的hand
//                             if comp_hand.num == *focus_num {
//                                 // 遍历hand的children
//                                 for &hand_child in hand_children {
//                                     // 检索hand的children中的cards
//                                     if let Ok(entity_cards) = q_player_hand_cards.get(hand_child) {
//                                         // 向cards中插入card
//                                         let entity_card = CompCard::get_entity(
//                                             commands,
//                                             res_asset_server,
//                                             card,
//                                             true,
//                                         );
//                                         commands.entity(entity_cards).push_children(&[entity_card]);
//                                         // 插入成功后更新value
//                                         comp_hand.value = card.value + comp_hand.value;
//                                         // 检索value 修改text
//                                         if let Ok(mut text) = q_player_hand_value.get_mut(hands_child) {
//                                             text.sections = vec![TextSection::new(
//                                                 comp_hand.value.to_string(), get_title_text_style(res_asset_server)
//                                             )];
//                                         }
//                                         break;
//                                     }
//                                 }
//                             }
//                         }
//                     }
//                 }
//             }
//         }
//     } else {
//         {
//             error!("Spawn Player Card in Wrong Focus:{state_focus:?}");
//         }
//     }
// }
