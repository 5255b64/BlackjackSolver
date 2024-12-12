use bevy::{ecs::{identifier::error, observer::TriggerTargets}, prelude::*};

use crate::server::{card::ECard, table::STable, value::EValue};

use super::{card::components::Card, components::*};

/// 调用后端的STable
#[derive(Resource)]
pub struct ResGameTable {
    pub table: STable,
}

impl Default for ResGameTable {
    fn default() -> Self {
        Self {
            table: Default::default(),
        }
    }
}

/// 保存Game中的所有Entity的Handler
#[derive(Resource, Default)]
pub struct ResFrameworkHandler {
    pub dealer_handler: Option<DealerHandler>,
    pub infobar_handler: Option<InfobarHandler>,
    pub player_handler: Option<PlayerHandler>,
    pub button_bar_handler: Option<ButtonBarHandler>,
    pub entity: Option<Entity>,
    pub focus: Focus,
}

/// 焦点
#[derive(Default, Debug)]
pub enum Focus {
    #[default]
    None, // 牌局结束/等待开始 状态
    Dealer,        // 庄家行动
    Player(usize), // 玩家行动 对第n手牌进行操作
}

pub struct DealerHandler {
    pub hands_handler: HandsHandler,
    pub entity: Entity,
}

pub struct InfobarHandler {
    pub info: String,
    pub entity: Entity,
}

pub struct PlayerHandler {
    pub hands_handler: HandsHandler,
    pub entity: Entity,
}

pub struct HandsHandler {
    pub hand_handler_list: Vec<HandHandler>,
    pub entity: Entity,
}

pub struct HandHandler {
    pub cards_list: Vec<ECard>,
    pub value: EValue,
    pub cards_handler: CardsHandler,
    pub value_handler: ValueHandler,
    pub entity: Entity,
}

pub struct CardsHandler {
    pub card_handler_list: Vec<CardHandler>,
    pub entity: Entity,
}

pub struct ValueHandler {
    pub value: EValue,
    pub entity: Entity,
}

pub struct CardHandler {
    pub card: ECard,
    pub entity: Entity,
}

// -------------------- Button Bar ------------------------
pub struct ButtonBarHandler {
    pub bet_button_handler: BetButtonHandler,
    pub split_button_handler: SplitButtonHandler,
    pub double_down_button_handler: DoubleDownButtonHandler,
    pub hit_button_handler: HitButtonHandler,
    pub stand_button_handler: StandButtonHandler,
    pub entity: Entity,
}

pub struct BetButtonHandler {
    pub entity: Entity,
}

pub struct SplitButtonHandler {
    pub entity: Entity,
}

pub struct DoubleDownButtonHandler {
    pub entity: Entity,
}

pub struct HitButtonHandler {
    pub entity: Entity,
}

pub struct StandButtonHandler {
    pub entity: Entity,
}

// --- Impl ----
impl ResFrameworkHandler {
    pub fn reset_hands(
        &mut self,
        commands: &mut Commands,
        assert_server: &Res<AssetServer>,
        q_text: &mut Query<(&mut Text, Entity)>,
    ) {
        info!("ResFrameworkHandler:reset hands");
        // 重置dealer
        if let Some(dealer_handler) = &mut self.dealer_handler {
            dealer_handler.reset(commands, assert_server);
        }
        // 重置player
        if let Some(player_handler) = &mut self.player_handler {
            player_handler.reset(commands, assert_server);
        }
        // 重置InfoBar
        if let Some(infobar_handler) = &mut self.infobar_handler {
            infobar_handler.set_value(q_text, String::from("Reset"));
        }
    }
}

impl InfobarHandler {
    pub fn set_value(&mut self, q_text: &mut Query<(&mut Text, Entity)>, new_info: String) {
        self.info = new_info.clone();
        for (mut text, entity) in q_text.iter_mut() {
            if entity == self.entity {
                text.sections[0].value = new_info.clone();
            }
        }
    }
}

impl CardHandler {
    pub fn new(
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        card: ECard,
        is_revealed: bool,
    ) -> Self {
        Self {
            card,
            entity: CompCard::get_entity(commands, asset_server, card, is_revealed),
        }
    }

    pub fn reveal_card(
        &mut self,
        assert_server: &Res<AssetServer>,
        q_img: &mut Query<(&mut UiImage, &Parent)>,
    ) {
        info!("reveal card:{:?}", self.card);
        for (mut img, parent) in q_img.iter_mut() {
            info!("querying entity with parent:{:?}", parent);
            if parent.get() == self.entity {
                info!("reveal card\t find entity:{:?}", self.card);
                let card: Card = self.card.into();
                *img = assert_server.load(card.get_img_addr()).into();
            }
        }
    }
}

impl CardsHandler {
    pub fn draw_new_card(
        &mut self,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        card: ECard,
        is_revealed: bool,
    ) {
        let card_handler = CardHandler::new(commands, asset_server, card, is_revealed);
        let card_entity = card_handler.entity;
        self.card_handler_list.push(card_handler);
        commands.entity(self.entity).push_children(&[card_entity]);
    }

    pub fn split(&mut self, commands: &mut Commands) -> Option<CardHandler> {
        if self.card_handler_list.len() == 2 {
            let last_card_handler = self.card_handler_list.pop().unwrap();
            commands
                .entity(self.entity)
                .remove_children(&[last_card_handler.entity]);
            Some(last_card_handler)
        } else {
            error!("split fail: cards num != 2");
            None
        }
    }

    pub fn add_card_handler(&mut self, commands: &mut Commands, card_handler: CardHandler) {
        let entity_card = card_handler.entity;
        self.card_handler_list.push(card_handler);
        commands.entity(self.entity).push_children(&[entity_card]);
    }

    /// Only for Dealer
    pub fn reveal_card(
        &mut self,
        assert_server: &Res<AssetServer>,
        q_img: &mut Query<(&mut UiImage, &Parent)>,
        card_num: usize,
    ) {
        self.card_handler_list
            .get_mut(card_num)
            .unwrap()
            .reveal_card(assert_server, q_img);
    }
}

impl ValueHandler {
    pub fn set_value(&mut self, q_text: &mut Query<(&mut Text, Entity)>, new_value: EValue) {
        self.value = new_value.clone();
        for (mut text, entity) in q_text.iter_mut() {
            if entity == self.entity {
                text.sections[0].value = new_value.to_string();
            }
        }
    }
}

impl HandHandler {
    pub fn draw_new_card(
        &mut self,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        q_text: &mut Query<(&mut Text, Entity)>,
        card: ECard,
        is_revealed: bool,
    ) {
        // 调整数据
        self.cards_list.push(card.clone());

        if is_revealed {
            self.value = card + self.value;
            self.value_handler.set_value(q_text, self.value);
        }

        // 调整children handler
        self.cards_handler
            .draw_new_card(commands, asset_server, card.clone(), is_revealed);
    }

    pub fn new(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Self {
        let entity = CompHand::get_entity(commands);
        let entity_cards = CompCards::get_entity(commands);
        let entity_value = CompValue::get_entity(commands, default(), asset_server);

        commands
            .entity(entity)
            .push_children(&[entity_cards, entity_value]);

        Self {
            cards_list: default(),
            value: default(),
            cards_handler: CardsHandler {
                card_handler_list: default(),
                entity: entity_cards,
            },
            value_handler: ValueHandler {
                value: default(),
                entity: entity_value,
            },
            entity,
        }
    }

    pub fn add_card_handler(
        &mut self,
        commands: &mut Commands,
        q_text: &mut Query<(&mut Text, Entity)>,
        card_handler: CardHandler,
    ) {
        self.value = card_handler.card + self.value;
        self.cards_handler.add_card_handler(commands, card_handler);
        self.value_handler.set_value(q_text, self.value);
    }

    pub fn split(
        &mut self,
        commands: &mut Commands,
        q_text: &mut Query<(&mut Text, Entity)>,
    ) -> CardHandler {
        match self.cards_handler.split(commands) {
            Some(card_handler) => {
                self.value = self
                    .cards_handler
                    .card_handler_list
                    .get(0)
                    .unwrap()
                    .card
                    .value
                    .into();
                self.value_handler.set_value(q_text, self.value);
                card_handler
            }
            None => {
                error!("split fail!");
                todo!()
            }
        }
    }

    /// Only for Dealer
    pub fn reveal_card(
        &mut self,
        assert_server: &Res<AssetServer>,
        q_img: &mut Query<(&mut UiImage, &Parent)>,
    ) {
        // 调整展示的图像
        self.cards_handler.reveal_card(assert_server, q_img, 1);

        // 调整value
        self.value = self.value + self.cards_list.get(1).unwrap().value;
    }
}

impl HandsHandler {
    pub fn draw_new_card(
        &mut self,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        q_text: &mut Query<(&mut Text, Entity)>,
        hand_index: usize,
        card: ECard,
        is_revealed: bool,
    ) {
        if let Some(hand_handler) = self.hand_handler_list.get_mut(hand_index) {
            hand_handler.draw_new_card(commands, asset_server, q_text, card, is_revealed);
        } else {
            error!("Can not get the #{hand_index:?} hand!");
        }
    }

    pub fn reset(&mut self, commands: &mut Commands) {
        for hand_handler in &self.hand_handler_list {
            commands.entity(hand_handler.entity).despawn_recursive();
        }
        self.hand_handler_list.clear();
    }

    pub fn push_blank_hand(&mut self, commands: &mut Commands, asset_server: &Res<AssetServer>) {
        let hand_handler = HandHandler::new(commands, asset_server);
        let entity_hand = hand_handler.entity;
        commands.entity(self.entity).push_children(&[entity_hand]);
        self.hand_handler_list.push(hand_handler);
    }

    pub fn add_hand_handler(
        &mut self,
        commands: &mut Commands,
        hand_handler: HandHandler,
        hand_index: usize,
    ) {
        let entity_hand = hand_handler.entity;
        self.hand_handler_list.push(hand_handler);
        commands
            .entity(self.entity)
            .insert_children(hand_index, &[entity_hand]);
    }

    /// 分牌
    /// Only for Player
    pub fn split(
        &mut self,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        q_text: &mut Query<(&mut Text, Entity)>,
        hand_index: usize,
    ) {
        if let Some(hand_handler) = self.hand_handler_list.get_mut(hand_index) {
            let card_handler: CardHandler = hand_handler.split(commands, q_text);
            let mut new_hand_handler = HandHandler::new(commands, asset_server);
            new_hand_handler.add_card_handler(commands, q_text, card_handler);
            self.add_hand_handler(commands, new_hand_handler, hand_index + 1);
        } else {
            error!("split error:can not find #{hand_index} hand!");
        }
    }

    /// Only for Dealer
    pub fn reveal_card(
        &mut self,
        assert_server: &Res<AssetServer>,
        q_img: &mut Query<(&mut UiImage, &Parent)>,
    ) {
        self.hand_handler_list
            .get_mut(0)
            .unwrap()
            .reveal_card(assert_server, q_img);
    }
}

impl DealerHandler {
    pub fn draw_new_card(
        &mut self,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        q_text: &mut Query<(&mut Text, Entity)>,
        hand_index: usize,
        card: ECard,
        is_revealed: bool,
    ) {
        self.hands_handler.draw_new_card(
            commands,
            asset_server,
            q_text,
            hand_index,
            card,
            is_revealed,
        );
    }

    /// 揭示暗牌
    pub fn reveal_card(
        &mut self,
        assert_server: &Res<AssetServer>,
        q_img: &mut Query<(&mut UiImage, &Parent)>,
    ) {
        self.hands_handler.reveal_card(assert_server, q_img);
    }

    pub fn reset(&mut self, commands: &mut Commands, assert_server: &Res<AssetServer>) {
        // 刪除所有hand
        self.hands_handler.reset(commands);

        // 新增一個空hand
        self.hands_handler.push_blank_hand(commands, assert_server);
    }
}

impl PlayerHandler {
    pub fn draw_new_card(
        &mut self,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        q_text: &mut Query<(&mut Text, Entity)>,
        hand_index: usize,
        card: ECard,
        is_revealed: bool,
    ) {
        self.hands_handler.draw_new_card(
            commands,
            asset_server,
            q_text,
            hand_index,
            card,
            is_revealed,
        );
    }

    pub fn reset(&mut self, commands: &mut Commands, assert_server: &Res<AssetServer>) {
        // 刪除所有hand
        self.hands_handler.reset(commands);

        // 新增一個空hand
        self.hands_handler.push_blank_hand(commands, assert_server);
    }

    pub fn split(
        &mut self,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        q_text: &mut Query<(&mut Text, Entity)>,
        hand_index: usize,
    ) {
        self.hands_handler
            .split(commands, asset_server, q_text, hand_index);
    }
}

impl ResFrameworkHandler {
    pub fn player_draw_new_card(
        &mut self,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        q_text: &mut Query<(&mut Text, Entity)>,
        hand_index: usize,
        card: ECard,
    ) {
        if let Some(player_handler) = &mut self.player_handler {
            player_handler.draw_new_card(commands, asset_server, q_text, hand_index, card, true);
        } else {
            error!("player_handler not constructured!");
        }
    }

    pub fn dealer_draw_new_card(
        &mut self,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        q_text: &mut Query<(&mut Text, Entity)>,
        card: ECard,
        is_revealed: bool,
    ) {
        if let Some(dealer_handler) = &mut self.dealer_handler {
            dealer_handler.draw_new_card(commands, asset_server, q_text, 0, card, is_revealed);
        } else {
            error!("dealer_handler not constructured!");
        }
    }

    pub fn infobar_set_new_info(
        &mut self,
        q_text: &mut Query<(&mut Text, Entity)>,
        new_info: String,
    ) {
        if let Some(infobar_handler) = &mut self.infobar_handler {
            infobar_handler.set_value(q_text, new_info);
        } else {
            error!("infobar_handler not constructured!");
        }
    }

    pub fn player_split(
        &mut self,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        q_text: &mut Query<(&mut Text, Entity)>,
        hand_index: usize,
        card1: ECard,
        card2: ECard,
    ) {
        if let Some(player_handler) = &mut self.player_handler {
            player_handler.split(commands, asset_server, q_text, hand_index);
            player_handler.draw_new_card(commands, asset_server, q_text, hand_index, card1, true);
            player_handler.draw_new_card(
                commands,
                asset_server,
                q_text,
                hand_index + 1,
                card2,
                true,
            );
        } else {
            error!("player_handler not constructured!");
        }
    }

    pub fn set_focus(&mut self, focus: Focus) {
        self.focus = focus;
    }

    pub fn dealer_reveal_card(
        &mut self,
        assert_server: &Res<AssetServer>,
        q_img: &mut Query<(&mut UiImage, &Parent)>,
    ) {
        if let Some(dealer_handler) = &mut self.dealer_handler {
            let cards_len =  dealer_handler.hands_handler.hand_handler_list.get(0).unwrap().cards_list.len();
            if cards_len == 2 {
                dealer_handler.reveal_card(assert_server, q_img);
            }
        } else {
            error!("dealer_handler not constructured!");
        }
    }
}
