use bevy::prelude::*;

use crate::client::{
    card::systems::get_card_boundle,
    resources::GameTable,
};

// pub fn update_player_hand(
//     mut commands: Commands,
//     mut player_hand_query: Query<Entity, With<PlayerHand>>,
//     asset_server: Res<AssetServer>,
//     table: Res<GameTable>,
// ) {
//     if let Ok(player_hand_entity) = player_hand_query.get_single_mut() {
//         let player_hand = &table.table.player_hand;
//         commands.entity(player_hand_entity).clear_children();
//         println!("player card:{:?}", &player_hand.get(0).unwrap().hand.cards);
//         for card in &player_hand.get(0).unwrap().hand.cards {
//             commands.entity(player_hand_entity).with_children(|parent| {
//                 parent
//                     .spawn(NodeBundle {
//                         style: Style {
//                             width: Val::Percent(100.),
//                             height: Val::Percent(100.),
//                             align_items: AlignItems::Center,
//                             justify_content: JustifyContent::Center,
//                             flex_direction: FlexDirection::Row,
//                             column_gap: Val::Px(12.),
//                             ..Default::default()
//                         },
//                         ..Default::default()
//                     })
//                     .with_children(|builder| {
//                         builder.spawn(get_card_boundle(&card.color, &card.value, &asset_server));
//                     });
//             });
//         }
//     }
// }
