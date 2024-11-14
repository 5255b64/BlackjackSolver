use super::components::*;
use bevy::prelude::*;

pub fn spawn_chips(mut commands: Commands, asset_server: Res<AssetServer>) {
    const MARGIN: Val = Val::Px(12.);
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                flex_direction: FlexDirection::Column,
                row_gap: MARGIN,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|builder| {
            for chip_color in vec!["BlackWhite", "BlueWhite", "GreenWhite", "RedWhite"] {
                builder
                    .spawn(NodeBundle {
                        style: Style {
                            width: Val::Percent(100.),
                            height: Val::Percent(100.),
                            flex_direction: FlexDirection::Row,
                            column_gap: MARGIN,
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .with_children(|builder| {
                        builder.spawn((
                            SpriteBundle {
                                texture: asset_server.load(format!(
                                    "{}{}{}",
                                    "sprites/chips/chip", chip_color, "_border.png"
                                )),
                                ..default()
                            },
                            Chip {},
                        ));
                    });
            }
        });
}

pub fn despawn_chips(mut commands: Commands, chip_query: Query<Entity, With<Chip>>) {
    for chip_entity in chip_query.iter() {
        commands.entity(chip_entity).despawn();
    }
}
