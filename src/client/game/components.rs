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
pub struct CompHand;

/// Cards包含一个或多个Card
#[derive(Component)]
pub struct CompCards;

#[derive(Component)]
pub struct CompCard;

#[derive(Component)]
pub struct CompValue;

/// Button的容器
#[derive(Component)]
pub struct CompButtonBar {}

/// 选定筹码量 开始游戏
#[derive(Component)]
pub struct CompBetButton {}

#[derive(Component)]
pub struct CompHitButton {}

#[derive(Component)]
pub struct CompStandButton {}

#[derive(Component)]
pub struct CompSplitButton {}

#[derive(Component)]
pub struct CompDoubleDownButton {}

// --- Impl s ----------------------------
impl CompFramework {
    pub fn get_entity(commands: &mut Commands) -> Entity {
        commands
            .spawn((
                NodeBundle {
                    style: FRAMEWORK_STYLE,
                    background_color: FRAMEWORK_BACKGROUND_COLOR,
                    ..default()
                },
                CompFramework {},
            ))
            .id()
    }
}

impl CompDealer {
    pub fn get_entity(commands: &mut Commands) -> Entity {
        commands
            .spawn((
                NodeBundle {
                    style: DEALER_STYLE,
                    background_color: DEALER_BACKGROUND_COLOR,
                    ..default()
                },
                CompDealer {},
            ))
            .id()
    }
}

impl CompPlayer {
    pub fn get_entity(commands: &mut Commands) -> Entity {
        commands
            .spawn((
                NodeBundle {
                    style: PLAYER_STYLE,
                    background_color: PLAYER_BACKGROUND_COLOR,
                    ..default()
                },
                CompPlayer {},
            ))
            .id()
    }
}

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
    pub fn get_entity(commands: &mut Commands) -> Entity {
        commands
            .spawn((
                NodeBundle {
                    style: HAND_STYLE,
                    background_color: HAND_BACKGROUND_COLOR,
                    ..default()
                },
                CompHand,
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
            .spawn((
                NodeBundle {
                    style: CARD_STYLE,
                    ..Default::default()
                },
                CompCard,
            ))
            .with_children(|builder| {
                builder.spawn(CardBundle::from(
                    asset_server,
                    Card {
                        color: card.color,
                        num: card.value,
                        is_revealed,
                    },
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

impl CompButtonBar {
    pub fn get_entity(commands: &mut Commands) -> Entity {
        commands
            .spawn((
                NodeBundle {
                    style: BUTTON_BAR_STYLE,
                    ..default()
                },
                CompButtonBar {},
            ))
            .id()
    }
}

impl CompBetButton {
    pub fn get_entity(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
        commands
            .spawn((
                ButtonBundle {
                    style: BUTTON_STYLE,
                    background_color: DEACTIVE_BUTTON_COLOR.into(),
                    ..default()
                },
                CompBetButton {},
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
            })
            .id()
    }
}

impl CompSplitButton {
    pub fn get_entity(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
        commands
            .spawn((
                ButtonBundle {
                    style: BUTTON_STYLE,
                    background_color: DEACTIVE_BUTTON_COLOR.into(),
                    ..default()
                },
                CompSplitButton {},
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
            })
            .id()
    }
}

impl CompDoubleDownButton {
    pub fn get_entity(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
        commands
            .spawn((
                ButtonBundle {
                    style: BUTTON_STYLE,
                    background_color: DEACTIVE_BUTTON_COLOR.into(),
                    ..default()
                },
                CompDoubleDownButton {},
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
            })
            .id()
    }
}

impl CompHitButton {
    pub fn get_entity(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
        commands
            .spawn((
                ButtonBundle {
                    style: BUTTON_STYLE,
                    background_color: DEACTIVE_BUTTON_COLOR.into(),
                    ..default()
                },
                CompHitButton {},
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
            })
            .id()
    }
}

impl CompStandButton {
    pub fn get_entity(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
        commands
            .spawn((
                ButtonBundle {
                    style: BUTTON_STYLE,
                    background_color: DEACTIVE_BUTTON_COLOR.into(),
                    ..default()
                },
                CompStandButton {},
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
            })
            .id()
    }
}
