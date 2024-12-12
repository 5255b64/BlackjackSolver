// use bevy::prelude::*;

// use crate::{
//     client::{game::components::{CompCard, CompCards, CompDealer, CompHand, CompHands, CompValue}, styles::get_title_text_style},
//     server::card::ECard,
// };

// // use super::resources::ResDealer;

// pub fn spawn_new_dealer_card(
//     commands: &mut Commands,
//     res_asset_server: &Res<AssetServer>,
//     res_dealer: &mut ResMut<ResDealer>,
//     card: ECard,
//     q_dealer: &Query<&Children, With<CompDealer>>,
//     q_dealer_hands: &Query<&Children, With<CompHands>>,
//     q_dealer_hand: &mut Query<(&mut CompHand, &Children), With<CompHand>>,
//     q_dealer_hand_cards: &Query<Entity, With<CompCards>>,
//     q_dealer_hand_value: &mut Query<&mut Text, With<CompValue>>,

//     is_revealed: bool,
// ) {
//     let cards = &mut res_dealer.cards;
//     cards.push(card.clone());

//     // 遍历dealer
//     for dealer_children in q_dealer.iter() {
//         // 遍历dealer的children
//         for &dealer_child in dealer_children.iter() {
//             // 检索dealer的children中的hands
//             if let Ok(hands_children) = q_dealer_hands.get(dealer_child) {
//                 // 遍历hands的children
//                 for &hands_child in hands_children {
//                     // 检索hands的children中的hand
//                     if let Ok((mut comp_hand, hand_children)) = q_dealer_hand.get_mut(hands_child) {
//                         // let entity_card =
//                         //     CompCard::get_entity(commands, res_asset_server, card, is_revealed);
//                         // commands.entity(entity).push_children(&[entity_card]);
//                         // 遍历hand的children
//                         for &hand_child in hand_children {
//                             // 检索hand的children中的cards
//                             if let Ok(entity_cards) = q_dealer_hand_cards.get(hand_child) {
//                                 // 向cards中插入card
//                                 let entity_card = CompCard::get_entity(
//                                     commands,
//                                     res_asset_server,
//                                     card,
//                                     is_revealed,
//                                 );
//                                 commands.entity(entity_cards).push_children(&[entity_card]);
//                                 // 插入成功后更新value
//                                 comp_hand.value = card.value + comp_hand.value;
//                                 // 检索value 修改text
//                                 if let Ok(mut text) = q_dealer_hand_value.get_mut(hands_child) {
//                                     text.sections = vec![TextSection::new(
//                                         comp_hand.value.to_string(), get_title_text_style(res_asset_server)
//                                     )];
//                                 }
//                                 break;
//                             }
//                         }
//                     }
//                 }
//             }
//         }
//     }
// }
