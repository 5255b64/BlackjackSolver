use bevy::{prelude::*, window::PrimaryWindow};

use crate::client::{
    game::{components::*, player_request_events::*},
    states::GameState,
    styles::{
        get_button_text_style, ACTIVE_BUTTON_COLOR, BUTTON_BAR_STYLE, BUTTON_STYLE,
        DEACTIVE_BUTTON_COLOR, HOVERED_BUTTON_COLOR, PRESSED_BUTTON_COLOR,
    },
};

pub fn interact_with_start_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<CompBetButton>),
    >,
    game_state: Res<State<GameState>>,
    mut user_bet_event_writer: EventWriter<EventRequestPlayerBet>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match game_state.get() {
            GameState::PlayerBet => match *interaction {
                Interaction::Pressed => {
                    *background_color = PRESSED_BUTTON_COLOR.into();
                    // 创建新手牌
                    user_bet_event_writer.send(EventRequestPlayerBet {
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
        (Changed<Interaction>, With<CompSplitButton>),
    >,
    game_state: Res<State<GameState>>,
    mut user_split_event_writer: EventWriter<EventRequestPlayerSplit>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match game_state.get() {
            GameState::PlayerSplitOrDoubleDownOrHitOrStand(_) => match *interaction {
                Interaction::Pressed => {
                    *background_color = PRESSED_BUTTON_COLOR.into();
                    user_split_event_writer.send(EventRequestPlayerSplit {});
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
        (Changed<Interaction>, With<CompDoubleDownButton>),
    >,
    game_state: Res<State<GameState>>,
    mut user_double_down_event_writer: EventWriter<EventRequestPlayerDoubleDown>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match game_state.get() {
            GameState::PlayerDoubleDownOrHitOrStand(_)
            | GameState::PlayerSplitOrDoubleDownOrHitOrStand(_) => match *interaction {
                Interaction::Pressed => {
                    *background_color = PRESSED_BUTTON_COLOR.into();
                    user_double_down_event_writer.send(EventRequestPlayerDoubleDown {});
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
        (Changed<Interaction>, With<CompHitButton>),
    >,
    game_state: Res<State<GameState>>,
    mut user_hit_event_writer: EventWriter<EventRequestPlayerHit>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match game_state.get() {
            GameState::PlayerHitOrStand(_)
            | GameState::PlayerDoubleDownOrHitOrStand(_)
            | GameState::PlayerSplitOrDoubleDownOrHitOrStand(_) => match *interaction {
                Interaction::Pressed => {
                    *background_color = PRESSED_BUTTON_COLOR.into();
                    user_hit_event_writer.send(EventRequestPlayerHit {});
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
        (Changed<Interaction>, With<CompStandButton>),
    >,
    game_state: Res<State<GameState>>,
    mut user_stand_event_writer: EventWriter<EventRequestPlayerStand>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match game_state.get() {
            GameState::PlayerHitOrStand(_)
            | GameState::PlayerDoubleDownOrHitOrStand(_)
            | GameState::PlayerSplitOrDoubleDownOrHitOrStand(_) => match *interaction {
                Interaction::Pressed => {
                    *background_color = PRESSED_BUTTON_COLOR.into();
                    user_stand_event_writer.send(EventRequestPlayerStand {});
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
            CompButtonBar {},
        ))
        .with_children(|builder| {
            builder
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
                });

            builder
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
                });

            builder
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
                });
            builder
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
                });
            builder
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
                });
        });
}

// pub fn despawn_buttons(mut commands: Commands, button_bar_query: Query<Entity, With<ButtonBar>>) {
//     for button_bar_entity in button_bar_query.iter() {
//         commands.entity(button_bar_entity).despawn_recursive();
//     }
// }

pub fn update_bet_button_on_state_change(
    game_state: Res<State<GameState>>,
    mut bet_button_query: Query<&mut BackgroundColor, With<CompBetButton>>,
) {
    if game_state.is_changed() {
        info!(
            "update_bet_button_on_state_change:game_state={:?}",
            game_state.get()
        );

        // BetButton
        for mut background_color in bet_button_query.iter_mut() {
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
    mut split_button_query: Query<&mut BackgroundColor, With<CompSplitButton>>,
) {
    if game_state.is_changed() {
        // SplitButton
        for mut background_color in split_button_query.iter_mut() {
            match game_state.get() {
                GameState::PlayerSplitOrDoubleDownOrHitOrStand(_) => {
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
    mut double_down_button_query: Query<&mut BackgroundColor, With<CompDoubleDownButton>>,
) {
    if game_state.is_changed() {
        // DoubleDownButton
        for mut background_color in double_down_button_query.iter_mut() {
            match game_state.get() {
                GameState::PlayerDoubleDownOrHitOrStand(_)
                | GameState::PlayerSplitOrDoubleDownOrHitOrStand(_) => {
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
    mut hit_button_query: Query<&mut BackgroundColor, With<CompHitButton>>,
) {
    if game_state.is_changed() {
        // HitButton
        for mut background_color in hit_button_query.iter_mut() {
            match game_state.get() {
                GameState::PlayerHitOrStand(_)
                | GameState::PlayerDoubleDownOrHitOrStand(_)
                | GameState::PlayerSplitOrDoubleDownOrHitOrStand(_) => {
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
    mut stand_button_query: Query<&mut BackgroundColor, With<CompStandButton>>,
) {
    if game_state.is_changed() {
        // StandButton
        for mut background_color in stand_button_query.iter_mut() {
            match game_state.get() {
                GameState::PlayerHitOrStand(_)
                | GameState::PlayerDoubleDownOrHitOrStand(_)
                | GameState::PlayerSplitOrDoubleDownOrHitOrStand(_) => {
                    *background_color = ACTIVE_BUTTON_COLOR.into();
                }
                _ => {
                    *background_color = DEACTIVE_BUTTON_COLOR.into();
                }
            }
        }
    }
}
