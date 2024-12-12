use bevy::prelude::*;

use crate::server::card::{ECard, ECardColor, ECardNumber};

#[derive(Component, Clone)]
pub struct Card {
    pub color: ECardColor,
    pub num: ECardNumber,
    pub is_revealed: bool,
}

/// 标记一张未reveal的card
#[derive(Component)]
pub struct Covered;

#[derive(Bundle)]
pub struct CardBundle {
    pub card: Card,
    pub image: ImageBundle,
}

impl From<ECard> for Card {
    fn from(card: ECard) -> Self {
        Self {
            color: card.color,
            num: card.value,
            is_revealed: true,
        }
    }
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

impl Card {
    pub fn get_num(&self) -> String {
        String::from(match self.num {
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
        })
    }

    #[inline]
    pub fn get_type(&self) -> String {
        self.color.to_string()
    }

    pub fn get_img_addr(&self) -> String {
        let card_type = self.get_type();
        let card_num = self.get_num();
        match self.is_revealed {
            true => format!(
                "{}{}{}{}",
                "sprites/cards/card", card_type, card_num, ".png"
            ),
            false => String::from("sprites/cards/cardBack_blue5.png"),
        }
    }
}

impl CardBundle {
    pub fn from(asset_server: &Res<AssetServer>, card: Card) -> Self {
        let img_addr = card.get_img_addr();
        Self {
            card,
            image: ImageBundle {
                image: asset_server.load(img_addr).into(),
                ..default()
            },
        }
    }
}
