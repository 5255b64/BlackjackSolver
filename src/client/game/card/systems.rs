use crate::server::card::{ECardColor, ECardNumber};

use super::components::*;
use bevy::prelude::*;

pub fn spawn_cards(mut commands: Commands, asset_server: Res<AssetServer>) {
    const MARGIN: Val = Val::Px(12.);
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                flex_direction: FlexDirection::Column,
                row_gap: MARGIN,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|builder| {
            for card_type in vec!["Hearts", "Diamonds", "Clubs", "Spades"] {
                builder
                    .spawn(NodeBundle {
                        style: Style {
                            width: Val::Percent(100.),
                            height: Val::Percent(100.),
                            flex_direction: FlexDirection::Row,
                            column_gap: MARGIN,
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .with_children(|builder| {
                        for card_num in vec![
                            "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K", "A",
                        ] {
                            builder
                                .spawn(NodeBundle {
                                    style: Style {
                                        width: Val::Percent(100.),
                                        height: Val::Percent(100.),
                                        align_items: AlignItems::Center,
                                        justify_content: JustifyContent::Center,
                                        flex_direction: FlexDirection::Row,
                                        column_gap: MARGIN,
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                })
                                .with_children(|builder| {
                                    builder.spawn((
                                        SpriteBundle {
                                            texture: asset_server.load(format!(
                                                "{}{}{}{}",
                                                "sprites/cards/card", card_type, card_num, ".png"
                                            )),
                                            ..default()
                                        },
                                        Card {},
                                    ));
                                });
                        }
                    });
            }
        });
}

pub fn despawn_cards(mut commands: Commands, card_query: Query<Entity, With<Card>>) {
    for card_entity in card_query.iter() {
        commands.entity(card_entity).despawn();
    }
}

pub fn get_card_boundle(
    color: &ECardColor,
    number: &ECardNumber,
    asset_server: &Res<AssetServer>,
) -> (SpriteBundle, Card) {
    let card_type = color.to_string();
    let card_num = String::from(match number {
        ECardNumber::Ace => "A",
        ECardNumber::Two => "2",
        ECardNumber::Three => "3",
        ECardNumber::Four => "4",
        ECardNumber::Five => "5",
        ECardNumber::Six => "6",
        ECardNumber::Seven => "7",
        ECardNumber::Eight => "8",
        ECardNumber::Nine => "9",
        ECardNumber::Ten => "10",
        ECardNumber::Jack => "J",
        ECardNumber::Queen => "Q",
        ECardNumber::King => "K",
    });
    (
        SpriteBundle {
            texture: asset_server.load(format!(
                "{}{}{}{}",
                "sprites/cards/card", card_type, card_num, ".png"
            )),
            ..default()
        },
        Card {},
    )
}

pub fn get_card_sprite_boundle(
    color: &ECardColor,
    number: &ECardNumber,
    asset_server: &Res<AssetServer>,
) -> SpriteBundle {
    let card_type = color.to_string();
    let card_num = String::from(match number {
        ECardNumber::Ace => "A",
        ECardNumber::Two => "2",
        ECardNumber::Three => "3",
        ECardNumber::Four => "4",
        ECardNumber::Five => "5",
        ECardNumber::Six => "6",
        ECardNumber::Seven => "7",
        ECardNumber::Eight => "8",
        ECardNumber::Nine => "9",
        ECardNumber::Ten => "10",
        ECardNumber::Jack => "J",
        ECardNumber::Queen => "Q",
        ECardNumber::King => "K",
    });
    SpriteBundle {
        texture: asset_server.load(format!(
            "{}{}{}{}",
            "sprites/cards/card", card_type, card_num, ".png"
        )),
        ..default()
    }
}

pub fn spawn_child_node_card(
    builder: &mut ChildBuilder,
    color: &ECardColor,
    number: &ECardNumber,
    asset_server: &Res<AssetServer>,
) {
    builder
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(12.),
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|builder| {
            builder.spawn(get_card_boundle(color, number, asset_server));
        });
}
