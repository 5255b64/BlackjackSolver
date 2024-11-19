use bevy::prelude::*;

#[derive(Component)]
pub struct Chip {
    pub value:EChipValue,
}

enum EChipValue {
    Chip1,
    Chip5,
    Chip10,
    Chip50,
    Chip100
}