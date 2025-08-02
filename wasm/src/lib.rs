use engine::{card, hand::Hand, rules::Rules, shoe::InfiniteShoe, strategy};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
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

#[wasm_bindgen]
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

#[wasm_bindgen]
pub struct StrategyGenerator {
    inner: strategy::StrategyGenerator<InfiniteShoe>,
}

#[wasm_bindgen]
impl StrategyGenerator {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let rules = Rules::default();
        let shoe = InfiniteShoe::new();
        let strategy = strategy::StrategyGenerator::new(rules, shoe);
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
        return evs.best().into();
    }
}
