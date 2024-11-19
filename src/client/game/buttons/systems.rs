use bevy::{prelude::*, window::PrimaryWindow};

use crate::{
    client::{
        game::{events::RequestPlayerBet, RequestPlayerDoubleDown, RequestPlayerHit, RequestPlayerSplit, RequestPlayerStand},
        resources::GameTable,
        styles::{
            get_button_text_style, ACTIVE_BUTTON_COLOR, BUTTON_BAR_STYLE, BUTTON_STYLE,
            DEACTIVE_BUTTON_COLOR, HOVERED_BUTTON_COLOR, MAIN_MENU_STYLE, PRESSED_BUTTON_COLOR,
        },
        GameState,
    },
    server::player::EPlayerAction,
};

use super::components::*;

pub fn interact_with_start_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<StartButton>),
    >,
    game_state: Res<State<GameState>>,
    mut user_bet_event_writer: EventWriter<RequestPlayerBet>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match game_state.get() {
            GameState::PlayerBet => match *interaction {
                Interaction::Pressed => {
                    *background_color = PRESSED_BUTTON_COLOR.into();
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
    mut game_state_next_state: ResMut<NextState<GameState>>,
    mut user_stand_event_writer: EventWriter<RequestPlayerStand>,
    mut res_game_table: ResMut<GameTable>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match game_state.get() {
            GameState::PlayerHitOrStand
            | GameState::PlayerDoubleDownOrHitOrStand
            | GameState::PlayerSplitOrDoubleDownOrHitOrStand => match *interaction {
                Interaction::Pressed => {
                    *background_color = PRESSED_BUTTON_COLOR.into();
                    // user_stand_event_writer.send(PlayerStand {});
                    match res_game_table
                        .table
                        .receive_player_action(EPlayerAction::Stand)
                    {
                        Ok(_) => {
                            // loop {
                            //     match res_game_table.table.run() {
                            //         Ok(_) => break,
                            //         Err(_) => match res_game_table.table.run() {
                            //             Ok(_) => break,
                            //             Err(_) => {
                            //                 error!("Error")
                            //             }
                            //         },
                            //     }
                            // }
                            // let new_state = res_game_table.table.get_state();
                            // println!("New State:{new_state:?}");
                            // game_state_next_state.set(new_state.into());
                        }
                        Err(e) => {
                            error!("Error: {:?}", e)
                        }
                    }
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

pub fn despawn_buttons(mut commands: Commands, button_bar_query: Query<Entity, With<ButtonBar>>) {
    for button_bar_entity in button_bar_query.iter() {
        commands.entity(button_bar_entity).despawn_recursive();
    }
}
