use bevy::prelude::*;

use crate::client::{
    buttons::components::{
        BetButton, ButtonBar, DoubleDownButton, HitButton, SplitButton, StandButton,
    },
    game::components::{CompDealer, CompFramework, CompHands, CompInfoBar, CompPlayer},
    styles::*,
};

pub fn spawn_framework(mut commands: Commands, asset_server: Res<AssetServer>) {
    let _framework_entity = build_framework(&mut commands, &asset_server);
}

pub fn despawn_framework(
    mut commands: Commands,
    main_menu_query: Query<Entity, With<CompFramework>>,
) {
    if let Ok(main_menu_entity) = main_menu_query.get_single() {
        commands.entity(main_menu_entity).despawn_recursive();
    }
}

pub fn build_framework(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let entity_framework = commands
        .spawn((
            NodeBundle {
                style: FRAMEWORK_STYLE,
                background_color: FRAMEWORK_BACKGROUND_COLOR,
                ..default()
            },
            CompFramework {},
        ))
        .id();

    let entity_dealer = commands
        .spawn((
            NodeBundle {
                style: DEALER_STYLE,
                background_color: DEALER_BACKGROUND_COLOR,
                ..default()
            },
            CompDealer {},
        ))
        .id();

    let entity_player = commands
        .spawn((
            NodeBundle {
                style: PLAYER_STYLE,
                background_color: PLAYER_BACKGROUND_COLOR,
                ..default()
            },
            CompPlayer {},
        ))
        .id();

    let entity_infobar =
        CompInfoBar::get_entity(commands, String::from("This is Info Bar"), asset_server);

    let entity_button_bar = commands
        .spawn((
            NodeBundle {
                style: BUTTON_BAR_STYLE,
                ..default()
            },
            ButtonBar {},
        ))
        .id();

    let entity_bet_button = commands
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
        })
        .id();

    let entity_split_button = commands
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
        })
        .id();

    let entity_double_down_button = commands
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
        })
        .id();

    let entity_hit_button = commands
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
        })
        .id();

    let entity_stand_button = commands
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
        })
        .id();

    let entity_player_hands = CompHands::get_entity(commands);

    let entity_dealer_hands = CompHands::get_entity(commands);

    commands
        .entity(entity_dealer)
        .push_children(&[entity_dealer_hands]);

    commands
        .entity(entity_player)
        .push_children(&[entity_player_hands]);

    commands.entity(entity_button_bar).push_children(&[
        entity_bet_button,
        entity_split_button,
        entity_double_down_button,
        entity_hit_button,
        entity_stand_button,
    ]);

    commands.entity(entity_framework).push_children(&[
        entity_dealer,
        entity_infobar,
        entity_player,
        entity_button_bar,
    ]);
    entity_framework
}
