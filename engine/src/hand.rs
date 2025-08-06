use crate::card::{Card, Rank};

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct Hand {
    size: u8,
    value: u8,
    aces: u8,
    soft_aces: u8,
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
            aces: 0,
            soft_aces: 0,
        }
    }

    pub fn add_card(&mut self, card: &Card) {
        let value = card.rank.value();
        self.size += 1;
        self.value += value;
        if card.rank == Rank::Ace {
            self.aces += 1;
            self.soft_aces += 1;
        }

        if self.value > 21 && self.soft_aces > 0 {
            self.value -= 10;
            self.soft_aces -= 1;
        }
    }

    pub fn remove_card(&mut self, card: &Card) {
        let value = card.rank.value();

        self.size -= 1;

        if card.rank == Rank::Ace {
            self.aces -= 1;
            if self.soft_aces > 0 && self.value >= 11 {
                self.soft_aces -= 1;
                self.value -= 11;
            } else {
                self.value -= 1;
            }
        } else {
            self.value -= value;
        }

        if self.value <= 11 && self.aces > self.soft_aces {
            self.value += 10;
            self.soft_aces += 1;
        }
    }

    pub fn split(&mut self) {
        self.value /= 2;
        self.size = 1;
    }

    pub fn unsplit(&mut self) {
        self.value *= 2;
        self.size = 2;
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

    pub fn hard_from_value(target: u8) -> Self {
        Self {
            size: 2,
            value: target,
            soft_aces: 0,
            aces: 0,
        }
    }

    pub fn soft_from_value(target: u8) -> Self {
        Self {
            size: 2,
            value: target,
            soft_aces: 1,
            aces: if target == 12 { 2 } else { 1 },
        }
    }

    pub fn pair_from_single_value(value: u8) -> Self {
        let mut hand = Self::new();
        let card = Card::from_rank(Rank::from_value(value));
        hand.add_card(&card);
        hand.add_card(&card);
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
