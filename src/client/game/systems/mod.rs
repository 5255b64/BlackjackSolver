pub mod event_handler;
pub mod keyboard;
pub mod systems;

use bevy::prelude::*;
use keyboard::*;
use event_handler::*;
use systems::update_game_state;

pub struct SystemPlugin;


impl Plugin for SystemPlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            // States
            // Event
            // Plugin
            // On Enter Systems
            // Systems
            .add_systems(Update, (
                keyboard_esc_exit_game,
                update_game_state,
                handle_request_player_bet,
                handle_request_player_hit,
                handle_request_player_stand,
                handle_request_player_split,
                handle_request_player_double_down,
                handle_response_init_game_with_cards,
                handle_response_wait_player_buy_insurance,
                handle_response_insurance_result,
                handle_response_player_split_cards,
                handle_response_player_stand,
                handle_response_game_over,
                handle_response_player_draw_card,
                handle_response_dealer_draw_card
            ));
        // On Exit Systems
    }
}
