use bevy::prelude::*;

use crate::server::card::{ECardColor, ECardNumber};

#[derive(Component)]
pub struct Card {
    pub color: ECardColor,
    pub num: ECardNumber,
    pub is_revealed: bool,
}

#[derive(Bundle)]
pub struct CardBundle {
    pub card: Card,
    pub image: ImageBundle,
}

impl Default for Card {
    fn default() -> Self {
        Self {
            color: ECardColor::Hearts,
            num: ECardNumber::Ace,
            is_revealed: false,
        }
    }
}

impl CardBundle {
    pub fn from(card: Card, asset_server: &Res<AssetServer>) -> Self {
        let card_type = card.color.to_string();
        let card_num = String::from(match card.num {
            ECardNumber::Ace => "A",
            ECardNumber::Two => "2",
            ECardNumber::Three => "3",
            ECardNumber::Four => "4",
            ECardNumber::Five => "5",
            ECardNumber::Six => "6",
            ECardNumber::Seven => "7",
            ECardNumber::Eight => "8",
            ECardNumber::Nine => "9",
            ECardNumber::Ten => "10",
            ECardNumber::Jack => "J",
            ECardNumber::Queen => "Q",
            ECardNumber::King => "K",
        });
        Self {
            card,
            image: ImageBundle {
                image: asset_server
                    .load(format!(
                        "{}{}{}{}",
                        "sprites/cards/card", card_type, card_num, ".png"
                    ))
                    .into(),
                ..default()
            },
        }
    }
}
