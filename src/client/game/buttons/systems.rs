use bevy::{prelude::*, window::PrimaryWindow};

use crate::client::{
    card::{components::Card, systems::despawn_cards},
    dealer::resources::DealerHand,
    game::{
        events::RequestPlayerBet, RequestPlayerDoubleDown, RequestPlayerHit, RequestPlayerSplit,
        RequestPlayerStand,
    },
    player::resources::PlayerHand,
    styles::{
        get_button_text_style, ACTIVE_BUTTON_COLOR, BUTTON_BAR_STYLE, BUTTON_STYLE,
        DEACTIVE_BUTTON_COLOR, HOVERED_BUTTON_COLOR, PRESSED_BUTTON_COLOR,
    },
    GameState,
};

use super::components::*;

pub fn interact_with_start_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<BetButton>),
    >,
    game_state: Res<State<GameState>>,
    mut user_bet_event_writer: EventWriter<RequestPlayerBet>,
    mut dealer_hand: ResMut<DealerHand>,
    mut player_hand: ResMut<PlayerHand>,
    mut commands: Commands,
    card_query: Query<Entity, With<Card>>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match game_state.get() {
            GameState::PlayerBet => match *interaction {
                Interaction::Pressed => {
                    *background_color = PRESSED_BUTTON_COLOR.into();
                    // 清除上一局手牌
                    despawn_cards(&mut commands, &card_query);
                    dealer_hand.reset();
                    player_hand.reset();
                    // 创建新手牌
                    user_bet_event_writer.send(RequestPlayerBet {
                        value: 2, // TODO 修改value
                    });
                }
                Interaction::Hovered => {
                    *background_color = HOVERED_BUTTON_COLOR.into();
                }
                Interaction::None => {
                    *background_color = ACTIVE_BUTTON_COLOR.into();
                }
            },
            _ => {
                *background_color = DEACTIVE_BUTTON_COLOR.into();
            }
        };
    }
}

pub fn interact_with_split_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<SplitButton>),
    >,
    game_state: Res<State<GameState>>,
    mut user_split_event_writer: EventWriter<RequestPlayerSplit>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match game_state.get() {
            GameState::PlayerSplitOrDoubleDownOrHitOrStand => match *interaction {
                Interaction::Pressed => {
                    *background_color = PRESSED_BUTTON_COLOR.into();
                    user_split_event_writer.send(RequestPlayerSplit {});
                }
                Interaction::Hovered => {
                    *background_color = HOVERED_BUTTON_COLOR.into();
                }
                Interaction::None => {
                    *background_color = ACTIVE_BUTTON_COLOR.into();
                }
            },
            _ => {
                *background_color = DEACTIVE_BUTTON_COLOR.into();
            }
        };
    }
}

pub fn interact_with_double_down_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<DoubleDownButton>),
    >,
    game_state: Res<State<GameState>>,
    mut user_double_down_event_writer: EventWriter<RequestPlayerDoubleDown>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match game_state.get() {
            GameState::PlayerDoubleDownOrHitOrStand
            | GameState::PlayerSplitOrDoubleDownOrHitOrStand => match *interaction {
                Interaction::Pressed => {
                    *background_color = PRESSED_BUTTON_COLOR.into();
                    user_double_down_event_writer.send(RequestPlayerDoubleDown {});
                }
                Interaction::Hovered => {
                    *background_color = HOVERED_BUTTON_COLOR.into();
                }
                Interaction::None => {
                    *background_color = ACTIVE_BUTTON_COLOR.into();
                }
            },
            _ => {
                *background_color = DEACTIVE_BUTTON_COLOR.into();
            }
        };
    }
}

pub fn interact_with_hit_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<HitButton>),
    >,
    game_state: Res<State<GameState>>,
    mut user_hit_event_writer: EventWriter<RequestPlayerHit>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match game_state.get() {
            GameState::PlayerHitOrStand
            | GameState::PlayerDoubleDownOrHitOrStand
            | GameState::PlayerSplitOrDoubleDownOrHitOrStand => match *interaction {
                Interaction::Pressed => {
                    *background_color = PRESSED_BUTTON_COLOR.into();
                    user_hit_event_writer.send(RequestPlayerHit {});
                }
                Interaction::Hovered => {
                    *background_color = HOVERED_BUTTON_COLOR.into();
                }
                Interaction::None => {
                    *background_color = ACTIVE_BUTTON_COLOR.into();
                }
            },
            _ => {
                *background_color = DEACTIVE_BUTTON_COLOR.into();
            }
        };
    }
}

pub fn interact_with_stand_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<StandButton>),
    >,
    game_state: Res<State<GameState>>,
    mut user_stand_event_writer: EventWriter<RequestPlayerStand>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match game_state.get() {
            GameState::PlayerHitOrStand
            | GameState::PlayerDoubleDownOrHitOrStand
            | GameState::PlayerSplitOrDoubleDownOrHitOrStand => match *interaction {
                Interaction::Pressed => {
                    *background_color = PRESSED_BUTTON_COLOR.into();
                    user_stand_event_writer.send(RequestPlayerStand {});
                }
                Interaction::Hovered => {
                    *background_color = HOVERED_BUTTON_COLOR.into();
                }
                Interaction::None => {
                    *background_color = ACTIVE_BUTTON_COLOR.into();
                }
            },
            _ => {
                *background_color = DEACTIVE_BUTTON_COLOR.into();
            }
        };
    }
}

pub fn spawn_buttons(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    commands
        .spawn((
            NodeBundle {
                style: BUTTON_BAR_STYLE,
                transform: Transform::from_xyz(window.width() / 5.0, window.height() / 5.0, 0.0),
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
                    BetButton {},
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

pub fn despawn_buttons(mut commands: Commands, button_bar_query: Query<Entity, With<ButtonBar>>) {
    for button_bar_entity in button_bar_query.iter() {
        commands.entity(button_bar_entity).despawn_recursive();
    }
}

pub fn update_bet_button_on_state_change(
    game_state: Res<State<GameState>>,
    mut bet_button_query: Query<&mut BackgroundColor, With<BetButton>>,
) {
    if game_state.is_changed() {
        // BetButton
        if let Ok(mut background_color) = bet_button_query.get_single_mut() {
            match game_state.get() {
                GameState::PlayerBet => {
                    *background_color = ACTIVE_BUTTON_COLOR.into();
                }
                _ => {
                    *background_color = DEACTIVE_BUTTON_COLOR.into();
                }
            }
        }
    }
}

pub fn update_split_button_on_state_change(
    game_state: Res<State<GameState>>,
    mut split_button_query: Query<&mut BackgroundColor, With<SplitButton>>,
) {
    if game_state.is_changed() {
        // SplitButton
        if let Ok(mut background_color) = split_button_query.get_single_mut() {
            match game_state.get() {
                GameState::PlayerSplitOrDoubleDownOrHitOrStand => {
                    *background_color = ACTIVE_BUTTON_COLOR.into();
                }
                _ => {
                    *background_color = DEACTIVE_BUTTON_COLOR.into();
                }
            }
        }
    }
}

pub fn update_double_down_button_on_state_change(
    game_state: Res<State<GameState>>,
    mut double_down_button_query: Query<&mut BackgroundColor, With<DoubleDownButton>>,
) {
    if game_state.is_changed() {
        // DoubleDownButton
        if let Ok(mut background_color) = double_down_button_query.get_single_mut() {
            match game_state.get() {
                GameState::PlayerDoubleDownOrHitOrStand
                | GameState::PlayerSplitOrDoubleDownOrHitOrStand => {
                    *background_color = ACTIVE_BUTTON_COLOR.into();
                }
                _ => {
                    *background_color = DEACTIVE_BUTTON_COLOR.into();
                }
            }
        }
    }
}

pub fn update_hit_button_on_state_change(
    game_state: Res<State<GameState>>,
    mut hit_button_query: Query<&mut BackgroundColor, With<HitButton>>,
) {
    if game_state.is_changed() {
        // HitButton
        if let Ok(mut background_color) = hit_button_query.get_single_mut() {
            match game_state.get() {
                GameState::PlayerHitOrStand
                | GameState::PlayerDoubleDownOrHitOrStand
                | GameState::PlayerSplitOrDoubleDownOrHitOrStand => {
                    *background_color = ACTIVE_BUTTON_COLOR.into();
                }
                _ => {
                    *background_color = DEACTIVE_BUTTON_COLOR.into();
                }
            }
        }
    }
}



pub fn update_stand_button_on_state_change(
    game_state: Res<State<GameState>>,
    mut stand_button_query: Query<&mut BackgroundColor, With<StandButton>>,
) {
    if game_state.is_changed() {
        // StandButton
        if let Ok(mut background_color) = stand_button_query.get_single_mut() {
            match game_state.get() {
                GameState::PlayerHitOrStand
                | GameState::PlayerDoubleDownOrHitOrStand
                | GameState::PlayerSplitOrDoubleDownOrHitOrStand => {
                    *background_color = ACTIVE_BUTTON_COLOR.into();
                }
                _ => {
                    *background_color = DEACTIVE_BUTTON_COLOR.into();
                }
            }
        }
    }
}
