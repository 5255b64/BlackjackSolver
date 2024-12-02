use bevy::prelude::*;

use crate::{
    client::card::systems::get_card_ui_bundle_entity,
    server::card::{ECard, ECardColor, ECardNumber},
};

use super::{
    card::components::{Card, CardBundle},
    spawn_camara,
};

pub fn run() {
    App::new()
        // Bevy Plugins
        .add_plugins(DefaultPlugins)
        // Resources
        // My State
        // My Plugins
        // Startup Systems
        // Systems
        .add_systems(Startup, spawn_camara)
        // .add_systems(Startup, test_system)
        .add_systems(Startup, (spawn_framework, spawn_dealer_cards).chain())
        // .add_systems(Startup, spawn_cards)
        // .add_systems(Update, print_components)
        .run();
}

#[derive(Component)]
pub struct Framework;

#[derive(Component)]
pub struct DealerHand;

#[derive(Component)]
pub struct PlayerHand;

#[derive(Component)]
pub struct CCard;

pub fn spawn_framework(mut commands: Commands, asset_server: Res<AssetServer>) {
    const MARGIN: Val = Val::Px(12.);
    let component_frame_work = commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    flex_direction: FlexDirection::Column,
                    row_gap: MARGIN,
                    ..Default::default()
                },
                ..Default::default()
            },
            Framework,
        ))
        .id();
    info!("component_frame_work entity id:{component_frame_work:?}");

    let component_dealer_hand = commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    flex_direction: FlexDirection::Row,
                    column_gap: MARGIN,
                    ..Default::default()
                },
                ..Default::default()
            },
            DealerHand,
        ))
        .id();
    info!("component_dealer_hand entity id:{component_dealer_hand:?}");

    let component_player_hand = commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    flex_direction: FlexDirection::Row,
                    column_gap: MARGIN,
                    ..Default::default()
                },
                ..Default::default()
            },
            PlayerHand,
        ))
        .id();
    info!("component_player_hand entity id:{component_player_hand:?}");

    commands
        .entity(component_frame_work)
        .push_children(&[component_dealer_hand, component_player_hand]);
}

pub fn spawn_dealer_cards(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    q_dealer_hand: Query<Entity, With<DealerHand>>,
) {
    info!("spawn_dealer_cards");
    if let Ok(e) = q_dealer_hand.get_single() {
        info!("spawn_dealer_cards: cards");
        let card1 = get_card_ui_bundle_entity(
            Card {
                color: ECardColor::Hearts,
                num: ECardNumber::Ace,
                is_revealed: true,
            },
            &mut commands,
            &asset_server,
        );
        info!("card1 entity id:{card1:?}");
        let card2 = get_card_ui_bundle_entity(
            Card {
                color: ECardColor::Diamonds,
                num: ECardNumber::Ace,
                is_revealed: true,
            },
            &mut commands,
            &asset_server,
        );
        info!("card2 entity id:{card2:?}");
        let card3 = get_card_ui_bundle_entity(
            Card {
                color: ECardColor::Clubs,
                num: ECardNumber::Ace,
                is_revealed: true,
            },
            &mut commands,
            &asset_server,
        );
        info!("card3 entity id:{card3:?}");
        let card4 = get_card_ui_bundle_entity(
            Card {
                color: ECardColor::Spades,
                num: ECardNumber::Ace,
                is_revealed: true,
            },
            &mut commands,
            &asset_server,
        );
        info!("card4 entity id:{card4:?}");
        commands
            .entity(e)
            .push_children(&[card1, card2, card3, card4]);
    }
}

pub fn print_components(q_entity: Query<Entity, With<CCard>>) {
    for (id, entity) in q_entity.iter().enumerate() {
        info!("Entity No.{id}: {}", entity.to_string());
    }
}

// fn test_system(mut commands: Commands, asset_server: Res<AssetServer>) {
//     commands.spawn(CardBundle::from(
//         Card {
//             color: ECardColor::Hearts,
//             num: ECardNumber::Ace,
//             is_revealed: true,
//         },
//         &asset_server,
//     ));
//     commands.spawn(CardBundle::from(
//         Card {
//             color: ECardColor::Spades,
//             num: ECardNumber::King,
//             is_revealed: false,
//         },
//         &asset_server,
//     ));
// }
