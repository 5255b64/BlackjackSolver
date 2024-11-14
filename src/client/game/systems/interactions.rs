use bevy::prelude::*;

use crate::client::{
    components::{DoubleDownButton, StartButton},
    game::{events::UserBet, UserDoubleDown},
    styles::{
        ACTIVE_BUTTON_COLOR, DEACTIVE_BUTTON_COLOR, HOVERED_BUTTON_COLOR, PRESSED_BUTTON_COLOR,
    },
    GameState,
};

pub fn interact_with_start_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<StartButton>),
    >,
    game_state: Res<State<GameState>>,
    mut user_bet_event_writer: EventWriter<UserBet>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match game_state.get() {
            GameState::PlayerBet => match *interaction {
                Interaction::Pressed => {
                    *background_color = PRESSED_BUTTON_COLOR.into();
                    user_bet_event_writer.send(UserBet {
                        value: 1, // TODO 修改value
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

pub fn interact_with_double_down_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<DoubleDownButton>),
    >,
    game_state: Res<State<GameState>>,
    mut user_double_down_event_writer: EventWriter<UserDoubleDown>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match game_state.get() {
            GameState::PlayerDoubleDownOrHitOrStand => match *interaction {
                Interaction::Pressed => {
                    *background_color = PRESSED_BUTTON_COLOR.into();
                    user_double_down_event_writer.send(UserDoubleDown {});
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

// TODO 补充其他Button信息
