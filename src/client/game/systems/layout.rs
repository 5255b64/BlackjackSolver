use super::super::{components::*, styles::*};
use crate::client::card::systems::spawn_child_node_card;
use bevy::prelude::*;

pub fn spawn_game(mut commands: Commands, asset_server: Res<AssetServer>) {
    let _game_entity = build_game(&mut commands, &asset_server);
}

pub fn despawn_game(mut commands: Commands, game_query: Query<Entity, With<Game>>) {
    if let Ok(game_entity) = game_query.get_single() {
        commands.entity(game_entity).despawn_recursive();
    }
}

pub fn build_game(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let main_menu_entity = commands
        .spawn((
            NodeBundle {
                style: MAIN_MENU_STYLE,
                ..default()
            },
            Game {},
        ))
        .with_children(|parent| {
            // === Dealer Hand ===
            spawn_dealer_hand(parent, &DealerHand::default(), asset_server);
            // === Player Hand ===
            spawn_player_hand(parent, &PlayerHand::default(), asset_server);
            // === Player Buttons ===
            spawn_buttons(parent, asset_server);
        })
        .id();
    main_menu_entity
}

fn spawn_dealer_hand(
    builder: &mut ChildBuilder,
    dealer_hand: &DealerHand,
    asset_server: &Res<AssetServer>,
) {
    builder
        .spawn((
            NodeBundle {
                style: CARD_STYLE,
                ..Default::default()
            },
            dealer_hand.clone(),
        ))
        .with_children(|builder| {
            for card in &dealer_hand.cards {
                spawn_child_node_card(builder, &card.color, &card.value, asset_server);
            }
        });
}

fn spawn_player_hand(
    builder: &mut ChildBuilder,
    player_hand: &PlayerHand,
    asset_server: &Res<AssetServer>,
) {
    builder
        .spawn((
            NodeBundle {
                style: CARD_STYLE,
                ..Default::default()
            },
            player_hand.clone(),
        ))
        .with_children(|builder| {
            for card in &player_hand.cards {
                spawn_child_node_card(builder, &card.color, &card.value, asset_server);
            }
        });
}

fn spawn_buttons(builder: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
    builder
        .spawn((
            NodeBundle {
                style: BUTTON_BAR_STYLE,
                ..default()
            },
            ButtonBar {},
        ))
        .with_children(|builder| {
            builder
                .spawn((
                    ButtonBundle {
                        style: BUTTON_STYLE,
                        background_color: DEACTIVE_BUTTON_COLOR.into(),
                        ..default()
                    },
                    StartButton {},
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Start",
                                get_button_text_style(&asset_server),
                            )],
                            justify: JustifyText::Center,
                            ..default()
                        },
                        ..default()
                    });
                });

            builder
                .spawn((
                    ButtonBundle {
                        style: BUTTON_STYLE,
                        background_color: DEACTIVE_BUTTON_COLOR.into(),
                        ..default()
                    },
                    SplitButton {},
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Split",
                                get_button_text_style(&asset_server),
                            )],
                            justify: JustifyText::Center,
                            ..default()
                        },
                        ..default()
                    });
                });

            builder
                .spawn((
                    ButtonBundle {
                        style: BUTTON_STYLE,
                        background_color: DEACTIVE_BUTTON_COLOR.into(),
                        ..default()
                    },
                    DoubleDownButton {},
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Double",
                                get_button_text_style(&asset_server),
                            )],
                            justify: JustifyText::Center,
                            ..default()
                        },
                        ..default()
                    });
                });
            builder
                .spawn((
                    ButtonBundle {
                        style: BUTTON_STYLE,
                        background_color: DEACTIVE_BUTTON_COLOR.into(),
                        ..default()
                    },
                    HitButton {},
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Hit",
                                get_button_text_style(&asset_server),
                            )],
                            justify: JustifyText::Center,
                            ..default()
                        },
                        ..default()
                    });
                });
            builder
                .spawn((
                    ButtonBundle {
                        style: BUTTON_STYLE,
                        background_color: DEACTIVE_BUTTON_COLOR.into(),
                        ..default()
                    },
                    StandButton {},
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Stand",
                                get_button_text_style(&asset_server),
                            )],
                            justify: JustifyText::Center,
                            ..default()
                        },
                        ..default()
                    });
                });
        });
}
