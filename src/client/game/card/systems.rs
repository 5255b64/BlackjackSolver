use super::components::*;
use bevy::prelude::*;

pub fn get_card_ui_bundle_entity(
    card: Card,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) -> Entity {
    const MARGIN: Val = Val::Px(12.);
    let parent = commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Row,
                column_gap: MARGIN,
                ..Default::default()
            },
            ..Default::default()
        })
        .id();
    info!("parent entity id:{parent:?}");
    let child = commands
        .spawn((CardBundle::from(&asset_server, card),))
        .id();
    info!("child entity id:{child:?}");
    commands.entity(parent).push_children(&[child]);
    parent
}
