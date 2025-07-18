use crate::{card::Card, hand::Hand, shoe::Shoe};

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct GameState<S: Shoe> {
    pub player_hand: Hand,
    pub dealer_upcard: Card,
    pub shoe: S,
}
