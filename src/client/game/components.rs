use bevy::prelude::Component;

use crate::server::{card::ECard, value::EValue};

/// 游戏UI主体
/// 包含DealerHand、PlayerHand、PlayerChip、Buttons
#[derive(Component)]
pub struct Game{}

/// 庄家手牌
#[derive(Component, Default, Clone)]
pub struct DealerHand {
    pub point: EValue,
    pub cards: Vec<ECard>,
}

/// 玩家手牌
#[derive(Component, Default, Clone)]
pub struct PlayerHand {
    pub point: EValue,
    pub cards: Vec<ECard>,
}

/// 玩家筹码
#[derive(Component)]
pub struct PlayerChip {
    pub chips: usize,
}

impl Default for PlayerChip {
    fn default() -> Self {
        Self { chips: 0 }
    }
}

/// Button的容器
#[derive(Component)]
pub struct ButtonBar {}

/// 选定筹码量 开始游戏
#[derive(Component)]
pub struct StartButton {}

#[derive(Component)]
pub struct HitButton {}

#[derive(Component)]
pub struct StandButton {}

#[derive(Component)]
pub struct SplitButton {}

#[derive(Component)]
pub struct DoubleDownButton {}