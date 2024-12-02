use bevy::prelude::*;

use crate::server::{card::ECard, value::EValue};

use super::{
    card::components::{Card, CardBundle},
    styles::*,
};

/// Framework包含Dealer、Infobar、Player、ButtonBar
/// UI布局代码在systems::layout中
#[derive(Component)]
pub struct CompFramework;

/// Dealer 包含 Hands
/// 并且Hands中只有一个Hand
#[derive(Component)]
pub struct CompDealer;

/// 用于展示特定信息
#[derive(Component)]
pub struct CompInfoBar;

/// Dealer 包含 Hands
/// Hands中可能有多个Hand
#[derive(Component)]
pub struct CompPlayer;

/// Hands中可能有多个Hand
#[derive(Component)]
pub struct CompHands;

/// Hand包含Cards和Value
/// num表示该Hand在Hands中的序号
#[derive(Component, Default)]
pub struct CompHand {
    pub value: EValue,
    pub num: u8,
}

/// Cards包含一个或多个Card
#[derive(Component)]
pub struct CompCards;

#[derive(Component)]
pub struct CompCard;

#[derive(Component)]
pub struct CompValue;

impl CompHands {
    pub fn get_entity(commands: &mut Commands) -> Entity {
        commands
            .spawn((
                NodeBundle {
                    style: HANDS_STYLE,
                    background_color: HANDS_BACKGROUND_COLOR,
                    ..default()
                },
                CompHands,
            ))
            .id()
    }
}

impl CompHand {
    pub fn get_entity(commands: &mut Commands, num: u8, value: EValue) -> Entity {
        commands
            .spawn((
                NodeBundle {
                    style: HAND_STYLE,
                    background_color: HAND_BACKGROUND_COLOR,
                    ..default()
                },
                CompHand { num, value },
            ))
            .id()
    }
}

impl CompCards {
    pub fn get_entity(commands: &mut Commands) -> Entity {
        commands
            .spawn((
                NodeBundle {
                    style: CARDS_STYLE,
                    background_color: CARDS_BACKGROUND_COLOR,
                    ..default()
                },
                CompCards,
            ))
            .id()
    }
}

impl CompCard {
    pub fn get_entity(
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        card: ECard,
        is_revealed: bool,
    ) -> Entity {
        commands
            .spawn(NodeBundle {
                style: CARD_STYLE,
                ..Default::default()
            })
            .with_children(|builder| {
                builder.spawn(CardBundle::from(
                    Card {
                        color: card.color,
                        num: card.value,
                        is_revealed,
                    },
                    asset_server,
                ));
            })
            .id()
    }
}

impl CompValue {
    pub fn get_entity(
        commands: &mut Commands,
        value: EValue,
        asset_server: &Res<AssetServer>,
    ) -> Entity {
        commands
            .spawn((
                TextBundle {
                    style: VALUE_STYLE,
                    background_color: VALUE_BACKGROUND_COLOR,
                    text: Text {
                        sections: vec![TextSection::new(
                            value.to_string(),
                            get_title_text_style(&asset_server),
                        )],
                        justify: JustifyText::Center,
                        ..default()
                    },
                    ..default()
                },
                CompValue,
            ))
            .id()
    }
}

impl CompInfoBar {
    pub fn get_entity(
        commands: &mut Commands,
        info: String,
        asset_server: &Res<AssetServer>,
    ) -> Entity {
        commands
            .spawn((
                TextBundle {
                    style: INFO_BAR_STYLE,
                    background_color: INFO_BAR_BACKGROUND_COLOR,
                    text: Text {
                        sections: vec![TextSection::new(info, get_title_text_style(&asset_server))],
                        justify: JustifyText::Center,
                        ..default()
                    },
                    ..default()
                },
                CompInfoBar,
            ))
            .id()
    }
}
