use std::iter;

use crate::{
    card::{Card, Rank},
    hand::Hand,
};

pub enum HandType {
    Hard,
    Soft,
}

pub trait Shoe {
    fn iter_draws(&self) -> impl Iterator<Item = (Card, f64)>;

    fn iter_player_hands(
        &self,
        player_value: u8,
        hand_type: HandType,
    ) -> impl Iterator<Item = (Hand, f64)>;

    fn remove_card(&mut self, card: &Card) -> ();
}

const CARD_PROBABILITIES: [(Rank, f64); 10] = [
    (Rank::Two, 1.0 / 13.0),
    (Rank::Three, 1.0 / 13.0),
    (Rank::Four, 1.0 / 13.0),
    (Rank::Five, 1.0 / 13.0),
    (Rank::Six, 1.0 / 13.0),
    (Rank::Seven, 1.0 / 13.0),
    (Rank::Eight, 1.0 / 13.0),
    (Rank::Nine, 1.0 / 13.0),
    (Rank::Ten, 4.0 / 13.0),
    (Rank::Ace, 1.0 / 13.0),
];

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct InfiniteShoe;

impl Shoe for InfiniteShoe {
    fn iter_draws(&self) -> impl Iterator<Item = (Card, f64)> {
        CARD_PROBABILITIES
            .iter()
            .map(|(rank, probability)| (Card::from_rank(*rank), *probability))
    }

    fn iter_player_hands(
        &self,
        player_value: u8,
        soft: HandType,
    ) -> impl Iterator<Item = (Hand, f64)> {
        let hand = match soft {
            HandType::Hard => Hand::hard_from_value(player_value),
            HandType::Soft => Hand::soft_from_value(player_value),
        };

        iter::once((hand, 1.0))
    }

    fn remove_card(&mut self, _card: &Card) -> () {}
}

impl InfiniteShoe {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct CountShoe {
    counts: [usize; 10],
    total: usize,
}

impl Shoe for CountShoe {
    fn iter_draws(&self) -> impl Iterator<Item = (Card, f64)> {
        let total = self.total as f64;
        self.counts
            .iter()
            .filter(|&&count| count > 0)
            .enumerate()
            .map(move |(i, &count)| {
                let card = Card::from_rank(Rank::from_value((i as u8) + 2));
                (card, count as f64 / total)
            })
    }

    fn iter_player_hands(
        &self,
        _player_value: u8,
        _hand_type: HandType,
    ) -> impl Iterator<Item = (Hand, f64)> {
        let vec = Vec::new();

        vec.into_iter()
    }

    fn remove_card(&mut self, card: &Card) -> () {
        let i = (card.rank.value() - 2) as usize;
        if self.counts[i] > 0 {
            self.counts[i] -= 1;
            self.total -= 1;
        }
    }
}

impl CountShoe {
    pub fn new(decks: u8) -> Self {
        let mut counts = [0; 10];

        for i in 0..10 {
            counts[i] = (decks as usize) * 4;
        }
        counts[8] = (decks as usize) * 16;

        Self {
            total: decks as usize * 52,
            counts,
        }
    }
}
