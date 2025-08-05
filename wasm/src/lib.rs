use engine::{card, hand::Hand, rules, shoe::InfiniteShoe, strategy};
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::*;

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl From<Rank> for card::Rank {
    fn from(value: Rank) -> Self {
        match value {
            Rank::Two => Self::Two,
            Rank::Three => Self::Three,
            Rank::Four => Self::Four,
            Rank::Five => Self::Five,
            Rank::Six => Self::Six,
            Rank::Seven => Self::Seven,
            Rank::Eight => Self::Eight,
            Rank::Nine => Self::Nine,
            Rank::Ten => Self::Ten,
            Rank::Jack => Self::Jack,
            Rank::Queen => Self::Queen,
            Rank::King => Self::King,
            Rank::Ace => Self::Ace,
        }
    }
}

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum PlayerAction {
    Hit,
    Stand,
    DoubleOrHit,
    DoubleOrStand,
    Split,
}

impl From<strategy::PlayerAction> for PlayerAction {
    fn from(value: strategy::PlayerAction) -> Self {
        match value {
            strategy::PlayerAction::Hit => Self::Hit,
            strategy::PlayerAction::Stand => Self::Stand,
            strategy::PlayerAction::DoubleOrHit => Self::DoubleOrHit,
            strategy::PlayerAction::DoubleOrStand => Self::DoubleOrStand,
            strategy::PlayerAction::Split => Self::Split,
        }
    }
}

#[derive(Tsify, Serialize, Deserialize)]
pub enum Soft17Rule {
    Hit,
    Stand,
}

impl From<Soft17Rule> for rules::Soft17Rule {
    fn from(value: Soft17Rule) -> Self {
        match value {
            Soft17Rule::Hit => Self::Hit,
            Soft17Rule::Stand => Self::Stand,
        }
    }
}

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Rules {
    #[serde(rename = "numDecks")]
    pub num_decks: u8,
    #[serde(rename = "dealerSoft17")]
    pub dealer_soft_17: Soft17Rule,
    #[serde(rename = "doubleAfterSplitAllowed")]
    pub double_after_split_allowed: bool,
    #[serde(rename = "surrenderAllowed")]
    pub surrender_allowed: bool,
    #[serde(rename = "maxSplits")]
    pub max_splits: u8,
}

impl From<Rules> for rules::Rules {
    fn from(value: Rules) -> Self {
        Self {
            num_decks: value.num_decks,
            dealer_soft_17: value.dealer_soft_17.into(),
            double_after_split_allowed: value.double_after_split_allowed,
            surrender_allowed: value.surrender_allowed,
            max_splits: value.max_splits,
        }
    }
}

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct RoundEvs {
    pub hit: f64,
    pub stand: f64,
    pub double: f64,
    pub split: Option<f64>,
}

impl From<strategy::RoundEvs> for RoundEvs {
    fn from(value: strategy::RoundEvs) -> Self {
        Self {
            hit: value.hit,
            stand: value.stand,
            double: value.double,
            split: value.split,
        }
    }
}

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct StrategyValue {
    pub ev: f64,
    pub evs: RoundEvs,
    pub action: PlayerAction,
}

impl From<strategy::StrategyValue> for StrategyValue {
    fn from(value: strategy::StrategyValue) -> Self {
        Self {
            ev: value.ev,
            evs: value.evs.into(),
            action: value.action.into(),
        }
    }
}

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct StrategyTable {
    pub values: Vec<StrategyValue>,
    pub from: u8,
    pub to: u8,
}

impl From<strategy::StrategyTable> for StrategyTable {
    fn from(value: strategy::StrategyTable) -> Self {
        Self {
            values: value.values.iter().map(|v| (*v).into()).collect(),
            from: value.from,
            to: value.to,
        }
    }
}

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct StrategyTables {
    pub hard: StrategyTable,
    pub soft: StrategyTable,
    pub pair: StrategyTable,
}

#[wasm_bindgen]
pub struct StrategyGenerator {
    inner: strategy::StrategyGenerator<InfiniteShoe>,
}

#[wasm_bindgen]
impl StrategyGenerator {
    #[wasm_bindgen(constructor)]
    pub fn new(rules: Rules) -> Self {
        let shoe = InfiniteShoe::new();
        let strategy = strategy::StrategyGenerator::new(rules.into(), shoe);
        Self { inner: strategy }
    }

    #[wasm_bindgen]
    pub fn action(&mut self, player: Vec<Rank>, dealer: Rank) -> PlayerAction {
        let mut hand = Hand::new();
        for value in player {
            hand.add_card(&card::Card::from_rank(value.into()));
        }
        let dealer_upcard = card::Card::from_rank(dealer.into());
        let evs = self.inner.eval_round(hand, dealer_upcard, false);

        evs.best().0.into()
    }

    #[wasm_bindgen]
    pub fn tables(&mut self) -> StrategyTables {
        let hard = self.inner.hard_table();
        let soft = self.inner.soft_table();
        let pair = self.inner.pair_table();

        StrategyTables {
            hard: hard.into(),
            soft: soft.into(),
            pair: pair.into(),
        }
    }
}
