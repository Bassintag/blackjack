use crate::card::{Card, Rank};

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct Hand {
    size: u8,
    value: u8,
    soft_aces: u8,
    is_pair: bool,
}

pub enum Outcome {
    Win,
    Push,
    Lose,
}

impl Hand {
    pub fn new() -> Self {
        Self {
            size: 0,
            value: 0,
            soft_aces: 0,
            is_pair: false,
        }
    }

    pub fn add_card(&mut self, card: Card) {
        let value = card.rank.value();
        self.is_pair = self.size == 1 && value == self.value;
        self.size += 1;
        self.value += card.rank.value();
        if card.rank == Rank::Ace {
            self.soft_aces += 1;
        }

        while self.value > 21 && self.soft_aces > 0 {
            self.value -= 10;
            self.soft_aces -= 1;
        }
    }

    pub fn split(&mut self) {
        self.value /= 2;
        self.size = 1;
        self.is_pair = false;
    }

    pub fn value(&self) -> u8 {
        self.value
    }

    pub fn is_bust(&self) -> bool {
        self.value > 21
    }

    pub fn is_blackjack(&self) -> bool {
        self.size == 2 && self.value == 21
    }

    pub fn is_soft(&self) -> bool {
        self.soft_aces > 0
    }

    pub fn is_pair(&self) -> bool {
        self.is_pair
    }

    pub fn hard_from_value(target: u8) -> Self {
        Self {
            size: 2,
            value: target,
            soft_aces: 0,
            is_pair: false,
        }
    }

    pub fn soft_from_value(target: u8) -> Self {
        Self {
            size: 2,
            value: target,
            soft_aces: 1,
            is_pair: false,
        }
    }

    pub fn pair_from_single_value(value: u8) -> Self {
        let mut hand = Self::new();
        let card = Card::from_rank(Rank::from_value(value));
        hand.add_card(card);
        hand.add_card(card);
        hand.is_pair = true;
        hand
    }

    pub fn compare(player: &Hand, dealer: &Hand) -> Outcome {
        if player.is_blackjack() && !dealer.is_blackjack() {
            return Outcome::Win;
        }

        if dealer.is_blackjack() && !player.is_blackjack() {
            return Outcome::Lose;
        }

        let player_value = player.value();
        let dealer_value = dealer.value();

        if player.is_bust() {
            Outcome::Lose
        } else if dealer.is_bust() {
            Outcome::Win
        } else if player_value > dealer_value {
            Outcome::Win
        } else if player_value < dealer_value {
            Outcome::Lose
        } else {
            Outcome::Push
        }
    }
}
