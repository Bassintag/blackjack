use crate::{card::Card, hand::Hand, shoe::Shoe};

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct GameState<S: Shoe> {
    pub player_hand: Hand,
    pub dealer_upcard: Card,
    pub shoe: S,
    pub splits: u8,
}

impl<S: Shoe> GameState<S> {
    pub fn split(&mut self) {
        self.player_hand.split();
        self.splits += 1;
    }

    pub fn unsplit(&mut self) {
        self.player_hand.unsplit();
        self.splits -= 1;
    }
}
