use bevy::app::Plugin;

pub mod card;
pub mod chip;
pub mod components;
pub mod styles;
mod events;
mod systems;

use bevy::prelude::*;

use events::*;
use interactions::interact_with_start_button;
use layout::{despawn_game, spawn_game};
use systems::*;

use super::AppState;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .init_state::<SimulationState>()
            .init_state::<GameState>()
            // Event
            .add_event::<UserBet>()
            // Plugin
            // On Enter Systems
            .add_systems(OnEnter(AppState::Game), spawn_game)
            // Systems
            .add_systems(Update, 
                (
                    interact_with_start_button
                ),
            )
            // On Exit Systems
            .add_systems(OnExit(AppState::Game), despawn_game)
            ;
    }
}

#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
pub enum SimulationState {
    Running,
    #[default]
    Paused,
}

#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
pub enum GameState {
    #[default]
    PlayerBet,
    DealerCheckBlackJack,
    // PlayerBuyInsurance, // TODO 添加Insurance功能
    PlayerSplit,
    PlayerDoubleDownOrHitOrStand,
    PlayerHitOrStand,
    DealerHitOrStand,
    CheckResultAndReset,
}
