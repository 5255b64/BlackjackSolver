use bevy::prelude::*;

/// Button的容器
#[derive(Component)]
pub struct ButtonBar {}

/// 选定筹码量 开始游戏
#[derive(Component)]
pub struct BetButton {}

#[derive(Component)]
pub struct HitButton {}

#[derive(Component)]
pub struct StandButton {}

#[derive(Component)]
pub struct SplitButton {}

#[derive(Component)]
pub struct DoubleDownButton {}