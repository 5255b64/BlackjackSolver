use bevy::{prelude::*, window::PrimaryWindow};

use crate::{client::{card::{components::Card, systems::get_card_sprite_boundle}, resources::GameTable}, server::card::ECard};

use super::resources::PlayerHand;

const X_START: f32 = 200.0;
const Y_START: f32 = 300.0;
const X_STEP: f32 = 200.0;

pub fn spawn_player_cards(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    game_table: Res<GameTable>,
) {
    let window = window_query.get_single().unwrap();
    let cards = &game_table.table.dealer_hand.hand.cards;

    for (idx, card) in cards.iter().enumerate() {
        let mut card_boundle = get_card_sprite_boundle(&card.color, &card.value, &asset_server);
        card_boundle.transform = Transform::from_xyz(X_START + X_STEP * idx as f32, Y_START, 0.0);
        commands.spawn((card_boundle, Card {}));
    }
}

pub fn spawn_new_player_card(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    player_hand: &mut ResMut<PlayerHand>,
    card: ECard,
) {
    let cards = &mut player_hand.cards;
    let idx = cards.len();
    let mut card_boundle = get_card_sprite_boundle(&card.color, &card.value, &asset_server);
    card_boundle.transform = Transform::from_xyz(X_START + X_STEP * idx as f32, Y_START, 0.0);
    commands.spawn((card_boundle, Card {}));
    cards.push(card);
}