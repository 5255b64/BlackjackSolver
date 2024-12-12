use bevy::prelude::*;

use crate::client::{
    game::components::{CompBetButton, CompButtonBar, CompDealer, CompDoubleDownButton, CompFramework, CompHands, CompHitButton, CompInfoBar, CompPlayer, CompSplitButton, CompStandButton},
    resources::{
        BetButtonHandler, ButtonBarHandler, DealerHandler, DoubleDownButtonHandler, HandsHandler,
        HitButtonHandler, InfobarHandler, PlayerHandler, ResFrameworkHandler, SplitButtonHandler,
        StandButtonHandler,
    },
};

pub fn spawn_framework(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut res_structure_handler: ResMut<ResFrameworkHandler>,
) {
    let _framework_entity =
        build_framework(&mut commands, &asset_server, &mut res_structure_handler);
}

pub fn despawn_framework(
    mut commands: Commands,
    main_menu_query: Query<Entity, With<CompFramework>>,
) {
    if let Ok(main_menu_entity) = main_menu_query.get_single() {
        commands.entity(main_menu_entity).despawn_recursive();
    }
}

pub fn build_framework(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    res_structure_handler: &mut ResMut<ResFrameworkHandler>,
) -> Entity {
    let entity_framework = CompFramework::get_entity(commands);

    let entity_dealer = CompDealer::get_entity(commands);

    let entity_player = CompPlayer::get_entity(commands);

    let entity_infobar =
        CompInfoBar::get_entity(commands, String::from("This is Info Bar"), asset_server);

    let entity_button_bar = CompButtonBar::get_entity(commands);

    let entity_bet_button = CompBetButton::get_entity(commands, asset_server);

    let entity_split_button = CompSplitButton::get_entity(commands, asset_server);

    let entity_double_down_button = CompDoubleDownButton::get_entity(commands, asset_server);

    let entity_hit_button = CompHitButton::get_entity(commands, asset_server);

    let entity_stand_button = CompStandButton::get_entity(commands, asset_server);

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

    // 构建ResStructureHandler
    res_structure_handler.entity = Some(entity_framework);
    res_structure_handler.dealer_handler = Some(DealerHandler {
        hands_handler: HandsHandler {
            hand_handler_list: Vec::new(),
            entity: entity_dealer_hands,
        },
        entity: entity_dealer,
    });
    res_structure_handler.player_handler = Some(PlayerHandler {
        hands_handler: HandsHandler {
            hand_handler_list: Vec::new(),
            entity: entity_player_hands,
        },
        entity: entity_player,
    });
    res_structure_handler.infobar_handler = Some(InfobarHandler {
        info: String::from("This is info bar"),
        entity: entity_infobar,
    });
    res_structure_handler.button_bar_handler = Some(ButtonBarHandler {
        bet_button_handler: BetButtonHandler {
            entity: entity_bet_button,
        },
        split_button_handler: SplitButtonHandler {
            entity: entity_split_button,
        },
        double_down_button_handler: DoubleDownButtonHandler {
            entity: entity_double_down_button,
        },
        hit_button_handler: HitButtonHandler {
            entity: entity_hit_button,
        },
        stand_button_handler: StandButtonHandler {
            entity: entity_stand_button,
        },
        entity: entity_button_bar,
    });

    entity_framework
}
